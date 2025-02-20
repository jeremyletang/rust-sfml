use sfml::{
    SfResult,
    graphics::{Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    window::{ContextSettings, Event, Key, Style, VideoMode},
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
            title: "(Windowed) Retro",
            style: Style::CLOSE,
        },
        WindowConfig {
            mode: (640, 480),
            title: "(Windowed) Classic",
            style: Style::DEFAULT,
        },
        WindowConfig {
            mode: (800, 600),
            title: "(Windowed) Big",
            style: Style::TITLEBAR,
        },
    ];
    let mut cfg_idx = 2usize;
    let mut rw = RenderWindow::new(
        configs[cfg_idx].mode,
        "Window test",
        Style::CLOSE,
        &ContextSettings::default(),
    )?;
    let font = Font::from_memory_static(include_bytes!("resources/sansation.ttf"))?;
    let fs_modes = VideoMode::fullscreen_modes();

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Up => cfg_idx = cfg_idx.saturating_sub(1),
                    Key::Down => {
                        if cfg_idx + 1 < configs.len() + fs_modes.len() {
                            cfg_idx += 1
                        }
                    }
                    Key::Enter => match configs.get(cfg_idx) {
                        Some(cfg) => {
                            rw.recreate(cfg.mode, cfg.title, cfg.style, &ContextSettings::default())
                        }
                        None => match fs_modes.get(cfg_idx - configs.len()) {
                            Some(mode) => rw.recreate(
                                *mode,
                                "Fullscreen",
                                Style::FULLSCREEN,
                                &ContextSettings::default(),
                            ),
                            None => {
                                eprintln!("Invalid index: {cfg_idx}");
                            }
                        },
                    },
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
        let mut i = configs.len();
        y += fontsize as f32;
        txt.set_position((0., y));
        txt.set_fill_color(Color::WHITE);
        txt.set_string("= Fullscreen modes =");
        rw.draw(&txt);
        for mode in fs_modes.iter() {
            let n_rows = 23;
            let column = i / n_rows;
            let row = i % n_rows;
            let fc = if i == cfg_idx {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            txt.set_fill_color(fc);
            let left_pad = 16.0;
            let x = left_pad + (column * 128) as f32;
            let gap = 16.0;
            let y = y + gap + (row * fontsize as usize) as f32;
            txt.set_position((x, y));
            txt.set_string(&format!(
                "{}x{}x{}",
                mode.width, mode.height, mode.bits_per_pixel
            ));
            rw.draw(&txt);
            i += 1;
        }
        rw.display();
    }
    Ok(())
}
