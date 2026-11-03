# AGENTS.md — `src/audio/`

Safe Rust API for SFML audio: playback, streaming, recording, and spatialization.

## Files

- `mod.rs`: module root. Declares the submodules and re-exports the main types (`Sound`, `Music`, `SoundBuffer`, `SoundStream`, `SoundStreamPlayer`, `SoundChannel`, `TimeSpan`, and the recorder types).
- `capture.rs`: audio capture. `SoundRecorder` trait, `SoundRecorderDriver` (runs a recorder), `SoundBufferRecorder` (records into a buffer), and capture device queries.
- `listener.rs`: the global 3D listener. Sets its position, direction, up vector, and global volume.
- `music.rs`: `Music`, a streamed sound read from an audio file.
- `playback_device.rs`: selects and queries the output device, and reports device change notifications.
- `sound.rs`: `Sound`, a sound played from a `SoundBuffer`.
- `sound_buffer.rs`: `SoundBuffer`, the loaded sample data for a sound, with load and save.
- `sound_channel.rs`: `SoundChannel`, the speaker channel layout used for spatialization.
- `sound_source.rs`: `SoundSource` trait with shared playback and 3D properties, plus `Cone`, `Status`, and the effect processor type.
- `sound_stream.rs`: `SoundStream` trait for custom streamed sources, and `SoundStreamPlayer` that plays one.
- `time_span.rs`: `TimeSpan`, a start offset and length used for loop ranges.

## Keeping this file current

If you add a new file to this folder, or make a significant change to an existing file that affects the summary above, you must update this `AGENTS.md` to match.
