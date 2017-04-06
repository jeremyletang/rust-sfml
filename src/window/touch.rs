//! Access to the real-time state of the touches.
//!
//! `Touch` provides an interface to the state of the touches.
//!
//! This module allows users to query the touches state at any time and directly,
//! without having to deal with a window and its events.
//! Compared to the `TouchBegan`, `TouchMoved` and `TouchEnded` events, `touch` can retrieve the
//! state of the touches at any time (you don't need to store and update a boolean on your side in
//! order to know if a touch is down), and you always get the real state of the touches,
//! even if they happen when your window is out of focus and no event is triggered.
//!
//! The `position` function can be used to retrieve the current position of a touch.
//! There are two versions: one that operates in global coordinates (relative to the desktop)
//! and one that operates in window coordinates (relative to a specific window).
//!
//! Touches are identified by an index (the "finger"), so that in multi-touch events, individual
//! touches can be tracked correctly. As long as a finger touches the screen, it will keep the same
//! index even if other fingers start or stop touching the screen in the meantime.
//! As a consequence, active touch indices may not always be sequential
//! (i.e. touch number 0 may be released while touch number 1 is still down).
//!
//! # Usage example
//! ```ignore
//! use sfml::window::touch;
//!
//! if touch::is_down(0) {
//!     // touch 0 is down
//! }
//!
//! // get global position of touch 1
//! let _global_pos = touch::desktop_position(1);
//!
//! // get position of touch 1 relative to a window
//! let _relative_pos = window.touch_position(1);
//! ```

use csfml_window_sys::*;
use sf_bool_ext::SfBoolExt;
use system::Vector2i;
use system::raw_conv::FromRaw;

/// Check if a touch event is currently down.
pub fn is_down(finger: u32) -> bool {
    unsafe { sfTouch_isDown(finger).to_bool() }
}

/// Get the current position of a touch in desktop coordinates.
///
/// This function returns the current touch position in global (desktop) coordinates.
pub fn desktop_position(finger: u32) -> Vector2i {
    unsafe { FromRaw::from_raw(sfTouch_getPosition(finger, ::std::ptr::null())) }
}
