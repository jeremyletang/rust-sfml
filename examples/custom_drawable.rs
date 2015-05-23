//! Example from SFML: Custom drawable

extern crate sfml;

use sfml::graphics::{RenderWindow, Color, CircleShape, RectangleShape,
                      RenderTarget, RenderStates, Drawable, Shape, Transformable};
use sfml::window::{VideoMode, ContextSettings, event, WindowStyle};
use sfml::window::keyboard::Key;
use sfml::system::Vector2f;

// Create a struct who contains two drawable for the example
struct CustomDrawable<'s> {
    circle:    CircleShape<'s>,
    rect:      RectangleShape<'s>
}

impl<'s> CustomDrawable<'s> {
    pub fn new() -> CustomDrawable<'s> {
        let mut c = CircleShape::new_init(50f32, 50).expect("Cannot create the CircleShape");
        c.set_position2f(100f32, 100f32);
        c.set_fill_color(&Color::red());
        let mut r = RectangleShape::new_init(&Vector2f {x: 100f32, y: 200f32}).expect("Cannot create the RectangleShape");
        r.set_position2f(100f32, 150f32);
        r.set_fill_color(&Color::blue());

        CustomDrawable {
            circle: c,
            rect:   r
        }
    }
}

// Implements the drawable trait, only this function is mendatory.
impl<'s> Drawable for CustomDrawable<'s> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, _: &mut RenderStates) {
        render_target.draw(&self.circle);
        render_target.draw(&self.rect)
    }
}

fn main() {
    // Create the window of the application
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Shape Example", WindowStyle::Close, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    // create our Drawable
    let custom_drawable = CustomDrawable::new();

    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => {
                        window.close();
                        break;
                    },
                    _ => {}
                },
                _ => {}
            }
        }
        // Clear the window
        window.clear(&Color::black());

        // Draw it ! yeah you create a custome shape!
        window.draw(&custom_drawable);

        // Display things on screen
        window.display()

    }
}

