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
* Handle concurrents access
*
* Blocks concurrent access to shared resources from multiple threads.
*
*/

#[doc(hidden)]
pub mod csfml {
    
    pub use std::libc::{c_void};
    
    pub struct sfMutex {
        This : *c_void
    }
    
    pub extern "C" {
        fn sfMutex_create() -> *sfMutex;
        fn sfMutex_destroy(mutex : *sfMutex) -> ();
        fn sfMutex_lock(mutex : *sfMutex) -> ();
        fn sfMutex_unlock(mutex : *sfMutex) -> ();
    }
}

#[doc(hidden)]
pub struct Mutex {
    priv mutex : *csfml::sfMutex
}

impl Mutex {

    /**
    * Create a new Mutex
    */
    pub fn new() -> Mutex {
        Mutex {
            mutex : unsafe { csfml::sfMutex_create() }
        }
    }

    
    /**
    * Lock a Mutex
    */
    pub fn lock(&self) -> () {
        unsafe {
            csfml::sfMutex_lock(self.mutex)
        }
    }
    
    /**
    * Unlock a Mutex
    */
    pub fn unlock(&self) -> () {
        unsafe {
            csfml::sfMutex_unlock(self.mutex)
        }
    }
}

impl Drop for Mutex {

    /**
    * Destroy a Mutex
    */
    fn drop(&self) -> () {
        unsafe {
            csfml::sfMutex_destroy(self.mutex)
        }
    }
}