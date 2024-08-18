extern crate glib;

use std::ffi::{c_int, c_void};

use ffi::gboolean;
use glib::*;
use object::{ObjectExt, ObjectType};
use translate::FromGlibPtrNone;

use crate::channel::Channel;

extern "C" {
    fn spice_session_new() -> *mut c_void;
    fn spice_session_connect(session: *mut c_void) -> gboolean;
    // fn spice_session_channel_new(session: gpointer, channel: gpointer);
    pub fn spice_main_set_display(
        main_channel: *mut c_void,
        id: c_int,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    );
    pub fn spice_main_send_monitor_config(main_channel: *mut c_void) -> gboolean;
}

pub struct Session {
    inner: Object,
    pub channels: Vec<Channel>,
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    pub fn new() -> Self {
        let session = unsafe { spice_session_new() };
        let obj = unsafe { Object::from_glib_none(session as *const _) };
        let session = Session {
            inner: obj,
            channels: Vec::new(),
        };
        // let session_clone = Arc::clone(&session);
        // session.lock().unwrap().on_chanel_create(session_clone);
        session
    }

    pub fn set_host(&self, host: &str) {
        self.inner.set_property("host", host);
    }

    pub fn set_port(&self, port: &str) {
        self.inner.set_property("port", port);
    }

    pub fn uri(&self) -> Option<String> {
        let value = self.inner.property_value("uri");
        if let Some(val) = value.get::<String>().ok() {
            Some(val)
        } else {
            None
        }
    }

    pub fn signal_connect<F>(&self, signale_key: &str, callback: F)
    where
        F: Fn(&[Value]) + Sync + Send + 'static,
    {
        self.inner
            .connect(signale_key, false, move |values: &[Value]| {
                callback(values);
                None
            });
    }

    // pub fn on_chanel_create(&self, session_ref: Arc<Mutex<Self>>) {
    //     let obj = self.obj();
    //     let weak_session = Arc::downgrade(&session_ref);
    //     obj.connect("channel-new", false, move |values| {
    //         let _session = values.get(0);
    //         let _channel = values.get(1);
    //         let _user_data = values.get(2);

    //         if let Some(channel) = _channel {
    //             if let Some(obj) = channel.get::<Object>().ok() {
    //                 let channel_type = {
    //                     let value = obj.property_value("channel-type");
    //                     if let Some(val) = value.get::<i32>().ok() {
    //                         Some(val)
    //                     } else {
    //                         None
    //                     }
    //                 };
    //                 if let Some(channel_type) = channel_type {
    //                     dbg!(channel_type);

    //                     if channel_type == 1 {
    //                         let callback = |values: &[Value]| {
    //                             let event = values.get(1);
    //                             if let Some(event) = event {
    //                                 if let Some(event) = event.get::<i32>().ok() {
    //                                     dbg!(event);
    //                                 }
    //                             }
    //                             None
    //                         };
    //                         obj.connect("channel-event", false, callback);
    //                         obj.connect("main-mouse-update", false, |values: &[Value]| {
    //                             dbg!("Main mouse update");
    //                             None
    //                         });
    //                     }

    //                     if channel_type == 2 {
    //                         if let Some(session) = weak_session.upgrade() {
    //                             let display_channel = DisplayChannel::from(obj.as_ptr() as *mut _);
    //                             display_channel.lock().unwrap().connect();
    //                             session
    //                                 .lock()
    //                                 .unwrap()
    //                                 .channels
    //                                 .push(Channel::DisplayChannel(display_channel));
    //                         }
    //                     }
    //                 }
    //             }
    //         }

    //         None
    //     });
    // }

    pub fn connect(&self) -> bool {
        let res = unsafe { spice_session_connect(self.inner.as_ptr() as *mut _) };
        res.is_positive()
    }
}
