use crate::models::globals::Globals;
use glium::Frame;
use crate::config::{TITLE_TEXT_WIDTH, SUBTITLE_TEXT_WIDTH};

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
        0.5 / TITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / TITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.92, 0.0, 1.0f32,
    )
    .into();

    #[rustfmt::skip]
    let y_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / TITLE_TEXT_WIDTH, 0.0, 0.0,
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
        0.5 / TITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / TITLE_TEXT_WIDTH, 0.0, 0.0,
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

/// Draws the current speed multiplier to the frame
///
/// # Arguments
///
/// * `globals` - A struct of global values that includes the current multiplier value and text properties
/// * `target` - A reference to the current frame buffer
///
pub fn draw_speed_multiplier_text(globals: &Globals, target: &mut Frame) {
    let (system, texture) = globals.text_system_font_texture();

    let truncated_multiplier = format!("{:.2}", globals.speed_multiplier());
    let speed_multiplier_text = "Speed Multiplier: ".to_owned() + truncated_multiplier.as_ref();

    let speed_multiplier_text =
        glium_text::TextDisplay::new(system, texture, speed_multiplier_text.as_ref());
    let (w, h) = globals.display().get_framebuffer_dimensions();

    #[rustfmt::skip]
    let speed_multiplier_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / SUBTITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / SUBTITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.60, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &speed_multiplier_text,
        system,
        target,
        speed_multiplier_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}

/// Draws the current point size to the frame
///
/// # Arguments
///
/// * `globals` - A struct of global values that includes the current point size and text properties
/// * `target` - A reference to the current frame buffer
///
pub fn draw_point_size_text(globals: &Globals, target: &mut Frame) {
    let (system, texture) = globals.text_system_font_texture();

    let point_size_string = (globals.point_size() + 1).to_string();
    let point_size_text = "Point Size: ".to_owned() + point_size_string.as_ref();

    let point_size_text =
        glium_text::TextDisplay::new(system, texture, point_size_text.as_ref());
    let (w, h) = globals.display().get_framebuffer_dimensions();

    #[rustfmt::skip]
    let point_size_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / SUBTITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * (w as f32) / (h as f32) / SUBTITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.50, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &point_size_text,
        system,
        target,
        point_size_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}
