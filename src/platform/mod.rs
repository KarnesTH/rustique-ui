pub mod x11;

pub trait Window {
    fn new() -> Self
    where
        Self: Sized;
    fn show(&self);
    fn hide(&self);
}
