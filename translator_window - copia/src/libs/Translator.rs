
use super::russty_tesseractlib;
extern crate glium;
use glium::{
    glutin::{self, event::MouseButton},
    Surface,
};
use screenshots::Screen;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn init_function() {
    let (translator_sender, translator_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();
    let (window_sender, window_receiver): (Sender<WindowSize>, Receiver<WindowSize>) =
        mpsc::channel();

    let current_colors = Colors {
        red: 0.3,
        blue: 0.0,
        green: 0.3,
        alpha: 0.3,
    };
    let display1 = create_window();
    // let display2 = create_window();
    // thread::spawn(move|| {render_loop(display1, current_colors)});
    // thread::spawn(move|| {render_loop(display2, current_colors2)});
    translator_treadh(translator_receiver, window_receiver);
    render_loop(display1, current_colors, window_sender, translator_sender);
    // render_loop(display2,current_colors2);
}

fn create_window() -> glium::glutin::window::WindowBuilder {
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 300.0))
        .with_title("TranslaroWin")
        .with_transparent(true);
    wb
}

fn render_loop(
    wb: glium::glutin::window::WindowBuilder,
    mut current_colors: Colors,
    window_sender: Sender<WindowSize>,
    translator_sender: Sender<Actions>,
) {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let cb: glutin::ContextBuilder<glutin::NotCurrent> = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut translator_state = false;
    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(
            current_colors.red,
            current_colors.green,
            current_colors.blue,
            current_colors.alpha,
        );
        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::Moved(_) => {
                    let dimensions = display.get_framebuffer_dimensions();
                    let position = display.gl_window().window().inner_position().unwrap();

                    let wsize = WindowSize {
                        p1: position.x,
                        p2: position.y,
                        width: dimensions.0,
                        height: dimensions.1,
                    };
                    window_sender
                        .send(wsize)
                        .expect("Error sending window size");
                }
                #[allow(unused_variables)]
                #[allow(deprecated)]
                glutin::event::WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                    modifiers,
                } => match button {
                    MouseButton::Left => match state {
                        glutin::event::ElementState::Pressed => {
                            translator_state = !translator_state;
                            println!("{}", (if *&translator_state{ "Init" }else{"Off"} ));
                            if translator_state {
                                current_colors.alpha = 0.1;
                                current_colors.blue = 0.1;
                                current_colors.red = 0.1;
                                current_colors.green = 0.1;
                                translator_sender
                                    .send(Actions::Capture)
                                    .expect("Error translator state window size");
                                let dimensions = display.get_framebuffer_dimensions();
                                let position =
                                    display.gl_window().window().inner_position().unwrap();

                                let wsize = WindowSize {
                                    p1: position.x,
                                    p2: position.y,
                                    width: dimensions.0,
                                    height: dimensions.1,
                                };
                                window_sender
                                    .send(wsize)
                                    .expect("Error sending window size");
                            } else {
                                translator_sender
                                    .send(Actions::Stop)
                                    .expect("Error translator state window size");
                                current_colors.alpha = 0.3;
                                current_colors.blue = 0.0;
                                current_colors.red = 0.3;
                                current_colors.green = 0.3;
                            }
                        }
                        glutin::event::ElementState::Released => {}
                    },
                    _ => (),
                },

                _ => return,
            },
            _ => (),
        }
    }); //event loop
}

fn translator_treadh(
    translator_receiver: Receiver<Actions>,
    window_receiver: Receiver<WindowSize>,
) {
    thread::spawn(move || {
        let mut text_state = String::new();
        let mut translate_state = false;
        let mut previus_window_position = WindowSize {
            height: 0,
            p1: 0,
            p2: 0,
            width: 0,
        };
        loop {
            match window_receiver.try_recv() {
                Ok(value) => {
                    previus_window_position.p1 = value.p1;
                    previus_window_position.p2 = value.p2;
                    previus_window_position.height = value.height;
                    previus_window_position.width = value.width;
                }
                _ => (),
            };
            match translator_receiver.try_recv() {
                Ok(value) => {
                    match value {
                        Actions::Capture => {
                            translate_state = true;
                        } //end capture
                        Actions::Stop => {
                            translate_state = false;
                        } //end capture
                    };
                }
                Err(_) => (),
            }
            if translate_state {
                let image = capture_screen(
                    XYposition {
                        x: *&previus_window_position.p1,
                        y: *&previus_window_position.p2,
                    },
                    previus_window_position,
                );
                let text = russty_tesseractlib::rustyTextFromImage(image);
                if text != text_state {
                    println!("{}", text);
                    text_state = text;
                }
            }
        }
    });
}

fn capture_screen(screen_position: XYposition, window_size: WindowSize) -> Vec<u8> {
    let screen = Screen::from_point(screen_position.x, screen_position.y).unwrap();
    let image = screen
        .capture_area(
            window_size.p1,
            window_size.p2,
            window_size.width,
            window_size.height,
        )
        .unwrap();
    let buffer = image.buffer();
    buffer.clone()
}

struct Colors {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}
// #[derive(Debug)]
enum Actions {
    // None,
    // Exit,
    Capture,
    // Translate,
    Stop,
}
struct XYposition {
    x: i32,
    y: i32,
}
#[derive(Copy, Clone)]
struct WindowSize {
    p1: i32,
    p2: i32,
    width: u32,
    height: u32,
}
