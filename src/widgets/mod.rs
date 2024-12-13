pub mod basic;
pub mod containers;

pub trait Widget {
    fn draw(&self);
    fn handle_event(&self);
}
