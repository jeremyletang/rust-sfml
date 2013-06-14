/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
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
* Mouse events.
*
* Give access to the real-time state of the mouse.
*
*/

use std::libc::{c_uint};
use window::window::*;
use system::vector2;

/// Mouse buttons
pub enum MouseButton {
    MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseXButton1,
    MouseXButton2
}

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_uint};
    use rsfml::sfTypes::{sfBool};
    use window::window::*;
    use system::vector2;

    pub extern "C" {
        fn sfMouse_isButtonPressed(button : c_uint) -> sfBool;
        fn sfMouse_getPosition(relativeTo : *csfml::sfWindow) -> vector2::Vector2i;
        fn sfMouse_setPosition(position : vector2::Vector2i, relativeTo : *csfml::sfWindow) -> ();
    }
}

/**
* Check if a mouse button is pressed
*
* # Arguments
* * button - Button to check
*
* Return true if the button is pressed, false otherwise
*/
pub fn mouse_is_button_pressed(button : MouseButton) -> bool {
    unsafe {
        match csfml::sfMouse_isButtonPressed(button as c_uint) {
            0   => false,
            _   => true
        }
    }
}

/**
*  Get the current position of the mouse
*
* This function returns the current position of the mouse cursor relative to the given window.
*
* # Arguments
* * relativeTo - Reference Window
*
* Return the position of the mouse cursor, relative to the given window
*/
pub fn mouse_get_position(relativeTo : &Window) -> vector2::Vector2i {
    unsafe {
        csfml::sfMouse_getPosition(relativeTo.unwrap())
    }
}

/**
* Set the current position of the mouse
*
* This function sets the current position of the mouse cursor relative to the given window.
*
* # Arguments
* * position - New position of the mouse
* * relativeTo - Reference Window
*
*/
pub fn mouse_set_position(position : &vector2::Vector2i, relativeTo : &Window) -> () {
    unsafe {
        csfml::sfMouse_setPosition(*position, relativeTo.unwrap())
    }
}