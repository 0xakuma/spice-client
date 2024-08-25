use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::{
    clone::Downgrade,
    ffi::{g_main_context_push_thread_default, g_main_context_ref, g_main_loop_run, gboolean},
    object::{ObjectExt, ObjectType},
    MainContext, Object, Value,
};

use crate::{
    channel::Channel,
    cursor_channel::CursorChannel,
    display_channel::{Display, DisplayChannel},
    input_channel::InputChannel,
    main_channel::MainChannel,
    session::Session,
};

extern "C" {
    pub fn spice_util_set_debug(enabled: gboolean);
    pub fn spice_util_set_main_context(ctx: *const c_void);
}

pub struct SpiceConnection<'a> {
    pub session: Arc<Mutex<Session>>,
    host: Option<&'a str>,
    port: Option<u32>,
    channels: Arc<Mutex<Vec<Channel>>>,
}

impl<'a> SpiceConnection<'a> {
    pub fn new() -> Self {
        let session = Session::new();

        let channels = Arc::new(Mutex::new(Vec::new()));
        let _channels = channels.downgrade();

        session.signal_connect("channel-new", move |values: &[Value]| {
            let _session = values.get(0);
            let _channel = values.get(1);
            if let Some(channel) = _channel {
                if let Some(obj) = channel.get::<Object>().ok() {
                    let channel_type = {
                        let value = obj.property_value("channel-type");
                        if let Some(val) = value.get::<i32>().ok() {
                            Some(val)
                        } else {
                            None
                        }
                    };

                    if let Some(_channel_type) = channel_type {
                        dbg!(_channel_type);
                        let callback = |values: &[Value]| {
                            let event = values.get(1);
                            if let Some(event) = event {
                                if let Some(event) = event.get::<i32>().ok() {
                                    dbg!(event);
                                }
                            }
                            None
                        };
                        obj.connect("channel-event", false, callback);
                        if _channel_type == 1 {
                            let main_channel = MainChannel::from(obj);
                            if let Some(_channels) = _channels.upgrade() {
                                _channels
                                    .lock()
                                    .unwrap()
                                    .push(Channel::MainChannel(main_channel));
                            }
                            return;
                        }

                        if _channel_type == 2 {
                            let display_channel = DisplayChannel::from(obj.as_ptr() as *mut _);
                            display_channel.lock().unwrap().connect();
                            if let Some(_channels) = _channels.upgrade() {
                                _channels
                                    .lock()
                                    .unwrap()
                                    .push(Channel::DisplayChannel(display_channel));
                            }
                        }

                        if _channel_type == 3 {
                            let input_channel = InputChannel::from(obj.as_ptr() as *mut _);
                            input_channel.lock().unwrap().connect();
                            if let Some(_channels) = _channels.upgrade() {
                                if let Some(display_id) =
                                    _channels.lock().unwrap().iter().find_map(|e| {
                                        if let Channel::DisplayChannel(display_channel) = e {
                                            Some(display_channel.lock().unwrap().id())
                                        } else {
                                            None
                                        }
                                    })
                                {
                                    input_channel
                                        .lock()
                                        .unwrap()
                                        .set_cursor_pos(display_id, 0, 0);
                                }

                                _channels
                                    .lock()
                                    .unwrap()
                                    .push(Channel::InputChannel(input_channel));
                            }
                        }

                        if _channel_type == 4 {
                            let cursor_channel = CursorChannel::from(obj);
                            cursor_channel.lock().unwrap().connect();
                            if let Some(_channels) = _channels.upgrade() {
                                _channels
                                    .lock()
                                    .unwrap()
                                    .push(Channel::CursorChannel(cursor_channel));
                            }
                        }
                    }
                }
            }
        });

        SpiceConnection {
            session: Arc::new(Mutex::new(session)),
            host: None,
            port: None,
            channels,
        }
    }

    pub fn host(&mut self, host: &'a str) -> &mut Self {
        self.session.lock().unwrap().set_host(host);
        self.host = Some(host);
        self
    }

    pub fn port(&mut self, port: u32) -> &mut Self {
        self.session.lock().unwrap().set_port(&port.to_string());
        self.port = Some(port);
        self
    }

    pub fn connect(&self) -> bool {
        self.session.lock().unwrap().connect();
        true
    }

    pub fn spawn(&self) {
        unsafe {
            spice_util_set_debug(1);
        }
        let ctx = MainContext::new();
        let _loop = glib::MainLoop::new(Some(&ctx), false);
        unsafe {
            spice_util_set_main_context(ctx.as_ptr() as *const _);
        };
        let _session = self.session.clone();
        std::thread::spawn(move || {
            unsafe {
                spice_util_set_debug(1);
                g_main_context_ref(ctx.as_ptr());
                g_main_context_push_thread_default(ctx.as_ptr());
                _session.lock().unwrap().connect();
                g_main_loop_run(_loop.as_ptr());
            };
        });
    }

    pub fn display(&self) -> Option<Display> {
        self.channels.lock().unwrap().iter().find_map(|e| {
            if let Channel::DisplayChannel(display_channel) = e {
                display_channel.lock().unwrap().display()
            } else {
                None
            }
        })
    }

    pub fn input(&self) -> Option<Arc<Mutex<InputChannel>>> {
        self.channels.lock().unwrap().iter().find_map(|e| {
            if let Channel::InputChannel(input_channel) = e {
                Some(input_channel.clone())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::SpiceConnection;

    #[test]
    fn connection_init() {
        let mut connection = SpiceConnection::new();
        connection.host("localhost").port(5930).spawn();
        loop {}
    }

    #[test]
    fn connection_spawn() {}
}
