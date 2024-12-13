pub mod x11;

use crate::core::event::Event;
use crate::core::Config;

pub trait Window {
    fn new(config: Config) -> Self
    where
        Self: Sized;
    fn show(&self);
    fn hide(&self);
    fn handle_events(&mut self) -> Vec<Event>;
    fn swap_buffers(&self);
}
