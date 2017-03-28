//! Provides OpenGL-based windows, and abstractions for events and input handling.

pub use self::window::Window;
pub use self::video_mode::VideoMode;
pub use self::context::Context;
pub use self::context_settings::{ContextSettings, CONTEXT_DEFAULT, CONTEXT_CORE, CONTEXT_DEBUG};
pub use self::style::Style;
pub use self::keyboard::{Key, set_virtual_keyboard_visible};
pub use self::event::Event;

#[cfg_attr(feature="clippy", allow(module_inception))]
mod window;
mod video_mode;
mod context;
mod context_settings;
pub mod joystick;
mod keyboard;
pub mod mouse;
mod event;
pub mod style;
pub mod sensor;
pub mod touch;
