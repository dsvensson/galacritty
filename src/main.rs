use relm::Widget;

mod header;
mod window;

fn main() {
    crate::window::Win::run(()).expect("Window::run");
}
