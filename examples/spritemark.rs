use rand::{thread_rng, Rng};
use sfml::{
    graphics::{
        Color, Font, Rect, RenderTarget, RenderWindow, Sprite, Text, Texture, Transformable,
    },
    system::{Vector2f, Vector2i},
    window::{mouse::Button, ContextSettings, Event, Style},
};

const SUBIMAGE_SIZE: u8 = 96;
const N_IMAGES: u8 = 6;
const GRAVITY: f32 = 0.5;
const GROUND_Y: f32 = 600.;
const RIGHT_WALL_X: f32 = 800.;

struct Object {
    position: Vector2f,
    speed: Vector2f,
    image_id: u8,
}

impl Object {
    fn update(&mut self) {
        let Vector2f { x, y } = &mut self.position;
        self.speed.y += GRAVITY;
        *x += self.speed.x;
        *y += self.speed.y;
        let size = f32::from(SUBIMAGE_SIZE);
        if *y + size >= GROUND_Y {
            *y = GROUND_Y - size;
            self.speed.y = -self.speed.y;
        }
        if *x + size >= RIGHT_WALL_X {
            *x = RIGHT_WALL_X - size;
            self.speed.x = -self.speed.x;
        }
        if *x <= 0. {
            *x = 0.;
            self.speed.x = -self.speed.x;
        }
    }
}

fn fconv(in_: Vector2i) -> Vector2f {
    Vector2f {
        x: in_.x as f32,
        y: in_.y as f32,
    }
}

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Spritemark",
        Style::default(),
        &ContextSettings::default(),
    );
    window.set_vertical_sync_enabled(true);
    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let texture = Texture::from_file("resources/devices.png").unwrap();
    let mut text = Text::new("", &font, 18);
    let mut sprite = Sprite::with_texture(&texture);
    let mut click_counter = 0;
    let mut objects = Vec::new();
    let mut rng = thread_rng();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed {
                    button: Button::LEFT,
                    ..
                } => click_counter += 1,
                _ => {}
            }
        }

        if Button::LEFT.is_pressed() {
            let mp = window.mouse_position();
            for _ in 0..100 {
                objects.push(Object {
                    position: fconv(mp),
                    speed: Vector2f::new(rng.gen_range(-3.0..3.0), 0.0),
                    image_id: click_counter % N_IMAGES,
                });
            }
        }

        window.clear(Color::BLACK);
        for obj in &mut objects {
            obj.update();
            sprite.set_position(obj.position);
            sprite.set_texture_rect(&rect_from_id(obj.image_id));
            window.draw(&sprite);
        }
        text.set_string(&format!("{} sprites", objects.len()));
        window.draw(&text);
        window.display();
    }
}

fn rect_from_id(image_id: u8) -> Rect<i32> {
    Rect {
        top: 0,
        left: i32::from(image_id) * i32::from(SUBIMAGE_SIZE),
        width: SUBIMAGE_SIZE.into(),
        height: SUBIMAGE_SIZE.into(),
    }
}
