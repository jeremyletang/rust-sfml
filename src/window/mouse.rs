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
* Mouse events.
*
* Give access to the real-time state of the mouse
*/

use std::libc::c_uint;

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi = ffi::window::mouse;

/// Mouse buttons
#[deriving(Clone, Eq, Ord, Show)]
pub enum MouseButton {
    /// The left mouse button.
    MouseLeft,
    /// The right mouse button.
    MouseRight,
    /// The middle (wheel) mouse button.
    MouseMiddle,
    /// The first extra mouse button.
    MouseXButton1,
    /// The second extra mouse button.
    MouseXButton2
}

/**
* Check if a mouse button is pressed
*
* # Arguments
* * button - Button to check
*
* Return true if the button is pressed, false otherwise
*/
pub fn is_button_pressed(button : MouseButton) -> bool {
    unsafe {
        match ffi::sfMouse_isButtonPressed(button as c_uint) {
            SFFALSE   => false,
            SFTRUE    => true
        }
    }
}
