use crate::models::globals::Globals;
use glium::{Display, Frame};
use glium_text::{FontTexture, TextSystem};

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

/// Provides the common values like TextSystem and FontTexture that are necessary for drawing text
///
/// # Arguments
///
/// * `display` - A reference to the GL context with a facade for drawing upon
fn text_setup(display: &Display) -> (TextSystem, FontTexture) {
    (
        glium_text::TextSystem::new(display),
        glium_text::FontTexture::new(display, &include_bytes!("font/comic.ttf")[..], 70).unwrap(),
    )
}

/// Draws the current equations to the frame
///
/// # Arguments
///
/// * `x_prime` - A string equation that is currently used to map all x values for the vertices
/// * `y_prime` - A string equation that is currently used to map all y values for the vertices
/// * `text_dimensions` - A struct that contains values used to shape and size the text drawn
/// * `display` - A reference to the GL context with a facade for drawing upon
/// * `target` - A reference to the current frame buffer
///
pub fn draw_equation_text(
    x_prime: &str,
    y_prime: &str,
    text_dimensions: &mut TextDimensions,
    display: &Display,
    target: &mut Frame,
) {
    let (system, font) = text_setup(display);

    let x_prime_text = glium_text::TextDisplay::new(&system, &font, x_prime);
    let y_prime_text = glium_text::TextDisplay::new(&system, &font, y_prime);

    let normalized_width = if x_prime_text.get_width() > y_prime_text.get_width() {
        x_prime_text.get_width()
    } else {
        y_prime_text.get_width()
    };
    text_dimensions.set_width(normalized_width);

    let (w, h) = text_dimensions.wh();

    #[rustfmt::skip]
    let x_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / normalized_width, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / normalized_width, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.92, 0.0, 1.0f32,
    )
    .into();

    #[rustfmt::skip]
    let y_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / normalized_width, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / normalized_width, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.80, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &x_prime_text,
        &system,
        target,
        x_prime_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
    glium_text::draw(
        &y_prime_text,
        &system,
        target,
        y_prime_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}

/// Draws the current time to the frame
///
/// # Arguments
///
/// * `globals` - A struct of global values that includes the current t value
/// * `text_dimensions` - A struct that contains values used to shape and size the text drawn
/// * `display` - A reference to the GL context with a facade for drawing upon
/// * `target` - A reference to the current frame buffer
///
pub fn draw_time_text(
    globals: Globals,
    text_dimensions: &TextDimensions,
    display: &Display,
    target: &mut Frame,
) {
    let (system, font) = text_setup(display);

    let truncated_time = format!("{:.6}", globals.t());
    let time_text = "t: ".to_owned() + truncated_time.as_ref();

    let time_text = glium_text::TextDisplay::new(&system, &font, time_text.as_ref());
    let text_width = text_dimensions.width();

    let (w, h) = text_dimensions.wh();

    #[rustfmt::skip]
    let time_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / text_width, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / text_width, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.78, 0.92, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &time_text,
        &system,
        target,
        time_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}
