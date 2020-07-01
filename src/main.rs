use gtk::prelude::*;
use gtk::BoxBuilder;
use gtk::Inhibit;
use gtk::Orientation::Vertical;
use gtk::{Bin, Box, GtkSocketExt, NotebookExt, StackExt, WidgetExt};
use relm::{connect, Relm};
use relm::{init, Component, Widget};
use relm_derive::{widget, Msg};

// GtkNotebook in headerbar
// https://gist.github.com/cristian99garcia/21cbccc3fd00749e40f22a294d89767f

// Gtk XEmbed
// https://gist.github.com/bert/809126
// https://github.com/ThomasAdam/Gtkabber/blob/master/gtkabber.c

// window title perhaps... at least some xembed x11 window title property
// https://stackoverflow.com/questions/44833160/how-do-i-get-the-x-window-class-given-a-window-id-with-rust-xcb

#[derive(Msg)]
pub enum HeaderMsg {
    Add,
}

#[widget]
impl Widget for Header {
    fn model() -> () {}

    fn update(&mut self, event: HeaderMsg) {
        match event {
            HeaderMsg::Add => {}
        }
    }

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            title: Some("Title"),
            show_close_button: true,

            #[name="add_tab_button"]
            gtk::ToolButton {
                clicked => HeaderMsg::Add,
                icon_name: Some("tab-new-symbolic"),
            },
        }
    }
}

pub struct Model {
    header: Component<Header>,
    relm: Relm<Win>,
}

#[derive(Msg)]
pub enum WinMsg {
    Quit,
    Add,
    PageAdded(gtk::Widget),
    SwitchPage(gtk::Widget),
    PlugAdded(u32),
    PlugRemoved(u32),
    Hello,
}

#[widget]
impl Widget for Win {
    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let header = &self.model.header;
        connect!(header@HeaderMsg::Add, relm, WinMsg::Add);
    }

    fn model(relm: &Relm<Win>, _param: ()) -> Model {
        Model {
            header: init::<Header>(()).expect("Header"),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            WinMsg::Hello => {
                println!("hello latest");
            }
            WinMsg::PageAdded(widget) => {
                self.notebook.set_show_tabs(self.notebook.get_n_pages() > 1);
            }
            WinMsg::SwitchPage(widget) => {
                println!("switch page");
                widget.grab_focus();
            }
            WinMsg::PlugAdded(num) => {
                self.notebook.set_current_page(Some(num));
            }
            WinMsg::PlugRemoved(num) => {
                self.notebook.remove_page(Some(num));
                if self.notebook.get_n_pages() == 0 {
                    gtk::main_quit();
                }
            }
            WinMsg::Add => {
                let socket = gtk::SocketBuilder::new()
                    .can_focus(true)
                    .expand(true)
                    .build();

                let container = BoxBuilder::new().child(&socket).build();
                container.show_all();

                let num = self.notebook.append_page(
                    &container,
                    Some(&gtk::LabelBuilder::new().label("hello").build()),
                );

                connect!(
                    self.model.relm,
                    socket,
                    connect_plug_added(_),
                    WinMsg::PlugAdded(num)
                );

                connect!(
                    self.model.relm,
                    socket,
                    connect_plug_removed(_),
                    return (WinMsg::PlugRemoved(num), false)
                );

                socket.grab_focus();

                glib::spawn_command_line_async(format!("alacritty --embed {}", socket.get_id()))
                    .unwrap();
            }
            WinMsg::Quit => gtk::main_quit(),
        }
    }

    view! {
        #[name="window"]
        gtk::Window {
            titlebar: Some(self.model.header.widget()),
            title: "",

            property_default_height: 650,
            property_default_width: 1000,

            delete_event(_, _) => (WinMsg::Quit, Inhibit(false)),

            #[name="notebook"]
            gtk::Notebook {
                hexpand: true,
                show_tabs: false,
                page_added(_, widget, _) => WinMsg::PageAdded(widget.clone()),
                switch_page(_, widget, _) => WinMsg::SwitchPage(widget.clone()),
            }
        }
    }
}

fn main() {
    Win::run(()).expect("Window::run");
}
