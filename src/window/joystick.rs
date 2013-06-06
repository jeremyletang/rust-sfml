/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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

use core::libc::{c_uint};

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
pub mod csfml {
    
    use core::libc::{c_float, c_uint};
    use rsfml::sfTypes::{sfBool};
    

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

    pub extern "C" {
        fn sfJoystick_isConnected(joystick : c_uint) -> sfBool;
        fn sfJoystick_getButtonCount(joystick : c_uint) -> c_uint;
        fn sfJoystick_hasAxis(joystick : c_uint, axis : c_uint) -> sfBool;
        fn sfJoystick_isButtonPressed(joystick : c_uint, button : c_uint) -> sfBool;
        fn sfJoystick_getAxisPosition(joystick : c_uint, axis : c_uint) -> c_float;
        fn sfJoystick_update() -> ();
    }
}

/**
* Check if the joystick is connected.
*/
pub fn joystick_is_connected(joystick : uint) -> bool {
    unsafe {
        match csfml::sfJoystick_isConnected(joystick as c_uint) {
            0   => false,
            _   => true
        }
    }
}

/**
* Return the number of buttons supported by a joystick
*/
pub fn joystick_get_button_count(joystick : uint) -> uint {
    unsafe {
        csfml::sfJoystick_getButtonCount(joystick as c_uint) as uint
    }
}

/**
* Check if the joystick support a given Axis
*/
pub fn joystick_has_axis(joystick : uint, axis : Axis) -> bool {
    unsafe {
        match csfml::sfJoystick_hasAxis(joystick as c_uint, axis as c_uint) {
            0   => false,
            _   => true
        }
    }
    
}

/**
* Check if the button is pressed on a given joystick.
*/
pub fn joystick_is_button_pressed(joystick : uint, button : uint) -> bool {
    unsafe {
        match csfml::sfJoystick_isButtonPressed(joystick as c_uint, button as c_uint) {
            0   => false,
            _   => true
        }
    }
}


/**
* Get the current position on a given Axis, on a given joystick.
*/
pub fn joystick_get_axis_position(joystick : uint, axis : Axis) -> float {
    unsafe {
        csfml::sfJoystick_getAxisPosition(joystick as c_uint, axis as c_uint) as float
    }
}

/**
* Update All joystick states.
*/
pub fn joystick_update() -> () {
    unsafe {
        csfml::sfJoystick_update();
    }
}