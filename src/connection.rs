use std::sync::{Arc, Mutex};

use glib::{
    clone::Downgrade,
    ffi::gboolean,
    object::{ObjectExt, ObjectType},
    Object, Value,
};

use crate::{channel::Channel, display_channel::DisplayChannel, session::Session};

extern "C" {
    pub fn spice_util_set_debug(enabled: gboolean);
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
                        if _channel_type == 1 {
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
                            obj.connect("main-mouse-update", false, |values: &[Value]| {
                                dbg!("Main mouse update");
                                None
                            });
                        }

                        if _channel_type == 2 {
                            let display_channel = DisplayChannel::from(obj.as_ptr() as *mut _);
                            if let Some(_channels) = _channels.upgrade() {
                                _channels
                                    .lock()
                                    .unwrap()
                                    .push(Channel::DisplayChannel(display_channel));
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
        let _session = Arc::downgrade(&self.session);
        std::thread::spawn(move || {
            unsafe {
                spice_util_set_debug(1);
            }
            if let Some(_session) = _session.upgrade() {
                _session.lock().unwrap().connect();
                let _loop = glib::MainLoop::new(None, false);
                _loop.run();
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::{spice_util_set_debug, SpiceConnection};

    #[test]
    fn connection_init() {
        // unsafe {
        // spice_util_set_debug(1);
        // }
        // let ctx = glib::MainContext::new();

        let mut connection = SpiceConnection::new();
        connection.host("localhost").port(5930).spawn();

        // let _loop = glib::MainLoop::new(None, false);
        // _loop.run();
        loop {}
    }
}
