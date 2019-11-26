extern crate sfml;

use sfml::graphics::{
    Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, VertexBuffer, VertexBufferUsage,
};
use sfml::window::{Event, Style};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML VertexBuffer accessors Example",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut vertex_buffer =
        VertexBuffer::new(PrimitiveType::LineStrip, 6, VertexBufferUsage::Static);

    let vertices = vec![
        Vertex::with_pos_color((20.0, 30.0), Color::GREEN),
        Vertex::with_pos_color((30.0, 30.0), Color::GREEN),
        Vertex::with_pos_color((40.0, 40.0), Color::GREEN),
        Vertex::with_pos_color((50.0, 50.0), Color::GREEN),
        Vertex::with_pos_color((60.0, 60.0), Color::GREEN),
        Vertex::with_pos_color((50.0, 80.0), Color::GREEN),
    ];
    vertex_buffer.update(&vertices, 0);

    loop {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed {
                return;
            }
        }
        // Clear the window
        window.clear(Color::BLACK);
        window.draw(&vertex_buffer);
        // Display things on screen
        window.display()
    }
}
