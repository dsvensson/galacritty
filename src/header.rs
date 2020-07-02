#![allow(clippy::redundant_field_names)]
use gtk::prelude::*;
use relm::Widget;
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {
    Add,
}

#[widget]
impl Widget for Header {
    fn model() {}

    fn update(&mut self, _event: Msg) {}

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            show_close_button: true,

            gtk::ToolButton {
                clicked => Msg::Add,
                icon_name: Some("tab-new-symbolic"),
            },
        }
    }
}
