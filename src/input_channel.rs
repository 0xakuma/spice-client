use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::{translate::FromGlibPtrNone, Object};

extern "C" {
    pub fn spice_inputs_motion(channel: *const c_void, dx: i32, dy: i32, button_state: i32);
}

pub struct InputChannel {
    inner: Object,
    pub dy: i32,
    pub dx: i32,
}

impl InputChannel {
    pub fn from(inner: *mut c_void) -> Arc<Mutex<Self>> {
        let object = unsafe { Object::from_glib_none(inner as *const _) };
        Arc::new(Mutex::new(Self {
            inner: object,
            dx: 0,
            dy: 0,
        }))
    }

    pub fn move_cursor(&self, dx: i32, dy: i32) {}
    pub fn press_button(&self) {}
    pub fn release_button(&self) {}
    pub fn key_press(&self) {}
}

unsafe impl Send for InputChannel {}
unsafe impl Sync for InputChannel {}
