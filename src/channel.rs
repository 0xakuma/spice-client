use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use glib::ffi::gboolean;

use crate::{
    cursor_channel::CursorChannel, display_channel::DisplayChannel, input_channel::InputChannel,
    main_channel::MainChannel,
};

extern "C" {
    pub fn spice_channel_connect(channel: *mut c_void) -> gboolean;
}

pub enum Channel {
    DisplayChannel(Arc<Mutex<DisplayChannel>>),
    CursorChannel(Arc<Mutex<CursorChannel>>),
    InputChannel(Arc<Mutex<InputChannel>>),
    MainChannel(Arc<Mutex<MainChannel>>),
}
