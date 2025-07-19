use sfml::{
    SfResult,
    graphics::{
        Color, RcFont, RcSprite, RcText, RcTexture, RectangleShape, RenderTarget, RenderWindow,
        Shape, Sprite, Texture, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style, clipboard, window_enums::State},
};

include!("../example_common.rs");

enum EitherResource {
    Sprite(RcSprite),
    Text(RcText),
}

struct FloatingResource {
    up: bool,
    left: bool,
    either_resource: EitherResource,
    speed: f32,
}

impl FloatingResource {
    fn with_texture(texture: &RcTexture, up: bool, left: bool, speed: f32) -> Self {
        Self {
            up,
            left,
            either_resource: EitherResource::Sprite(RcSprite::with_texture(texture)),
            speed,
        }
    }

    fn with_font(font: &RcFont, up: bool, left: bool, speed: f32) -> Self {
        let mut text = RcText::new("", font, 16);
        text.scale(Vector2f::new(2., 2.));
        Self {
            up,
            left,
            either_resource: EitherResource::Text(text),
            speed,
        }
    }

    fn render(&self, window: &mut RenderWindow) {
        match &self.either_resource {
            EitherResource::Sprite(sprite) => window.draw(sprite),
            EitherResource::Text(text) => window.draw(text),
        }
    }

    fn move_resources(&mut self, window_size: Vector2f) {
        match &mut self.either_resource {
            EitherResource::Sprite(sprite) => {
                if sprite.position().y <= 0f32 {
                    self.up = false;
                }
                if sprite.position().y + sprite.global_bounds().size.y >= window_size.y {
                    self.up = true;
                }
                if sprite.position().x <= 0f32 {
                    self.left = false;
                }
                if sprite.position().x + sprite.global_bounds().size.x >= window_size.x {
                    self.left = true;
                }

                sprite.set_position(
                    sprite.position()
                        + Vector2f::new(
                            if self.left { -self.speed } else { self.speed },
                            if self.up { -self.speed } else { self.speed },
                        ),
                );
            }
            EitherResource::Text(text) => {
                // Modify the sprite position freely
                if text.position().y <= 0f32 {
                    self.up = false;
                }
                if text.position().y + text.global_bounds().size.y >= window_size.y {
                    self.up = true;
                }
                if text.position().x <= 0f32 {
                    self.left = false;
                }
                if text.position().x + text.global_bounds().size.x >= window_size.x {
                    self.left = true;
                }

                text.set_position(
                    text.position()
                        + Vector2f::new(
                            if self.left { -self.speed } else { self.speed },
                            if self.up { -self.speed } else { self.speed },
                        ),
                );
            }
        }
    }
}

fn test_getting_rc_texture_from_texture() -> SfResult<RcTexture> {
    Ok(RcTexture::from_texture(Texture::from_file("frank.jpeg")?))
}

fn get_set_smooth_rc_text(font: &RcFont) -> RcText {
    let mut set_smooth_text = RcText::new(
        "Press 's' to enable/disable font smoothing\n\
         Press 't' to toggle showing font texture atlas",
        font,
        16,
    );
    set_smooth_text.scale(Vector2f::new(2., 2.));

    set_smooth_text
}

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut window = RenderWindow::new(
        (800, 600),
        "SFML window",
        Style::CLOSE,
        State::Windowed,
        &Default::default(),
    )?;
    window.set_framerate_limit(60);

    // Create a new texture.
    let texture = RcTexture::from_file("logo.png")?;
    let texture2 = test_getting_rc_texture_from_texture()?;

    // Create a new font.
    let font_path = match std::env::args().nth(1) {
        Some(path) => path,
        None => "sansation.ttf".into(),
    };
    let mut font = RcFont::from_file(&font_path)?;

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
    let mut show_texture_atlas = false;
    let mut text_buf = String::from("SFML");

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }

            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, ctrl, .. } => match code {
                    Key::S => {
                        let smooth = !font.is_smooth();
                        font.set_smooth(smooth);
                    }
                    Key::T => {
                        show_texture_atlas ^= true;
                    }
                    Key::V if ctrl => {
                        text_buf.push_str(&clipboard::get_string());
                    }
                    _ => {}
                },
                Event::TextEntered { unicode } if show_texture_atlas => {
                    if unicode == 0x8 as char {
                        text_buf.pop();
                    } else if !unicode.is_ascii_control() && unicode != 's' && unicode != 't' {
                        text_buf.push(unicode);
                    }
                }
                _ => {}
            }
        }

        // Update floating_resource positions so they move around on the screen
        for floating_resource in &mut floating_resources {
            floating_resource.move_resources(Vector2f::new(800f32, 600f32));
            if let EitherResource::Text(text) = &mut floating_resource.either_resource {
                text.set_string(&text_buf);
            }
        }

        window.clear(Color::BLACK);

        // Fetch and draw all the sprites in floating_resources
        for floating_resource in &floating_resources {
            floating_resource.render(&mut window);
        }

        window.draw(&set_smooth_text);
        if show_texture_atlas {
            let scale = 3.0;
            let tex = font.texture(16);
            let mut rs = RectangleShape::with_size(tex.size().as_other());
            rs.set_fill_color(Color::MAGENTA);
            rs.set_scale(scale);
            window.draw(&rs);
            let mut s = Sprite::with_texture(&tex);
            s.set_scale(scale);
            window.draw(&s);
        }
        window.display();
    }
    Ok(())
}
