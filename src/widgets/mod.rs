//! Widget system for building user interfaces.

pub mod basic;
pub mod containers;

use crate::core::event::Event;
use crate::core::render::Renderer;

/// Trait for all widgets.
pub trait Widget {
    /// Draw the widget using the provided renderer.
    fn draw(&self, renderer: &mut dyn Renderer);
    /// Handle an event and return true if the event was consumed.
    fn handle_event(&self, event: &Event) -> bool;
    /// Get the children of the widget.
    fn get_children(&self) -> Vec<&dyn Widget>;
    /// Get the mutable children of the widget.
    fn get_children_mut(&mut self) -> Vec<&mut dyn Widget>;
}
