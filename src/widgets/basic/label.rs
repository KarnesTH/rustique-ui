pub struct Label {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            text: text.into(),
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
}
