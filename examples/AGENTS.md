# AGENTS.md — Examples

Standalone programs demonstrating the `sfml` crate. Run one with `cargo run --example <name>`.
Most examples load resources by relative path after calling
`example_ensure_right_working_dir()` (from `../example_common.rs`, which chdirs
into `examples/resources` at startup). `shader.rs`, `unicode-text-entry.rs`,
and `window-test.rs` instead embed their assets at compile time with
`include_str!`/`include_bytes!`.

Examples that need `window`, `graphics`, or `audio` have a `[[example]]` entry with `required-features` in `Cargo.toml`. Only `clock.rs` and `version.rs` need no features.

## Files

- `borrowed-resources.rs`: Draws textures, sprites, fonts, and text that are borrowed from owners outside the sprite/text objects.
- `clock.rs`: Minimal check that the crate links against only the system module.
- `cursor.rs`: Custom mouse cursors and a small grid-drawing app that changes the cursor based on hover state.
- `custom-drawable.rs`: Implements `Drawable` for a custom bullet-shaped type built from shapes.
- `custom-shape.rs`: Implements `CustomShapePoints` to define a custom triangle shape that is animated.
- `custom-sound-recorder.rs`: Implements `SoundRecorder` to capture microphone samples and play them back.
- `custom-sound-stream.rs`: Implements `SoundStream` to generate a looping chiptune-style melody procedurally.
- `handle-events.rs`: Drains the event queue each frame with `Window::handle_events` and prints events, closing on Escape or the close button.
- `joystick.rs`: Prints the name and vendor/product IDs of every connected joystick.
- `mouse.rs`: Shows mouse position, buttons, wheel, and cursor grab/visibility handling.
- `music-stream.rs`: Streams an Ogg file as `Music` from a borrowed `File` via `InputStream`.
- `music-stream-owned.rs`: Same as `music-stream.rs`, but the stream owns the `File`.
- `opengl.rs`: Mixes raw OpenGL (via `glu_sys`) with SFML drawing, and toggles sRGB by recreating the window.
- `opengl-context.rs`: Creates and activates a standalone `Context` and checks which context is active.
- `pong.rs`: A complete Pong game with an AI opponent, sounds, and a font-based score display.
- `positional-audio.rs`: Moves a sound source around the screen to demonstrate 3D positional audio and the listener.
- `rc-resources.rs`: Uses the `Rc*` twins (`RcFont`, `RcTexture`, `RcSprite`, `RcText`) with bouncing floating resources and clipboard input.
- `resource-holder.rs`: A generic `ResourceHolder` (from the SFML game dev book) for caching textures and sounds by identifier.
- `shader.rs`: Applies several GLSL shader effects (pixelate, blur, edge, wave, storm, and others) to a scene.
- `sound.rs`: Lists the playback devices, switches the active device during playback, and plays a `Sound` and `Music`.
- `sound-capture.rs`: Lists the recording devices, records from the chosen one with `SoundBufferRecorder`, and plays the result.
- `spritemark.rs`: Benchmark that renders many sprites using a raw vertex array instead of `Sprite`.
- `unicode-text-entry.rs`: Unicode text entry and rendering using shaped glyphs, clusters, and per-glyph bounding boxes.
- `vertex-arrays.rs`: Draws a colored shape from a `Vertex` array with `PrimitiveType`.
- `vertex-buffers.rs`: Uses a `VertexBuffer` and updates its vertices on mouse click, and draws a range of them with `RenderTarget::draw_vertex_buffer_range`. Right-click hides the oldest visible vertex.
- `version.rs`: Prints the SFML version loaded at runtime.
- `vulkan.rs`: Opens a window, creates a Vulkan instance and a surface for it, and clears the window to a color that changes on each key press. Escape or closing the window exits.
- `vulkan-info.rs`: Prints whether Vulkan is available and which instance extensions graphics rendering requires.
- `window-test.rs`: Opens several windows with different sizes, styles, and fullscreen modes.

Shared setup code lives in `../example_common.rs` at the crate root, not in this folder. Examples pull it in with `include!`.

## Resources

`resources/` holds the assets the examples load at runtime, such as images, fonts, audio, and shaders. Most examples load them by relative path after `example_ensure_right_working_dir()`, so they must stay in this folder. (`shader.rs`, `unicode-text-entry.rs`, and `window-test.rs` embed copies at compile time instead.)

Files:

- `background.jpg`
- `ball.wav`
- `billboard.frag`
- `billboard.geom`
- `billboard.vert`
- `blink.frag`
- `blur.frag`
- `canary.wav`
- `devices.png`
- `edge.frag`
- `frank.jpeg`
- `logo.png`
- `opengl-background.jpg`
- `orchestral.ogg`
- `pixelate.frag`
- `sansation.ttf`
- `sfml.png`
- `storm.vert`
- `text-background.png`
- `texture.jpg`
- `wave.vert`

## Keeping this file current

If you add a new example, or modify an existing one significantly enough to change the description above, you must update this file.
