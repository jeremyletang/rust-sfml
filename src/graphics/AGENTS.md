# AGENTS.md — `src/graphics/`

2D rendering API: render targets, drawables, text, textures, shaders, and related types. Gated by the `graphics` feature.

## Files

- `mod.rs`: module root. Declares submodules, re-exports the public types, and defines `vertex_array_bounds`.
- `blend_mode.rs`: `BlendMode` and its blend factor and equation constants.
- `circle_shape.rs`: `CircleShape`, a circle drawable with radius and point count.
- `color.rs`: `Color`, an RGBA color.
- `convex_shape.rs`: `ConvexShape`, a convex polygon drawable with settable points.
- `coordinate_type.rs`: `CoordinateType`, normalized or pixel texture coordinates.
- `custom_shape.rs`: `CustomShape` and the `CustomShapePoints` trait, for user-defined shape geometry.
- `drawable.rs`: `Drawable` trait for anything that can be drawn to a `RenderTarget`.
- `font.rs`: `Font`, an opaque Rust handle for loading and glyph/kerning/metric queries, plus `FontInfo`. Stream loading is unsafe; the `InputStream` adapter must outlive every use of the font.
- `glsl.rs`: GLSL-style vector and matrix types (`Vec2`, `IVec4`, `Mat4`, and others) for shader uniforms.
- `glyph.rs`: `Glyph`, the metrics and texture rect for one character.
- `glyph_pre_processor.rs`: `GlyphAppearance` and the C trampoline that lets users adjust glyphs before text layout.
- `image.rs`: `Image`, CPU-side pixel data with load, save, pixel access, and transform operations.
- `primitive_type.rs`: `PrimitiveType`, the vertex topology (points, lines, triangles, and so on).
- `rc_font.rs`: `RcFont`, a reference-counted twin of `Font`. `Text::font()` and `RcText::font()` return `Option<&Font>`.
- `rc_sprite.rs`: `RcSprite`, a reference-counted twin of `Sprite`.
- `rc_text.rs`: `RcText`, a reference-counted twin of `Text`.
- `rc_texture.rs`: `RcTexture`, a reference-counted twin of `Texture`.
- `rect.rs`: generic `Rect<T>` with `IntRect` and `FloatRect` aliases.
- `rectangle_shape.rs`: `RectangleShape`, a rectangle drawable.
- `render_states.rs`: `RenderStates`, the blend mode, transform, texture, shader, stencil mode, and texture coordinate type used for a draw call.
- `render_target.rs`: `RenderTarget` trait with the draw methods shared by windows and textures.
- `render_texture.rs`: `RenderTexture`, an off-screen render target.
- `render_window.rs`: `RenderWindow`, a window that is also a render target and has event polling.
- `shader.rs`: `Shader`, loading GLSL programs and setting uniforms.
- `shape.rs`: `Shape` trait shared by the shape types (rectangle, circle, convex, custom).
- `shaped_glyph.rs`: `ShapedGlyph`, a glyph after text shaping, with its position and advance.
- `sprite.rs`: `Sprite`, a textured quad drawable.
- `text.rs`: `Text`, a drawable string built from a font.
- `text_enums.rs`: text layout enums: `LineAlignment`, `TextOrientation`, `ClusterGrouping`, `TextDirection`.
- `text_style.rs`: `TextStyle` flags (bold, italic, underline, and so on).
- `texture.rs`: `Texture`, GPU texture loading, updating, and mipmap generation.
- `transform.rs`: `Transform`, a 3x3 affine transform stored as 16 floats (the SFML/OpenGL layout), with point and rect transformation.
- `transformable.rs`: `Transformable` trait for position, rotation, scale, and origin.
- `vertex.rs`: `Vertex`, a position, color, and texture coordinate, plus slice helpers.
- `vertex_buffer.rs`: `VertexBuffer` and `VertexBufferUsage`, GPU-side vertex storage.
- `view.rs`: `View`, a camera with center, size, rotation, viewport, and scissor.

## Keeping this file current

If you add a new file to this folder, or change an existing file enough that its summary above is no longer accurate, update this `AGENTS.md` in the same change.
