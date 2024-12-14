use std::ffi::c_void;

#[link(name = "X11")]
extern "C" {
    pub fn XOpenDisplay(display_name: *const i8) -> *mut c_void;
    pub fn XDefaultRootWindow(display: *mut c_void) -> u64;
    pub fn XCreateSimpleWindow(
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
    pub fn XMapWindow(display: *mut c_void, window: u64);
    pub fn XFlush(display: *mut c_void);
    pub fn XDefaultScreen(display: *mut c_void) -> i32;
    pub fn XBlackPixel(display: *mut c_void, screen: i32) -> u64;
    pub fn XWhitePixel(display: *mut c_void, screen: i32) -> u64;
    pub fn XPending(display: *mut c_void) -> i32;
    pub fn XNextEvent(display: *mut c_void, event_return: *mut XEvent) -> i32;
    pub fn XDestroyWindow(display: *mut c_void, window: u64);
    pub fn XCloseDisplay(display: *mut c_void) -> i32;
    pub fn XSelectInput(display: *mut c_void, window: u64, mask: i64) -> i32;
    pub fn XInternAtom(display: *mut c_void, atom_name: *const i8, only_if_exists: i32) -> u64;
    pub fn XSetWMProtocols(
        display: *mut c_void,
        window: u64,
        protocols: *const u64,
        count: i32,
    ) -> i32;
}

#[repr(C)]
pub struct XEvent {
    pub type_: i32,
    pub pad: [u8; 192],
}

#[repr(C)]
pub struct XClientMessageEvent {
    pub type_: i32,
    pub serial: u64,
    pub send_event: i32,
    pub display: *mut c_void,
    pub window: u64,
    pub message_type: u64,
    pub format: i32,
    pub data: [i64; 5],
}

pub const CLIENT_MESSAGE: i32 = 33;
pub const DESTROY_NOTIFY: i32 = 17;
pub const STRUCTURE_NOTIFY_MASK: i64 = 1 << 17;
pub const EXPOSURE_MASK: i64 = 1 << 15;
pub const WM_DELETE_WINDOW: &str = "WM_DELETE_WINDOW\0";
