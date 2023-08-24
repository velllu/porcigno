use layouts::common::Layout;
use undici::x11::{
    common::{MouseButton, Vector2},
    display::Display,
    events::event::EventType,
    window::{Modifier, Window},
};

mod events;
mod layouts;

struct PorcignoWM {
    monitor_size: Vector2<i32>,
    windows: Vec<Window>,
    current_layout: Layout,
}

#[rustfmt::skip]
fn main() {
    // Command: `Xephyr -br -ac -noreset -screen 800x800 :90`
    #[cfg(debug_assertions)]
    std::env::set_var("DISPLAY", ":90");

    let display = Display::new().expect("could not open display");
    let root_window = display.get_root_window();

    root_window.grab_key("l", Modifier::Alt);
    root_window.grab_key("r", Modifier::Alt);
    root_window.grab_mouse_button(MouseButton::Left, Modifier::Alt);
    root_window.grab_mouse_button(MouseButton::Right, Modifier::Alt);
    root_window.grab_children_substucture();

    let mut porcigno = PorcignoWM {
        monitor_size: root_window.get_data().scale,
        windows: Vec::new(),
        current_layout: Layout::Master,
    };

    loop {
        let event = display.get_event();

        match event.type_ {
            EventType::WindowCreated(window_created_event) => {
                porcigno.windows.push(window_created_event.window);
                porcigno.on_window_created();
            }
                
            _ => {}
        }
    }
}
