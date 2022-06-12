//! Access to the real-time state of the mouse.
//!
//! `mouse` provides an interface to the state of the mouse.
//!
//! This module allows users to query the mouse state at any time and directly, without having to
//! deal with a window and its events. Compared to the [`MouseMoved`], [`MouseButtonPressed`] and
//! [`MouseButtonReleased`] events, `mouse` can retrieve the state of the cursor and
//! the buttons at any time (you don't need to store and update a boolean on your side in order to
//! know if a button is pressed or released), and you always get the real state of the mouse, even
//! if it is moved, pressed or released when your window is out of focus and no event is triggered.
//!
//! [`MouseMoved`]: crate::window::Event::MouseMoved
//! [`MouseButtonPressed`]: crate::window::Event::MouseButtonPressed
//! [`MouseButtonReleased`]: crate::window::Event::MouseButtonReleased
//!
//! # Usage example
//!
//! ```no_run
//! use sfml::window::{Window, mouse};
//! use sfml::system::Vector2i;
//!
//! # let window: Window = unimplemented!();
//!
//! if mouse::Button::Left.is_pressed() {
//!     // left click
//! }
//!
//! // get global mouse position
//! let _position = mouse::desktop_position();
//!
//! // set mouse position relative to a window
//! window.set_mouse_position(Vector2i::new(100, 200));
//! ```

use crate::{ffi::window as ffi, sf_bool_ext::SfBoolExt, system::Vector2i, window::thread_safety};
pub use ffi::window::{MouseButton as Button, MouseWheel as Wheel};

impl Button {
    /// Return whether this mouse button is currently pressed.
    ///
    /// Queries the real-time state of the mouse, even if buttons have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    #[must_use]
    pub fn is_pressed(self) -> bool {
        thread_safety::set_window_thread();

        unsafe { ffi::sfMouse_isButtonPressed(self) }.into_bool()
    }
}

/// Get the current position of the mouse in desktop coordinates.
///
/// This function returns the global position of the mouse cursor on the desktop.
#[must_use]
pub fn desktop_position() -> Vector2i {
    unsafe { Vector2i::from_raw(ffi::sfMouse_getPosition(std::ptr::null())) }
}

/// Set the current position of the mouse in desktop coordinates.
///
/// This function sets the global position of the mouse cursor on the desktop.
pub fn set_desktop_position(position: Vector2i) {
    unsafe { ffi::sfMouse_setPosition(position.raw(), std::ptr::null()) }
}
