use std::sync::Arc;

use objc::rc::autoreleasepool;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
};

use crate::{canvas::MetalCanvas, connection::SpiceConnection, display_channel::Display};

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
        let _connection = self.connection.clone();

        let mut display: Option<Display> = None;

        loop {
            if let Some(display_channel) = _connection.display_channel() {
                if let Some(_display) = display_channel.lock().unwrap().display() {
                    display = Some(_display);
                    break;
                }
            }
        }

        let _display = display.unwrap();

        let event_loop = EventLoop::new().unwrap();
        let window_size = LogicalSize::new(_display.width as u32, _display.height as u32);

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(window_size)
            .with_title("Spice client".to_string())
            .build(&event_loop)
            .unwrap();

        self.canvas.set_window(&window);
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
                                let window_size = window.inner_size();
                                let dx_moved = position.x / window_size.width as f64;
                                let dy_moved = position.y / window_size.height as f64;

                                let pos_x = dx_moved * _display.width as f64;
                                let pos_y = dy_moved * _display.height as f64;

                                if let Some(input_channel) = _connection.input() {
                                    input_channel.lock().unwrap().set_cursor_pos(
                                        0,
                                        pos_x as i32,
                                        pos_y as i32,
                                    );
                                }
                            }
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
                            WindowEvent::MouseWheel {
                                device_id,
                                delta,
                                phase,
                            } => {
                                if let Some(input_channel) = _connection.input() {
                                    if let MouseScrollDelta::LineDelta(dx, dy) = delta {
                                        if dy > 0. {
                                            dbg!(dy);
                                            input_channel.lock().unwrap().press_button(5, 1 << 3);
                                        }
                                    }

                                    if let MouseScrollDelta::PixelDelta(d) = delta {
                                        if d.y > 0. {
                                            input_channel.lock().unwrap().press_button(5, 1 << 3);
                                        }
                                        if d.y < 0. {
                                            input_channel.lock().unwrap().press_button(6, 1 << 4);
                                        }
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
