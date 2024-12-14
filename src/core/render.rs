pub trait Renderer {
    fn clear(&mut self);
    fn present(&mut self);
    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32);
    fn draw_text(&mut self, text: &str, x: f32, y: f32);
}

pub trait GraphicsContext {
    fn init() -> Self
    where
        Self: Sized;
    fn make_current(&self);
    fn swap_buffers(&self);
}
