use std::{
    collections::HashMap,
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::{
    clone::Downgrade,
    ffi::{g_main_context_push_thread_default, g_main_context_ref, g_main_loop_run, gboolean},
    object::{ObjectExt, ObjectType},
    property::PropertyGet,
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
    pub fn spice_main_update_display(
        channel: *const c_void,
        id: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        update: gboolean,
    );
    pub fn spice_main_set_display(channel: *const c_void, id: i32, x: i32, y: i32, w: i32, h: i32);
}

type Channels = HashMap<i32, Channel>;

pub struct SpiceConnection<'a> {
    pub session: Arc<Mutex<Session>>,
    host: Option<&'a str>,
    port: Option<u32>,
    channels: Arc<Mutex<Channels>>,
}

impl<'a> SpiceConnection<'a> {
    pub fn new() -> Self {
        let session = Session::new();

        let channels = Arc::new(Mutex::new(HashMap::new()));
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
                                    .insert(1, Channel::MainChannel(main_channel));
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
                                    .insert(2, Channel::DisplayChannel(display_channel));
                            }
                        }

                        if _channel_type == 3 {
                            let input_channel = InputChannel::from(obj.as_ptr() as *mut _);
                            input_channel.lock().unwrap().connect();
                            if let Some(_channels) = _channels.upgrade() {
                                if let Some(display_id) =
                                    if let Some(Channel::DisplayChannel(display_channel)) =
                                        _channels.lock().unwrap().get(&2)
                                    {
                                        Some(display_channel.lock().unwrap().id())
                                    } else {
                                        None
                                    }
                                {
                                    input_channel
                                        .lock()
                                        .unwrap()
                                        .set_cursor_pos(display_id, 0, 0);
                                }

                                _channels
                                    .lock()
                                    .unwrap()
                                    .insert(3, Channel::InputChannel(input_channel));
                            }
                        }

                        if _channel_type == 4 {
                            let cursor_channel = CursorChannel::from(obj);
                            cursor_channel.lock().unwrap().connect();
                            if let Some(_channels) = _channels.upgrade() {
                                _channels
                                    .lock()
                                    .unwrap()
                                    .insert(4, Channel::CursorChannel(cursor_channel));
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
        if let Some(Channel::DisplayChannel(display_channel)) =
            self.channels.lock().unwrap().get(&2)
        {
            display_channel.lock().unwrap().display()
        } else {
            None
        }
    }

    pub fn input(&self) -> Option<Arc<Mutex<InputChannel>>> {
        if let Some(Channel::InputChannel(input_channel)) = self.channels.lock().unwrap().get(&3) {
            Some(input_channel.clone())
        } else {
            None
        }
    }

    pub fn main_channel(&self) -> Option<Arc<Mutex<MainChannel>>> {
        if let Some(Channel::MainChannel(main_channel)) = self.channels.lock().unwrap().get(&1) {
            Some(main_channel.clone())
        } else {
            None
        }
    }

    pub fn change_monitor_config(&self, w: i32, h: i32) {
        if let Some(main_channel) = self.main_channel() {
            if let Some(Channel::DisplayChannel(display_channel)) =
                self.channels.lock().unwrap().get(&2)
            {
                let id = display_channel.lock().unwrap().id();
                unsafe {
                    spice_main_set_display(main_channel.lock().unwrap().as_ptr(), id, 0, 0, w, h);
                };
            }
        }
    }

    pub fn dimention(&self) -> (u32, u32) {
        if let Some(Channel::DisplayChannel(display_channel)) =
            self.channels.lock().unwrap().get(&2)
        {
            display_channel.lock().unwrap().dimention()
        } else {
            (0, 0)
        }
    }

    pub fn display_channel(&self) -> Option<Arc<Mutex<DisplayChannel>>> {
        if let Some(Channel::DisplayChannel(display_channel)) =
            self.channels.lock().unwrap().get(&2)
        {
            Some(display_channel.clone())
        } else {
            None
        }
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
