// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Access to the real-time state of the joysticks.
//!
//! `joystick` provides an interface to the state of the joysticks.
//!
//! Each joystick is identified by an index that is passed to the functions of this module.
//!
//! This module allows users to query the state of joysticks at any time and directly,
//! without having to deal with a window and its events. Compared to the `JoystickMoved`,
//! `JoystickButtonPressed` and `JoystickButtonReleased` events, `Joystick` can retrieve the
//! state of axes and buttons of joysticks at any time (you don't need to store and update a
//! boolean on your side in order to know if a button is pressed or released), and you always get
//! the real state of joysticks, even if they are moved, pressed or released when your window is
//! out of focus and no event is triggered.
//!
//! SFML supports:
//!
//! - 8 joysticks (`COUNT`)
//! - 32 buttons per joystick (`BUTTON_COUNT`)
//! - 8 axes per joystick (`AXIS_COUNT`)
//!
//! Unlike the keyboard or mouse, the state of joysticks is sometimes not directly
//! available (depending on the OS), therefore an `update()` function must be called in order to
//! update the current state of joysticks. When you have a window with event handling, this is
//! done automatically, you don't need to call anything. But if you have no window, or if you want
//! to check joysticks state before creating one, you must call `joystick::update` explicitly.
//! # Usage example
//!
//! ```
//! use sfml::window::joystick;
//!
//! // If joystick #0 is connected
//! if joystick::is_connected(0) {
//!     // How many buttons does joystick #0 support?
//!     let _buttons = joystick::button_count(0);
//!     // Does joystick #0 define a X axis?
//!     let _hax_x = joystick::has_axis(0, joystick::Axis::X);
//!     // Is button #2 pressed on joystick #0?
//!     let _pressed = joystick::is_button_pressed(0, 2);
//!     // What's the current position of the Y axis on joystick #0?
//!     let _position = joystick::get_axis_position(0, joystick::Axis::Y);
//! }
//! ```
//!

use libc::c_uint;

use csfml_window_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::{Raw, FromRaw};

/// Maximum number of supported joysticks.
pub const COUNT: u32 = 8;
/// Maximum number of supported buttons.
pub const BUTTON_COUNT: u32 = 32;
/// Maximum number of supported axes.
pub const AXIS_COUNT: u32 = 8;

/// Axes supported by SFML joysticks
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[repr(u32)]
pub enum Axis {
    /// The X axis.
    X = 0,
    /// The Y axis.
    Y = 1,
    /// The Z axis.
    Z = 2,
    /// The R axis.
    R = 3,
    /// The U axis.
    U = 4,
    /// The V axis.
    V = 5,
    /// The X axis of the point-of-view hat.
    PovX = 6,
    /// The Y axis of the point-of-view hat.
    PovY = 7,
}

impl Raw for Axis {
    type Raw = ffi::sfJoystickAxis;
    fn raw(&self) -> Self::Raw {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for Axis {
    fn from_raw(raw: Self::Raw) -> Self {
        unsafe { ::std::mem::transmute(raw) }
    }
}

/// Structure holding a joystick's identification.
pub struct Identification {
    /// Name of the joystick.
    pub name: String,
    /// Manufacturer identifier.
    pub vendor_id: u32,
    /// Product identifier.
    pub product_id: u32,
}

/**
 * Check if the joystick is connected
 *
 * # Arguments
 * * joystick - Index of the joystick to check
 *
 * Return true if the joystick is connected, false otherwise
 */
pub fn is_connected(joystick: u32) -> bool {
    unsafe { ffi::sfJoystick_isConnected(joystick as c_uint).to_bool() }
}

/**
 * Return the number of buttons supported by a joystick
 *
 * # Arguments
 * * joystick - Index of the joystick
 *
 * Return the number of buttons supported by the joystick.
 */
pub fn button_count(joystick: u32) -> u32 {
    unsafe { ffi::sfJoystick_getButtonCount(joystick as c_uint) as u32 }
}

/**
 * Check if the joystick support a given Axis
 *
 * If the joystick is not connected, this function returns false.
 *
 * # Arguments
 * * joystick - Index of the joystick
 * * axis - Axis to check
 *
 * Return true if the joystick supports the axis, false otherwise
 */
pub fn has_axis(joystick: u32, axis: Axis) -> bool {
    unsafe { ffi::sfJoystick_hasAxis(joystick as c_uint, axis.raw()).to_bool() }
}

/**
 * Check if the button is pressed on a given joystick.
 *
 * If the joystick is not connected, this function returns false.
 *
 * # Arguments
 * * joystick - Index of the joystick
 * * button - Button to check
 *
 * Return true if the button is pressed, false otherwise
 */
pub fn is_button_pressed(joystick: u32, button: u32) -> bool {
    unsafe { ffi::sfJoystick_isButtonPressed(joystick as c_uint, button as c_uint).to_bool() }
}


/**
 * Get the current position on a given Axis, on a given joystick.
 *
 * If the joystick is not connected, this function returns 0.
 *
 * # Arguments
 * * joystick - Index of the joystick
 * * axis - Axis to check
 *
 * Return the current position of the axis, in range [-100 .. 100]
 */
pub fn get_axis_position(joystick: u32, axis: Axis) -> f32 {
    unsafe { ffi::sfJoystick_getAxisPosition(joystick as c_uint, axis.raw()) as f32 }
}

/**
 * Update the states of all joysticks
 *
 * This function is used internally by SFML, so you normally
 * don't have to call it explicitely. However, you may need to
 * call it if you have no window yet (or no window at all):
 * in this case the joysticks states are not updated automatically.
 *
 */
pub fn update() {
    unsafe {
        ffi::sfJoystick_update();
    }
}

/// Get the joystick information.
pub fn get_identification(joystick: u32) -> Identification {
    use std::ffi::CStr;

    let raw = unsafe { ffi::sfJoystick_getIdentification(joystick) };

    Identification {
        name: unsafe { CStr::from_ptr(raw.name).to_string_lossy().into_owned() },
        vendor_id: raw.vendorId,
        product_id: raw.productId,
    }
}
