use sfml::{graphics::*, system::*, window::*};

include!("../example_common.rs");

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Mouse events",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_mouse_cursor_visible(false);
    window.set_vertical_sync_enabled(true);

    let font = Font::from_file(example_res!("sansation.ttf")).unwrap();
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

    loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => return,
                Event::MouseWheelScrolled { wheel, delta, x, y } => {
                    push_text!(x, y, "Scroll: {:?}, {}, {}, {}", wheel, delta, x, y);
                }
                Event::MouseButtonPressed { button, x, y } => {
                    push_text!(x, y, "Press: {:?}, {}, {}", button, x, y);
                }
                Event::MouseButtonReleased { button, x, y } => {
                    push_text!(x, y, "Release: {:?}, {}, {}", button, x, y);
                }
                Event::KeyPressed { code, .. } => {
                    if code == Key::W {
                        window.set_mouse_position(Vector2i::new(400, 300));
                    } else if code == Key::D {
                        let dm = VideoMode::desktop_mode();
                        let center = Vector2i::new(dm.width as i32 / 2, dm.height as i32 / 2);
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
             [{}] [{}] ('V'/'G') to toggle\n\
             'W' to center mouse on window\n\
             'D' to center mouse on desktop",
            mp.x, mp.y, dmp.x, dmp.y, cur_vis_msg, grab_msg
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
                    texts[j].move_((0., -intersect.height));
                }
            }
        }
        texts.retain(|txt| txt.fill_color().alpha() > 0);
        for txt in &mut texts {
            let mut color = txt.fill_color();
            *color.alpha_mut() -= 1;
            txt.set_fill_color(color);
            window.draw(txt);
        }
        if !cursor_visible {
            window.draw(&circle);
        }
        window.draw(&mp_text);
        window.display();
    }
}
