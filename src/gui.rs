use widget::Widget;

mod button;
mod label;
mod widget;
mod window;

// use widget::Widget;

pub fn gui_run() {
    let mut window = window::Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(label::Label::new(
        "This is a small text GUI demo.",
    )));
    window.add_widget(Box::new(button::Button::new("Click Me!")));
    window.draw();
}
