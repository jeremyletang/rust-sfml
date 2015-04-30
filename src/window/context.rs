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

//! Drawing context
//!
//! Class holding a valid drawing context.

use ffi::{SfBool, Foreign};
use ffi::window as ffi;

/// Container for an OpenGL drawing context.
pub struct Context(Foreign<ffi::sfContext>);

impl Context {
    /// Create a new context, also activating it.
    pub fn new() -> Context {
		unsafe {
			Foreign::new(ffi::sfContext_create())
		}.map(Context).expect("Failed to create Context")
    }

    /// Explicitly activate or deactivate the context.
    pub fn set_active(&mut self, active: bool) -> () {
        unsafe {
            ffi::sfContext_setActive(self.0.as_mut(), SfBool::from_bool(active))
        }
    }
}
