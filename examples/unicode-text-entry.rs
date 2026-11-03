use sfml::{
    SfResult,
    graphics::{
        ClusterGrouping, Color, Font, GlyphAppearance, LineAlignment, RectangleShape, RenderStates,
        RenderTarget, RenderWindow, Shape, ShapedGlyph, Text, TextOrientation, TextStyle,
        Transformable, Vertex,
    },
    system::Vector2f,
    window::{Event, Key, Style, clipboard, window_enums::State},
};

include!("../example_common.rs");

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    let mut window = RenderWindow::new(
        (800, 600),
        "◢◤ Unicode text entry ◥◣",
        Style::CLOSE,
        State::Windowed,
        &Default::default(),
    )?;
    window.set_vertical_sync_enabled(true);

    // Showcase delayed initialization of font
    let mut font = Font::new()?;

    match std::env::args().nth(1) {
        Some(path) => font.open_from_file(&path)?,
        None => font.load_from_memory_static(include_bytes!("resources/sansation.ttf"))?,
    }
    let mut string = String::from("This text can be edited.\nTry it!");

    let mut text = Text::new(&string, &font, 24);
    text.set_fill_color(Color::RED);
    text.set_outline_color(Color::YELLOW);
    text.set_outline_thickness(2.0);
    // SFML aligns the lines around the origin of the text, so move the origin to the middle of
    // the window to make every alignment visible
    text.set_position((window.size().x as f32 / 2.0, 0.0));
    let mut status_text = Text::new("", &font, 16);
    status_text.set_position((0., window.size().y as f32 - 120.0));
    let mut bold = false;
    let mut italic = false;
    let mut underlined = false;
    let mut strikethrough = false;
    let mut show_cursor = true;
    let mut line_alignment = LineAlignment::Default;
    let mut text_orientation = TextOrientation::Default;
    let mut cluster_grouping = ClusterGrouping::Character;
    let mut color_glyphs = false;
    let mut vertex_boxes = false;

    'mainloop: loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed => break 'mainloop,
                Event::TextEntered { unicode } => {
                    if unicode == 0x08 as char {
                        string.pop();
                    } else if unicode == 0xD as char {
                        string.push('\n');
                    }
                    // Ignore ctrl+v/ctrl+v generated chars
                    else if unicode != 0x16 as char && unicode != 0x03 as char {
                        string.push(unicode);
                    }
                    text.set_string(&string);
                }
                Event::KeyPressed {
                    code: Key::V,
                    ctrl: true,
                    ..
                } => {
                    string.push_str(&clipboard::get_string());
                    text.set_string(&string);
                }
                Event::KeyPressed {
                    code: Key::C,
                    ctrl: true,
                    ..
                } => {
                    clipboard::set_string(text.string());
                }
                Event::KeyPressed { code, .. } => {
                    match code {
                        Key::Escape => break 'mainloop,
                        Key::F1 => bold ^= true,
                        Key::F2 => italic ^= true,
                        Key::F3 => underlined ^= true,
                        Key::F4 => strikethrough ^= true,
                        Key::F5 => show_cursor ^= true,
                        Key::F6 => {
                            line_alignment = cycle(
                                &[
                                    LineAlignment::Default,
                                    LineAlignment::Left,
                                    LineAlignment::Center,
                                    LineAlignment::Right,
                                ],
                                line_alignment,
                            );
                            text.set_line_alignment(line_alignment);
                        }
                        Key::F7 => {
                            text_orientation = cycle(
                                &[
                                    TextOrientation::Default,
                                    TextOrientation::TopToBottom,
                                    TextOrientation::BottomToTop,
                                ],
                                text_orientation,
                            );
                            text.set_text_orientation(text_orientation);
                        }
                        Key::F8 => {
                            cluster_grouping = cycle(
                                &[
                                    ClusterGrouping::Character,
                                    ClusterGrouping::Grapheme,
                                    ClusterGrouping::None,
                                ],
                                cluster_grouping,
                            );
                            text.set_cluster_grouping(cluster_grouping);
                        }
                        Key::F9 => {
                            color_glyphs ^= true;
                            text.set_glyph_pre_processor(color_glyphs.then_some(color_by_cluster));
                        }
                        Key::F10 => vertex_boxes ^= true,
                        _ => {}
                    }
                    let mut style = TextStyle::default();
                    if bold {
                        style |= TextStyle::BOLD;
                    }
                    if italic {
                        style |= TextStyle::ITALIC;
                    }
                    if underlined {
                        style |= TextStyle::UNDERLINED;
                    }
                    if strikethrough {
                        style |= TextStyle::STRIKETHROUGH;
                    }
                    text.set_style(style);
                }
                _ => {}
            }
        }

        let (glyph_count, last_direction) = {
            let glyphs = text.shaped_glyphs();
            (
                glyphs.len(),
                glyphs.last().map(|glyph| glyph.text_direction),
            )
        };
        let vertex_count = text.vertex_data().len();
        let outline_vertex_count = text.outline_vertex_data().len();
        let status_string = {
            let fc = text.fill_color();
            let oc = text.outline_color();
            format!(
                "fill: {:02x}{:02x}{:02x}{:02x} outline: {:02x}{:02x}{:02x}{:02x} outline thickness: {}\n\
            style: {:?} (F1-F4) cursor: {} (F5)\n\
            alignment: {:?} (F6) orientation: {:?} (F7) clusters: {:?} (F8)\n\
            glyph colors: {} (F9) vertex boxes: {} (F10)\n\
            glyphs: {} last direction: {:?} vertices: {} outline vertices: {}\n\
            font family: {}",
                fc.r,
                fc.g,
                fc.b,
                fc.a,
                oc.r,
                oc.g,
                oc.b,
                oc.a,
                text.outline_thickness(),
                text.style(),
                show_cursor,
                line_alignment,
                text_orientation,
                cluster_grouping,
                color_glyphs,
                vertex_boxes,
                glyph_count,
                last_direction,
                vertex_count,
                outline_vertex_count,
                font.info().family
            )
        };
        status_text.set_string(&status_string);

        window.clear(Color::BLACK);
        window.draw(&text);
        if vertex_boxes {
            draw_vertex_boxes(&mut window, &mut text);
        }
        if show_cursor {
            let vertical = text_orientation != TextOrientation::Default;
            let mut end = end_of_text(&mut text, &font, string.ends_with('\n'));
            end.x += 2.0;
            end.y += 2.0;
            let mut rs = RectangleShape::new();
            rs.set_fill_color(Color::TRANSPARENT);
            rs.set_outline_color(Color::YELLOW);
            rs.set_outline_thickness(-3.0);
            rs.set_position(end);
            rs.set_size(if vertical { (24.0, 8.0) } else { (8.0, 24.0) });
            window.draw(&rs);
        }
        window.draw(&status_text);
        window.display();
    }
    println!("The final text is {:?}", text.string().to_rust_string());
    Ok(())
}

