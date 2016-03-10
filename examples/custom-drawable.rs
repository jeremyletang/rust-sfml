extern crate sfml;

use sfml::graphics::{CircleShape, Color, Drawable, RectangleShape, RenderStates, RenderTarget,
                     RenderWindow, Shape, Transformable};
use sfml::window::{Key, VideoMode, event, window_style};
use sfml::system::Vector2f;

/// Our custom drawable type. It looks like a bullet.
struct Bullet<'s> {
    head: CircleShape<'s>,
    torso: RectangleShape<'s>,
}

impl<'s> Bullet<'s> {
    pub fn new() -> Self {
        let mut head = CircleShape::new_init(50f32, 50).unwrap();
        head.set_position2f(100f32, 100f32);
        head.set_fill_color(&Color::red());
        let mut torso = RectangleShape::new_init(&Vector2f {
                            x: 100f32,
                            y: 200f32,
                        })
                            .unwrap();
        torso.set_position2f(100f32, 150f32);
        torso.set_fill_color(&Color::blue());

        Bullet {
            head: head,
            torso: torso,
        }
    }
}

// Implement the Drawable trait for our custom drawable.
impl<'s> Drawable for Bullet<'s> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, _: &mut RenderStates) {
        render_target.draw(&self.head);
        render_target.draw(&self.torso)
    }
}

fn main() {
    let mut window = RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                       "Custom drawable",
                                       window_style::CLOSE,
                                       &Default::default())
                         .unwrap();
    window.set_vertical_sync_enabled(true);

    let bullet = Bullet::new();

    loop {
        for event in window.events() {
            match event {
                event::Closed => return,
                event::KeyPressed { code: Key::Escape, .. } => return,
                _ => {}
            }
        }

        window.clear(&Color::black());
        window.draw(&bullet);
        window.display()
    }
}
