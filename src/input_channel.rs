use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::{object::ObjectType, translate::FromGlibPtrNone, Object};

use crate::channel::spice_channel_connect;

extern "C" {
    pub fn spice_inputs_motion(channel: *const c_void, dx: i32, dy: i32, button_state: i32);
    pub fn spice_inputs_button_press(channel: *const c_void, button: i32, button_state: i32);
    pub fn spice_inputs_button_release(channel: *const c_void, button: i32, button_state: i32);
    pub fn spice_inputs_key_press(channel: *const c_void, scancode: u32);
    pub fn spice_inputs_key_release(channel: *const c_void, scancode: u32);
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

    pub fn move_cursor(&self, dx: f64, dy: f64) {
        unsafe {
            spice_inputs_motion(self.inner.as_ptr() as *const _, dx as i32, dy as i32, 0);
        };
    }

    pub fn press_button(&self, button: i32, button_state: i32) {
        unsafe { spice_inputs_button_press(self.inner.as_ptr() as *const _, button, button_state) };
    }
    pub fn release_button(&self, button: i32, button_state: i32) {
        unsafe {
            spice_inputs_button_release(self.inner.as_ptr() as *const _, button, button_state);
        };
    }
    pub fn key_press(&self, key: u32) {
        unsafe {
            spice_inputs_key_press(self.inner.as_ptr() as *const _, key);
        };
    }

    pub fn key_release(&self, key: u32) {
        unsafe { spice_inputs_key_release(self.inner.as_ptr() as *const _, key) };
    }

    pub fn connect(&self) -> bool {
        let rt = unsafe { spice_channel_connect(self.inner.as_ptr() as *mut _) };
        rt.is_positive()
    }
}

unsafe impl Send for InputChannel {}
unsafe impl Sync for InputChannel {}
