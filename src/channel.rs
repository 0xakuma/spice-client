use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::ffi::gboolean;

use crate::{display_channel::DisplayChannel, mouse_channel::MouseChannel};

extern "C" {
    pub fn spice_channel_connect(channel: *mut c_void) -> gboolean;
}

pub enum Channel {
    DisplayChannel(Arc<Mutex<DisplayChannel>>),
    MouseChannel(Arc<Mutex<MouseChannel>>),
}
