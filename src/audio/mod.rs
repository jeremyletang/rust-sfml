//! Sounds, streaming (musics or custom sources), recording, spatialization
//!

extern crate csfml_audio_sys;

pub use self::music::Music;
pub use self::sound::Sound;
pub use self::sound_buffer::{SoundBuffer, SoundBufferRef};
pub use self::sound_recorder::SoundRecorder;
pub use self::sound_buffer_recorder::SoundBufferRecorder;
pub use self::sound_source::SoundSource;
pub use self::sound_status::SoundStatus;
pub use self::sound_stream::{SoundStream, SoundStreamPlayer};

#[derive(Debug, Clone, Copy)]
pub struct SetDeviceError;

mod sound_buffer;
mod sound_source;
pub mod listener;
mod sound_status;
mod music;
mod sound;
mod sound_recorder;
mod sound_buffer_recorder;
mod sound_stream;
