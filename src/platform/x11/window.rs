use super::bindings::*;
use super::events::EventHandler;
use crate::core::{event::Event, Config};
use crate::platform::Window;
use std::ffi::c_void;

pub struct X11Window {
    pub display: *mut c_void,
    pub window: u64,
    pub config: Config,
    pub event_handler: EventHandler,
}

impl Window for X11Window {
    fn new(config: Config) -> Self {
        unsafe {
            let display = XOpenDisplay(std::ptr::null());
            if display.is_null() {
                panic!("Failed to open X11 display");
            }

            let screen = XDefaultScreen(display);
            let root = XDefaultRootWindow(display);
            let black = XBlackPixel(display, screen);
            let white = XWhitePixel(display, screen);

            let window = XCreateSimpleWindow(
                display,
                root,
                0,
                0,
                config.width,
                config.height,
                1,
                black,
                white,
            );

            let wm_delete_window = XInternAtom(display, WM_DELETE_WINDOW.as_ptr() as *const i8, 0);
            XSetWMProtocols(display, window, &wm_delete_window, 1);

            XSelectInput(display, window, STRUCTURE_NOTIFY_MASK | EXPOSURE_MASK);

            XFlush(display);

            Self {
                display,
                window,
                config,
                event_handler: EventHandler::new(display),
            }
        }
    }

    fn show(&self) {
        unsafe {
            XMapWindow(self.display, self.window);
            XFlush(self.display);
        }
    }

    fn hide(&self) {
        unimplemented!()
    }

    fn handle_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        unsafe {
            while XPending(self.display) > 0 {
                let mut x_event: XEvent = std::mem::zeroed();
                XNextEvent(self.display, &mut x_event);

                if let Some(event) = self.event_handler.handle_x11_event(&x_event) {
                    events.push(event);
                }
            }
        }
        events
    }

    fn swap_buffers(&self) {
        unsafe {
            XFlush(self.display);
        }
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            XDestroyWindow(self.display, self.window);
            XCloseDisplay(self.display);
        }
    }
}
