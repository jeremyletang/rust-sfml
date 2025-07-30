//! Access to the real-time state of the touches.
//!
//! `Touch` provides an interface to the state of the touches.
//!
//! This module allows users to query the touches state at any time and directly,
//! without having to deal with a window and its events.
//! Compared to the [`TouchBegan`], [`TouchMoved`] and [`TouchEnded`] events,
//! `touch` can retrieve the state of the touches at any time
//! (you don't need to store and update a boolean on your side in
//! order to know if a touch is down), and you always get the real state of the touches,
//! even if they happen when your window is out of focus and no event is triggered.
//!
//! [`TouchBegan`]: crate::window::Event::TouchBegan
//! [`TouchMoved`]: crate::window::Event::TouchMoved
//! [`TouchEnded`]: crate::window::Event::TouchEnded
//!
//! There are two functions that can be used to retrieve the current position of a touch.
//! 1. [`desktop_position`]: operates in global coordinates (relative to the desktop)
//! 2. [`Window::touch_position`]: operates in window coordinates (relative to a specific window).
//!
//! Touches are identified by an index (the "finger"), so that in multi-touch events, individual
//! touches can be tracked correctly. As long as a finger touches the screen, it will keep the same
//! index even if other fingers start or stop touching the screen in the meantime.
//! As a consequence, active touch indices may not always be sequential
//! (i.e. touch number 0 may be released while touch number 1 is still down).
//!
//! [`desktop_position`]: desktop_position
//! [`Window::touch_position`]: crate::window::Window::touch_position
//!

use crate::{ffi::window::{sfTouch_isDown, sfTouch_getPosition}, system::Vector2i};

/// Check if a touch event is currently down.
#[must_use]
pub fn is_down(finger: u32) -> bool {
    unsafe { sfTouch_isDown(finger) }
}

/// Get the current position of a touch in desktop coordinates.
///
/// This function returns the current touch position in global (desktop) coordinates.
#[must_use]
pub fn desktop_position(finger: u32) -> Vector2i {
    unsafe { sfTouch_getPosition(finger) }
}
