//! An example to show off how fast it is possible to render sprites.
//!
//! It uses an array of primitives to achieve a speed faster than using `sf::Sprite`.

use rand::{thread_rng, Rng};
use sfml::{
    graphics::{
        Color, Font, PrimitiveType, Rect, RenderStates, RenderTarget, RenderWindow, Text, Texture,
        Transform, Vertex, View,
    },
    system::{Clock, Vector2, Vector2f, Vector2i},
    window::{mouse::Button, ContextSettings, Event, Key, Style, VideoMode},
};

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

fn main() {
    let native_mode = VideoMode::desktop_mode();
    let mut window = RenderWindow::new(
        native_mode,
        "Spritemark",
        Style::NONE,
        &ContextSettings::default(),
    );
    window.set_position(Vector2::new(0, 0));
    window.set_vertical_sync_enabled(true);
    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let texture = Texture::from_file("resources/devices.png").unwrap();
    let mut text = Text::new("", &font, 18);
    let mut click_counter = 0;
    let mut objects = Vec::new();
    let mut rng = thread_rng();
    let mut rs = RenderStates::default();
    let mut buf = Vec::new();
    let mut frames_rendered = 0;
    let mut sec_clock = Clock::start();
    let mut fps = 0;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => window.close(),
                Event::MouseButtonPressed {
                    button: Button::LEFT,
                    ..
                } => click_counter += 1,
                Event::Resized { width, height } => {
                    window.set_view(&View::from_rect(&Rect::new(
                        0.,
                        0.,
                        width as f32,
                        height as f32,
                    )));
                }
                _ => {}
            }
        }

        if Button::LEFT.is_pressed() {
            let mp = window.mouse_position();
            for _ in 0..25 {
                objects.push(Object {
                    position: fconv(mp),
                    speed: Vector2f::new(rng.gen_range(-3.0..3.0), 0.0),
                    image_id: click_counter % N_IMAGES,
                    angle: 0.0,
                    rot_speed: rng.gen_range(-2.0..2.0),
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
        text.set_string(&format!("{} sprites\n{} fps", objects.len(), fps));
        window.draw_text(&text, &rs);
        rs.set_texture(Some(&texture));
        window.draw_primitives(&buf, PrimitiveType::QUADS, &rs);
        rs.set_texture(None);
        window.display();
        buf.clear();
        frames_rendered += 1;
        if sec_clock.elapsed_time().as_milliseconds() >= 1000 {
            fps = frames_rendered;
            sec_clock.restart();
            frames_rendered = 0;
        }
    }
}
