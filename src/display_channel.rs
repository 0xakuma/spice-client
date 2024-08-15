use std::ffi::c_void;

use glib::{object::ObjectExt, translate::FromGlibPtrNone, Object, Value};

use crate::channel::{spice_channel_connect, Channel};

pub struct DisplayChannel {
    inner: *mut c_void,
}

impl From<*mut c_void> for DisplayChannel {
    fn from(value: *mut c_void) -> Self {
        let display_channel = Self { inner: value };
        display_channel.handle_glib_events();
        display_channel
    }
}

impl Channel for DisplayChannel {
    fn obj(&self) -> glib::Object {
        unsafe { Object::from_glib_none(self.inner as *const _) }
    }
    fn connect(&self) -> bool {
        let rt = unsafe { spice_channel_connect(self.inner) };
        rt.is_positive()
    }
}

impl DisplayChannel {
    fn handle_glib_events(&self) {
        let obj = self.obj();

        obj.connect("display-primary-create", false, |values: &[Value]| {
            let display_channel = values.get(0);
            let format = values.get(1);
            let width = values.get(2);
            let shmid = values.get(5);

            let stride: i32 = {
                let val = if let Some(_stride) = values.get(4) {
                    let _val = if let Some(_stride) = _stride.get::<i32>().ok() {
                        _stride
                    } else {
                        0
                    };
                    _val
                } else {
                    0
                };
                val
            };

            let height: i32 = {
                let val = if let Some(_height) = values.get(3) {
                    let _val = if let Some(_height) = _height.get::<i32>().ok() {
                        _height
                    } else {
                        0
                    };
                    _val
                } else {
                    0
                };
                val
            };

            if let Some(imgdata) = values.get(6) {
                if let Some(imgdata) = imgdata.get::<*mut c_void>().ok() {
                    let img_slice = unsafe {
                        std::slice::from_raw_parts(imgdata as *const _, (stride * height) as usize)
                    };
                }
            }
            dbg!("Primary display created");
            None
        });

        obj.connect("display-primary-destroy", false, |values: &[Value]| {
            if let Some(_display_channel) = values.get(0) {
                if let Some(_display_channel) = _display_channel.get::<Object>().ok() {}
            }
            None
        });
    }
}

unsafe impl Send for DisplayChannel {}
unsafe impl Sync for DisplayChannel {}
