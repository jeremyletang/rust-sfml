extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new(800, 600, 32),
                                       "Unicode text entry",
                                       style::CLOSE,
                                       &Default::default())
        .unwrap();

    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let mut string = String::from("This text can be edited.\nTry it!");

    let mut text = Text::new_init(&string, &font, 24);

    loop {
        for ev in window.events() {
            match ev {
                Event::Closed => return,
                Event::TextEntered { unicode } => {
                    if unicode == 0x08 as char {
                        string.pop();
                    } else if unicode == 0xD as char {
                        string.push('\n');
                    } else {
                        string.push(unicode);
                    }
                    text.set_string(&string);
                }
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&text);
        window.display();
    }
}
