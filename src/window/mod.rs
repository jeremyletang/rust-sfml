//! Provides OpenGL-based windows, and abstractions for events and input handling.

pub use self::{
    context::Context,
    context_settings::ContextSettings,
    cursor::{Cursor, Type as CursorType},
    event::Event,
    keyboard::{set_virtual_keyboard_visible, Key},
    style::Style,
    video_mode::VideoMode,
    window::{Handle, Window},
};

pub use crate::ffi::window::Scancode;

pub mod clipboard;
mod context;
mod context_settings;
mod cursor;
mod event;
pub mod joystick;
mod keyboard;
pub mod mouse;
pub mod sensor;
mod style;
pub(crate) mod thread_safety;
pub mod touch;
mod video_mode;
#[allow(clippy::module_inception)]
mod window;
