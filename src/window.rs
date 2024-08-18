use std::sync::{Arc, Mutex};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{canvas::MetalCanvas, channel::Channel, connection::SpiceConnection};

const INITIAL_WINDOW_WIDTH: u32 = 800;
const INITIAL_WINDOW_HEIGHT: u32 = 600;

pub struct CocoaWindow<'a> {
    canvas: MetalCanvas,
    connection: Arc<SpiceConnection<'a>>,
}

impl<'a> CocoaWindow<'a> {
    pub fn new(connection: SpiceConnection<'a>, canvas: MetalCanvas) -> Self {
        CocoaWindow {
            canvas,
            connection: Arc::new(connection),
        }
    }

    pub fn open(&self) {
        self.connection.spawn();

        std::thread::spawn(move || {
            let event_loop = EventLoop::new().unwrap();
            let window_size = LogicalSize::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

            let window = winit::window::WindowBuilder::new()
                .with_inner_size(window_size)
                .with_title("Spice client".to_string())
                .build(&event_loop)
                .unwrap();
            event_loop
                .run(move |event, _, control_flow| match event {
                    Event::AboutToWait => window.request_redraw(),
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::RedrawRequested => {}
                        _ => {}
                    },
                    _ => {}
                })
                .unwrap();
        });
    }
}
