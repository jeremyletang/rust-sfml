use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    window::{ContextSettings, Event, Key, Style},
    SfResult,
};

struct WindowConfig {
    mode: (u32, u32),
    title: &'static str,
    style: Style,
}

fn main() -> SfResult<()> {
    let configs = [
        WindowConfig {
            mode: (320, 240),
            title: "Retro",
            style: Style::CLOSE,
        },
        WindowConfig {
            mode: (640, 480),
            title: "Classic",
            style: Style::DEFAULT,
        },
        WindowConfig {
            mode: (800, 600),
            title: "Big",
            style: Style::TITLEBAR,
        },
    ];
    let mut cfg_idx = 0usize;
    let mut rw = RenderWindow::new(
        configs[cfg_idx].mode,
        "Window test",
        Style::CLOSE,
        &ContextSettings::default(),
    )?;
    let font = Font::from_memory_static(include_bytes!("resources/sansation.ttf"))?;

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Up => cfg_idx = cfg_idx.saturating_sub(1),
                    Key::Down => {
                        if cfg_idx + 1 < configs.len() {
                            cfg_idx += 1
                        }
                    }
                    Key::Enter => rw.recreate(
                        configs[cfg_idx].mode,
                        configs[cfg_idx].title,
                        configs[cfg_idx].style,
                        &ContextSettings::default(),
                    ),
                    _ => {}
                },
                _ => {}
            }
        }
        rw.clear(Color::BLACK);
        let fontsize = 16;
        let mut txt = Text::new("Arrow keys to select mode, enter to set.", &font, fontsize);
        rw.draw(&txt);
        let mut y = fontsize as f32;
        for (i, cfg) in configs.iter().enumerate() {
            let fc = if i == cfg_idx {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            txt.set_fill_color(fc);
            txt.set_string(&format!(
                "{}x{} \"{}\" ({:?})",
                cfg.mode.0, cfg.mode.1, cfg.title, cfg.style
            ));
            txt.set_position((0., y));
            rw.draw(&txt);
            y += fontsize as f32;
        }
        rw.display();
    }
    Ok(())
}
