use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::{translate::FromGlibPtrNone, Object};

pub struct MouseChannel {
    inner: *mut c_void,
}

impl MouseChannel {
    fn connect(&self) -> bool {
        false
    }
    fn from(value: *mut std::ffi::c_void) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { inner: value }))
    }
    fn obj(&self) -> glib::Object {
        unsafe { Object::from_glib_none(self.inner as *const _) }
    }
}

unsafe impl Send for MouseChannel {}
unsafe impl Sync for MouseChannel {}
