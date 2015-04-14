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


//! Measuring elapsed time
//!
//! Utility class that measures the elapsed time

use traits::Wrappable;
use system::Time;

use ffi::system::clock as ffi;

/// Measuring elapsed time
///
/// Utility class that measures the elapsed time
pub struct Clock {
    clock: *mut ffi::sfClock
}

impl Clock {
    /// Create a new Clock and start it.
    pub fn new() -> Clock {
        Clock {
            clock: unsafe { ffi::sfClock_create() }
        }
    }

    /// Create a clock by copying an extant one
    pub fn clone(&self) -> Clock {
        Clock {
            clock: unsafe { ffi::sfClock_copy(self.clock) }
        }
    }

    /// Get the time elapsed in a clock
    pub fn get_elapsed_time(&self) -> Time {
        unsafe {
            Wrappable::wrap(ffi::sfClock_getElapsedTime(self.clock))
        }
    }

    /// Restart a Clock.
    pub fn restart(&mut self) -> Time {
        unsafe {
            Wrappable::wrap(ffi::sfClock_restart(self.clock))
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *mut ffi::sfClock {
        self.clock
    }
}

impl Drop for Clock {
    /// Destroy a clock
    fn drop(&mut self) {
        unsafe {
            ffi::sfClock_destroy(self.clock)
        }
    }
}
