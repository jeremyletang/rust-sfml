//! Access for the real-time state of touches.
//!
//! This module allows users to query the state of touches at any time and
//! directly, without having to deal with a window and its events. Compared to
//! the TouchBegan, TouchMoved, and TouchEnded events,
//! this module can retrieve the state of touches at any
//! time, and always returns the real state, even if it has changed while the
//! window is out of focus and no event is triggered.
//!
//! Touches are identified by an index (the "finger"), so that in multi-touch
//! events, individual touches can be tracked correctly. As long as a finger
//! touches the screen, it will keep the same index even if other fingers start
//! or stop touching the screen in the meantime. As a consequence, active touch
//! indices may not always be sequential (i.e. touch number 0 may be released
//! while touch number 1 is still down).

use system::Vector2i;
use window::Window;

use libc::c_uint;
use ffi::window as ffi;

/// Check if a touch event is currently down.
pub fn is_down(finger: u32) -> bool {
	unsafe { ffi::sfTouch_isDown(finger as c_uint) }.to_bool()
}

/// Get the current position of a touch relative to the specified window.
///
/// If the touch is not down, the result is undefined.
pub fn get_position(finger: u32, relative_to: &Window) -> Vector2i {
	unsafe { ffi::sfTouch_getPosition(finger as c_uint, relative_to.unwrap()) }
}

// TODO: get_position equivalent for RenderWindow
// maybe move get_position above to method on Window?
