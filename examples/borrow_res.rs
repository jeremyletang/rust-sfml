//! Example from SFML: borrow_res

extern crate sfml;

use sfml::graphics::{RenderWindow, Color, CircleShape, Sprite,
    RenderTarget, Texture, Text, Font, ConvexShape, Transformable};
use sfml::window::{VideoMode, ContextSettings, Event, window_style, Key};
use sfml::system::Vector2f;

fn main() {
    // Create the window of the application
    let mut window = RenderWindow::new(
        VideoMode::new(800, 600),
        "Borrow Resources - SFML Examples",
        window_style::CLOSE,
        &ContextSettings::default()).expect("Failed to create RenderWindow");
    window.set_vertical_sync_enabled(true);

    let clear_color = Color::black();

    // Create a new texture (hey frank !)
    let frank = Texture::new_from_file("resources/frank.jpeg").expect("Cannot found resource: frank.jpeg");
    // Create a font.
    let font = Font::new_from_file("resources/sansation.ttf").expect("Cannot found the font: sansation.ttf");

    // Create a circle with the Texture.
    let mut circle = CircleShape::new_with_texture(&frank).expect("Cannot create a new CircleShape");
    circle.set_radius(70.);
    circle.set_position2f(100., 100.);

    // Create a Sprite
    let mut sprite = Sprite::new().expect("Cannot create a new Sprite");
    // Set the same texture than the circle to the Sprite
    sprite.set_texture(&frank, true);
    sprite.set_position2f(400., 300.);
    sprite.set_scale2f(0.5, 0.5);

    // Create a convex_shape using the texture
    let convex_shape = ConvexShape::new_with_texture(vec![
        Vector2f::new(400., 100.),
        Vector2f::new(500., 70.),
        Vector2f::new(450., 100.),
        Vector2f::new(580., 150.),
        Vector2f::new(420., 230.),
        Vector2f::new(420., 120.),
    ], &frank).expect("Cannot create a ConvexShape");

    // Create an initialized text
    let title = Text::new_init("Borrow resources example!", &font, 50).expect("Cannot create a new text");

    // Create a Text an initialize it after
    let mut second_text = Text::new().expect("Cannot create a new font");
    second_text.set_string("This text shares the same font as the title!");
    second_text.set_font(&font);
    second_text.set_color(&Color::green());
    second_text.set_position2f(10., 350.);
    second_text.set_character_size(20);

    // another text
    let mut third_text = Text::new_init("This one too!", &font, 20).expect("Cannot create a new text");
    third_text.set_position2f(300., 100.);
    third_text.set_color(&Color::red());

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => ()
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
        window.display();
    }
}

