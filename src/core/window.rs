use crate::platform;

pub struct Window {
    pub(crate) handle: platform::WindowHandle,
    pub size: (u32, u32),
    pub title: String,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let handle = platform::create_window(title, width, height);
        Window {
            handle,
            size: (width, height),
            title: title.to_string(),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}
