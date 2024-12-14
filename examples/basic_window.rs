use event::{Event, WindowEvent};
use rustique_ui::{platform::x11::window::X11Window, prelude::*};

fn main() {
    let config = Config {
        title: "Basic Window".to_string(),
        width: 800,
        height: 600,
    };

    let mut window = X11Window::new(config);
    window.show();

    'main: loop {
        let events = window.handle_events();
        for event in events {
            match event {
                Event::Window(WindowEvent::Close) => {
                    break 'main;
                }
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
        window.swap_buffers();
    }
}
