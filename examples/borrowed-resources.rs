extern crate sfml;

use sfml::graphics::{CircleShape, Color, ConvexShape, Font, RenderTarget, RenderWindow, Sprite,
                     Text, Texture, Transformable};
use sfml::window::{Key, VideoMode, Event, window_style};
use sfml::system::Vector2f;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "Borrowed resources",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    // Create a new texture. (Hey Frank!)
    let frank = Texture::new_from_file("resources/frank.jpeg").unwrap();

    // Create a font.
    let font = Font::new_from_file("resources/sansation.ttf").unwrap();

    // Create a circle with the Texture.
    let mut circle = CircleShape::new_with_texture(&frank).unwrap();
    circle.set_radius(70f32);
    circle.set_position2f(100f32, 100f32);

    // Create a Sprite.
    let mut sprite = Sprite::new().unwrap();
    // Have it use the same texture as the circle.
    sprite.set_texture(&frank, true);
    sprite.set_position2f(400f32, 300f32);
    sprite.set_scale2f(0.5f32, 0.5f32);

    // Create a ConvexShape using the same texture.
    let mut convex_shape = ConvexShape::new_with_texture(&frank, 6).unwrap();
    convex_shape.set_point(0,
                           &Vector2f {
                               x: 400f32,
                               y: 100f32,
                           });
    convex_shape.set_point(1,
                           &Vector2f {
                               x: 500f32,
                               y: 70f32,
                           });
    convex_shape.set_point(2,
                           &Vector2f {
                               x: 450f32,
                               y: 100f32,
                           });
    convex_shape.set_point(3,
                           &Vector2f {
                               x: 580f32,
                               y: 150f32,
                           });
    convex_shape.set_point(4,
                           &Vector2f {
                               x: 420f32,
                               y: 230f32,
                           });
    convex_shape.set_point(5,
                           &Vector2f {
                               x: 420f32,
                               y: 120f32,
                           });

    // Create an initialized text using the font.
    let title = Text::new_init("Borrowed resources example!", &font, 50).unwrap();

    // Create a second text using the same font.
    // This time, we create and initialize it separately.
    let mut second_text = Text::new().unwrap();
    second_text.set_string("This text shares the same font with the title!");
    second_text.set_font(&font);
    second_text.set_color(&Color::green());
    second_text.set_position2f(10f32, 350f32);
    second_text.set_character_size(20);

    // Create a third text using the same font.
    let mut third_text = Text::new_init("This one too!", &font, 20).unwrap();
    third_text.set_position2f(300f32, 100f32);
    third_text.set_color(&Color::red());

    loop {
        for event in window.events() {
            match event {
                Event::Closed => return,
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&circle);
        window.draw(&sprite);
        window.draw(&convex_shape);
        window.draw(&title);
        window.draw(&second_text);
        window.draw(&third_text);
        window.display();
    }
}
