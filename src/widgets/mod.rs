pub mod basic;
pub mod containers;

use crate::core::event::Event;

pub trait Widget {
    fn draw(&self, renderer: &mut dyn crate::core::render::Renderer);
    fn handle_event(&self, event: &Event) -> bool;
    fn get_children(&self) -> Vec<&dyn Widget>;
    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget>;
}
