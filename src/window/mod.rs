//! Provides OpenGL-based windows, and abstractions for events and input handling.

pub use {
    self::{
        context::Context,
        context_settings::ContextSettings,
        cursor::{Cursor, Type as CursorType},
        event::Event,
        keyboard::{Key, set_virtual_keyboard_visible},
        video_mode::VideoMode,
        window::{Handle, Window},
        window_enums::Style,
    },
    crate::ffi::window::Scancode,
};

pub mod clipboard;
mod context;
mod context_settings;
mod cursor;
pub(crate) mod event;
pub mod joystick;
mod keyboard;
pub mod mouse;
pub mod sensor;
pub(crate) mod thread_safety;
mod video_mode;
pub mod vulkan;
#[expect(clippy::module_inception)]
mod window;
pub mod window_enums;
