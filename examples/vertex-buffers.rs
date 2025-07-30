use sfml::{
    SfResult,
    graphics::{
        Color, PrimitiveType, RenderTarget, RenderWindow, Vertex, VertexBuffer, VertexBufferUsage,
    },
    window::{Event, Style, mouse::Button, window_enums::State},
};

fn main() -> SfResult<()> {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML VertexBuffer accessors Example",
        Style::CLOSE,
        State::Windowed,
        &Default::default(),
    )?;
    window.set_vertical_sync_enabled(true);

    let mut vertices = vec![
        Vertex::with_pos_color((20.0, 30.0).into(), Color::GREEN),
        Vertex::with_pos_color((30.0, 30.0).into(), Color::GREEN),
        Vertex::with_pos_color((40.0, 40.0).into(), Color::GREEN),
        Vertex::with_pos_color((50.0, 50.0).into(), Color::GREEN),
        Vertex::with_pos_color((60.0, 60.0).into(), Color::GREEN),
        Vertex::with_pos_color((50.0, 80.0).into(), Color::GREEN),
    ];
    let mut vertex_buffer = VertexBuffer::new(
        PrimitiveType::LINE_STRIP,
        vertices.len(),
        VertexBufferUsage::DYNAMIC,
    )?;
    vertex_buffer.update(&vertices, 0)?;

    'mainloop: loop {
        while let Some(e) = window.poll_event() {
            match e {
                Event::Closed => break 'mainloop,
                Event::MouseButtonPressed {
                    button: Button::Left,
                    position,
                } => {
                    vertices.push(Vertex::with_pos_color(
                        (position.x as f32, position.y as f32).into(),
                        Color::GREEN,
                    ));
                    vertex_buffer.update(&vertices, 0)?;
                }
                _ => {}
            }
        }
        // Clear the window
        window.clear(Color::BLACK);
        window.draw(&*vertex_buffer);
        // Display things on screen
        window.display();
    }
    Ok(())
}
