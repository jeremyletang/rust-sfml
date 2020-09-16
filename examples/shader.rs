extern crate rand;
extern crate sfml;

use sfml::{graphics::*, system::*, window::*};

trait Effect: Drawable {
    fn update(&mut self, t: f32, x: f32, y: f32);
    fn name(&self) -> &str;
    fn as_drawable(&self) -> &dyn Drawable;
}

struct Pixelate<'t> {
    sprite: Sprite<'t>,
    shader: Shader<'static>,
}

impl<'t> Pixelate<'t> {
    fn new(texture: &'t Texture) -> Self {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, false);
        Self {
            sprite,
            shader: Shader::from_file(None, None, Some("resources/pixelate.frag")).unwrap(),
        }
    }
}

impl<'t> Drawable for Pixelate<'t> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.set_shader(Some(&self.shader));
        target.draw_with_renderstates(&self.sprite, &states);
    }
}

impl<'t> Effect for Pixelate<'t> {
    fn update(&mut self, _t: f32, x: f32, y: f32) {
        self.shader
            .set_uniform_float("pixel_threshold", (x + y) / 30.0);
    }
    fn name(&self) -> &str {
        "pixelate"
    }
    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
}

struct WaveBlur<'fo> {
    text: Text<'fo>,
    shader: Shader<'static>,
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
    fn new(font: &'fo Font) -> Self {
        let mut text = Text::new(WAVEBLUR_TEXT, font, 22);
        text.set_position((30., 20.));
        Self {
            text,
            shader: Shader::from_file(
                Some("resources/wave.vert"),
                None,
                Some("resources/blur.frag"),
            )
            .unwrap(),
        }
    }
}

impl<'fo> Drawable for WaveBlur<'fo> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.set_shader(Some(&self.shader));
        target.draw_with_renderstates(&self.text, &states);
    }
}

impl<'fo> Effect for WaveBlur<'fo> {
    fn update(&mut self, t: f32, x: f32, y: f32) {
        self.shader.set_uniform_float("wave_phase", t);
        self.shader
            .set_uniform_vec2("wave_amplitude", Vector2f::new(x * 40., y * 40.));
        self.shader
            .set_uniform_float("blur_radius", (x + y) * 0.008);
    }
    fn name(&self) -> &str {
        "wave + blur"
    }
    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
}

struct StormBlink {
    points: VertexArray,
    shader: Shader<'static>,
}

impl StormBlink {
    fn new() -> Self {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();

        let mut points = VertexArray::default();
        points.set_primitive_type(PrimitiveType::POINTS);
        for _ in 0..40_000 {
            let x = rng.gen_range(0., 800.);
            let y = rng.gen_range(0., 600.);
            let (red, green, blue) = (rng.gen(), rng.gen(), rng.gen());
            points.append(&Vertex::with_pos_color(
                Vector2f::new(x, y),
                Color::rgb(red, green, blue),
            ));
        }

        let shader = Shader::from_file(
            Some("resources/storm.vert"),
            None,
            Some("resources/blink.frag"),
        )
        .unwrap();
        Self { points, shader }
    }
}

impl Drawable for StormBlink {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.set_shader(Some(&self.shader));
        target.draw_with_renderstates(&self.points, &states);
    }
}

impl Effect for StormBlink {
    fn update(&mut self, t: f32, x: f32, y: f32) {
        let radius = 200. + t.cos() * 150.;
        self.shader
            .set_uniform_vec2("storm_position", Vector2f::new(x * 800., y * 600.));
        self.shader
            .set_uniform_float("storm_inner_radius", radius / 3.);
        self.shader.set_uniform_float("storm_total_radius", radius);
        self.shader
            .set_uniform_float("blink_alpha", 0.5 + (t * 3.).cos() * 0.25);
    }
    fn name(&self) -> &str {
        "storm + blink"
    }
    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
}

struct Edge<'t> {
    surface: RenderTexture,
    bg_sprite: Sprite<'t>,
    entities: Vec<Sprite<'t>>,
    shader: Shader<'static>,
}

impl<'t> Edge<'t> {
    fn new(bg_texture: &'t Texture, entity_texture: &'t Texture) -> Self {
        let mut surface = RenderTexture::new(800, 600, false).unwrap();
        surface.set_smooth(true);
        let mut bg_sprite = Sprite::with_texture(bg_texture);
        bg_sprite.set_position((135., 100.));
        let mut entities = Vec::new();

        for i in 0..6 {
            entities.push(Sprite::with_texture_and_rect(
                entity_texture,
                &IntRect::new(96 * i, 0, 96, 96),
            ));
        }

        let mut shader = Shader::from_file(None, None, Some("resources/edge.frag")).unwrap();
        shader.set_uniform_current_texture("texture");

        Self {
            surface,
            bg_sprite,
            entities,
            shader,
        }
    }
}

impl<'t> Drawable for Edge<'t> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut states = *states;
        states.set_shader(Some(&self.shader));
        target.draw_with_renderstates(&Sprite::with_texture(self.surface.texture()), &states);
    }
}

impl<'t> Effect for Edge<'t> {
    fn update(&mut self, t: f32, x: f32, y: f32) {
        self.shader
            .set_uniform_float("edge_threshold", 1. - (x + y) / 2.);
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
    }
    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
    fn name(&self) -> &str {
        "edge post-effect"
    }
}

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML Shader",
        Style::TITLEBAR | Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let font = Font::from_file("resources/sansation.ttf").unwrap();
    let bg = Texture::from_file("resources/background.jpg").unwrap();
    let mut bg_texture = Texture::from_file("resources/sfml.png").unwrap();
    bg_texture.set_smooth(true);
    let mut entity_texture = Texture::from_file("resources/devices.png").unwrap();
    entity_texture.set_smooth(true);
    let mut effects: [Box<dyn Effect>; 4] = [
        Box::new(Pixelate::new(&bg)),
        Box::new(WaveBlur::new(&font)),
        Box::new(StormBlink::new()),
        Box::new(Edge::new(&bg_texture, &entity_texture)),
    ];
    let mut current = 0;
    let text_bg_texture = Texture::from_file("resources/text-background.png").unwrap();
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
    let clock = Clock::start();

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            use crate::Event::*;
            match event {
                Closed => window.close(),
                KeyPressed { code, .. } => match code {
                    Key::ESCAPE => window.close(),
                    Key::LEFT => {
                        if current == 0 {
                            current = effects.len() - 1;
                        } else {
                            current -= 1;
                        }
                        desc.set_string(&format!("Current effect: {}", effects[current].name()));
                    }
                    Key::RIGHT => {
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

        effects[current].update(clock.elapsed_time().as_seconds(), x, y);

        window.clear(Color::rgb(255, 128, 0));
        window.draw(effects[current].as_drawable());
        window.draw(&text_bg);
        window.draw(&instructions);
        window.draw(&desc);
        window.display();
    }
}
