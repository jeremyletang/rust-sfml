//! `ResourceHolder` class from the SFML game dev book

use {
    sfml::{
        SfResult,
        audio::{Sound, SoundBuffer},
        cpp::FBox,
        graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture},
        window::{Event, Key, Style},
    },
    std::{collections::HashMap, hash::Hash},
};

include!("../example_common.rs");

struct ResourceHolder<Resource, Identifier: Hash + Eq> {
    resource_map: HashMap<Identifier, FBox<Resource>>,
}

impl<Resource: ResLoad, Identifier: Hash + Eq> ResourceHolder<Resource, Identifier> {
    pub fn load(&mut self, identifier: Identifier, filename: &str) -> SfResult<()> {
        let res = Resource::load(filename)?;
        self.resource_map.insert(identifier, res);
        Ok(())
    }
    pub fn get(&self, id: Identifier) -> &Resource {
        &self.resource_map[&id]
    }
}

trait ResLoad {
    fn load(filename: &str) -> SfResult<FBox<Self>>;
}

impl ResLoad for Texture {
    fn load(filename: &str) -> SfResult<FBox<Self>> {
        Self::from_file(filename)
    }
}

impl ResLoad for SoundBuffer {
    fn load(filename: &str) -> SfResult<FBox<Self>> {
        Self::from_file(filename)
    }
}

impl<Resource, Identifier: Hash + Eq> Default for ResourceHolder<Resource, Identifier> {
    fn default() -> Self {
        Self {
            resource_map: HashMap::default(),
        }
    }
}

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut tex_holder = ResourceHolder::<Texture, _>::default();
    tex_holder.load("frank", "frank.jpeg")?;
    let mut sb_holder = ResourceHolder::<SoundBuffer, _>::default();
    sb_holder.load("canary", "canary.wav")?;
    let mut rw = RenderWindow::new(
        (800, 600),
        "Resource holder test",
        Style::CLOSE,
        &Default::default(),
    )?;
    rw.set_vertical_sync_enabled(true);
    let mut sound = Sound::with_buffer(sb_holder.get("canary"));
    sound.play();
    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => rw.close(),
                _ => {}
            }
        }
        rw.clear(Color::BLACK);
        rw.draw(&Sprite::with_texture(tex_holder.get("frank")));
        rw.display();
    }
    Ok(())
}
