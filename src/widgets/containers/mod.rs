use super::Widget;

pub struct Container {
    pub children: Vec<Box<dyn Widget>>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub layout: Layout,
}

impl Container {
    pub fn new() -> Self {
        Container {
            children: Vec::new(),
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            layout: Layout::Vertical,
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

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn add_child(mut self, child: Box<dyn Widget>) -> Self {
        self.children.push(child);
        self
    }
}

pub enum Layout {
    Vertical,
    Horizontal,
    Grid { clumns: usize },
}
