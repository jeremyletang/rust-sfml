//! Sounds, streaming (musics or custom sources), recording, spatialization
//!

pub use audio::music::Music;
pub use audio::sound::Sound;
pub use audio::sound_buffer::{SoundBuffer, SoundBufferRef};
pub use audio::sound_buffer_recorder::SoundBufferRecorder;
pub use audio::sound_source::SoundSource;
pub use audio::sound_status::SoundStatus;
pub use audio::sound_stream::{SoundStream, SoundStreamPlayer};

mod sound_buffer;
mod sound_source;
pub mod listener;
mod sound_status;
mod music;
mod sound;
mod sound_buffer_recorder;
mod sound_stream;
