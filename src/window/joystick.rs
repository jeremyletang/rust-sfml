/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Access for the real-time state of the joysticks.
//!
//! This module allows users to query the state of joysticks at any time and
//! directly, without having to deal with a window and its events. Compared to
//! the JoystickMoved, JoystickButtonPressed, and JoystickButtonReleased events,
//! this module can retrieve the state of axes and buttons of joysticks at any
//! time, and always returns the real state of joysticks, even if they are
//! moved, pressed or released when your window is out of focus and no event is
//! triggered.
//!
//! SFML supports:
//!
//! * 8 joysticks (`joystick::Count`)
//! * 32 buttons per joystick (`joystick::BUTTON_COUNT`)
//! * 8 axes per joystick (`joystick::AXIS_COUNT`)
//!
//! Unlike the keyboard and mouse, the state of joysticks is sometimes not
//! directly available (depending on the OS), and so an `update()` function must
//! be called in order to update the current state of joysticks. When you have a
//! window with event handling, this is done automatically, and there's no need
//! to call anything. If you have no window, or if you want to check joystick
//! state before creating one, you must call `update()` explicitly.

use libc::c_uint;
use ffi::window as ffi;

/// Maximum number of supported joysticks.
pub const COUNT: u32 = 8;
/// Maximum number of supported buttons per joystick.
pub const BUTTON_COUNT: u32 = 32;
/// Maximum number of supported axes per joystick.
pub const AXIS_COUNT: u32 = 8;

/// Axes supported by SFML joysticks.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum Axis {
    /// The X axis.
    X,
    /// The Y axis.
    Y,
    /// The Z axis.
    Z,
    /// The R axis.
    R,
    /// The U axis.
    U,
    /// The V axis.
    V,
    /// The X axis of the point-of-view hat.
    PovX,
    /// The Y axis of the point-of-view hat.
    PovY
}

/// Structure holding a joystick's identification.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct JoystickIdentification {
	/// Name of the joystick.
	pub name: String,
	/// Product identifier.
	pub product_id: u32,
	/// Manufacturer identifier.
	pub vendor_id: u32
}

/// Check if a joystick is connected, by index.
pub fn is_connected(joystick: u32) -> bool {
    unsafe {
        ffi::sfJoystick_isConnected(joystick as c_uint).to_bool()
    }
}

/// Return the number of buttons supported by a joystick, by index.
pub fn button_count(joystick: u32) -> u32 {
    unsafe {
        ffi::sfJoystick_getButtonCount(joystick as c_uint) as u32
    }
}

/// Check if a joystick supports a given Axis, given the joystick and Axis.
///
/// If the joystick is not connected, this function returns false.
pub fn has_axis(joystick: u32, axis: Axis) -> bool {
    unsafe {
        ffi::sfJoystick_hasAxis(joystick as c_uint, axis as c_uint).to_bool()
    }
}

/// Check if a joystick button is pressed, given the joystick and button.
///
/// If the joystick is not connected, this function returns false.
pub fn is_button_pressed(joystick: u32, button: u32) -> bool {
    unsafe {
        ffi::sfJoystick_isButtonPressed(joystick as c_uint, button as c_uint).to_bool()
    }
}

/// Get the current position of a joystick axis, given the joystick and Axis.
///
/// The returned value will be in the range [-100 .. 100]. If the joystick is
/// not connected, this function returns 0.
pub fn get_axis_position(joystick: u32, axis: Axis) -> f32 {
    unsafe {
        ffi::sfJoystick_getAxisPosition(joystick as c_uint, axis as c_uint) as f32
    }
}

/// Get the identification info for a joystick by index.
pub fn get_identification(joystick: u32) -> JoystickIdentification {
	unsafe {
		let raw = ffi::sfJoystick_getIdentification(joystick as c_uint);
		JoystickIdentification {
			name: ::ffi::from_c_str(raw.name),
			product_id: raw.product_id as u32,
			vendor_id: raw.vendor_id as u32
		}
	}
}

/// Update the states of all joysticks.
///
/// This function is used internally by SFML, so you normally don't have to call
/// it explicitly. However, you may need to call it if you have no window yet,
/// or no window at all, in which case the states are not updated automatically.
pub fn update() {
    unsafe {
        ffi::sfJoystick_update()
    }
}
