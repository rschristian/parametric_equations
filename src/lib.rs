#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate glium;

extern crate cgmath;

use crate::chaos::apply_chaos;
use crate::config::{ITERATIONS, STEPS, T_END, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::models::{globals::Globals, parameters::Parameters, text_dimensions::TextDimensions, vertex::populate_vertex_vector};
use crate::visuals::{drawing::draw_vertices, text::{draw_equation_text, draw_time_text}, utility::reset_and_generate_new};
use glium::{glutin, Surface};

mod chaos;
mod config;
mod models;
mod visuals;

/// Start the library. Creates a new window, and calls the main program loop
pub fn start() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = glium::Display::new(
        glutin::window::WindowBuilder::new()
            .with_title("Chaos Equations Visualizer")
            .with_inner_size((WINDOW_WIDTH, WINDOW_HEIGHT).into()),
        glutin::ContextBuilder::new(),
        &event_loop,
    )
    .unwrap();
    run_main_loop(event_loop, display);
}

fn run_main_loop(event_loop: glutin::event_loop::EventLoop<()>, display: glium::Display) {
    //Set up variables
    let mut globals = Globals::new();
    let mut equation_parameters = Parameters::new();
    let mut text_dimensions = TextDimensions::new(&display);
    let mut vertex_vector = populate_vertex_vector((ITERATIONS * STEPS) as usize);

    let equations = reset_and_generate_new(&mut globals, equation_parameters);
    let mut x_prime_equation = equations.0;
    let mut y_prime_equation = equations.1;

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            glutin::event::VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,
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

        if globals.t() + (0.01 * globals.speed_multiplier()) * STEPS as f64 >= T_END {
            equation_parameters.reset_dimensions();
            let equations = reset_and_generate_new(&mut globals, equation_parameters);
            x_prime_equation = equations.0;
            y_prime_equation = equations.1;
        }

        // Needs to be within the loop so that the value isn't borrowed after a move in a previous loop iteration
        let mut target = display.draw();

        // Draw a blank screen to start with
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Apply the math to the coordinates
        apply_chaos(&mut globals, equation_parameters, &mut vertex_vector);

        // Draw the new points
        draw_vertices(globals, &mut vertex_vector, &display, &mut target);

        // Draw the equations
        draw_equation_text(
            &x_prime_equation,
            &y_prime_equation,
            &mut text_dimensions,
            &display,
            &mut target,
        );

        // Draw the current t-value
        draw_time_text(globals, &text_dimensions, &display, &mut target);

        // Shows the prepared frame
        target.finish().unwrap();
    });
}
