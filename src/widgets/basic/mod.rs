pub struct Button {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Button {
            text: text.into(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 30.0,
            on_click: None,
        }
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}

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
