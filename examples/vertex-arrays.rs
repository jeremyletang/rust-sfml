use sfml::{
    graphics::{
        Color, PrimitiveType, RectangleShape, RenderTarget, RenderWindow, Shape, Vertex,
        VertexArray,
    },
    window::{Event, Style},
};

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "SFML VertexArray accessors Example",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);

    let mut vertex_array = VertexArray::default();
    vertex_array.set_primitive_type(PrimitiveType::LINE_STRIP);

    vertex_array.append(&Vertex::with_pos_color((20.0, 30.0).into(), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((30.0, 30.0).into(), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((40.0, 40.0).into(), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((50.0, 50.0).into(), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((60.0, 60.0).into(), Color::GREEN));
    vertex_array.append(&Vertex::with_pos_color((50.0, 80.0).into(), Color::GREEN));

    println!("\nIterate over the vertices of a VertexArray");
    for v in vertex_array.vertices() {
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

    unsafe {
        println!("\nMutable unchecked access to a vertex");
        let vertex = vertex_array.get_vertex_mut_unchecked(1);
        println!(
            "Before Vertex Color: {:?} | Position: {:?}",
            vertex.color, vertex.position
        );
        vertex.position.x = 100.0;
        println!(
            "After Vertex Color: {:?} | Position: {:?}",
            vertex.color, vertex.position
        );
    }

    // Or:
    unsafe {
        vertex_array.set_vertex_unchecked(1, &Vertex::with_pos((20., 40.).into()));
        println!("[2] After Vertex Position: {:?}", vertex_array[1].position);
    }

    println!("\nImmutable unchecked access to a vertex");
    unsafe {
        println!(
            "Vertex Color: {:?} | Position: {:?}",
            vertex_array.get_vertex_unchecked(1).color,
            vertex_array.get_vertex_unchecked(1).position
        );
    }
    let bounds = vertex_array.bounds();
    let mut bound_rect = RectangleShape::from_rect(bounds);
    bound_rect.set_fill_color(Color::TRANSPARENT);
    bound_rect.set_outline_thickness(1.0);
    bound_rect.set_outline_color(Color::YELLOW);

    loop {
        while let Some(e) = window.poll_event() {
            if e == Event::Closed {
                return;
            }
        }
        // Clear the window
        window.clear(Color::BLACK);
        window.draw(&vertex_array);
        window.draw(&bound_rect);
        // Display things on screen
        window.display()
    }
}
