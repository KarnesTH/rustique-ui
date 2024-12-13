use crate::core::event::{Event, WindowEvent};
use crate::core::Config;
use crate::platform::Window;
use std::ffi::c_void;

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(display_name: *const i8) -> *mut c_void;
    fn XDefaultRootWindow(display: *mut c_void) -> u64;
    fn XCreateSimpleWindow(
        display: *mut c_void,
        parent: u64,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        background: u64,
    ) -> u64;
    fn XMapWindow(display: *mut c_void, window: u64);
    fn XFlush(display: *mut c_void);
    fn XDefaultScreen(display: *mut c_void) -> i32;
    fn XBlackPixel(display: *mut c_void, screen: i32) -> u64;
    fn XWhitePixel(display: *mut c_void, screen: i32) -> u64;
    fn XPending(display: *mut c_void) -> i32;
    fn XNextEvent(display: *mut c_void, event_return: *mut XEvent) -> i32;
    fn XDestroyWindow(display: *mut c_void, window: u64);
    fn XCloseDisplay(display: *mut c_void) -> i32;
    fn XSelectInput(display: *mut c_void, window: u64, mask: i64) -> i32;
    fn XInternAtom(display: *mut c_void, atom_name: *const i8, only_if_exists: i32) -> u64;
    fn XSetWMProtocols(display: *mut c_void, window: u64, protocols: *const u64, count: i32)
        -> i32;
}

#[repr(C)]
struct XEvent {
    type_: i32,
    pad: [u8; 192],
}

#[repr(C)]
struct XClientMessageEvent {
    type_: i32,
    serial: u64,
    send_event: i32,
    display: *mut c_void,
    window: u64,
    message_type: u64,
    format: i32,
    data: [i64; 5],
}

const CLIENT_MESSAGE: i32 = 33;
const DESTROY_NOTIFY: i32 = 17;
const STRUCTURE_NOTIFY_MASK: i64 = 1 << 17;
const EXPOSURE_MASK: i64 = 1 << 15;
const WM_DELETE_WINDOW: &str = "WM_DELETE_WINDOW\0";

pub struct X11Window {
    pub display: *mut c_void,
    pub window: u64,
    pub config: Config,
}

impl X11Window {
    fn handle_client_message(&self, event: &XClientMessageEvent) -> Option<Event> {
        unsafe {
            let wm_protocols = XInternAtom(self.display, "WM_PROTOCOLS\0".as_ptr() as *const i8, 1);
            let wm_delete_window =
                XInternAtom(self.display, WM_DELETE_WINDOW.as_ptr() as *const i8, 1);

            if event.message_type == wm_protocols && event.data[0] == wm_delete_window as i64 {
                Some(Event::Window(WindowEvent::Close))
            } else {
                None
            }
        }
    }
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

                match x_event.type_ {
                    DESTROY_NOTIFY => {
                        events.push(Event::Window(WindowEvent::Close));
                    }
                    CLIENT_MESSAGE => {
                        let client_message =
                            &*((&x_event as *const XEvent) as *const XClientMessageEvent);
                        if let Some(event) = self.handle_client_message(client_message) {
                            events.push(event);
                        }
                    }
                    _ => {}
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
