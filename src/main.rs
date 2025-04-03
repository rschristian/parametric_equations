#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use glium::{glutin, Surface};

mod chaos;
mod constants;
mod lifecycle;
mod models;
mod visuals;

pub fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();

    let display = glium::Display::new(
        glutin::window::WindowBuilder::new()
            .with_title("Chaos Equations Visualizer")
            .with_inner_size(glutin::dpi::LogicalSize::new(
                constants::WINDOW_WIDTH.into(),
                constants::WINDOW_HEIGHT.into(),
            )),
        glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_multisampling(4),
        &event_loop,
    )
    .unwrap();

    run_main_loop(event_loop, display);
}

fn run_main_loop(event_loop: glutin::event_loop::EventLoop<()>, display: glium::Display) {
    let text_system = glium_text::TextSystem::new(&display);
    let font_texture =
        glium_text::FontTexture::new(&display, &include_bytes!("./font/sf-regular.otf")[..], 70)
            .unwrap();

    let mut state = models::state::State::new();

    let (mut params, mut equation_strings, mut vertex_vector) = lifecycle::initialize(&mut state);

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::event::VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit
                            }
                            glutin::event::VirtualKeyCode::Up => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    state.increase_speed_multiplier(0.1)
                                }
                            }
                            glutin::event::VirtualKeyCode::Down => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    state.increase_speed_multiplier(-0.1)
                                }
                            }
                            glutin::event::VirtualKeyCode::PageUp => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    state.increase_point_size();
                                }
                            }
                            glutin::event::VirtualKeyCode::PageDown => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    state.decrease_point_size();
                                }
                            }
                            glutin::event::VirtualKeyCode::N => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    state.increase_t(6.0);
                                }
                            }
                            _ => return,
                        }
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        if state.t >= constants::T_END {
            (params, equation_strings, vertex_vector) = lifecycle::initialize(&mut state);
        }

        chaos::apply_chaos(&mut state, &params, &mut vertex_vector);

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        visuals::text::draw_text(
            &display,
            &mut target,
            &text_system,
            &font_texture,
            &state,
            &equation_strings,
        );

        visuals::objects::draw_vertices(&display, &mut target, &state, &vertex_vector);

        target.finish().unwrap();
    });
}
