use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

use crate::canvas::Canvas;

const INITIAL_WINDOW_WIDTH: u32 = 800;
const INITIAL_WINDOW_HEIGHT: u32 = 600;

pub struct CocoaWindow {
    canvas: Box<dyn Canvas>,
}

impl CocoaWindow {
    pub fn new<T: Canvas + 'static>(canvas: T) -> Self {
        CocoaWindow {
            canvas: Box::new(canvas),
        }
    }

    pub fn open(&self) {
        let event_loop = EventLoop::new().unwrap();
        let window_size = LogicalSize::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(window_size)
            .with_title("Spice client".to_string())
            .build(&event_loop)
            .unwrap();
        event_loop
            .run(|event, event_loop| match event {
                Event::AboutToWait => window.request_redraw(),
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => event_loop.exit(),
                    _ => {}
                },
                _ => {}
            })
            .unwrap();
    }
}
