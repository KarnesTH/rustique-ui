pub mod event;
pub mod render;

pub trait Application {
    fn run(&self);
}

pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
}
