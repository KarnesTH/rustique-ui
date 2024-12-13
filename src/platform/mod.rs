//! Platform specific window and rendering implementations.

pub mod x11;

use crate::core::event::Event;
use crate::core::Config;

/// Platform agnostic window trait.
pub trait Window {
    /// Create a new window with the given configuration.
    fn new(config: Config) -> Self
    where
        Self: Sized;
    /// Show the window.
    fn show(&self);
    /// Hide the window.
    fn hide(&self);
    /// Handle events and return a list of events that occurred.
    fn handle_events(&mut self) -> Vec<Event>;
    /// Swap the buffers of the window.
    fn swap_buffers(&self);
}
