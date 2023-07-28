use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use framebrush::{Canvas, Color};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

enum MyColor {
    Red,
    Green,
    Blue,
}

impl Color<u32> for MyColor {
    fn pixel(&self) -> u32 {
        match self {
            Self::Red => 0xff0000,
            Self::Green => 0x00ff00,
            Self::Blue => 0x0000ff,
        }
    }
}

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("Custom Color Example")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::WaitUntil(
            Instant::now()
                .checked_add(Duration::from_micros(1_000_000 / 60))
                .unwrap(),
        );

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            Event::RedrawRequested(id) if id == window.id() => {
                let (width, height) = {
                    let window_size = window.inner_size();

                    (window_size.width, window_size.height)
                };
                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                let mut buffer = surface.buffer_mut().unwrap();

                let mut canvas =
                    Canvas::new(&mut buffer, (width as usize, height as usize), (320, 240));
                canvas.fill(0);

                canvas.put_rect(10, 10, 40, 35, MyColor::Red.pixel());
                canvas.put_rect(55, 10, 40, 35, MyColor::Green.pixel());
                canvas.put_rect(10, 50, 40, 35, MyColor::Blue.pixel());

                buffer.present().expect("Couldn't present frame buffer.");
            }

            Event::WindowEvent {
                window_id: id,
                event: WindowEvent::CloseRequested,
            } if id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }

            _ => (),
        }
    });
}