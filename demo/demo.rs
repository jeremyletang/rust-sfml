/*!
* Demo of rust-sfml
*
* Create an render window, load a Texture and display it with a sprite, load a text and display it.
*
*/


extern mod rsfml;
use rsfml::graphics::render_window::*;
use rsfml::window::video_mode::*;
use rsfml::window::context_settings::*;
use rsfml::system::vector2::*;
use rsfml::window::event;
use rsfml::graphics::text;
use rsfml::graphics::color::*;
use rsfml::graphics::font;
use rsfml::graphics::sprite;
use rsfml::graphics::texture;

fn main() {
    let mode = VideoMode::new_init(800, 600, 32);
    let setting: ContextSettings = ContextSettings{depthBits: 10, stencilBits: 10, antialiasingLevel: 1, majorVersion: 0, minorVersion: 1};
    let rwindow : @RenderWindow = match RenderWindow::new(mode, ~"Ma super window", sfClose, &setting) {
        Some(rwindow) => @rwindow,
        None => fail!(~"Error on creating window")
    };
    
    let text : @text::Text = match text::Text::new() {
        Some(text) => @text,
        None => fail!(~"Error on creating text")
    };
    let fnt : @font::Font = @font::Font::new_from_file(~"./res/o.ttf");
    

    rwindow.set_framerate_limit(30);
    text.set_string(~"A great rust-sfml Window");
    text.set_font(fnt);
    text.set_color(@Color::red());
    let texture : @texture::Texture = @texture::Texture::new_from_file(~"./res/raton1.jpeg");
    let sprite : @sprite::Sprite = @sprite::Sprite::new();
    sprite.set_texture(texture, false);
    sprite.set_position(@Vector2::new( 100., 100.));
    
    while rwindow.is_open() {
        loop {
            match rwindow.poll_event() {
                event::Closed => rwindow.close(),
                event::NoEvent => break,
                _ => {}
            }
        }
        rwindow.clear(@Color::green());
        rwindow.draw(sprite);
        rwindow.draw(text);
        rwindow.display();
    }
    rwindow.close();    
}

