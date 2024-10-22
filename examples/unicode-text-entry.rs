use {
    sfml::{graphics::*, window::*, SfResult},
    std::usize,
};

include!("../example_common.rs");

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut window = RenderWindow::new(
        (800, 600),
        "◢◤ Unicode text entry ◥◣",
        Style::CLOSE,
        &Default::default(),
    )?;
    window.set_vertical_sync_enabled(true);

    // Showcase delayed initialization of font
    let mut font = Font::new()?;

    match std::env::args().nth(1) {
        Some(path) => font.load_from_file(&path)?,
        None => font.load_from_memory_static(include_bytes!("resources/sansation.ttf"))?,
    };
    let mut string = String::from("This text can be edited.\nTry it!");

    let mut text = Text::new(&string, &font, 24);
    text.set_fill_color(Color::RED);
    text.set_outline_color(Color::YELLOW);
    text.set_outline_thickness(2.0);
    let mut status_text = Text::new("", &font, 16);
    status_text.set_position((0., window.size().y as f32 - 64.0));
    let mut bold = false;
    let mut italic = false;
    let mut underlined = false;
    let mut strikethrough = false;
    let mut show_cursor = true;

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
                    string.push_str(&clipboard::get_string());
                    text.set_string(&string);
                }
                Event::KeyPressed {
                    code: Key::C,
                    ctrl: true,
                    ..
                } => {
                    clipboard::set_string(text.string());
                }
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => break 'mainloop,
                        Key::F1 => bold ^= true,
                        Key::F2 => italic ^= true,
                        Key::F3 => underlined ^= true,
                        Key::F4 => strikethrough ^= true,
                        Key::F5 => show_cursor ^= true,
                        _ => {}
                    }
                    let mut style = TextStyle::default();
                    if bold {
                        style |= TextStyle::BOLD;
                    }
                    if italic {
                        style |= TextStyle::ITALIC;
                    }
                    if underlined {
                        style |= TextStyle::UNDERLINED;
                    }
                    if strikethrough {
                        style |= TextStyle::STRIKETHROUGH;
                    }
                    text.set_style(style);
                }
                _ => {}
            }
        }

        let status_string = {
            let fc = text.fill_color();
            let oc = text.outline_color();
            format!(
            "fill: {:02x}{:02x}{:02x}{:02x} outline: {:02x}{:02x}{:02x}{:02x} outline thickness: {}\n\
            style: {:?} (F1-F4) cursor: {} (F5)\n\
            font family: {}",
            fc.r, fc.g, fc.b, fc.a,
            oc.r, oc.g, oc.b, oc.a,
            text.outline_thickness(),
            text.style(),
            show_cursor,
            font.info().family
        )
        };
        status_text.set_string(&status_string);

        window.clear(Color::BLACK);
        window.draw(&text);
        if show_cursor {
            let mut end = text.find_character_pos(usize::MAX);
            end.x += 2.0;
            end.y += 2.0;
            let mut rs = RectangleShape::new();
            rs.set_fill_color(Color::TRANSPARENT);
            rs.set_outline_color(Color::YELLOW);
            rs.set_outline_thickness(-3.0);
            rs.set_position(end);
            rs.set_size((8.0, 24.0));
            window.draw(&rs);
        }
        window.draw(&status_text);
        window.display();
    }
    println!("The final text is {:?}", text.string().to_rust_string());
    Ok(())
}
