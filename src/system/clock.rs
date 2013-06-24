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
* Measuring elapsed time
*
* Utility class that measures the elapsed time
*
*/

use system::time::Time;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_void};
    use system::time::*;

    pub struct sfClock {
        This : *c_void
    }

    pub extern "C" {
        fn sfClock_create() -> *sfClock;
        fn sfClock_copy(clock: *sfClock) -> *sfClock;
        fn sfClock_destroy(clock : *sfClock) -> ();
        fn sfClock_getElapsedTime(clock : *sfClock) -> csfml::sfTime;
        fn sfClock_restart(clock : *sfClock) -> csfml::sfTime;
    }
}


#[doc(hidden)]
pub struct Clock {
    priv clock : *csfml::sfClock
}

impl Clock {
    
    /**
    * Create a new Clock and start it.
    */
    pub fn new() -> Clock {
        Clock {clock : unsafe {csfml::sfClock_create()}}
    }

    /**
    * Get the time elapsed in a clock
    */
    pub fn get_elapsed_time(&self) -> Time {
        unsafe {
            Time::wrap(csfml::sfClock_getElapsedTime(self.clock))
        }
    }

    /**
    * Restart a Clock.
    */
    pub fn restart(&self) -> Time {
        unsafe {
            Time::wrap(csfml::sfClock_restart(self.clock))
        }
    }
}

impl Clone for Clock {
    fn clone(&self) -> Clock {
        unsafe {
            Clock {
                clock: csfml::sfClock_copy(self.clock)
            }
        }
    }
}

impl Drop for Clock {
    
    /**
    * Create a new Clock and start it.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfClock_destroy(self.clock)
        }
    }
}