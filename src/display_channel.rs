use std::{
    ffi::{c_int, c_void},
    sync::{Arc, Mutex},
};

use glib::{
    object::{ObjectExt, ObjectType},
    translate::FromGlibPtrNone,
    Object, Value,
};

use crate::channel::spice_channel_connect;

#[derive(Default, Clone, Copy)]
pub struct Display {
    pub canvas_img: Option<*mut c_void>,
    pub width: c_int,
    pub height: c_int,
    pub stride: c_int,
    pub format: c_int,
}

pub struct DisplayChannel {
    inner: Object,
    pub display: Option<Display>,
}

impl DisplayChannel {
    pub fn from(value: *mut c_void) -> Arc<Mutex<Self>> {
        let obj = unsafe { Object::from_glib_none(value as *const _) };
        let display_channel = Arc::new(Mutex::new(Self {
            inner: obj,
            display: None,
        }));
        let _display_channel = display_channel.clone();
        display_channel
            .lock()
            .unwrap()
            .handle_glib_events(_display_channel);
        display_channel
    }

    pub fn connect(&self) -> bool {
        let rt = unsafe { spice_channel_connect(self.inner.as_ptr() as *mut _) };
        rt.is_positive()
    }
    fn handle_glib_events(&self, _ref: Arc<Mutex<Self>>) {
        self.inner.connect(
            "display-primary-create",
            false,
            move |values: &[Value]| {
                // let shmid = values.get(5);
                dbg!("Primary display created");
                let mut display = Display::default();

                if let Some(_format) = values.get(1) {
                    if let Some(_format) = _format.get::<c_int>().ok() {
                        display.format = _format;
                    }
                }

                if let Some(_stride) = values.get(4) {
                    if let Some(_stride) = _stride.get::<c_int>().ok() {
                        display.stride = _stride;
                    }
                }

                if let Some(_width) = values.get(2) {
                    if let Some(_width) = _width.get::<c_int>().ok() {
                        display.width = _width;
                    }
                }

                if let Some(_height) = values.get(3) {
                    if let Some(_height) = _height.get::<i32>().ok() {
                        display.height = _height;
                    }
                }

                if let Some(imgdata) = values.get(6) {
                    if let Some(imgdata) = imgdata.get::<*mut c_void>().ok() {
                        display.canvas_img = Some(imgdata);
                    }
                }
                _ref.lock().unwrap().display = Some(display);
                None
            },
        );

        self.inner.connect("gl-draw", false, |values: &[Value]| {
            dbg!("GL scanout");
            None
        });

        self.inner
            .connect("display-invalidate", false, |values: &[Value]| {
                dbg!("Display invalidate");
                None
            });

        self.inner
            .connect("display-primary-destroy", false, |values: &[Value]| {
                dbg!("Display destroyed");
                if let Some(_display_channel) = values.get(0) {
                    if let Some(_display_channel) = _display_channel.get::<Object>().ok() {}
                }
                None
            });
    }

    pub fn display(&self) -> Option<Display> {
        self.display.clone()
    }
}

unsafe impl Send for DisplayChannel {}
unsafe impl Sync for DisplayChannel {}
