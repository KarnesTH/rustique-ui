//! # Rustique UI
//!
//! A modern GUI framework written in pure Rust.
//!
//! This framework provides a platform-agnostic API for creating native GUI applications
//! with hardware-accelerated rendering.

pub mod core;
pub mod platform;
pub mod widgets;

pub mod prelude {
    pub use crate::core::*;
    pub use crate::platform::Window;
    pub use crate::widgets::*;
}
