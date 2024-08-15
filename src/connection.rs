use std::sync::{Arc, Mutex};

use glib::ffi::gboolean;

use crate::session::Session;

extern "C" {
    pub fn spice_util_set_debug(enabled: gboolean);
}

pub struct Connection<'a> {
    session: Arc<Mutex<Session>>,
    host: Option<&'a str>,
    port: Option<u32>,
}

impl<'a> Connection<'a> {
    pub fn new() -> Self {
        let session = Session::new();

        Connection {
            session,
            host: None,
            port: None,
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
}

#[cfg(test)]
mod test {

    use glib::{object::ObjectExt, value::ToValue};

    use crate::session::Session;

    use super::{spice_util_set_debug, Connection};

    #[test]
    fn connection_init() {
        unsafe {
            spice_util_set_debug(1);
        }
        // let ctx = glib::MainContext::new();

        let mut connection = Connection::new();
        connection.host("localhost").port(5930).connect();

        let _loop = glib::MainLoop::new(None, false);
        _loop.run();
    }

    #[test]
    fn check_socket() {
        let session = Session::new();
        let obj = session.lock().unwrap().obj();
        obj.set_property("client-sockets", &true.to_value());
        if let Some(val) = obj.property_value("client-sockets").get::<bool>().ok() {
            dbg!(val);
            assert_eq!(val, true);
        }
    }
}
