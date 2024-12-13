pub mod event;
pub mod render;

pub trait Application {
    fn run(&self);
}
