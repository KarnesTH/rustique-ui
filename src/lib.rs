pub mod core;
pub mod platform;
pub mod widgets;

pub mod prelude {
    pub use crate::core::*;
    pub use crate::platform::Window;
    pub use crate::widgets::*;
}
