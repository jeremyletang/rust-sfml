//! Example from SFML: Shape

extern crate sfml;

use sfml::graphics::{RenderWindow, Color, BaseShape, RenderTarget, ShapeImpl, Shape};
use sfml::window::{VideoMode, ContextSettings, Event, window_style, Key};
use sfml::system::Vector2f;

#[derive(Clone, Copy)]
pub struct MyShape;

impl ShapeImpl for MyShape {
    fn get_point_count(&self) -> u32 {
        3
    }

    fn get_point(&self, point: u32) -> Vector2f {
        match point {
            0 => Vector2f {x: 10., y: 10.},
            1 => Vector2f {x: 100., y: 100.},
            2 => Vector2f {x: 200., y: 300.},
            _ => panic!("error")
        }
    }
}

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600),
        "SFML Shape Example",
        window_style::CLOSE,
        ContextSettings::default()).expect("Cannot create a new Render Window.");
    window.set_vertical_sync_enabled(true);

    let my_shape = MyShape;
    let mut shape = BaseShape::new(&my_shape).expect("Error, cannot create a Shape");
    shape.set_fill_color(Color::red());
    shape.set_outline_color(Color::green());
    shape.set_outline_thickness(3.);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => {}
            }
        }
        // Clear the window
        window.clear(Color::black());
        window.draw(&shape);
        // Display things on screen
        window.display();
    }
}

