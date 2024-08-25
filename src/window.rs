use std::sync::{Arc, Mutex};

use objc::rc::autoreleasepool;
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

use crate::{canvas::MetalCanvas, connection::SpiceConnection, scancodes};

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

        let event_loop = EventLoop::new().unwrap();
        let window_size = LogicalSize::new(INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT);

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(window_size)
            .with_title("Spice client".to_string())
            .build(&event_loop)
            .unwrap();

        self.canvas.set_window(&window);
        let _connection = self.connection.clone();
        event_loop
            .run(move |event, event_loop| {
                autoreleasepool(|| {
                    event_loop.set_control_flow(ControlFlow::Poll);
                    match event {
                        Event::AboutToWait => window.request_redraw(),
                        Event::WindowEvent { event, .. } => match event {
                            WindowEvent::CloseRequested => event_loop.exit(),
                            WindowEvent::RedrawRequested => {
                                if let Some(_display) = _connection.display() {
                                    if let Some(img) = _display.canvas_img {
                                        self.canvas.redraw(
                                            img,
                                            _display.stride as u64,
                                            _display.height as u64,
                                            _display.width as u64,
                                        );
                                    }
                                }
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                if let Some(input_channel) = _connection.input() {
                                    input_channel.lock().unwrap().set_cursor_pos(
                                        0,
                                        position.x as i32,
                                        position.y as i32,
                                    );
                                }
                            }
                            WindowEvent::Resized(size) => {}
                            WindowEvent::MouseInput { state, button, .. } => {
                                let mut mask = 1;
                                match button {
                                    winit::event::MouseButton::Left => {
                                        mask = mask << 0;
                                    }
                                    winit::event::MouseButton::Middle => {
                                        mask = mask << 1;
                                    }
                                    winit::event::MouseButton::Right => {
                                        mask = mask << 2;
                                    }
                                    _ => {}
                                }

                                if let Some(input_channel) = _connection.input() {
                                    if state == ElementState::Pressed {
                                        input_channel.lock().unwrap().press_button(2, mask);
                                    }
                                    if state == ElementState::Released {
                                        input_channel.lock().unwrap().release_button(2, mask);
                                    }
                                }
                            }
                            WindowEvent::KeyboardInput {
                                event,
                                is_synthetic,
                                ..
                            } => {
                                if is_synthetic {
                                    return;
                                }
                                if let Some(input_channel) = _connection.input() {
                                    if let PhysicalKey::Code(code) = event.physical_key {
                                        if let Some(scancode) =
                                            crate::scancodes::scancode_to_xt(code)
                                        {
                                            if event.state == ElementState::Pressed {
                                                input_channel.lock().unwrap().key_press(scancode);
                                            }

                                            if event.state == ElementState::Released {
                                                input_channel.lock().unwrap().key_release(scancode);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                });
            })
            .unwrap();
    }
}
