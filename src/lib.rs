#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use crate::config::{DELTA_PER_STEP, STEPS, T_END, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::{globals::Globals, parameters::Parameters, vertex};
use crate::visuals::utility;
use glium::glutin;
use std::io::Cursor;

mod chaos;
mod config;
mod models;
mod visuals;

/// Start the library. Creates a new window, and calls the main program loop
pub fn start() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let icon = image::load(
        Cursor::new(&include_bytes!("images/temp-logo.png")[..]),
        image::PNG,
    )
    .unwrap()
    .to_rgba();
    let icon_dimensions = icon.dimensions();
    let display = glium::Display::new(
        glutin::window::WindowBuilder::new()
            .with_title("Chaos Equations Visualizer")
            .with_window_icon(Some(
                glutin::window::Icon::from_rgba(
                    icon.to_vec(),
                    icon_dimensions.0,
                    icon_dimensions.1,
                )
                .unwrap(),
            ))
            .with_inner_size((WINDOW_WIDTH, WINDOW_HEIGHT).into()),
        glutin::ContextBuilder::new(),
        &event_loop,
    )
    .unwrap();
    run_main_loop(event_loop, display);
}

fn run_main_loop(event_loop: glutin::event_loop::EventLoop<()>, display: glium::Display) {
    //Set up variables
    let mut globals = Globals::new(display);
    let mut params = Parameters::new();
    let mut vertex_vector = vertex::create_vertex_vector();
    let mut equation_text = utility::reset_and_generate_new(&mut globals, &params);

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(18_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::event::VirtualKeyCode::Escape => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit
                            }
                            glutin::event::VirtualKeyCode::Up => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    globals.increase_speed_multiplier(0.1)
                                }
                            }
                            glutin::event::VirtualKeyCode::Down => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    globals.increase_speed_multiplier(-0.1)
                                }
                            }
                            glutin::event::VirtualKeyCode::PageUp => {
                                if input.state == glutin::event::ElementState::Pressed {
                                    globals.increase_point_size();
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

        if globals.t() + (DELTA_PER_STEP * STEPS as f64) >= T_END {
            params.reset_dimensions();
            equation_text = utility::reset_and_generate_new(&mut globals, &params);
            vertex_vector = vertex::create_vertex_vector();
        }

        chaos::apply_chaos(&equation_text, &mut globals, &params, &mut vertex_vector);
    });
}
