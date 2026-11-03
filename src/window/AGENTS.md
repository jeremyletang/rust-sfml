# AGENTS.md — `src/window/`

Windows, events, and input devices. Gated by the `window` feature.

## Files

- `mod.rs`: module root. Declares submodules and re-exports the main window, event, context, and cursor types.
- `window.rs`: `Window`, a native OS window with OpenGL context, size, style, and visibility controls.
- `window_enums.rs`: `Style` flags and `State` enum for window creation and fullscreen state.
- `event.rs`: `Event` enum for window and input events, plus event payload types and the `drain_events` helper behind `handle_events`.
- `context.rs`: `Context`, a standalone OpenGL context usable without a window.
- `context_settings.rs`: `ContextSettings`, OpenGL version, depth, stencil, and antialiasing options.
- `video_mode.rs`: `VideoMode`, a display resolution and bit depth, with desktop and fullscreen-mode queries.
- `cursor.rs`: `Cursor` and `CursorType`, system or image-based mouse cursors.
- `clipboard.rs`: free functions for reading and writing the system clipboard.
- `keyboard.rs`: `Key` and `Scancode` helpers, plus the virtual keyboard toggle.
- `mouse.rs`: real-time mouse state queries (position, button state).
- `joystick.rs`: real-time joystick state queries (connection, buttons, axes, identification).
- `sensor.rs`: real-time sensor state queries (accelerometer, gyroscope, and similar).
- `thread_safety.rs`: crate-private check that windowing calls stay on a single thread, through `set_window_thread` and its `pin_thread` helper.
- `vulkan.rs`: Vulkan availability, function lookup, and the instance extensions for graphics rendering, plus the `VkInstanceT`, `VkInstance`, `VkSurfaceKHR`, and `VkAllocationCallbacks` handle types re-exported from `ffi`.

## Keeping this file current

If you add a new file to this folder, or change an existing file enough that its summary above is no longer accurate, update this `AGENTS.md` in the same change.
