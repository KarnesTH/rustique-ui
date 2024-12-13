pub mod x11;

pub trait Window {
    fn new(config: crate::core::Config) -> Self
    where
        Self: Sized;
    fn show(&self);
    fn hide(&self);
    fn handle_events(&mut self) -> Vec<crate::core::event::Event>;
    fn swap_buffers(&self);
}
