# AGENTS.md — `src/`

Safe Rust API for SFML 3.1 and the layers that connect it to the C++ library.

## Top-level files

- `lib.rs`: crate root. Declares modules behind feature gates, sets the strict lint list, and defines the crate-wide `SfError` and `SfResult` types.
- `ffi.rs`: private FFI root. Declares the `ffi` submodules behind feature gates and defines shared C-ABI type aliases.

## Directories

- `ffi/`: raw `extern "C"` declarations for the CSFML shim. Contains generated bindings (do not edit by hand) plus hand-written FFI types. Private to the crate.
- `cpp/`: public low-level wrapper module for C++ objects returned by SFML (owned boxes, strings, vectors). It is also used internally by the safe API.
- `system/`: foundation types shared by all modules, such as time, clocks, vectors, angles, strings, buffers, and version info.
- `window/`: windows, events, input devices (keyboard, mouse, joystick, sensors), video modes, context settings, cursors, and clipboard. Gated by the `window` feature.
- `graphics/`: 2D rendering: render targets and windows, shapes, sprites, text and fonts, textures, images, shaders, views, blend modes, and vertices. Gated by the `graphics` feature, which implies `window`.
- `audio/`: sound playback and recording: sounds, music, buffers, streams, listeners, capture, and playback devices. Gated by the `audio` feature.

## Per-folder AGENTS.md

Each folder under `src/` has its own `AGENTS.md` with file-level detail: `ffi/`, `cpp/`, `system/`, `window/`, `graphics/`, and `audio/`. Read the one for the folder you are working in before making changes there.

## Keeping this file current

If your changes make anything in this file inaccurate or incomplete, update this `AGENTS.md` so it matches the code.
