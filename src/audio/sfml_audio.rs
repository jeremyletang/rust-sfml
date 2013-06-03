/*!
* Sounds, streaming (musics or custom sources), recording, spatialization
*
*
*/

#[cfg(mac_dylib)]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod platform {
    #[link_args="-lcsfml-audio"]
    extern {}
}

pub mod sound_buffer;
pub mod listener;
pub mod sound_status;
pub mod music;
pub mod sound;
pub mod sound_buffer_recorder;
pub mod sound_recorder;