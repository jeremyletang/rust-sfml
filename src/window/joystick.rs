//! Access to the real-time state of the joysticks.
//!
//! `joystick` provides an interface to the state of the joysticks.
//!
//! Each joystick is identified by an index that is passed to the functions of this module.
//!
//! This module allows users to query the state of joysticks at any time and directly,
//! without having to deal with a window and its events. Compared to the [`JoystickMoved`],
//! [`JoystickButtonPressed`] and [`JoystickButtonReleased`] events, `Joystick` can retrieve the
//! state of axes and buttons of joysticks at any time (you don't need to store and update a
//! boolean on your side in order to know if a button is pressed or released),
//! and you always get the real state of joysticks, even if they are moved,
//! pressed or released when your window is out of focus and no event is triggered.
//!
//! [`JoystickMoved`]: crate::window::Event::JoystickMoved
//! [`JoystickButtonPressed`]: crate::window::Event::JoystickButtonPressed
//! [`JoystickButtonReleased`]: crate::window::Event::JoystickButtonReleased
//!
//! SFML supports:
//!
//! - 8 joysticks ([`COUNT`])
//! - 32 buttons per joystick ([`BUTTON_COUNT`])
//! - 8 axes per joystick ([`AXIS_COUNT`])
//!
//! Unlike the keyboard or mouse, the state of joysticks is sometimes not directly
//! available (depending on the OS), therefore an [`update`] function must be called in order to
//! update the current state of joysticks. When you have a window with event handling, this is
//! done automatically, you don't need to call anything. But if you have no window, or if you want
//! to check joysticks state before creating one, you must call [`update`] explicitly.
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
//!     let _position = joystick::axis_position(0, joystick::Axis::Y);
//! }
//! ```
//!
//! [`COUNT`]: COUNT
//! [`BUTTON_COUNT`]: BUTTON_COUNT
//! [`AXIS_COUNT`]: AXIS_COUNT
//! [`update`]: update
//!

use crate::{ffi::window as ffi, system::SfString, SfBox};
pub use ffi::JoystickAxis as Axis;

/// Maximum number of supported joysticks.
pub const COUNT: u32 = 8;
/// Maximum number of supported buttons.
pub const BUTTON_COUNT: u32 = 32;
/// Maximum number of supported axes.
pub const AXIS_COUNT: u32 = 8;

/// Check if the joystick is connected
///
/// # Arguments
/// * joystick - Index of the joystick to check
///
/// Return true if the joystick is connected, false otherwise
#[must_use]
pub fn is_connected(joystick: u32) -> bool {
    unsafe { ffi::sfJoystick_isConnected(joystick) }
}

/// Return the number of buttons supported by a joystick
///
/// # Arguments
/// * joystick - Index of the joystick
///
/// Return the number of buttons supported by the joystick.
#[must_use]
pub fn button_count(joystick: u32) -> u32 {
    unsafe { ffi::sfJoystick_getButtonCount(joystick) }
}

/// Check if the joystick support a given axis
///
/// If the joystick is not connected, this function returns false.
///
/// # Arguments
/// * joystick - Index of the joystick
/// * axis - Axis to check
///
/// Return true if the joystick supports the axis, false otherwise
#[must_use]
pub fn has_axis(joystick: u32, axis: Axis) -> bool {
    unsafe { ffi::sfJoystick_hasAxis(joystick, axis) }
}

/// Check if the button is pressed on a given joystick.
///
/// If the joystick is not connected, this function returns false.
///
/// # Arguments
/// * joystick - Index of the joystick
/// * button - Button to check
///
/// Return true if the button is pressed, false otherwise
#[must_use]
pub fn is_button_pressed(joystick: u32, button: u32) -> bool {
    unsafe { ffi::sfJoystick_isButtonPressed(joystick, button) }
}

/// Get the current position on a given axis, on a given joystick.
///
/// If the joystick is not connected, this function returns 0.
///
/// # Arguments
/// * joystick - Index of the joystick
/// * axis - Axis to check
///
/// Return the current position of the axis, in range [-100 .. 100]
#[must_use]
pub fn axis_position(joystick: u32, axis: Axis) -> f32 {
    unsafe { ffi::sfJoystick_getAxisPosition(joystick, axis) }
}

/// Update the states of all joysticks
///
/// This function is used internally by SFML, so you normally
/// don't have to call it explicitely. However, you may need to
/// call it if you have no window yet (or no window at all):
/// in this case the joysticks states are not updated automatically.
pub fn update() {
    unsafe {
        ffi::sfJoystick_update();
    }
}

decl_opaque! {
    /// Structure holding a joystick's identification.
    pub Identification;
}

/// Get the joystick information.
#[must_use]
pub fn identification(joystick: u32) -> SfBox<Identification> {
    unsafe {
        SfBox::new(ffi::sfJoystick_getIdentification(joystick))
            .expect("Failed to create JoystickIdentification")
    }
}

impl Identification {
    /// Name of the joystick.
    #[must_use]
    pub fn name(&self) -> &SfString {
        unsafe { &*ffi::sfJoystickIdentification_getName(self) }
    }
    /// Manufacturer identifier.
    #[must_use]
    pub fn vendor_id(&self) -> u32 {
        unsafe { ffi::sfJoystickIdentification_getVendorId(self) }
    }
    /// Product identifier.
    #[must_use]
    pub fn product_id(&self) -> u32 {
        unsafe { ffi::sfJoystickIdentification_getProductId(self) }
    }
}

impl Drop for Identification {
    fn drop(&mut self) {
        unsafe {
            ffi::sfJoystickIdentification_del(self);
        }
    }
}
