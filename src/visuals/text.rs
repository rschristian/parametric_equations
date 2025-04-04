use glium::glutin::surface::WindowSurface;
use glium::{Display, Frame};

use crate::constants::{SUBTITLE_TEXT_WIDTH, TITLE_TEXT_WIDTH};
use crate::models::state::State;

use glium_text::{FontTexture, TextSystem};

pub fn draw_text(
    display: &Display<WindowSurface>,
    target: &mut Frame,
    text_system: &TextSystem,
    font_texture: &FontTexture,
    state: &State,
    equation_strings: &(String, String),
) {
    let (w, h) = display.get_framebuffer_dimensions();
    let (width, height) = (w as f32, h as f32);

    draw_equation_text(
        target,
        text_system,
        font_texture,
        (width, height),
        equation_strings,
    );
    draw_time_text(target, text_system, font_texture, (width, height), state);
    draw_speed_multiplier_text(target, text_system, font_texture, (width, height), state);
    draw_point_size_text(target, text_system, font_texture, (width, height), state);
}

fn draw_equation_text(
    target: &mut Frame,
    text_system: &TextSystem,
    font_texture: &FontTexture,
    dimensions: (f32, f32),
    equation_strings: &(String, String),
) {
    let (width, height) = dimensions;
    let (x_prime, y_prime) = equation_strings;

    let x_prime_text = glium_text::TextDisplay::new(text_system, font_texture, x_prime);
    let y_prime_text = glium_text::TextDisplay::new(text_system, font_texture, y_prime);

    #[rustfmt::skip]
    let x_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * width / height / TITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.92, 0.0, 1.0f32,
    )
    .into();

    #[rustfmt::skip]
    let y_prime_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * width / height / TITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.80, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &x_prime_text,
        text_system,
        target,
        x_prime_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
    glium_text::draw(
        &y_prime_text,
        text_system,
        target,
        y_prime_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}

fn draw_time_text(
    target: &mut Frame,
    text_system: &TextSystem,
    font_texture: &FontTexture,
    dimensions: (f32, f32),
    state: &State,
) {
    let (width, height) = dimensions;

    let truncated_time = format!("{:.4}", state.t);
    let time_text = "t: ".to_owned() + truncated_time.as_ref();

    let time_text = glium_text::TextDisplay::new(text_system, font_texture, time_text.as_ref());

    #[rustfmt::skip]
    let time_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / TITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * width / height / TITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.72, 0.92, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &time_text,
        text_system,
        target,
        time_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}

fn draw_speed_multiplier_text(
    target: &mut Frame,
    text_system: &TextSystem,
    font_texture: &FontTexture,
    dimensions: (f32, f32),
    state: &State,
) {
    let (width, height) = dimensions;

    let truncated_multiplier = format!("{:.2}", state.speed_multiplier);
    let speed_multiplier_text = "Speed Multiplier: ".to_owned() + truncated_multiplier.as_ref();

    let speed_multiplier_text =
        glium_text::TextDisplay::new(text_system, font_texture, speed_multiplier_text.as_ref());

    #[rustfmt::skip]
    let speed_multiplier_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / SUBTITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * width / height / SUBTITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.60, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &speed_multiplier_text,
        text_system,
        target,
        speed_multiplier_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}

fn draw_point_size_text(
    target: &mut Frame,
    text_system: &TextSystem,
    font_texture: &FontTexture,
    dimensions: (f32, f32),
    state: &State,
) {
    let (width, height) = dimensions;

    let point_size_string = (state.point_size + 1).to_string();
    let point_size_text = "Point Size: ".to_owned() + point_size_string.as_ref();

    let point_size_text =
        glium_text::TextDisplay::new(text_system, font_texture, point_size_text.as_ref());

    #[rustfmt::skip]
    let point_size_matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(
        0.5 / SUBTITLE_TEXT_WIDTH, 0.0, 0.0, 0.0,
        0.0, 0.5 * width / height / SUBTITLE_TEXT_WIDTH, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -0.98, 0.50, 0.0, 1.0f32,
    )
    .into();

    glium_text::draw(
        &point_size_text,
        text_system,
        target,
        point_size_matrix,
        (1.0, 1.0, 1.0, 1.0),
    );
}
