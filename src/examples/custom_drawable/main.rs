/*!
* Example from SFML : Custom drawable
*/

#![crate_id = "custom_drawable"]
#![desc = "Custom drawable example for rsfml"]
#![crate_type = "bin"]

extern crate native;
extern crate rsfml;

use rsfml::graphics::{RenderWindow, RenderTexture, Color, CircleShape, RectangleShape};
use rsfml::window::{VideoMode, ContextSettings, event, keyboard, Close};
use rsfml::system::Vector2f;
use rsfml::traits::Drawable;

#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

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

// Implements the drawable trait, only these two method are mendatory.
impl<'s> Drawable for CustomDrawable<'s> {
    fn draw_in_render_window(&self, render_window : &mut RenderWindow) -> () {
        render_window.draw(&self.circle);
        render_window.draw(&self.rect)
    }

    fn draw_in_render_texture(&self, render_texture : &mut RenderTexture) -> () {
        render_texture.draw(&self.circle);
        render_texture.draw(&self.rect)
    }
}

fn main () -> () {
    // Create the window of the application
    let setting : ContextSettings = ContextSettings::default();
    let mut window : RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32), "SFML Shape Example", Close, &setting) {
        Some(window) => window,
        None => fail!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    // create our Drawable
    let custom_drawable = CustomDrawable::new();

    while window.is_open() {
        loop {
            match window.poll_event() {
                event::Closed               => window.close(),
                event::KeyPressed{code, ..} => match code {
                    keyboard::Escape    => {window.close(); break},
                    _                   => {}
                },
                event::NoEvent              => break,
                _                           => {}
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

