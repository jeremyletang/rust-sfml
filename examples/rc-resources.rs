use sfml::{
    graphics::{
        Color, RcFont, RcSprite, RcText, RcTexture, RenderTarget, RenderWindow, Transformable,
    },
    system::Vector2f,
    window::{Event, Style},
};

include!("../example_common.rs");

struct FloatingResource {
    up: bool,
    left: bool,
    render_sprite: bool,
    sprite: RcSprite,
    text: RcText,
    speed: f32,
}

impl FloatingResource {
    fn with_texture(texture: &RcTexture, up: bool, left: bool, speed: f32) -> Self {
        Self {
            up,
            left,
            sprite: RcSprite::with_texture(texture),
            text: Default::default(),
            speed,
            render_sprite: true,
        }
    }

    fn with_font(font: &RcFont, up: bool, left: bool, speed: f32) -> Self {
        Self {
            up,
            left,
            sprite: Default::default(),
            text: RcText::new("SFML", font, 32),
            speed,
            render_sprite: false,
        }
    }

    fn render(&self, window: &mut RenderWindow) {
        if self.render_sprite {
            window.draw(&self.sprite)
        } else {
            window.draw(&self.text)
        }
    }

    fn move_resources(&mut self, window_size: Vector2f) {
        if self.render_sprite {
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
        } else {
            // Modify the sprite position freely
            if self.text.position().y <= 0f32 {
                self.up = false;
            }
            if self.text.position().y + self.text.global_bounds().height >= window_size.y {
                self.up = true;
            }
            if self.text.position().x <= 0f32 {
                self.left = false;
            }
            if self.text.position().x + self.text.global_bounds().width >= window_size.x {
                self.left = true;
            }

            self.text.set_position(
                self.text.position()
                    + Vector2f::new(
                        if self.left { -self.speed } else { self.speed },
                        if self.up { -self.speed } else { self.speed },
                    ),
            );
        }
    }
}

fn main() {
    let mut window =
        RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);

    // Create a new texture.
    let texture = RcTexture::from_file(example_res!("logo.png")).unwrap();

    // Create a new font.
    let font = RcFont::from_file(example_res!("sansation.ttf")).unwrap();

    // Load many resources with no lifetime contingencies
    let mut floating_resources = Vec::from([
        FloatingResource::with_texture(&texture, true, true, 1f32),
        FloatingResource::with_texture(&texture, true, false, 1.5f32),
        FloatingResource::with_texture(&texture, false, true, 2f32),
        FloatingResource::with_texture(&texture, false, false, 2.5f32),
        FloatingResource::with_font(&font, true, true, 1.25f32),
        FloatingResource::with_font(&font, true, true, 1.75f32),
        FloatingResource::with_font(&font, true, true, 2.25f32),
        FloatingResource::with_font(&font, true, true, 2.75f32),
    ]);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        // Update floating_resource positions so they move around on the screen
        for floating_resource in &mut floating_resources {
            floating_resource.move_resources(Vector2f::new(800f32, 600f32));
        }

        window.clear(Color::BLACK);

        // Fetch and draw all the sprites in floating_resources
        for floating_resource in &floating_resources {
            floating_resource.render(&mut window);
        }
        window.display();
    }
}
