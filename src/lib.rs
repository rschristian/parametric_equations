#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate glium;

extern crate cgmath;

use crate::config::{ITERATIONS, STEPS, WINDOW_HEIGHT, WINDOW_WIDTH, T_END};
use crate::models::globals::Globals;
use crate::models::parameters::Parameters;
use crate::models::vertex::populate_vertex_vector;
use crate::visuals::drawing::draw_vertices;
use crate::visuals::text::{draw_equation_text, TextDimensions, draw_time_text};
use crate::visuals::utility::reset_and_generate_new;
use glium::backend::glutin::glutin::{
    ContextBuilder, Event, EventsLoop, VirtualKeyCode, WindowBuilder, WindowEvent,
};
use glium::{Display, Surface};

mod chaos;
mod config;
mod models;
mod visuals;

/// Start the library. Creates a new window, and calls the main program loop
pub fn start() {
    let events_loop = EventsLoop::new();
    let display = Display::new(
        WindowBuilder::new()
            .with_title("Chaos Equations Visualizer")
            .with_dimensions((WINDOW_WIDTH, WINDOW_HEIGHT).into()),
        ContextBuilder::new(),
        &events_loop,
    )
    .unwrap();
    run_main_loop(events_loop, display);
}

fn run_main_loop(mut events_loop: EventsLoop, display: Display) {
    //Set up variables
    let mut window_open = true;
    let mut globals = Globals::new();
    let mut equation_parameters = Parameters::new();
    let mut text_dimensions = TextDimensions::new(&display);
//    let mut vertex_vector = populate_vertex_vector((ITERATIONS * STEPS) as usize);

    let (ref mut x_prime_equation, ref mut y_prime_equation) = reset_and_generate_new(&mut globals, equation_parameters);

    while window_open {
        if globals.t() >= T_END {
            equation_parameters = Parameters::new();
            let (nx_prime_equation, ny_prime_equation) = reset_and_generate_new(&mut globals, equation_parameters);
            *x_prime_equation = nx_prime_equation;
            *y_prime_equation = ny_prime_equation;
        }

        // Needs to be within the loop so that the value isn't borrowed after a move in a previous loop iteration
        let mut target = display.draw();

        // Draw a blank screen to start with
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        // Draw the new points
//        draw_vertices(&display, &mut target, globals, &mut vertex_vector,
//                      equation_parameters, &mut text_dimensions);

        // Draw the equations
        draw_equation_text(&x_prime_equation,&y_prime_equation,
                           &mut text_dimensions, &display, &mut target);

        // Draw the current t-value
        draw_time_text(globals, &text_dimensions, &display, &mut target);

        globals.increase_t(0.10);

        // Shows the prepared frame
        target.finish().unwrap();

        // Handle all inputs given by the user, and check to see if window should be closed
        window_open = event_handler(&mut events_loop)
    }
}

/// Handles all user input events.
fn event_handler(events_loop: &mut EventsLoop) -> bool {
    let mut window_open = true;
    events_loop.poll_events(|event| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => window_open = false,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match key {
                            VirtualKeyCode::Escape => window_open = false,
                            VirtualKeyCode::C => window_open = false,
                            _ => {}
                        }
                    }
                }
                _ => (),
            }
        }
    });
    window_open
}
