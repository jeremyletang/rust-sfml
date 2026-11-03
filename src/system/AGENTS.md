# AGENTS.md — `src/system/`

Foundation types shared by all modules.

## Files

- `mod.rs`: module root. Declares the submodules and re-exports the public types and functions (`Angle`, `Clock`, `InputStream`, `sleep`, `SfStr`, `SfStrConv`, `SfStrConvError`, `Time`, `Vector2*`, `Vector3*`, `Version`, `version`).
- `angle.rs`: `Angle`, an angle stored in radians with degree/radian conversions, arithmetic, and wrapping.
- `buffer.rs`: `Buffer` (crate-private), a read-only byte view over an SFML-owned `sfBuffer`. Derefs to `[u8]` and frees the buffer on drop.
- `clock.rs`: `Clock`, a wrapper around `sfClock` for measuring elapsed time.
- `input_stream.rs`: `InputStream`, an adapter that exposes a Rust `Read + Seek` value to SFML as a stream.
- `sleep.rs`: `sleep(Time)`, which blocks the current thread for the given duration.
- `string.rs`: `SfStr` (borrowed UTF-32 string) and the `SfStrConv` trait for converting Rust strings to SFML strings, plus `SfStrConvError`.
- `time.rs`: `Time`, a duration type with arithmetic, comparison, and conversions to seconds, milliseconds, and microseconds.
- `vector2.rs`: generic `Vector2<T>` with `Vector2f`, `Vector2i`, and `Vector2u` aliases, plus operators and conversions.
- `vector3.rs`: generic `Vector3<T>` with `Vector3f` and `Vector3i` aliases, plus operators and conversions.
- `version.rs`: `Version` and `version()`, which report the SFML library version.

## Keeping this file current

If you add a new file to `src/system/`, or change an existing file enough that its summary above is no longer accurate, update this `AGENTS.md` in the same change.
