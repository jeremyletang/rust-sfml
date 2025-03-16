//! Sounds, streaming (musics or custom sources), recording, spatialization
//!

#[doc(inline)]
pub use self::capture::{SoundBufferRecorder, SoundRecorder, SoundRecorderDriver};
pub use self::{
    music::Music,
    sound::Sound,
    sound_buffer::SoundBuffer,
    sound_channel::SoundChannel,
    sound_stream::{SoundStream, SoundStreamPlayer},
    time_span::TimeSpan,
};

/// Types and helper functions dealing with audio capture.
pub mod capture;
pub mod listener;
mod music;
mod sound;
mod sound_buffer;
mod sound_channel;
/// Types and traits for dealing with audio playback
pub mod sound_source;
mod sound_stream;
mod time_span;
