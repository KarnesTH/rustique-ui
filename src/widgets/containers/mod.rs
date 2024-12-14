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
    /// Create a new container.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::containers::Container;
    /// let container = Container::new();
    /// ```
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

    /// Set the position of the container.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the container.
    /// * `y` - The y-coordinate of the container.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::containers::Container;
    /// let container = Container::new().position(10.0, 20.0);
    /// ```
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the size of the container.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the container.
    /// * `height` - The height of the container.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::containers::Container;
    /// let container = Container::new().size(200.0, 100.0);
    /// ```
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the layout of the container.
    ///
    /// # Arguments
    ///
    /// * `layout` - The layout of the container.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::containers::{Container, Layout};
    /// let container = Container::new().layout(Layout::Horizontal);
    /// ```
    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Add a child widget to the container.
    ///
    /// # Arguments
    ///
    /// * `child` - The child widget to add to the container.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rustique_ui::widgets::containers::Container;
    /// use rustique_ui::widgets::basic::Label;
    /// let container = Container::new().add_child(Box::new(Label::new("Hello, world!")));
    /// ```
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
