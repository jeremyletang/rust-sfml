/*
 * The example found in the official SFML documentation, ported to Rust.
 */

extern mod rsfml;

pub use rsfml::window::context_settings::*;
pub use rsfml::window::event::*;
pub use rsfml::window::video_mode::*;
pub use rsfml::window::window::*;

pub use rsfml::graphics::color::*;
pub use rsfml::graphics::font::*;
pub use rsfml::graphics::render_window::*;
pub use rsfml::graphics::sprite::*;
pub use rsfml::graphics::text::*;
pub use rsfml::graphics::texture::*;

pub use rsfml::audio::music::*;

fn main() {
    // Create the main window
    let settings = ContextSettings{
        depthBits: 10,
        stencilBits: 10,
        antialiasingLevel: 1,
        majorVersion: 0,
        minorVersion: 1
    };
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32), ~"Test", sfClose, &settings).unwrap();

    // Load a sprite to display
    let texture = match Texture::new_from_file(~"cute_image.jpg") {
        Some(t) => t,
        None    => fail!("Couldn't load image")
    };
    let mut sprite = Sprite::new();
    sprite.set_texture(&texture, true);

    // Create a graphical text to display
    let font = match Font::new_from_file(~"arial.ttf") {
        Some(f) => f,
        None    => fail!("Couldn't load font")
    };
    let mut text = Text::new().unwrap();
    text.set_string(~"Hello World!");
    text.set_font(&font);
    text.set_character_size(50);

    // Load a music to play
    let mut music = match Music::new_from_file(~"nice_music.ogg") {
        Some(m) => m,
        None    => fail!("Couldn't load music")
    };

    // Play the music
    music.play();

    // Start the game loop
    let mut running = true;
    while running {
        // Process events
        for window.each_event |ev| {
            match ev {
                Closed => running = false,
                _      => ()
            }
        }

        // Clear screen
        window.clear(&Color::black());

        // Draw the sprite
        window.draw(&sprite);

        // Draw the string
        window.draw(&text);

        // Update the window
        window.display();
    }
}