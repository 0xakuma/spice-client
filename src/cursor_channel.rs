use std::sync::{Arc, Mutex};

use glib::{translate::FromGlibPtrNone, Object};

pub struct CursorChannel {
    inner: Object,
}

impl CursorChannel {
    fn connect(&self) -> bool {
        false
    }
    pub fn from(value: Object) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self { inner: value }))
    }
}

unsafe impl Send for CursorChannel {}
unsafe impl Sync for CursorChannel {}
