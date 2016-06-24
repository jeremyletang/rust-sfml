extern crate sfml;

use sfml::graphics::{Color, LinesStrip, RenderTarget, RenderWindow, Vertex, VertexArray};
use sfml::window::{VideoMode, Event, window_style};
use sfml::system::Vector2f;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "SFML VertexArray accessors Example",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = VertexArray::new().unwrap();
    vertex_array.set_primitive_type(LinesStrip);

    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {
                                                        x: 20f32,
                                                        y: 30f32,
                                                    },
                                                    &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {
                                                        x: 30f32,
                                                        y: 30f32,
                                                    },
                                                    &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {
                                                        x: 40f32,
                                                        y: 40f32,
                                                    },
                                                    &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {
                                                        x: 50f32,
                                                        y: 50f32,
                                                    },
                                                    &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {
                                                        x: 60f32,
                                                        y: 60f32,
                                                    },
                                                    &Color::green()));
    vertex_array.append(&Vertex::new_with_pos_color(&Vector2f {
                                                        x: 50f32,
                                                        y: 80f32,
                                                    },
                                                    &Color::green()));

    println!("\nIterate over the vertices of a VertexArray");
    for v in vertex_array.vertices() {
        println!("Vertex Color: {:?} | Position: {:?}",
                 v.0.color,
                 v.0.position)
    }

    println!("\nMutable access to a vertex");
    println!("Before Vertex Color: {:?} | Position: {:?}",
             vertex_array[1].0.color,
             vertex_array[1].0.position);
    vertex_array.get_vertex(1).0.position.x = 100f32;
    println!("After Vertex Color: {:?} | Position: {:?}",
             vertex_array[1].0.color,
             vertex_array[1].0.position);

    println!("\nImmutable access to a vertex");
    println!("Vertex Color: {:?} | Position: {:?}",
             vertex_array[1].0.color,
             vertex_array[1].0.position);

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
