use crate::models::globals::Globals;
use glium::Frame;

// Arbitrary value that looks about correct for a time value with 3 decimal places.
// Keeps the text static in size
const TEXT_WIDTH: f32 = 16.2;

/// Draws the current equations to the frame
///
/// # Arguments
///
/// * `equation_text` - A reference to a tuple that contains the strings for the x & y prime equations
/// * `globals` - A struct of global values that includes the display and text properties
/// * `target` - A reference to the current frame buffer
///
pub fn draw_equation_text(equation_text: &(String, String), globals: &Globals, target: &mut Frame) {
    let (system, texture) = globals.text_system_font_texture();
    let (x_prime, y_prime) = equation_text;

    let x_prime_text = glium_text::TextDisplay::new(system, texture, x_prime);
    let y_prime_text = glium_text::TextDisplay::new(system, texture, y_prime);
    let (w, h) = globals.display().get_framebuffer_dimensions();

    #[rustfmt::skip]
    let x_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.92, 0.0, 1.0f32,
    )
    .into();

    #[rustfmt::skip]
    let y_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.80, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &x_prime_text,
        system,
        target,
        x_prime_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
    glium_text::draw(
        &y_prime_text,
        system,
        target,
        y_prime_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}

/// Draws the current time to the frame
///
/// # Arguments
///
/// * `globals` - A struct of global values that includes the current t value and text properties
/// * `target` - A reference to the current frame buffer
///
pub fn draw_time_text(globals: &Globals, target: &mut Frame) {
    let (system, texture) = globals.text_system_font_texture();

    let truncated_time = format!("{:.4}", globals.t());
    let time_text = "t: ".to_owned() + truncated_time.as_ref();

    let time_text = glium_text::TextDisplay::new(system, texture, time_text.as_ref());
    let (w, h) = globals.display().get_framebuffer_dimensions();

    #[rustfmt::skip]
    let time_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.72, 0.92, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &time_text,
        system,
        target,
        time_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}
