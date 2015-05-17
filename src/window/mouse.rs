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

use libc::c_uint;
use ffi::window as ffi;

/// Mouse buttons.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle (wheel) mouse button.
    Middle,
    /// The first extra mouse button.
    XButton1,
    /// The second extra mouse button.
    XButton2
}

impl MouseButton {
    /// Return whether this mouse button is currently pressed.
	///
	/// Queries the real-time state of the mouse, even if buttons have been
	/// pressed or released while no window was focused and no events were
	/// triggered.
    pub fn is_pressed(self) -> bool {
        unsafe { ffi::sfMouse_isButtonPressed(self as c_uint) }.to_bool()
    }
}
