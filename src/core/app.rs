use super::{render::context::RenderContext, window::Window};

pub struct App {
    pub window: Option<Window>,
    pub render_context: Option<RenderContext>,
}

impl App {
    pub fn new() -> Self {
        App {
            window: None,
            render_context: None,
        }
    }

    pub fn window(mut self, title: &str, width: u32, height: u32) -> Self {
        self.window = Some(Window::new(title, width, height));
        self
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(window) = &self.window {
            self.render_context = Some(RenderContext::new(window)?);
        }

        loop {
            // TODO: Handle events, update, and render
        }

        Ok(())
    }
}
