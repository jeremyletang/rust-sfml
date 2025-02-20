//! An example to show off how fast it is possible to render sprites.
//!
//! It uses an array of primitives to achieve a speed faster than using `sf::Sprite`.

use {
    rand::{Rng as _, SeedableRng, rngs::SmallRng},
    sfml::{
        SfResult,
        graphics::{
            Color, Font, PrimitiveType, Rect, RenderStates, RenderTarget, RenderWindow, Text,
            Texture, Transform, Vertex, View,
        },
        system::{Clock, Vector2, Vector2f, Vector2i},
        window::{ContextSettings, Event, Key, Style, VideoMode, mouse::Button},
    },
};

include!("../example_common.rs");

const SUBIMAGE_SIZE: u8 = 96;
const N_IMAGES: u8 = 6;
const GRAVITY: f32 = 0.5;

struct Object {
    position: Vector2f,
    speed: Vector2f,
    image_id: u8,
    angle: f32,
    rot_speed: f32,
}

impl Object {
    fn update(&mut self, ground_y: f32, right_wall_x: f32) {
        let Vector2f { x, y } = &mut self.position;
        self.speed.y += GRAVITY;
        *x += self.speed.x;
        *y += self.speed.y;
        let size = f32::from(SUBIMAGE_SIZE);
        if *y + size >= ground_y {
            *y = ground_y - size;
            self.speed.y = -self.speed.y;
        }
        if *x + size >= right_wall_x {
            *x = right_wall_x - size;
            self.speed.x = -self.speed.x;
        }
        if *x <= 0. {
            *x = 0.;
            self.speed.x = -self.speed.x;
        }
        self.angle += self.rot_speed;
    }
}

fn fconv(in_: Vector2i) -> Vector2f {
    Vector2f {
        x: in_.x as f32,
        y: in_.y as f32,
    }
}

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let native_mode = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
        native_mode,
        "Spritemark",
        Style::default(),
        &ContextSettings::default(),
    )?;
    window.set_position(Vector2::new(0, 0));
    window.set_vertical_sync_enabled(true);
    let font = Font::from_file("sansation.ttf")?;
    let texture = Texture::from_file("devices.png")?;
    let mut text = Text::new("", &font, 18);
    text.set_outline_color(Color::BLACK);
    text.set_outline_thickness(1.0);
    let mut click_counter = 0;
    let mut objects = Vec::new();
    let mut rng = SmallRng::seed_from_u64(1);
    let mut rs = RenderStates::default();
    let mut buf = Vec::new();
    let mut frames_rendered = 0;
    let mut sec_clock = Clock::start()?;
    let mut fps = 0;
    let mut lmb_down = false;
    let mut view = View::new()?;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => window.close(),
                Event::MouseButtonPressed {
                    button: Button::Left,
                    ..
                } => {
                    click_counter += 1;
                    lmb_down = true;
                }
                Event::MouseButtonReleased {
                    button: Button::Left,
                    ..
                } => {
                    lmb_down = false;
                }
                Event::Resized { width, height } => {
                    view.reset(Rect::new(0., 0., width as f32, height as f32));
                    window.set_view(&view);
                }
                _ => {}
            }
        }

        if lmb_down {
            let mp = window.mouse_position();
            for _ in 0..25 {
                objects.push(Object {
                    position: fconv(mp),
                    speed: Vector2f::new(rng.random_range(-3.0..3.0), 0.0),
                    image_id: click_counter % N_IMAGES,
                    angle: 0.0,
                    rot_speed: rng.random_range(-2.0..2.0),
                });
            }
        }

        for obj in &mut objects {
            let size = f32::from(SUBIMAGE_SIZE);
            let tex_x = f32::from(obj.image_id) * size;
            let mut tf = Transform::default();
            tf.translate(obj.position.x, obj.position.y);
            tf.rotate_with_center(obj.angle, size / 2.0, size / 2.0);
            buf.push(Vertex {
                color: Color::WHITE,
                position: tf.transform_point(Vector2f::new(0., 0.)),
                tex_coords: Vector2f::new(tex_x, 0.),
            });
            buf.push(Vertex {
                color: Color::WHITE,
                position: tf.transform_point(Vector2f::new(0., size)),
                tex_coords: Vector2f::new(tex_x, size),
            });
            buf.push(Vertex {
                color: Color::WHITE,
                position: tf.transform_point(Vector2f::new(size, size)),
                tex_coords: Vector2f::new(tex_x + size, size),
            });
            buf.push(Vertex {
                color: Color::WHITE,
                position: tf.transform_point(Vector2f::new(size, 0.)),
                tex_coords: Vector2f::new(tex_x + size, 0.),
            });
            obj.update(window.size().y as f32, window.size().x as f32);
        }
        window.clear(Color::BLACK);
        rs.texture = Some(&texture);
        window.draw_primitives(&buf, PrimitiveType::QUADS, &rs);
        rs.texture = None;
        text.set_string(&format!("{} sprites\n{fps} fps", objects.len()));
        window.draw_text(&text, &rs);
        window.display();
        buf.clear();
        frames_rendered += 1;
        if sec_clock.elapsed_time().as_milliseconds() >= 1000 {
            fps = frames_rendered;
            sec_clock.restart();
            frames_rendered = 0;
        }
    }
    Ok(())
}
