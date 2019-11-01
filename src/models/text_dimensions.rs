use glium::Display;

/// Struct for common values needed for all text that is drawn to the screen. Width determines font size,
/// and wh is for the window's width and height. It's important the value is global, so that a bit of text
/// drawn later isn't larger due to changed window dimensions
pub struct TextDimensions {
    width: f32,
    wh: (u32, u32),
}

impl TextDimensions {
    /// Creates a new TextDimensions struct with a 0 width, so it will need to be set,
    /// but a final width and height tuple.
    ///
    /// # Arguments
    ///
    /// * `display` - A reference to the GL context with a facade for drawing upon
    pub fn new(display: &Display) -> TextDimensions {
        TextDimensions {
            width: 0.0,
            wh: display.get_framebuffer_dimensions(),
        }
    }

    /// Simple getter for the width
    pub fn width(&self) -> f32 {
        self.width
    }

    /// Simple setter for the width
    pub fn set_width(&mut self, new_width: f32) {
        self.width = new_width;
    }

    /// Simple getter for the window dimensions
    pub fn wh(&self) -> (u32, u32) {
        self.wh
    }
}
