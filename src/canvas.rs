use std::path::PathBuf;

use metal::{Device, Library};

pub trait Canvas {}

pub struct MetalCanvas {
    device: Device,
    library: Library,
}

impl Canvas for MetalCanvas {}

impl MetalCanvas {
    pub fn new() -> Self {
        let device = Device::system_default().expect("No device found");
        let library = device
            .new_library_with_file(
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shader.metallib"),
            )
            .unwrap();

        Self { device, library }
    }
}
