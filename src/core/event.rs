#[derive(Debug)]
pub enum Event {
    Window(WindowEvent),
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[derive(Debug)]
pub enum WindowEvent {
    Close,
    Resize(u32, u32),
    Focus(bool),
}

#[derive(Debug)]
pub enum MouseEvent {
    Move(i32, i32),
    Press(MouseButton),
    Release(MouseButton),
}

#[derive(Debug)]
pub enum KeyboardEvent {
    Press(Key),
    Release(Key),
}

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug)]
pub struct Key(pub u32);
