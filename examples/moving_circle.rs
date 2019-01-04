extern crate sfml;

use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape};

use sfml::window::{Event, Key, Style};

use sfml::system::Vector2f;

use sfml::graphics::Transformable;

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut shape = CircleShape::new(32.0, 56);
    shape.set_fill_color(&Color::RED);
    shape.set_outline_color(&Color::GREEN);
    shape.set_outline_thickness(1.);

    let mut position = Vector2f { x: 100., y: 250. };

    loop {
        window.clear(&Color::BLACK);

        shape.set_position(position);
        window.draw(&shape);

        window.display();

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,

                Event::KeyPressed { code: Key::D, .. } => position.x += 5.0,
                Event::KeyPressed { code: Key::A, .. } => position.x -= 5.0,
                Event::KeyPressed { code: Key::S, .. } => position.y += 5.0,
                Event::KeyPressed { code: Key::W, .. } => position.y -= 5.0,
                _ => {}
            }
        }

        window.clear(&Color::BLACK);
        window.draw(&shape);
        window.display();
    }
}