/// The option after `current` in `options`, wrapping around to the first one
fn cycle<T: Copy + PartialEq>(options: &[T], current: T) -> T {
    let index = options
        .iter()
        .position(|&option| option == current)
        .unwrap_or(0);
    options[(index + 1) % options.len()]
}

/// Colors each cluster differently, so the effect of the cluster grouping is visible
fn color_by_cluster(glyph: &ShapedGlyph, appearance: &mut GlyphAppearance) {
    const PALETTE: [Color; 4] = [Color::RED, Color::GREEN, Color::CYAN, Color::MAGENTA];
    appearance.fill_color = PALETTE[glyph.cluster as usize % PALETTE.len()];
}

/// Draws a box around the vertices of each glyph, and one around all the outline vertices
fn draw_vertex_boxes(window: &mut RenderWindow, text: &mut Text) {
    let glyph_ranges: Vec<(usize, usize)> = text
        .shaped_glyphs()
        .iter()
        .map(|glyph| (glyph.vertex_offset, glyph.vertex_count))
        .collect();
    let vertices = text.vertex_data();
    let glyph_boxes: Vec<(Vector2f, Vector2f)> = glyph_ranges
        .into_iter()
        .filter(|&(_, count)| count > 0)
        .filter_map(|(offset, count)| bounds(&vertices[offset..offset + count]))
        .collect();
    let outline_box = bounds(text.outline_vertex_data());

    let render_states = RenderStates {
        transform: *text.transform(),
        ..RenderStates::default()
    };
    let boxes = glyph_boxes
        .into_iter()
        .map(|surrounding| (Color::GREEN, surrounding))
        .chain(outline_box.map(|surrounding| (Color::MAGENTA, surrounding)));
    for (color, (min, max)) in boxes {
        let mut rs = RectangleShape::new();
        rs.set_fill_color(Color::TRANSPARENT);
        rs.set_outline_color(color);
        rs.set_outline_thickness(-1.0);
        rs.set_position(min);
        rs.set_size(max - min);
        window.draw_with_renderstates(&rs, &render_states);
    }
}

/// Corners (min, max) of the smallest rectangle that contains the positions of `vertices`
fn bounds(vertices: &[Vertex]) -> Option<(Vector2f, Vector2f)> {
    let first = vertices.first()?.position;
    Some(vertices.iter().fold((first, first), |(min, max), vertex| {
        let p = vertex.position;
        (
            Vector2f::new(min.x.min(p.x), min.y.min(p.y)),
            Vector2f::new(max.x.max(p.x), max.y.max(p.y)),
        )
    }))
}

/// Global position right after the last character, where the next character would be drawn
fn end_of_text(text: &mut Text, font: &Font, ends_with_newline: bool) -> Vector2f {
    let character_size = text.character_size();
    let char_size = character_size as f32;
    let line_spacing = font.line_spacing(character_size) * text.line_spacing();
    let bold = text.style().contains(TextStyle::BOLD);
    let whitespace_width = font
        .glyph(u32::from(' '), character_size, bold, 0.0)
        .advance();
    let letter_spacing = (whitespace_width / 3.0) * (text.letter_spacing() - 1.0);
    let orientation = text.text_orientation();
    let vertical = orientation != TextOrientation::Default;
    let local = match text.shaped_glyphs().last() {
        None => Vector2f::default(),
        Some(last) if ends_with_newline && vertical => {
            Vector2f::new(0.0, last.position.y + line_spacing)
        }
        Some(last) if ends_with_newline => {
            Vector2f::new(0.0, last.position.y + line_spacing - char_size)
        }
        Some(last) if vertical => {
            // Bottom-to-top text reads upwards, so the next character goes above the last one
            let step = last.glyph.advance() + letter_spacing;
            let direction = if orientation == TextOrientation::BottomToTop {
                -1.0
            } else {
                1.0
            };
            Vector2f::new(last.position.x, last.position.y + direction * step)
        }
        Some(last) => Vector2f::new(
            last.position.x + last.glyph.advance() + letter_spacing,
            last.position.y - char_size,
        ),
    };
    text.transform().transform_point(local)
}
