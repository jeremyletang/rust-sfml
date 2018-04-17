//! Sounds, streaming (musics or custom sources), recording, spatialization
//!

extern crate csfml_audio_sys;

pub use self::capture::{SoundBufferRecorder, SoundRecorder, SoundRecorderDriver};
pub use self::music::Music;
pub use self::sound::Sound;
pub use self::sound_buffer::SoundBuffer;
pub use self::sound_source::SoundSource;
pub use self::sound_status::SoundStatus;
pub use self::sound_stream::{SoundStream, SoundStreamPlayer};

/// Types and helper functions dealing with audio capture.
pub mod capture;
pub mod listener;
mod music;
mod sound;
mod sound_buffer;
mod sound_source;
mod sound_status;
mod sound_stream;
