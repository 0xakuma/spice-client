use std::ffi::c_void;

use glib::{ffi::gboolean, Object};

extern "C" {
    pub fn spice_channel_connect(channel: *mut c_void) -> gboolean;
}

pub trait Channel: Send + Sync {
    fn connect(&self) -> bool;
    fn obj(&self) -> Object;
}
