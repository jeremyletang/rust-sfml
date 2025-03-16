use {
    rand::{Rng as _, SeedableRng, rngs::SmallRng},
    sfml::{
        SfError, SfResult,
        cpp::FBox,
        graphics::{
            Color, Drawable, Font, IntRect, PrimitiveType, RenderStates, RenderTarget,
            RenderTexture, RenderWindow, Shader, ShaderType, Sprite, Text, Texture, Transform,
            Transformable, Vertex,
        },
        system::{Clock, Vector2, Vector2f},
        window::{Event, Key, Style, window_enums::State},
    },
};

include!("../example_common.rs");

trait Effect: Drawable {
    fn update(&mut self, t: f32, x: f32, y: f32) -> SfResult<()>;
    fn name(&self) -> &str;
}

struct Pixelate<'t> {
    sprite: Sprite<'t>,
    shader: FBox<Shader<'static>>,
}

impl<'t> Pixelate<'t> {
    fn new(texture: &'t Texture) -> SfResult<Self> {
        Ok(Self {
            sprite: Sprite::with_texture(texture),
            shader: Shader::from_file("pixelate.frag", ShaderType::Fragment)?,
        })
    }
}

impl Drawable for Pixelate<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.shader = Some(&self.shader);
        target.draw_with_renderstates(&self.sprite, &states);
    }
}

impl Effect for Pixelate<'_> {
    fn update(&mut self, _t: f32, x: f32, y: f32) -> SfResult<()> {
        self.shader
            .set_uniform_float("pixel_threshold", (x + y) / 30.0)
    }
    fn name(&self) -> &str {
        "pixelate"
    }
}

struct WaveBlur<'fo> {
    text: Text<'fo>,
    shader: FBox<Shader<'static>>,
}

const WAVEBLUR_TEXT: &str = "\
Praesent suscipit augue in velit pulvinar hendrerit varius purus aliquam.
Mauris mi odio, bibendum quis fringilla a, laoreet vel orci. Proin vitae vulputate tortor.
Praesent cursus ultrices justo, ut feugiat ante vehicula quis.
Donec fringilla scelerisque mauris et viverra.
Maecenas adipiscing ornare scelerisque. Nullam at libero elit.
Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas.
Nullam leo urna, tincidunt id semper eget, ultricies sed mi.
Morbi mauris massa, commodo id dignissim vel, lobortis et elit.
Fusce vel libero sed neque scelerisque venenatis.
Integer mattis tincidunt quam vitae iaculis.
Vivamus fringilla sem non velit venenatis fermentum.
Vivamus varius tincidunt nisi id vehicula.
Integer ullamcorper, enim vitae euismod rutrum, massa nisl semper ipsum,
vestibulum sodales sem ante in massa.
Vestibulum in augue non felis convallis viverra.
Mauris ultricies dolor sed massa convallis sed aliquet augue fringilla.
Duis erat eros, porta in accumsan in, blandit quis sem.
In hac habitasse platea dictumst. Etiam fringilla est id odio dapibus sit amet semper dui laoreet.";

impl<'fo> WaveBlur<'fo> {
    fn new(font: &'fo Font) -> SfResult<Self> {
        let mut text = Text::new(WAVEBLUR_TEXT, font, 22);
        text.set_position((30., 20.));
        Ok(Self {
            text,
            shader: Shader::from_file_vert_frag("wave.vert", "blur.frag")?,
        })
    }
}

impl Drawable for WaveBlur<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.shader = Some(&self.shader);
        target.draw_with_renderstates(&self.text, &states);
    }
}

impl Effect for WaveBlur<'_> {
    fn update(&mut self, t: f32, x: f32, y: f32) -> SfResult<()> {
        self.shader.set_uniform_float("wave_phase", t)?;
        self.shader
            .set_uniform_vec2("wave_amplitude", Vector2f::new(x * 40., y * 40.))?;
        self.shader
            .set_uniform_float("blur_radius", (x + y) * 0.008)
    }
    fn name(&self) -> &str {
        "wave + blur"
    }
}

struct StormBlink {
    points: Vec<Vertex>,
    shader: FBox<Shader<'static>>,
}

