extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "◢◤ Unicode text entry ◥◣",
        Style::CLOSE,
        &Default::default(),
    );

    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let mut string = String::from("This text can be edited.\nTry it!");

    let mut text = Text::new(&string, &font, 24);
    text.set_fill_color(Color::RED);
    text.set_outline_color(Color::YELLOW);
    text.set_outline_thickness(2.0);
    println!(
        "== Text information ==\n\
         fill color: {:?}\n\
         outline color: {:?}\n\
         outline thickness: {:?}",
        text.fill_color(),
        text.outline_color(),
        text.outline_thickness()
    );

    'mainloop: loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => break 'mainloop,
                Event::TextEntered { unicode } => {
                    if unicode == 0x08 as char {
                        string.pop();
                    } else if unicode == 0xD as char {
                        string.push('\n');
                    }
                    // Ignore ctrl+v/ctrl+v generated chars
                    else if unicode != 0x16 as char && unicode != 0x03 as char {
                        string.push(unicode);
                    }
                    text.set_string(&string);
                }
                Event::KeyPressed {
                    code: Key::V,
                    ctrl: true,
                    ..
                } => {
                    let clipstring = clipboard::get_string();
                    string.push_str(&clipstring.to_rust_string());
                    text.set_string(&string);
                }
                Event::KeyPressed {
                    code: Key::C,
                    ctrl: true,
                    ..
                } => {
                    clipboard::set_string(text.string());
                }
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&text);
        window.display();
    }
    println!("The final text is {:?}", text.string().to_rust_string());
}
