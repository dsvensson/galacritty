#![allow(clippy::redundant_field_names)]
use gtk::prelude::*;
use gtk::BoxBuilder;
use gtk::Inhibit;
use gtk::{GtkSocketExt, NotebookExt, WidgetExt};
use relm::{connect, Relm};
use relm::{init, Component, Widget};
use relm_derive::{widget, Msg};

use crate::header;
use crate::header::Header;

pub struct Model {
    header: Component<Header>,
    relm: Relm<Win>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Add,
    PageCountChanged,
    SwitchPage(u32),
    PlugAdded(u32),
    PlugRemoved(u32),
}

#[widget]
impl Widget for Win {
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let header = &self.model.header;
        connect!(header@header::Msg::Add, relm, Msg::Add);

        // Create initial terminal
        relm.stream().emit(Msg::Add);
    }

    fn model(relm: &Relm<Win>, _param: ()) -> Model {
        Model {
            header: init::<Header>(()).expect("Header"),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::PageCountChanged => match self.notebook.get_n_pages() {
                0 => gtk::main_quit(),
                1 => self.notebook.set_show_tabs(false),
                _ => self.notebook.set_show_tabs(true),
            },
            Msg::SwitchPage(num) => {
                if let Some(page) = self.notebook.get_nth_page(Some(num)) {
                    if let Some(container) = page.downcast_ref::<gtk::Container>() {
                        container.foreach(|w| {
                            if let Some(socket) = w.downcast_ref::<gtk::Socket>() {
                                socket.grab_focus();
                            }
                        });
                    }
                }
            }
            Msg::PlugAdded(num) => {
                self.notebook.set_current_page(Some(num));
            }
            Msg::PlugRemoved(num) => {
                self.notebook.remove_page(Some(num));
            }
            Msg::Add => {
                let socket = gtk::SocketBuilder::new()
                    .can_focus(true)
                    .expand(true)
                    .build();

                let container = BoxBuilder::new().child(&socket).build();
                container.show_all();

                let num = self.notebook.append_page(
                    &container,
                    Some(&gtk::LabelBuilder::new().label("Terminal").build()),
                );

                connect!(
                    self.model.relm,
                    socket,
                    connect_plug_added(_),
                    Msg::PlugAdded(num)
                );

                connect!(
                    self.model.relm,
                    socket,
                    connect_plug_removed(_),
                    return (Msg::PlugRemoved(num), false)
                );

                socket.grab_focus();

                glib::spawn_command_line_async(format!("alacritty --embed {}", socket.get_id()))
                    .unwrap();
            }
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            titlebar: Some(self.model.header.widget()),

            property_default_height: 650,
            property_default_width: 1000,

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),

            #[name="notebook"]
            gtk::Notebook {
                show_tabs: false,
                page_added(_, _, _) => Msg::PageCountChanged,
                page_removed(_, _, _) => Msg::PageCountChanged,
                switch_page(_, _, num) => Msg::SwitchPage(num),
            }
        }
    }
}