impl StormBlink {
    fn new() -> SfResult<Self> {
        let mut rng = SmallRng::seed_from_u64(1);

        let mut points = Vec::new();
        for _ in 0..40_000 {
            let x = rng.random_range(0.0..800.);
            let y = rng.random_range(0.0..600.);
            let (red, green, blue) = (rng.random(), rng.random(), rng.random());
            points.push(Vertex::with_pos_color(
                Vector2f::new(x, y),
                Color::rgb(red, green, blue),
            ));
        }

        let shader = Shader::from_file_vert_frag("storm.vert", "blink.frag")?;
        Ok(Self { points, shader })
    }
}

impl Drawable for StormBlink {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.shader = Some(&self.shader);
        target.draw_primitives(&self.points, PrimitiveType::POINTS, &states);
    }
}

impl Effect for StormBlink {
    fn update(&mut self, t: f32, x: f32, y: f32) -> SfResult<()> {
        let radius = 200. + t.cos() * 150.;
        self.shader
            .set_uniform_vec2("storm_position", Vector2f::new(x * 800., y * 600.))?;
        self.shader
            .set_uniform_float("storm_inner_radius", radius / 3.)?;
        self.shader
            .set_uniform_float("storm_total_radius", radius)?;
        self.shader
            .set_uniform_float("blink_alpha", 0.5 + (t * 3.).cos() * 0.25)
    }
    fn name(&self) -> &str {
        "storm + blink"
    }
}

struct Edge<'t> {
    surface: FBox<RenderTexture>,
    bg_sprite: Sprite<'t>,
    entities: Vec<Sprite<'t>>,
    shader: FBox<Shader<'static>>,
}

impl<'t> Edge<'t> {
    fn new(bg_texture: &'t Texture, entity_texture: &'t Texture) -> SfResult<Self> {
        let mut surface = RenderTexture::new(Vector2::new(800, 600))?;
        surface.set_smooth(true);
        let mut bg_sprite = Sprite::with_texture(bg_texture);
        bg_sprite.set_position((135., 100.));
        let mut entities = Vec::new();

        for i in 0..6 {
            entities.push(Sprite::with_texture_and_rect(
                entity_texture,
                IntRect::new(Vector2::new(96 * i, 0), Vector2::new(96, 96)),
            ));
        }

        let mut shader = Shader::from_file("edge.frag", ShaderType::Fragment)?;
        shader.set_uniform_current_texture("texture")?;

        Ok(Self {
            surface,
            bg_sprite,
            entities,
            shader,
        })
    }
}

impl Drawable for Edge<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.shader = Some(&self.shader);
        target.draw_with_renderstates(&Sprite::with_texture(self.surface.texture()), &states);
    }
}

impl Effect for Edge<'_> {
    fn update(&mut self, t: f32, x: f32, y: f32) -> SfResult<()> {
        self.shader
            .set_uniform_float("edge_threshold", 1. - (x + y) / 2.)?;
        let entities_len = self.entities.len() as f32;

        for (i, en) in self.entities.iter_mut().enumerate() {
            let pos = (
                (0.25 * (t * i as f32 + (entities_len - i as f32))).cos() * 300. + 350.,
                (0.25 * (t * (entities_len - i as f32) + i as f32)).cos() * 200. + 250.,
            );
            en.set_position(pos);
        }
        self.surface.clear(Color::WHITE);
        self.surface.draw(&self.bg_sprite);
        for en in &self.entities {
            self.surface.draw(en);
        }
        self.surface.display();
        Ok(())
    }
    fn name(&self) -> &str {
        "edge post-effect"
    }
}

struct Geometry<'tex> {
    point_cloud: [Vertex; 10_000],
    shader: FBox<Shader<'tex>>,
    texture: &'tex Texture,
    transform: Transform,
}

