use super::bindings::*;
use super::events::EventHandler;
use super::glx::GLXContext;
use crate::core::render::Renderer;
use crate::core::{event::Event, Config};
use crate::platform::Window;
use std::ffi::c_void;

pub struct X11Window {
    pub display: *mut c_void,
    pub window: u64,
    pub config: Config,
    pub event_handler: EventHandler,
    pub gl_context: GLXContext,
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

            XStoreName(display, window, config.title.as_ptr() as *const i8);

            let wm_delete_window = XInternAtom(display, WM_DELETE_WINDOW.as_ptr() as *const i8, 0);
            XSetWMProtocols(display, window, &wm_delete_window, 1);

            XSelectInput(display, window, STRUCTURE_NOTIFY_MASK | EXPOSURE_MASK);

            XFlush(display);

            let gl_context = GLXContext::new(display, window);
            gl_context.make_current();

            gl::load_with(|s| super::glx::get_proc_address(s));

            Self {
                display,
                window,
                config,
                event_handler: EventHandler::new(display),
                gl_context,
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
        self.gl_context.swap_buffers();
    }
}

impl Renderer for X11Window {
    fn clear(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn present(&mut self) {
        self.gl_context.swap_buffers();
    }

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        unimplemented!()
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        unimplemented!()
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
