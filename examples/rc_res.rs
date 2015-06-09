fn main() {}
/* This example does not work currently

//! Example from SFML: borrow_res

extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;

use sfml::graphics::{RenderWindow, Color, Texture, Font, RenderTarget};
use sfml::graphics::rc::{CircleShape, Sprite, ConvexShape, Text};
use sfml::window::{VideoMode, ContextSettings, event, WindowStyle};
use sfml::window::keyboard::Key;
use sfml::system::Vector2f;

fn main() {
    // Create the window of the application
    let setting: ContextSettings = ContextSettings::default();
    let mut window: RenderWindow = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
        "SFML borrow ressources Example", WindowStyle::Close, &setting) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };
    window.set_vertical_sync_enabled(true);

    let clear_color = Color::black();

    // Create a new reference counted texture
    let frank: Rc<RefCell<Texture>> = match Texture::new_from_file("resources/frank.jpeg") {
        Some(tex)   => Rc::new(RefCell::new(tex)),
        None        => panic!("Cannot found resource: frank.jpeg")
    };

    // Create a font.
    let font: Rc<RefCell<Font>> = match Font::new_from_file("resources/sansation.ttf") {
        Some(fnt)   => Rc::new(RefCell::new(fnt)),
        None        => panic!("Cannot found the font: sansation.ttf")
    };

    // Create a circle with the Texture.
    let mut circle = CircleShape::new_with_texture(frank.clone()).expect("Cannot create a new CircleShape");
    circle.set_radius(70f32);
    circle.set_position2f(100f32, 100f32);

    // Create a Sprite
    let mut sprite = Sprite::new().expect("Cannot create a new Sprite");
    // Set the same texture than the circle to the Sprite
    sprite.set_texture(frank.clone(), true);
    sprite.set_position2f(400f32, 300f32);
    sprite.set_scale2f(0.5f32, 0.5f32);

    // Create a convex_shape using the texture
    let mut convex_shape = ConvexShape::new_with_texture(frank.clone(), 6).expect("Cannot create a ConvexShape");
    convex_shape.set_point(0, &Vector2f{x:400f32, y:100f32});
    convex_shape.set_point(1, &Vector2f{x:500f32, y:70f32});
    convex_shape.set_point(2, &Vector2f{x:450f32, y:100f32});
    convex_shape.set_point(3, &Vector2f{x:580f32, y:150f32});
    convex_shape.set_point(4, &Vector2f{x:420f32, y:230f32});
    convex_shape.set_point(5, &Vector2f{x:420f32, y:120f32});

    // Create an initialized text
    let title = Text::new_init("Borrow ressources example!", font.clone(), 50).expect("Cannot create a new font");

    // Create a Text an initialize it after
    let mut second_text = Text::new().expect("Cannot create a new font");
    second_text.set_string("This text share the same font than the title !");
    second_text.set_font(font.clone());
    second_text.set_color(&Color::green());
    second_text.set_position2f(10f32, 350f32);
    second_text.set_character_size(20);

    // another text
    let mut third_text = Text::new_init("This one too!", font.clone(), 20).expect("Cannot create a new font");
    third_text.set_position2f(300f32, 100f32);
    third_text.set_color(&Color::red());

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
        window.clear(&clear_color);

        // Draw the stuff
        window.draw(&circle);
        window.draw(&sprite);
        window.draw(&convex_shape);
        window.draw(&title);
        window.draw(&second_text);
        window.draw(&third_text);
        // Display things on screen
        window.display()

    }
}
*/
