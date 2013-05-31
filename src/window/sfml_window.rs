/*!
* Provides OpenGL-based windows, and abstractions for events and input handling.
*
*
*/

#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod others {
    #[link_args="-lcsfml-window"]
    extern {}
}

pub mod video_mode;
pub mod context;
pub mod context_settings;
pub mod window;
pub mod joystick;
pub mod keyboard;
pub mod mouse;
pub mod event;