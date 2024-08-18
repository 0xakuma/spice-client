use spice_client::{canvas::MetalCanvas, connection::SpiceConnection, window::CocoaWindow};

fn main() {
    let canvas = MetalCanvas::new();
    let mut connection = SpiceConnection::new();
    connection.host("localhost");
    connection.port(5930);
    let window = CocoaWindow::new(connection, canvas);
    window.open();
}
