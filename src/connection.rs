use std::{ffi::c_void, net::TcpStream, os::fd::AsRawFd};

use gio::Socket;
use glib::{
    ffi::gboolean,
    object::{ObjectExt, ObjectType},
    translate::ToGlibPtr,
    value::ToValue,
    Object, Value,
};

use crate::{channel::Channel, session::Session};

extern "C" {
    pub fn spice_util_set_debug(enabled: gboolean);
}

pub struct Connection<'a> {
    session: Session,
    host: Option<&'a str>,
    port: Option<u32>,
    main_channel: Channel,
    // display_channel: Channel,
    // cursor_channel: Channel,
}

impl<'a> Connection<'a> {
    pub fn new() -> Self {
        let session = Session::new();
        let obj = session.obj();
        obj.set_property("enable-usbredir", &false.to_value());
        obj.set_property("client-sockets", &true.to_value());

        Connection {
            main_channel: session.create_channel(1, 1),
            // display_channel: session.create_channel(2, 2),
            // cursor_channel: session.create_channel(4, 3),
            session,
            host: None,
            port: None,
        }
    }

    pub fn host(&mut self, host: &'a str) -> &mut Self {
        self.session.set_host(host);
        self.host = Some(host);
        self
    }

    pub fn port(&mut self, port: u32) -> &mut Self {
        self.session.set_port(&port.to_string());
        self.port = Some(port);
        self
    }

    pub fn connect(&self, stream: &TcpStream) -> bool {
        let socket_fd = stream.as_raw_fd();
        let g_socket = unsafe { Socket::from_fd(socket_fd).expect("Failed to create gsocket") };

        let fd = g_socket.as_raw_fd();

        let callback = move |val: &[Value]| {
            let _channel = val.get(0);
            if let Some(_ch) = _channel {
                if let Some(obj) = _ch.get::<Object>().ok() {
                    let channel_type = {
                        let value = obj.property_value("channel-type");
                        if let Some(val) = value.get::<i32>().ok() {
                            Some(val)
                        } else {
                            None
                        }
                    };
                    if let Some(channel_type_str) = channel_type {
                        dbg!(channel_type_str);
                        let rt = unsafe {
                            crate::channel::spice_channel_open_fd(obj.as_ptr() as *const _, fd)
                        };
                        dbg!(rt);
                        // let rt_monitor = unsafe {
                        //     crate::session::spice_main_set_display(
                        //         obj.as_ptr() as *mut _,
                        //         1,
                        //         0,
                        //         0,
                        //         200,
                        //         100,
                        //     );
                        //     crate::session::spice_main_send_monitor_config(obj.as_ptr() as *mut _)
                        // };
                        // dbg!(rt_monitor);
                    }
                }
            }
            None
        };

        self.main_channel.signal("open-fd", callback);
        // self.display_channel.signal("open-fd", callback);
        // self.cursor_channel.signal("open-fd", callback);

        self.session.open_fd(-1);
        true
    }
}

#[cfg(test)]
mod test {
    use std::net::TcpStream;

    use glib::{object::ObjectExt, value::ToValue};

    use crate::session::Session;

    use super::{spice_util_set_debug, Connection};

    #[test]
    fn connection_init() {
        unsafe {
            spice_util_set_debug(1);
        }
        let ctx = glib::MainContext::new();
        let mut connection = Connection::new();
        let stream = TcpStream::connect("127.0.0.1:5930").expect("Failed to connect");
        connection.host("localhost").port(5930).connect(&stream);

        let _loop = glib::MainLoop::new(Some(&ctx), false);
        _loop.run();
    }

    #[test]
    fn check_socket() {
        let session = Session::new();
        let obj = session.obj();
        obj.set_property("client-sockets", &true.to_value());
        if let Some(val) = obj.property_value("client-sockets").get::<bool>().ok() {
            dbg!(val);
            assert_eq!(val, true);
        }
    }
}
