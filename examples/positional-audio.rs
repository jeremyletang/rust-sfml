use {
    sfml::{
        audio::{Music, listener, sound_source::SoundSource},
        graphics::{
            CircleShape, Color, FloatRect, Font, RectangleShape, RenderStates, RenderTarget,
            RenderWindow, Shape, Text, Transformable,
        },
        system::{Angle, Clock, Vector2, Vector2f, Vector3},
        window::{Event, Key, Style},
    },
    std::error::Error,
};

include!("../example_common.rs");

/// Convert a line between two points to an equivalent rotated rectangle
fn line_to_rect(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32) -> (FloatRect, Angle) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    (
        FloatRect {
            position: Vector2::new(x1, y1 - thickness / 2.),
            size: Vector2::new((dx * dx + dy * dy).sqrt(), thickness),
        },
        Angle::radians(dy.atan2(dx)),
    )
}

trait RenderTargetExt {
    /// Draw line between 2 poins
    fn draw_line(&mut self, p1: Vector2f, p2: Vector2f, thickness: f32);
}

impl RenderTargetExt for RenderWindow {
    fn draw_line(&mut self, p1: Vector2f, p2: Vector2f, thickness: f32) {
        let (rect, angle) = line_to_rect(p1.x, p1.y, p2.x, p2.y, thickness);
        let mut rs = RectangleShape::from_rect(rect);
        rs.set_rotation(angle);
        self.draw_rectangle_shape(&rs, &RenderStates::DEFAULT);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    example_ensure_right_working_dir();

    let mut rw = RenderWindow::new(
        (800, 600),
        "Positional audio demo",
        Style::CLOSE,
        sfml::window::window_enums::State::Windowed,
        &Default::default(),
    )?;
    rw.set_vertical_sync_enabled(true);
    let font = Font::from_file("sansation.ttf")?;
    let mut text = Text::new("", &font, 20);
    let mut music = match std::env::args().nth(1) {
        Some(music_path) => Music::from_file(&music_path)?,
        None => Music::from_file("canary.wav")?,
    };
    if music.channel_count() != 1 {
        return Err("Sorry, only sounds with 1 channel are supported.".into());
    }
    music.set_looping(true);
    music.play();
    music.set_position(Vector3::new(0., 0., 0.));

    let mut listener_pos = Vector3::new(0.0, 0.0, 0.0);
    let center = Vector2::new(400., 300.);
    let [mut go_left, mut go_right, mut go_up, mut go_down] = [false; 4];
    let mut clock = Clock::new()?;
    clock.start();

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::A => go_left = true,
                    Key::D => go_right = true,
                    Key::W => go_up = true,
                    Key::S => go_down = true,
                    _ => {}
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::A => go_left = false,
                    Key::D => go_right = false,
                    Key::W => go_up = false,
                    Key::S => go_down = false,
                    _ => {}
                },
                _ => {}
            }
        }
        let Vector2f { x: mx, y: my } = rw.mouse_position().as_other();
        let speed = 0.05;
        if go_left {
            listener_pos.x -= speed;
        }
        if go_right {
            listener_pos.x += speed;
        }
        if go_up {
            listener_pos.y -= speed;
        }
        if go_down {
            listener_pos.y += speed;
        }
        let scale = 20.0; // Scale the positions for better visualization
        listener::set_position(listener_pos);
        listener::set_direction(Vector3::new(
            (mx - center.x) / scale,
            (my - center.y) / scale,
            -1.,
        ));
        let Vector3 {
            x: lx,
            y: ly,
            z: lz,
        } = listener::position();
        let Vector3 {
            x: dx,
            y: dy,
            z: dz,
        } = listener::direction();
        rw.clear(Color::BLACK);
        let mut circle_shape = CircleShape::new(8.0, 32);
        // Draw circle at center, representing position of music being played
        circle_shape.set_position(center);
        circle_shape.set_fill_color(Color::YELLOW);
        let t = clock.elapsed_time().as_seconds();
        let radius = 12.0 + t.sin() * 3.0;
        circle_shape.set_radius(radius);
        circle_shape.set_origin(radius);
        rw.draw(&circle_shape);
        // Draw circle representing listener
        circle_shape.set_position((center.x + lx * scale, center.y + ly * scale));
        circle_shape.set_origin(4.0);
        circle_shape.set_radius(4.0);
        circle_shape.set_fill_color(Color::GREEN);
        rw.draw(&circle_shape);
        // Draw line from listener to direction vector position
        rw.draw_line(
            circle_shape.position(),
            (center.x + dx * scale, center.y + dy * scale).into(),
            2.0,
        );
        text.set_string("WASD + mouse for movement of listener");
        text.set_position(0.);
        rw.draw(&text);
        text.set_string(&format!("Listener position: {lx}, {ly}, {lz}"));
        text.set_position((0., 20.0));
        rw.draw(&text);
        text.set_string(&format!("Listener direction: {dx}, {dy}, {dz}"));
        text.set_position((0., 40.0));
        rw.draw(&text);
        rw.display();
    }
    Ok(())
}
