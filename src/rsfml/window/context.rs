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
 * Drawing context
 *
 * Class holding a valid drawing context.
 */

use sfml_types::*;

#[doc(hidden)]
pub mod ffi {

    use std::libc::{c_void};
    use sfml_types::{SfBool};

    pub struct sfContext {
        This: *c_void
    }

    extern "C" {
        pub fn sfContext_create() -> *sfContext;
        pub fn sfContext_destroy(context : *sfContext) -> ();
        pub fn sfContext_setActive(context : *sfContext, active : SfBool) -> ();
    }
}

#[doc(hidden)]
pub struct Context {
    priv cont : *ffi::sfContext
}

impl Context {

    /**
    * Create a new context
    *
    * This function activates the new context.
    *
    * Return New Context object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Context {
        Context{
            cont : unsafe{ ffi::sfContext_create() }
        }
    }

    /**
    * Activate or deactivate explicitely a context
    *
    * # Arguments
    * * active - True to activate, False to deactivate
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_active(&mut self, active : bool) -> () {
        unsafe {
            match active {
                true    => ffi::sfContext_setActive(self.cont, SFTRUE),
                false   => ffi::sfContext_setActive(self.cont, SFFALSE)
            };

        }
    }

}

impl Drop for Context {
    /*
    *   Destructor for class Context.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) {
        unsafe {
            ffi::sfContext_destroy(self.cont);
        }
    }
}
