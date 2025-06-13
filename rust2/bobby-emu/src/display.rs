use std::sync::mpsc::{Receiver, Sender};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;
pub const WIDTH: u16 = 640;
pub const HEIGHT: u16 = 480;
pub const SCALE: u16 = 2;

#[derive(Debug)]
pub enum DisplayEvent {
    Rectangle {
        target_x: u16,
        target_y: u16,
        size_x: u16,
        size_y: u16,
        color: u32,
    },
    Copy {
        target_x: u16,
        target_y: u16,
        source_x: u16,
        source_y: u16,
        size_x: u16,
        size_y: u16,
    },
    Floodfill {
        color: u32,
    },
    Matrix {
        matrix: u64,
        target_x: u16,
        target_y: u16,
        color: u32,
    },
}

pub fn run(recv: Receiver<DisplayEvent>, send: Sender<KeyEvent>) {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * SCALE) as f64, (HEIGHT * SCALE) as f64);
        WindowBuilder::new()
            .with_title("Bobby's Display")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let _res = event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                update_frame(pixels.frame_mut(), &recv);
                if let Err(err) = pixels.render() {
                    eprintln!("pixels.render {:?}", err);
                    elwt.exit();
                    return;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { ref event, .. },
                ..
            } => {
                send.send(event.clone()).unwrap();
            }
            _ => (),
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });
}

pub fn update_frame(frame: &mut [u8], recv: &Receiver<DisplayEvent>) {
    for e in recv.try_iter() {
        eprintln!("e: {:?}", e);
        match e {
            DisplayEvent::Rectangle {
                target_x,
                target_y,
                size_x,
                size_y,
                color,
            } => {
                for yi in 0..size_y {
                    for xi in 0..size_x {
                        let pixel = pixel(frame, target_x + xi, target_y + yi);
                        set_color(pixel, color);
                    }
                }
            },
            DisplayEvent::Copy { .. } => todo!(),
            DisplayEvent::Floodfill { color } => {
                for pixel in frame.chunks_exact_mut(4) {
                    set_color(pixel.try_into().unwrap(), color);
                }
            }
            DisplayEvent::Matrix {
                mut matrix,
                target_x,
                target_y,
                color,
            } => {
                for yi in 0..8 {
                    for xi in 0..8 {
                        if matrix & 1 == 0 {
                            matrix >>= 1;
                            continue;
                        }
                        let pixel = pixel(frame, target_x + xi, target_y + yi);
                        set_color(pixel, color);
                        matrix >>= 1;
                    }
                }
            }
        }
    }
}

pub fn pixel(frame: &mut [u8], x: u16, y: u16) -> &mut [u8; 4] {
    let (x, y) = (x as usize, y as usize);
    let idx = (y * WIDTH as usize + x) * 4;
    (&mut frame[idx..idx + 4]).try_into().unwrap()
}

pub fn set_color(pixel: &mut [u8; 4], color: u32) {
    pixel[0] = (color >> 16) as u8;
    pixel[1] = (color >> 8) as u8;
    pixel[2] = color as u8;
    pixel[3] = 255;
}