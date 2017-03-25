extern crate sfml;

use sfml::graphics::{Color, LineStrip, RenderTarget, RenderWindow, Vertex, VertexArray};
use sfml::system::Vector2f;
use sfml::window::{VideoMode, Event, style};

fn main() {
    let mut window = RenderWindow::new(VideoMode::new(800, 600, 32),
                                       "SFML VertexArray accessors Example",
                                       style::CLOSE,
                                       &Default::default())
            .unwrap();
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = VertexArray::new();
    vertex_array.set_primitive_type(LineStrip);

    vertex_array.append(&Vertex::with_pos_color(Vector2f { x: 20.0, y: 30.0 }, Color::green()));
    vertex_array.append(&Vertex::with_pos_color(Vector2f { x: 30.0, y: 30.0 }, Color::green()));
    vertex_array.append(&Vertex::with_pos_color(Vector2f { x: 40.0, y: 40.0 }, Color::green()));
    vertex_array.append(&Vertex::with_pos_color(Vector2f { x: 50.0, y: 50.0 }, Color::green()));
    vertex_array.append(&Vertex::with_pos_color(Vector2f { x: 60.0, y: 60.0 }, Color::green()));
    vertex_array.append(&Vertex::with_pos_color(Vector2f { x: 50.0, y: 80.0 }, Color::green()));

    println!("\nIterate over the vertices of a VertexArray");
    for v in vertex_array.vertices() {
        println!("Vertex Color: {:?} | Position: {:?}", v.color, v.position)
    }

    println!("\nMutable access to a vertex");
    println!("Before Vertex Color: {:?} | Position: {:?}",
             vertex_array[1].color,
             vertex_array[1].position);
    vertex_array.vertex(1).position.x = 100.0;
    println!("After Vertex Color: {:?} | Position: {:?}",
             vertex_array[1].color,
             vertex_array[1].position);

    println!("\nImmutable access to a vertex");
    println!("Vertex Color: {:?} | Position: {:?}",
             vertex_array[1].color,
             vertex_array[1].position);

    loop {
        for e in window.events() {
            if e == Event::Closed {
                return;
            }
        }
        // Clear the window
        window.clear(&Color::black());
        window.draw(&vertex_array);
        // Display things on screen
        window.display()

    }
}
