use rand::{thread_rng, Rng};
use sfml::{
    graphics::{
        Color, Font, PrimitiveType, RenderStates, RenderTarget, RenderWindow, Text, Texture, Vertex,
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
    let mut click_counter = 0;
    let mut objects = Vec::new();
    let mut rng = thread_rng();
    let mut rs = RenderStates::default();
    let mut buf = Vec::new();

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
            for _ in 0..25 {
                objects.push(Object {
                    position: fconv(mp),
                    speed: Vector2f::new(rng.gen_range(-3.0..3.0), 0.0),
                    image_id: click_counter % N_IMAGES,
                });
            }
        }

        for obj in &mut objects {
            let size = f32::from(SUBIMAGE_SIZE);
            let tex_x = f32::from(obj.image_id) * size;
            buf.push(Vertex {
                color: Color::WHITE,
                position: obj.position,
                tex_coords: Vector2f::new(tex_x, 0.),
            });
            buf.push(Vertex {
                color: Color::WHITE,
                position: Vector2f::new(obj.position.x, obj.position.y + size),
                tex_coords: Vector2f::new(tex_x, size),
            });
            buf.push(Vertex {
                color: Color::WHITE,
                position: Vector2f::new(obj.position.x + size, obj.position.y + size),
                tex_coords: Vector2f::new(tex_x + size, size),
            });
            buf.push(Vertex {
                color: Color::WHITE,
                position: Vector2f::new(obj.position.x + size, obj.position.y),
                tex_coords: Vector2f::new(tex_x + size, 0.),
            });
            obj.update();
        }
        window.clear(Color::BLACK);
        text.set_string(&format!("{} sprites", objects.len()));
        window.draw_text(&text, &rs);
        rs.set_texture(Some(&texture));
        window.draw_primitives(&buf, PrimitiveType::QUADS, &rs);
        rs.set_texture(None);
        window.display();
        buf.clear();
    }
}
