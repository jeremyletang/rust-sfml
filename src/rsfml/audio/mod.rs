/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Sounds, streaming (musics or custom sources), recording, spatialization
*
*
*/

pub use self::sound_buffer::SoundBuffer;
pub use self::listener::*;
pub use self::sound_status::{Status, Playing, Paused, Stopped};
pub use self::music::Music;
pub use self::sound::Sound;
pub use self::sound_buffer_recorder::SoundBufferRecorder;
pub use self::sound_recorder::SoundRecorder;

#[doc(hidden)]
#[cfg(target_os="macos")]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod platform {
    #[link(name = "csfml-audio")]
    extern {}
}

pub mod sound_buffer;
pub mod listener;
pub mod sound_status;
pub mod music;
pub mod sound;
pub mod sound_buffer_recorder;
pub mod sound_recorder;
