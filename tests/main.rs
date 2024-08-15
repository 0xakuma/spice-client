use spice_client::{canvas::MetalCanvas, session::Session, window::CocoaWindow};

fn main() {
    let canvas = MetalCanvas::new();
    let window = CocoaWindow::new(canvas);
    window.open();
}
