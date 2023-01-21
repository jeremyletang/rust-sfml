use sfml::{
    graphics::{Color, RcSprite, RcTexture, RenderTarget, RenderWindow, Transformable},
    system::Vector2f,
    window::{Event, Style},
};

include!("../example_common.rs");

struct FloatingSprite {
    up: bool,
    left: bool,
    sprite: RcSprite,
    speed: f32,
}

impl FloatingSprite {
    fn new(texture: &RcTexture, up: bool, left: bool, speed: f32) -> Self {
        Self {
            up,
            left,
            sprite: RcSprite::with_texture(&texture),
            speed,
        }
    }

    fn sprite(&self) -> &RcSprite {
        &self.sprite
    }

    fn move_sprites(&mut self, window_size: Vector2f) {
        // Modify the sprite position freely
        if self.sprite.position().y <= 0f32 {
            self.up = false;
        }
        if self.sprite.position().y + self.sprite.global_bounds().height >= window_size.y {
            self.up = true;
        }
        if self.sprite.position().x <= 0f32 {
            self.left = false;
        }
        if self.sprite.position().x + self.sprite.global_bounds().width >= window_size.x {
            self.left = true;
        }

        self.sprite.set_position(
            self.sprite.position()
                + Vector2f::new(
                    if self.left { -self.speed } else { self.speed },
                    if self.up { -self.speed } else { self.speed },
                ),
        );
    }
}

fn main() {
    let mut window =
        RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);

    // Create a new texture.
    let texture = RcTexture::from_file(example_res!("logo.png")).unwrap();

    // Load many sprites with no lifetime contingencies
    let mut floating_sprites = Vec::from([
        FloatingSprite::new(&texture, true, true, 1f32),
        FloatingSprite::new(&texture, true, false, 1.5f32),
        FloatingSprite::new(&texture, false, true, 2f32),
        FloatingSprite::new(&texture, false, false, 2.5f32),
    ]);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        // Update floating_sprite positions so they move around on the screen
        for floating_sprite in &mut floating_sprites {
            floating_sprite.move_sprites(Vector2f::new(800f32, 600f32));
        }

        window.clear(Color::BLACK);

        // Fetch and draw all the sprites in floating_sprites
        for floating_sprite in &floating_sprites {
            window.draw(floating_sprite.sprite());
        }
        window.display();
    }
}
