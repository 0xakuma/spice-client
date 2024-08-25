use std::sync::{Arc, Mutex};

use glib::{object::ObjectType, translate::FromGlibPtrNone, Object};

use crate::channel::spice_channel_connect;

pub struct CursorChannel {
    inner: Object,
}

impl CursorChannel {
    pub fn connect(&self) -> bool {
        let rt = unsafe { spice_channel_connect(self.inner.as_ptr() as *mut _) };
        rt.is_positive()
    }
    pub fn from(value: Object) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { inner: value }))
    }
}

unsafe impl Send for CursorChannel {}
unsafe impl Sync for CursorChannel {}
