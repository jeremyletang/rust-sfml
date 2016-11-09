extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "Mouse events",
                                       window_style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_mouse_cursor_visible(false);
    window.set_framerate_limit(60);

    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let mut circle = CircleShape::new_init(4., 30);
    let mut texts: Vec<Text> = Vec::new();
    let mut mp_text = Text::new_init("", &font, 14);
    macro_rules! push_text {
        ($x:expr, $y:expr, $fmt:expr, $($arg:tt)*) => {
            let mut text = Text::new_init(&format!($fmt, $($arg)*), &font, 14);
            text.set_position2f($x as f32, $y as f32);
            texts.push(text);
        }
    }

    loop {
        for ev in window.events() {
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
                _ => {}
            }
        }

        let mp = window.mouse_position();
        let dmp = sfml::window::mouse::desktop_position();
        mp_text.set_string(&format!("x: {}, y: {} (Window)\nx:{}, y: {} (Desktop)",
                                    mp.x,
                                    mp.y,
                                    dmp.x,
                                    dmp.y));

        circle.set_position2f(mp.x as f32, mp.y as f32);

        window.clear(&Color::black());
        // Push texts out of each other's way
        for i in (0..texts.len()).rev() {
            for j in (0..i).rev() {
                if let Some(intersect) = texts[i]
                    .get_global_bounds()
                    .intersection(&texts[j].get_global_bounds()) {
                    texts[j].move2f(0., -intersect.height);
                }
            }
        }
        texts.retain(|txt| txt.get_color().0.a > 0);
        for txt in &mut texts {
            let mut color = txt.get_color();
            color.0.a -= 1;
            txt.set_color(&color);
            window.draw(txt);
        }
        window.draw(&circle);
        window.draw(&mp_text);
        window.display();
    }
}