impl<'tex> Geometry<'tex> {
    fn new(texture: &'tex Texture) -> SfResult<Self> {
        if !Shader::is_geometry_available() {
            eprintln!("Geometry shaders not available");
            return Err(SfError::CallFailed);
        }
        let mut shader = Shader::from_memory_all(
            include_str!("resources/billboard.vert"),
            include_str!("resources/billboard.geom"),
            include_str!("resources/billboard.frag"),
        )?;
        shader.set_uniform_texture("texture", texture)?;
        shader.set_uniform_vec2("resolution", (800., 600.).into())?;
        let mut rng = SmallRng::seed_from_u64(1);
        Ok(Self {
            point_cloud: std::array::from_fn(|_| {
                Vertex::new(
                    (
                        rng.random_range(-480.0..480.0),
                        rng.random_range(-480.0..480.0),
                    )
                        .into(),
                    Color::WHITE,
                    Default::default(),
                )
            }),
            shader,
            texture,
            transform: Transform::IDENTITY,
        })
    }
}

impl Drawable for Geometry<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.shader = Some(&self.shader);
        states.texture = Some(self.texture);
        states.transform = self.transform;
        target.draw_primitives(&self.point_cloud, PrimitiveType::POINTS, &states);
    }
}

impl Effect for Geometry<'_> {
    fn update(&mut self, _t: f32, x: f32, y: f32) -> SfResult<()> {
        self.transform = Transform::IDENTITY;
        self.transform.translate(Vector2::new(400., 300.));
        self.transform.rotate(x * 360.0);
        let size = 25. + y.abs() * 50.;
        self.shader.set_uniform_vec2("size", size.into())?;
        Ok(())
    }

    fn name(&self) -> &str {
        "Geometry Shader Billboards"
    }
}

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut window = RenderWindow::new(
        (800, 600),
        "SFML Shader",
        Style::TITLEBAR | Style::CLOSE,
        State::Windowed,
        &Default::default(),
    )?;
    window.set_vertical_sync_enabled(true);
    let font = Font::from_file("sansation.ttf")?;
    let bg = Texture::from_file("background.jpg")?;
    let mut bg_texture = Texture::from_file("sfml.png")?;
    bg_texture.set_smooth(true);
    let mut entity_texture = Texture::from_file("devices.png")?;
    entity_texture.set_smooth(true);
    let logo_texture = Texture::from_file("logo.png")?;
    let effects: [&mut dyn Effect; 5] = [
        &mut Pixelate::new(&bg)?,
        &mut WaveBlur::new(&font)?,
        &mut StormBlink::new()?,
        &mut Edge::new(&bg_texture, &entity_texture)?,
        &mut Geometry::new(&logo_texture)?,
    ];
    let mut current = 0;
    let text_bg_texture = Texture::from_file("text-background.png")?;
    let mut text_bg = Sprite::with_texture(&text_bg_texture);
    text_bg.set_position((0., 520.));
    text_bg.set_color(Color::rgba(255, 255, 255, 200));
    let msg = format!("Current effect: {}", effects[current].name());
    let mut desc = Text::new(&msg, &font, 20);
    desc.set_position((10., 530.));
    desc.set_fill_color(Color::rgb(80, 80, 80));
    let msg = "Press left and right arrows to change the current shader";
    let mut instructions = Text::new(msg, &font, 20);
    instructions.set_position((280., 555.));
    instructions.set_fill_color(Color::rgb(80, 80, 80));
    let mut clock = Clock::new()?;
    clock.start();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            use crate::Event::*;
            match event {
                Closed => window.close(),
                KeyPressed { code, .. } => match code {
                    Key::Escape => window.close(),
                    Key::Left => {
                        if current == 0 {
                            current = effects.len() - 1;
                        } else {
                            current -= 1;
                        }
                        desc.set_string(&format!("Current effect: {}", effects[current].name()));
                    }
                    Key::Right => {
                        if current == effects.len() - 1 {
                            current = 0;
                        } else {
                            current += 1;
                        }
                        desc.set_string(&format!("Current effect: {}", effects[current].name()));
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let x = window.mouse_position().x as f32 / window.size().x as f32;
        let y = window.mouse_position().y as f32 / window.size().y as f32;

        effects[current].update(clock.elapsed_time().as_seconds(), x, y)?;

        window.clear(Color::rgb(255, 128, 0));
        window.draw(effects[current]);
        window.draw(&text_bg);
        window.draw(&instructions);
        window.draw(&desc);
        window.display();
    }
    Ok(())
}
