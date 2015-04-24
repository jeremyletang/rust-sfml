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

use ffi::sfml_types::SfBool;
use ffi::window::context as ffi;

/// Drawing context
///
/// Class holding a valid drawing context.
pub struct Context {
    cont: *mut ffi::sfContext
}

impl Context {

    /// Create a new context
    ///
    /// This function activates the new context.
    ///
    /// Return New Context object
    pub fn new() -> Context {
        Context{
            cont: unsafe { ffi::sfContext_create() }
        }
    }

    /// Activate or deactivate explicitely a context
    ///
    /// # Arguments
    /// * active - True to activate, False to deactivate
    pub fn set_active(&mut self, active: bool) -> () {
        unsafe {
            ffi::sfContext_setActive(self.cont, SfBool::from_bool(active))
        }
    }
}

impl Drop for Context {
    /// Destructor for class Context.
    fn drop(&mut self) {
        unsafe {
            ffi::sfContext_destroy(self.cont);
        }
    }
}
