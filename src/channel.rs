use std::{ffi::c_void, os::raw::c_int};

use gio::Socket;
use glib::{
    ffi::{gboolean, gpointer},
    object::ObjectExt,
    translate::FromGlibPtrNone,
    value::ToValue,
    Object, Value,
};

extern "C" {
    pub fn spice_channel_new(session: gpointer, _type: c_int, _id: c_int) -> *mut c_void;
    pub fn spice_channel_open_fd(channel: *const c_void, fd: c_int) -> gboolean;
}

pub struct Channel {
    inner: *mut c_void,
}

impl Channel {
    pub fn new(_session: *mut c_void, _type: i32, id: i32) -> Self {
        let _channel = unsafe { spice_channel_new(_session, _type, id) };
        Self { inner: _channel }
    }

    pub fn obj(&self) -> Object {
        unsafe { glib::Object::from_glib_none(self.inner as *const _) }
    }

    pub fn socket(&self, socket: &Socket) {
        let obj = self.obj();
        obj.set_property("socket", &socket.to_value())
    }

    pub fn signal<F>(&self, signal_name: &str, callback: F)
    where
        F: Fn(&[Value]) -> Option<Value> + Send + 'static + Sync + Copy,
    {
        let obj = self.obj();
        obj.connect(signal_name, false, callback);
    }

    pub fn open_fd(&self, fd: i32) -> bool {
        let rt = unsafe { spice_channel_open_fd(self.inner, fd) };
        rt.is_positive()
    }
}
