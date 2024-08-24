use std::sync::{Arc, Mutex};

use glib::{object::ObjectExt, Object, Value};

pub struct MainChannel {
    inner: Object,
    mouse_mode: i32,
}

impl MainChannel {
    pub fn from(inner: Object) -> Arc<Mutex<Self>> {
        let _self = Arc::new(Mutex::new(Self {
            inner,
            mouse_mode: 0,
        }));
        let __self = _self.clone();
        _self.lock().unwrap().inner.connect(
            "main-mouse-update",
            false,
            move |values: &[Value]| {
                dbg!("Main mouse update");
                let _channel = values.get(0);
                if let Some(channel) = _channel {
                    if let Some(obj) = channel.get::<Object>().ok() {
                        let mouse_mode = obj.property::<i32>("mouse-mode");
                        dbg!(mouse_mode);
                        __self.lock().unwrap().mouse_mode = mouse_mode;
                    }
                }
                None
            },
        );
        _self
    }

    pub fn is_mouse_mode_server(&self) -> bool {
        self.mouse_mode == 1
    }

    pub fn is_mouse_mode_client(&self) -> bool {
        self.mouse_mode == 2
    }
}

unsafe impl Send for MainChannel {}
unsafe impl Sync for MainChannel {}
