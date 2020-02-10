extern crate sfml;

use sfml::{
    graphics::{Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, VertexArray},
    window::{Event, Style},
};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML VertexArray accessors Example",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = VertexArray::default();
    vertex_array.set_primitive_type(PrimitiveType::LineStrip);

    vertex_array.append(&Vertex::with_pos_color((20.0, 30.0), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((30.0, 30.0), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((40.0, 40.0), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((50.0, 50.0), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((60.0, 60.0), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((50.0, 80.0), Color::GREEN));

    println!("\nIterate over the vertices of a VertexArray");
    for v in vertex_array.vertices() {
        println!("Vertex Color: {:?} | Position: {:?}", v.color, v.position)
    }

    println!("\nMutable access to a vertex");
    println!(
        "Before Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );
    vertex_array[1].position.x = 100.0;
    println!(
        "After Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );

    println!("\nImmutable access to a vertex");
    println!(
        "Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );

    loop {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed {
                return;
            }
        }
        // Clear the window
        window.clear(Color::BLACK);
        window.draw(&vertex_array);
        // Display things on screen
        window.display()
    }
}
