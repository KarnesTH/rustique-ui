use super::bindings::*;
use crate::core::event::{Event, WindowEvent};
use std::ffi::c_void;

pub struct EventHandler {
    display: *mut c_void,
}

impl EventHandler {
    pub fn new(display: *mut c_void) -> Self {
        Self { display }
    }

    pub fn handle_x11_event(&self, event: &XEvent) -> Option<Event> {
        match event.type_ {
            DESTROY_NOTIFY => Some(Event::Window(WindowEvent::Close)),
            CLIENT_MESSAGE => {
                let client_message =
                    unsafe { &*(event as *const XEvent as *const XClientMessageEvent) };
                self.handle_client_message(client_message)
            }
            _ => None,
        }
    }

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
