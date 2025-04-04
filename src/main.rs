#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{glutin::surface::WindowSurface, winit, Surface};

mod chaos;
mod constants;
mod lifecycle;
mod models;
mod visuals;

pub fn main() {
    let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();

    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Chaos Equations Visualizer")
        .with_inner_size(constants::WINDOW_WIDTH, constants::WINDOW_HEIGHT)
        .build(&event_loop);

    run_main_loop(event_loop, window, display);
}

fn run_main_loop(
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    display: glium::backend::glutin::Display<WindowSurface>,
) {
    let text_system = glium_text::TextSystem::new(&display);
    let font_texture =
        glium_text::FontTexture::new(&display, &include_bytes!("./font/sf-regular.otf")[..], 70)
            .unwrap();

    let mut state = models::state::State::new();

    let (mut params, mut equation_strings, mut vertex_vector) = lifecycle::initialize(&mut state);

    #[allow(deprecated)]
    event_loop
        .run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    winit::event::WindowEvent::KeyboardInput {
                        event:
                            winit::event::KeyEvent {
                                logical_key: key,
                                state: glium::winit::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match key.as_ref() {
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape) => {
                            window_target.exit();
                        }
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowUp) => {
                            state.increase_speed_multiplier(0.1);
                        }
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::ArrowDown) => {
                            state.increase_speed_multiplier(-0.1);
                        }
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::PageUp) => {
                            state.increase_point_size();
                        }
                        winit::keyboard::Key::Named(winit::keyboard::NamedKey::PageDown) => {
                            state.decrease_point_size();
                        }
                        winit::keyboard::Key::Character("n") => {
                            state.increase_t(6.0);
                        }
                        _ => (),
                    },
                    winit::event::WindowEvent::RedrawRequested => {
                        //let time_start = std::time::Instant::now();
                        if state.t >= constants::T_END {
                            (params, equation_strings, vertex_vector) =
                                lifecycle::initialize(&mut state);
                        }

                        chaos::apply_chaos(&mut state, &params, &mut vertex_vector);

                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                        visuals::objects::draw_vertices(
                            &display,
                            &mut target,
                            &state,
                            &vertex_vector,
                        );
                        visuals::objects::draw_fade_overlay(&display, &mut target);

                        visuals::text::draw_text(
                            &display,
                            &mut target,
                            &text_system,
                            &font_texture,
                            &state,
                            &equation_strings,
                        );

                        target.finish().unwrap();

                        //let time_end = std::time::Instant::now();
                        //println!("Frame time: {} ms", (time_end - time_start).as_millis());
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    // This will run as fast as the device can handle, a proper timeout/sleep
                    // system should probably be implemented. At the moment I'm hovering around 60 FPS
                    // even without limiting though, probably not a huge issue due to the size of the
                    // calculations.
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
