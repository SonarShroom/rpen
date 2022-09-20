extern crate glium;

mod mementos;
mod shapes;

fn main() {
    use glium::{glutin, Surface};

    // Window creation
    let mut memento_manager = mementos::MementoManager::new();
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new()
        .with_title("rPen")
        .with_transparent(true)
        .with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)))
        .with_resizable(false);
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop);

    match display {
        Err(glium::backend::glutin::DisplayCreationError::GlutinCreationError(creation_error)) => {
            match creation_error {
                glutin::CreationError::OsError(err_str) => {
                    println!("OS Error when creating display: ${err_str}");
                }
                glutin::CreationError::NotSupported(err_str) => {
                    println!("OpenGL not supported: ${err_str}");
                }
                glutin::CreationError::PlatformSpecific(err_str) => {
                    println!("Platform specific error occured: ${err_str}");
                }
                _ => {
                    println!("Backend display creation error occured.");
                }
            }
        }
        Err(glium::backend::glutin::DisplayCreationError::IncompatibleOpenGl(error_string)) => {
            println!("Incompatible OpenGL version: ${error_string}")
        }
        Ok(display) => {
            event_loop.run(move |event, _, control_flow| {
                let next_frame_time = std::time::Instant::now() +
                    std::time::Duration::from_nanos(16_666_667);
                *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

                match event {
                    glutin::event::Event::WindowEvent { event, .. } => match event {
                        glutin::event::WindowEvent::CloseRequested => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                        }
                        glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                            Some(glutin::event::VirtualKeyCode::Escape) => {
                                *control_flow = glutin::event_loop::ControlFlow::Exit;
                            }
                            Some(glutin::event::VirtualKeyCode::C) => {
                                todo!()
                            }
                            _ => ()
                        }
                        glutin::event::WindowEvent::MouseInput { state, button, .. } => match button {
                            glutin::event::MouseButton::Left => match state {
                                glutin::event::ElementState::Pressed => {
                                    println!("LMB pressed at mouse position (x, y)");
                                }
                                glutin::event::ElementState::Released => {
                                    println!("LMB released at mouse position (x, y)");
                                }
                            }
                            _ => ()
                        }
                        glutin::event::WindowEvent::CursorMoved { position, .. } => {
                            let pos : (i32, i32) = position.cast::<i32>().into();
                            println!("Cursor moved to {}, {}", pos.0, pos.1);
                        }
                        _ => ()
                    }
                    glutin::event::Event::NewEvents (cause) => match cause {
                        glutin::event::StartCause::Init => (),
                        glutin::event::StartCause::ResumeTimeReached { .. } => (),
                        _ => ()
                    }
                    _ => ()
                }

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 0.0);
                target.finish().unwrap();
            });
        }
    }
}