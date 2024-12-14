pub enum Event {
    WindowClose,
    MouseClick { x: i32, y: i32 },
    KeyPress { key: Key },
}

pub enum Key {}

pub trait EventHandler {
    fn handle_event(&mut self, event: Event);
}
