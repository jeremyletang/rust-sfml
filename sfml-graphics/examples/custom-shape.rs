extern crate sfml;
extern crate sfml_window;
extern crate sfml_graphics;

use sfml_graphics::{Color, CustomShape, RenderTarget, RenderWindow, ShapeImpl};
use sfml_window::{Key, VideoMode, Event, style};
use sfml::system::Vector2f;

#[derive(Clone, Copy)]
pub struct TriangleShape;

impl ShapeImpl for TriangleShape {
    fn get_point_count(&self) -> u32 {
        3
    }

    fn get_point(&self, point: u32) -> Vector2f {
        match point {
            0 => Vector2f { x: 20., y: 580. },
            1 => Vector2f { x: 400., y: 20. },
            2 => Vector2f { x: 780., y: 580. },
            p => panic!("Non-existent point: {}", p),
        }
    }
}

fn main() {
    let mut window = RenderWindow::new(VideoMode::new(800, 600, 32),
                                       "Custom shape",
                                       style::CLOSE,
                                       &Default::default())
        .unwrap();
    window.set_vertical_sync_enabled(true);

    let mut shape = CustomShape::new(Box::new(TriangleShape));
    shape.set_fill_color(&Color::red());
    shape.set_outline_color(&Color::green());
    shape.set_outline_thickness(3.);

    loop {
        for event in window.events() {
            match event {
                Event::Closed |
                Event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&shape);
        window.display();
    }
}
