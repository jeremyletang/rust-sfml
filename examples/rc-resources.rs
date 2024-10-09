use sfml::{
    graphics::{
        Color, RcFont, RcSprite, RcText, RcTexture, RenderTarget, RenderWindow, Texture,
        Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
    SfResult,
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
        let mut self_ = Self {
            up,
            left,
            sprite: Default::default(),
            text: RcText::new("SFML", font, 16),
            speed,
            render_sprite: false,
        };
        self_.text.scale(Vector2f::new(2., 2.));

        self_
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

fn test_getting_rc_texture_from_texture() -> SfResult<RcTexture> {
    Ok(RcTexture::from_texture(Texture::from_file(example_res!(
        "frank.jpeg"
    ))?))
}

fn get_set_smooth_rc_text(font: &RcFont) -> RcText {
    let mut set_smooth_text = RcText::new("Press 's' to enable/disable font smoothing", font, 16);
    set_smooth_text.scale(Vector2f::new(2., 2.));

    set_smooth_text
}

fn main() -> SfResult<()> {
    let mut window =
        RenderWindow::new((800, 600), "SFML window", Style::CLOSE, &Default::default())?;
    window.set_framerate_limit(60);

    // Create a new texture.
    let texture = RcTexture::from_file(example_res!("logo.png"))?;
    let texture2 = test_getting_rc_texture_from_texture()?;

    // Create a new font.
    let mut font = RcFont::from_file(example_res!("sansation.ttf"))?;

    // Load many resources with no lifetime contingencies
    let mut floating_resources = Vec::from([
        FloatingResource::with_texture(&texture2, true, true, 1.1f32),
        FloatingResource::with_texture(&texture2, true, true, 1.2f32),
        FloatingResource::with_texture(&texture, true, true, 1f32),
        FloatingResource::with_texture(&texture, true, false, 1.5f32),
        FloatingResource::with_texture(&texture, false, true, 2f32),
        FloatingResource::with_texture(&texture, false, false, 2.5f32),
        FloatingResource::with_font(&font, true, true, 1.25f32),
        FloatingResource::with_font(&font, true, true, 1.75f32),
        FloatingResource::with_font(&font, true, true, 2.25f32),
        FloatingResource::with_font(&font, true, true, 2.75f32),
    ]);

    let set_smooth_text = get_set_smooth_rc_text(&font);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }

            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code: Key::S, .. } => {
                    let smooth = !font.is_smooth();
                    font.set_smooth(smooth)
                }
                _ => {}
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

        window.draw(&set_smooth_text);
        window.display();
    }
    Ok(())
}
