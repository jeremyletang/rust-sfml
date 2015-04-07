//! Example from SFML: Shape

extern crate rsfml;

use rsfml::graphics::{RenderWindow, RenderTarget, Color,
                      VertexArray, Vertex, LinesStrip};
use rsfml::window::{VideoMode, ContextSettings, event, Close};
use rsfml::system::Vector2f;

fn main () -> () {
    // Create the window of the application
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML VertexArray accessors Example", Close, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = VertexArray::new().expect("Error, cannot create a VertexArray");
    vertex_array.set_primitive_type(LinesStrip);


    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f{x: 20f32, y: 30f32}, &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f{x: 30f32, y: 30f32}, &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f{x: 40f32, y: 40f32}, &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f{x: 50f32, y: 50f32}, &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f{x: 60f32, y: 60f32}, &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f{x: 50f32, y: 80f32}, &Color::green()));

    println!("\nIterate over the vertices of a VertexArray");
    for v in vertex_array.vertices() {
        println!("Vertex Color: {:?} | Position: {:?}", v.color, v.position)
    }

    println!("\nMutable access to a vertex");
    println!("Before Vertex Color: {:?} | Position: {:?}", vertex_array[1].color, vertex_array[1].position);
    vertex_array.get_vertex(1).position.x = 100f32;
    println!("After Vertex Color: {:?} | Position: {:?}", vertex_array[1].color, vertex_array[1].position);

    println!("\nImmutable access to a vertex");
    println!("Vertex Color: {:?} | Position: {:?}", vertex_array[1].color, vertex_array[1].position);

    while window.is_open() {
        for e in window.events() {
            if e == event::Closed { window.close() }
        }
        // Clear the window
        window.clear(&Color::black());
        window.draw(&vertex_array);
        // Display things on screen
        window.display()

    }
}

