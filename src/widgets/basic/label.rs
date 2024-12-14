pub struct Label {
    pub text: String,
    pub x: f32,
    pub y: f32,
}

impl Label {
    /// Create a new label with the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to display on the label.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::basic::Label;
    /// let label = Label::new("Hello, world!");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Label {
            text: text.into(),
            x: 0.0,
            y: 0.0,
        }
    }

    /// Set the position of the label.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the label.
    /// * `y` - The y-coordinate of the label.
    ///
    /// # Example
    ///
    /// ```
    /// use rustique_ui::widgets::basic::Label;
    /// let label = Label::new("Hello, world!").position(10.0, 20.0);
    /// ```
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }
}
