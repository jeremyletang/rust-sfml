use sfml::{SfResult, graphics::{RenderWindow, Font, CircleShape, Text, Transformable, RenderTarget, Color, Shape, Drawable}, system::Vector2i, window::{Style, Event, Key, VideoMode, mouse}};

include!("../example_common.rs");

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut window = RenderWindow::new(
        (800, 600),
        "Mouse events",
        Style::CLOSE,
        Default::default(),
        &Default::default(),
    )?;
    window.set_mouse_cursor_visible(false);
    window.set_vertical_sync_enabled(true);

    let font = Font::from_file("sansation.ttf")?;
    let mut circle = CircleShape::new(4., 30);
    let mut texts: Vec<Text> = Vec::new();
    let mut mp_text = Text::new("", &font, 14);
    let mut cursor_visible = false;
    let mut grabbed = false;
    macro_rules! push_text {
        ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*) => {
            let mut text = Text::new(&format!($fmt, $($arg)*), &font, 14);
            text.set_position(($x as f32, $y as f32));
            texts.push(text);
        }
    }

    'mainloop: loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => break 'mainloop,
                Event::MouseWheelScrolled {
                    wheel,
                    delta,
                    position,
                } => {
                    push_text!(
                        position.x,
                        position.y,
                        "Scroll: {:?}, {}, {}, {}",
                        wheel,
                        delta,
                        position.x,
                        position.y
                    );
                }
                Event::MouseButtonPressed { button, position } => {
                    push_text!(
                        position.x,
                        position.y,
                        "Press: {:?}, {}, {}",
                        button,
                        position.x,
                        position.y
                    );
                }
                Event::MouseButtonReleased { button, position } => {
                    push_text!(
                        position.x,
                        position.y,
                        "Release: {:?}, {}, {}",
                        button,
                        position.x,
                        position.y
                    );
                }
                Event::KeyPressed { code, .. } => {
                    if code == Key::W {
                        window.set_mouse_position(Vector2i::new(400, 300));
                    } else if code == Key::D {
                        let dm = VideoMode::desktop_mode();
                        let center = Vector2i::new(dm.size.x as i32 / 2, dm.size.y as i32 / 2);
                        mouse::set_desktop_position(center);
                    } else if code == Key::V {
                        cursor_visible = !cursor_visible;
                        window.set_mouse_cursor_visible(cursor_visible);
                    } else if code == Key::G {
                        grabbed = !grabbed;
                        window.set_mouse_cursor_grabbed(grabbed);
                    }
                }
                _ => {}
            }
        }

        let mp = window.mouse_position();
        let dmp = mouse::desktop_position();
        let cur_vis_msg = if cursor_visible {
            "visible"
        } else {
            "invisible"
        };
        let grab_msg = if grabbed { "grabbed" } else { "not grabbed" };
        mp_text.set_string(&format!(
            "x: {}, y: {} (Window)\n\
             x: {}, y: {} (Desktop)\n\
             [{cur_vis_msg}] [{grab_msg}] ('V'/'G') to toggle\n\
             'W' to center mouse on window\n\
             'D' to center mouse on desktop",
            mp.x, mp.y, dmp.x, dmp.y
        ));

        circle.set_position((mp.x as f32, mp.y as f32));

        window.clear(Color::BLACK);
        // Push texts out of each other's way
        for i in (0..texts.len()).rev() {
            for j in (0..i).rev() {
                if let Some(intersect) = texts[i]
                    .global_bounds()
                    .intersection(&texts[j].global_bounds())
                {
                    texts[j].move_((0., -intersect.size.y));
                }
            }
        }
        texts.retain(|txt| txt.fill_color().a > 0);
        for txt in &mut texts {
            let mut color = txt.fill_color();
            color.a -= 1;
            txt.set_fill_color(color);
            window.draw(txt);
        }
        if !cursor_visible {
            window.draw(&circle);
        }
        window.draw(&mp_text);
        window.display();
    }
    Ok(())
}
