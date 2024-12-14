use std::ffi::c_void;

use super::bindings::XDefaultScreen;

#[repr(C)]
pub struct XVisualInfo {
    pub visual: *mut c_void,
    pub visualid: u64,
    pub screen: i32,
    pub depth: i32,
    pub class: i32,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub colormap_size: i32,
    pub bits_per_rgb: i32,
}

#[link(name = "GL")]
extern "C" {
    pub fn glXChooseVisual(
        display: *mut c_void,
        screen: i32,
        attrib_list: *const i32,
    ) -> *mut XVisualInfo;
    pub fn glXCreateContext(
        display: *mut c_void,
        visual: *mut XVisualInfo,
        share_list: *mut c_void,
        direct: i32,
    ) -> *mut c_void;
    pub fn glXMakeCurrent(display: *mut c_void, drawable: u64, context: *mut c_void) -> i32;
    pub fn glXSwapBuffers(display: *mut c_void, drawable: u64) -> i32;
    pub fn glXDestroyContext(display: *mut c_void, ctx: *mut c_void);
    pub fn glXGetProcAddress(name: *const u8) -> *mut c_void;
}

pub fn get_proc_address(name: &str) -> *mut c_void {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { glXGetProcAddress(name.as_ptr() as *const u8) }
}

pub const GLX_RGBA: i32 = 4;
pub const GLX_DEPTH_SIZE: i32 = 12;
pub const GLX_DOUBLEBUFFER: i32 = 5;

pub struct GLXContext {
    display: *mut c_void,
    window: u64,
    context: *mut c_void,
}

impl GLXContext {
    pub fn new(display: *mut c_void, window: u64) -> Self {
        unsafe {
            let attributes = [GLX_RGBA, GLX_DEPTH_SIZE, 24, GLX_DOUBLEBUFFER, 0];

            let screen = XDefaultScreen(display);
            let visual = glXChooseVisual(display, screen, attributes.as_ptr());

            if visual.is_null() {
                panic!("Failed to choose visual");
            }

            let context = glXCreateContext(display, visual, std::ptr::null_mut(), 1);

            if context.is_null() {
                panic!("Failed to create GL context");
            }

            Self {
                display,
                window,
                context,
            }
        }
    }

    pub fn make_current(&self) -> bool {
        unsafe { glXMakeCurrent(self.display, self.window, self.context) != 0 }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            glXSwapBuffers(self.display, self.window);
        }
    }
}

impl Drop for GLXContext {
    fn drop(&mut self) {
        unsafe {
            glXDestroyContext(self.display, self.context);
        }
    }
}
