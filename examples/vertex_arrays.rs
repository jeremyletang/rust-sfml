//! Example from SFML: Shape

extern crate sfml;

use sfml::graphics::{RenderWindow, RenderTarget, Color,
                     VertexArray, Vertex, PrimitiveType};
use sfml::window::{VideoMode, ContextSettings, Event, window_style};
use sfml::system::Vector2f;

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600),
        "VertexArray accessors - SFML Examples",
        window_style::CLOSE,
        ContextSettings::default()).expect("Cannot create a new Render Window.");
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = VertexArray::new(PrimitiveType::LinesStrip);
    vertex_array.push(Vertex::new_with_pos_color(Vector2f::new(20., 30.), Color::green()));
    vertex_array.push(Vertex::new_with_pos_color(Vector2f::new(30., 30.), Color::green()));
    vertex_array.push(Vertex::new_with_pos_color(Vector2f::new(40., 40.), Color::green()));
    vertex_array.push(Vertex::new_with_pos_color(Vector2f::new(50., 50.), Color::green()));
    vertex_array.push(Vertex::new_with_pos_color(Vector2f::new(60., 60.), Color::green()));
    vertex_array.push(Vertex::new_with_pos_color(Vector2f::new(50., 80.), Color::green()));

    println!("\nIterate over the vertices of a VertexArray");
    for v in vertex_array.iter() {
        println!("Vertex Color: {:?} | Position: {:?}", v.color, v.position)
    }

    println!("\nMutable access to a vertex");
    println!("Before Vertex Color: {:?} | Position: {:?}", vertex_array[1].color, vertex_array[1].position);
    vertex_array[1].position.x = 100f32;
    println!("After Vertex Color: {:?} | Position: {:?}", vertex_array[1].color, vertex_array[1].position);

    println!("\nImmutable access to a vertex");
    println!("Vertex Color: {:?} | Position: {:?}", vertex_array[1].color, vertex_array[1].position);

    while window.is_open() {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed { window.close() }
        }
        // Clear the window
        window.clear(Color::black());
        window.draw(&vertex_array);
        // Display things on screen
        window.display()
    }
}

