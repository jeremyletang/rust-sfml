use sfml::{
    graphics::{
        CircleShape, Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow,
        Shape, Transformable,
    },
    window::{Event, Key, Style},
};

/// Our custom drawable type. It looks like a bullet.
struct Bullet<'s> {
    head: CircleShape<'s>,
    torso: RectangleShape<'s>,
}

impl Bullet<'_> {
    pub fn new() -> Self {
        let mut head = CircleShape::new(50.0, 50);
        head.set_position((100.0, 100.0));
        head.set_fill_color(Color::RED);
        let mut torso = RectangleShape::with_size((100., 200.).into());
        torso.set_position((100.0, 150.0));
        torso.set_fill_color(Color::BLUE);

        Self { head, torso }
    }
}

// Implement the Drawable trait for our custom drawable.
impl Drawable for Bullet<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.head);
        render_target.draw(&self.torso)
    }
}

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Custom drawable",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let bullet = Bullet::new();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => return,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        window.draw(&bullet);
        window.display()
    }
}
