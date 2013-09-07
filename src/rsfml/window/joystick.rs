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

/*!
* Handle Joysticks
*
* Offers a set of function for manage joystick
*
*/

use std::libc::{c_uint};

/// Axes supported by SFML joysticks 
pub enum Axis {
    X,
    Y,
    Z,
    R,
    U,
    V,
    PovX,
    PovY
}

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_float, c_uint};
    use sfml_types::{SfBool};
    

    pub enum sfJoystickAxis {
        X,
        Y,
        Z,
        R,
        U,
        V,
        PovX,
        PovY
    }

    extern "C" {
        pub fn sfJoystick_isConnected(joystick : c_uint) -> SfBool;
        pub fn sfJoystick_getButtonCount(joystick : c_uint) -> c_uint;
        pub fn sfJoystick_hasAxis(joystick : c_uint, axis : c_uint) -> SfBool;
        pub fn sfJoystick_isButtonPressed(joystick : c_uint, button : c_uint) -> SfBool;
        pub fn sfJoystick_getAxisPosition(joystick : c_uint, axis : c_uint) -> c_float;
        pub fn sfJoystick_update() -> ();
    }
}

/**
* Check if the joystick is connected
*
* # Arguments
* * joystick - Index of the joystick to check
*
* Return true if the joystick is connected, false otherwise
*/
#[fixed_stack_segment] #[inline(never)]
pub fn joystick_is_connected(joystick : uint) -> bool {
    unsafe {
        match ffi::sfJoystick_isConnected(joystick as c_uint) {
            0   => false,
            _   => true
        }
    }
}

/**
* Return the number of buttons supported by a joystick
*
* # Arguments
* * joystick - Index of the joystick
*
* Return the number of buttons supported by the joystick.
*/
#[fixed_stack_segment] #[inline(never)]
pub fn joystick_get_button_count(joystick : uint) -> uint {
    unsafe {
        ffi::sfJoystick_getButtonCount(joystick as c_uint) as uint
    }
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
#[fixed_stack_segment] #[inline(never)]
pub fn joystick_has_axis(joystick : uint, axis : Axis) -> bool {
    unsafe {
        match ffi::sfJoystick_hasAxis(joystick as c_uint, axis as c_uint) {
            0   => false,
            _   => true
        }
    }
    
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
#[fixed_stack_segment] #[inline(never)]
pub fn joystick_is_button_pressed(joystick : uint, button : uint) -> bool {
    unsafe {
        match ffi::sfJoystick_isButtonPressed(joystick as c_uint, button as c_uint) {
            0   => false,
            _   => true
        }
    }
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
#[fixed_stack_segment] #[inline(never)]
pub fn joystick_get_axis_position(joystick : uint, axis : Axis) -> float {
    unsafe {
        ffi::sfJoystick_getAxisPosition(joystick as c_uint, axis as c_uint) as float
    }
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
#[fixed_stack_segment] #[inline(never)]
pub fn joystick_update() -> () {
    unsafe {
        ffi::sfJoystick_update();
    }
}
