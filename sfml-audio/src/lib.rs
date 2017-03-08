// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Sounds, streaming (musics or custom sources), recording, spatialization
//!

extern crate sfml;
extern crate csfml_system_sys;
extern crate csfml_audio_sys;

pub use sound_buffer::{SoundBuffer, SoundBufferRef};
pub use sound_status::SoundStatus;
pub use music::Music;
pub use sound::Sound;
pub use sound_buffer_recorder::SoundBufferRecorder;
pub use sound_source::SoundSource;
pub use sound_stream::{SoundStream, SoundStreamPlayer};

mod sound_buffer;
mod sound_source;
pub mod listener;
mod sound_status;
mod music;
mod sound;
mod sound_buffer_recorder;
mod sound_stream;
#[path="../../src/inputstream.rs"]
mod inputstream;
