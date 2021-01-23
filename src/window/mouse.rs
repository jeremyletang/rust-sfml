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
//! if mouse::Button::LEFT.is_pressed() {
//!     // left click
//! }
//!
//! // get global mouse position
//! let _position = mouse::desktop_position();
//!
//! // set mouse position relative to a window
//! window.set_mouse_position(Vector2i::new(100, 200));
//! ```

use crate::{sf_bool_ext::SfBoolExt, system::Vector2i, window::thread_safety};
use csfml_window_sys as ffi;

/// Mouse buttons.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[repr(transparent)]
pub struct Button(pub(super) ffi::sfMouseButton);

impl Button {
    /// The left mouse button.
    pub const LEFT: Self = Self(ffi::sfMouseButton_sfMouseLeft);
    /// The right mouse button.
    pub const RIGHT: Self = Self(ffi::sfMouseButton_sfMouseRight);
    /// The middle (wheel) mouse button.
    pub const MIDDLE: Self = Self(ffi::sfMouseButton_sfMouseMiddle);
    /// The first extra mouse button.
    pub const X_BUTTON_1: Self = Self(ffi::sfMouseButton_sfMouseXButton1);
    /// The second extra mouse button.
    pub const X_BUTTON_2: Self = Self(ffi::sfMouseButton_sfMouseXButton2);
    /// The total number of mouse buttons.
    pub const COUNT: ffi::sfMouseButton = ffi::sfMouseButton_sfMouseButtonCount;
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
/// Mouse wheels.
pub enum Wheel {
    /// The vertical mouse wheel.
    Vertical,
    /// The horizontal mouse wheel.
    Horizontal,
}

impl Wheel {
    pub(super) fn from_raw(raw: ffi::sfMouseWheel) -> Self {
        match raw {
            ffi::sfMouseWheel_sfMouseVerticalWheel => Wheel::Vertical,
            ffi::sfMouseWheel_sfMouseHorizontalWheel => Wheel::Horizontal,
            _ => unreachable!(),
        }
    }
}

impl Button {
    /// Return whether this mouse button is currently pressed.
    ///
    /// Queries the real-time state of the mouse, even if buttons have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    #[must_use]
    pub fn is_pressed(self) -> bool {
        thread_safety::set_window_thread();

        unsafe { ffi::sfMouse_isButtonPressed(self.0) }.into_bool()
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
