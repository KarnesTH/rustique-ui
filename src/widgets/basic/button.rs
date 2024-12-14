pub struct Button {
    pub text: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    /// Create a new button with the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to display on the button.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::basic::Button;
    /// let button = Button::new("Click me!");
    /// ```
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

    /// Set the position of the button.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the button.
    /// * `y` - The y-coordinate of the button.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::basic::Button;
    /// let button = Button::new("Click me!").position(10.0, 20.0);
    /// ```
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set the size of the button.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the button.
    /// * `height` - The height of the button.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::basic::Button;
    /// let button = Button::new("Click me!").size(100.0, 30.0);
    /// ```
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the function to be called when the button is clicked.
    ///
    /// # Arguments
    ///
    /// * `f` - The function to call when the button is clicked.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::basic::Button;
    /// let button = Button::new("Click me!").on_click(|| println!("Button clicked!"));
    /// ```
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}
