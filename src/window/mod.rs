//! Provides OpenGL-based windows, and abstractions for events and input handling.

pub use self::{
    context::Context,
    context_settings::ContextSettings,
    event::Event,
    keyboard::{set_virtual_keyboard_visible, Key},
    style::Style,
    video_mode::VideoMode,
    window::{Handle, Window},
};

pub mod clipboard;
mod context;
mod context_settings;
mod event;
pub mod joystick;
mod keyboard;
pub mod mouse;
pub mod sensor;
mod style;
pub mod touch;
mod video_mode;
#[allow(clippy::module_inception)]
mod window;
