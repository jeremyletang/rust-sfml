use sfml::{
    SfResult,
    graphics::{
        Color, PrimitiveType, RectangleShape, RenderStates, RenderTarget, RenderWindow, Shape,
        Vertex, vertex_array_bounds,
    },
    window::{Event, Style},
};

fn main() -> SfResult<()> {
    let mut window = RenderWindow::new(
        (800, 600),
        "Vertex array example",
        Style::CLOSE,
        Default::default(),
        &Default::default(),
    )?;
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = [
        Vertex::with_pos_color((20.0, 30.0).into(), Color::GREEN),
        Vertex::with_pos_color((30.0, 30.0).into(), Color::GREEN),
        Vertex::with_pos_color((40.0, 40.0).into(), Color::GREEN),
        Vertex::with_pos_color((50.0, 50.0).into(), Color::GREEN),
        Vertex::with_pos_color((60.0, 60.0).into(), Color::GREEN),
        Vertex::with_pos_color((50.0, 80.0).into(), Color::GREEN),
    ];

    println!("\nIterate over the vertices of a vertex array");
    for v in &vertex_array {
        println!("Vertex Color: {:?} | Position: {:?}", v.color, v.position)
    }

    println!("\nMutable access to a vertex");
    println!(
        "Before Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );
    vertex_array[1].position.x = 100.0;
    println!(
        "After Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );

    println!("\nImmutable access to a vertex");
    println!(
        "Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );

    let vertex = &mut vertex_array[1];
    println!(
        "Before Vertex Color: {:?} | Position: {:?}",
        vertex.color, vertex.position
    );
    vertex.position.x = 100.0;
    println!(
        "After Vertex Color: {:?} | Position: {:?}",
        vertex.color, vertex.position
    );

    // Or:
    vertex_array[1] = Vertex::with_pos((20., 40.).into());
    println!("[2] After Vertex Position: {:?}", vertex_array[1].position);
    println!(
        "Vertex Color: {:?} | Position: {:?}",
        vertex_array[1].color, vertex_array[1].position
    );
    let bounds = vertex_array_bounds(&vertex_array);
    let mut bound_rect = RectangleShape::from_rect(bounds);
    bound_rect.set_fill_color(Color::TRANSPARENT);
    bound_rect.set_outline_thickness(1.0);
    bound_rect.set_outline_color(Color::YELLOW);

    'mainloop: loop {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed {
                break 'mainloop;
            }
        }
        let rs = RenderStates::default();
        // Clear the window
        window.clear(Color::BLACK);
        window.draw_primitives(&vertex_array, PrimitiveType::LINE_STRIP, &rs);
        window.draw(&bound_rect);
        // Display things on screen
        window.display()
    }
    Ok(())
}
