use sfml::{
    SfResult,
    system::{Time, sleep},
    window::{Event, Key, Style, Window},
};

fn main() -> SfResult<()> {
    let mut window = Window::new_open(
        (800, 600),
        "Handle events",
        Style::CLOSE,
        Default::default(),
        &Default::default(),
    )?;

    let mut running = true;
    while running {
        window.handle_events(|event| match event {
            Event::Closed
            | Event::KeyPressed {
                code: Key::Escape, ..
            } => running = false,
            Event::MouseMoved { .. } => {}
            _ => println!("{event:?}"),
        });
        sleep(Time::milliseconds(16));
    }
    Ok(())
}
