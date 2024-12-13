//! Core functionality for the Rustique UI library.
//!
//! This module contains the core functionality for the Rustique UI library.
//! It provides the main application trait and the configuration for window creation.

pub mod event;
pub mod render;

/// Main application trait.
///
/// Provides the core functionality for running a GUI application,
/// including the main event loop and lifecycle management.
///
/// Note: This trait will be implemented in a future update to provide
/// a complete application lifecycle management system.
pub trait Application {
    fn run(&self);
}

/// Configuration for window creation.
///
/// This struct contains the configuration for creating a new window.
/// It includes the title of the window and the dimensions of the window.
///
/// # Example
///
/// ```
/// use rustique_ui::core::Config;
/// let config = Config {
///    title: "My Window".to_string(),
///    width: 800,
///    height: 600,
/// };
/// ```
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
