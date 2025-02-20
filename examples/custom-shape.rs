use sfml::{
    SfResult,
    graphics::{
        Color, CustomShape, CustomShapePoints, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::{Clock, Vector2f},
    window::{Event, Key, Style},
};

#[derive(Clone, Copy)]
pub struct TriangleShape;

impl CustomShapePoints for TriangleShape {
    fn point_count(&self) -> usize {
        3
    }

    fn point(&self, point: usize) -> Vector2f {
        match point {
            0 => Vector2f { x: 20., y: 580. },
            1 => Vector2f { x: 400., y: 20. },
            2 => Vector2f { x: 780., y: 580. },
            p => panic!("Non-existent point: {p}"),
        }
    }
}

fn hue_time(t: f32) -> Color {
    const fn lerp(from: f32, to: f32, amount: f32) -> f32 {
        from + amount * (to - from)
    }

    let frac = t.fract();

    let [r, g, b] = match (t % 6.0).floor() {
        0.0 => [255., lerp(0., 255., frac), 0.],
        1.0 => [lerp(255., 0., frac), 255., 0.],
        2.0 => [0., 255., lerp(0., 255., frac)],
        3.0 => [0., lerp(255., 0., frac), 255.],
        4.0 => [lerp(0., 255., frac), 0., 255.],
        _ => [255., 0., lerp(255., 0., frac)],
    };
    Color::rgb(r as u8, g as u8, b as u8)
}

fn main() -> SfResult<()> {
    let mut window = RenderWindow::new(
        [800, 600],
        "Custom shape",
        Style::DEFAULT,
        &Default::default(),
    )?;
    let clock = Clock::start()?;
    window.set_vertical_sync_enabled(true);

    let mut shape = CustomShape::new(Box::new(TriangleShape));
    shape.set_position((400., 300.));
    shape.set_origin((400., 300.));
    shape.set_outline_thickness(3.);

    'mainloop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => break 'mainloop,
                _ => {}
            }
        }

        let t = clock.elapsed_time().as_seconds();

        shape.set_rotation(t.sin().abs() * 360.0);
        let scale = t.cos().abs();
        shape.set_scale(scale);
        shape.set_fill_color(hue_time(t));
        shape.set_outline_color(hue_time(t / 2.0));
        window.clear(Color::BLACK);
        window.draw(&shape);
        window.display();
    }
    Ok(())
}
