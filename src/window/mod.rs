//! Provides OpenGL-based windows, and abstractions for events and input handling.

pub use self::context::Context;
pub use self::context_settings::ContextSettings;
pub use self::event::Event;
pub use self::keyboard::{set_virtual_keyboard_visible, Key};
pub use self::style::Style;
pub use self::video_mode::VideoMode;
pub use self::window::{Handle, Window};

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
