// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

use raw_conv::FromRaw;
use system::Time;

use csfml_system_sys as ffi;

/// Utility type that measures the elapsed time.
///
/// Its provides the most precise time that the underlying OS can
/// achieve (generally microseconds or nanoseconds).
/// It also ensures monotonicity, which means that the returned time can never go backward,
/// even if the system time is changed.
///
/// # Usage example
/// ```
/// # use sfml::system::Clock;
/// let mut clock = Clock::start();
/// // ...
/// let time1 = clock.elapsed_time();
/// // ...
/// let time2 = clock.restart();
/// ```
///
/// The `Time` value returned by the clock can then be converted to
/// a number of seconds, milliseconds or even microseconds.
pub struct Clock {
    clock: *mut ffi::sfClock,
}

impl Clock {
    /// Creates a new Clock and starts it automatically.
    pub fn start() -> Clock {
        Clock { clock: unsafe { ffi::sfClock_create() } }
    }

    /// Gets the elapsed time.
    ///
    /// This function returns the time elapsed since the last call to restart()
    /// (or the construction of the instance if restart() has not been called).
    pub fn elapsed_time(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_getElapsedTime(self.clock)) }
    }

    /// Restarts the clock.
    ///
    /// This function puts the time counter back to zero.
    /// It also returns the time elapsed since the clock was started.
    pub fn restart(&mut self) -> Time {
        unsafe { Time::from_raw(ffi::sfClock_restart(self.clock)) }
    }
}

impl Clone for Clock {
    fn clone(&self) -> Clock {
        Clock { clock: unsafe { ffi::sfClock_copy(self.clock) } }
    }
}

impl Drop for Clock {
    fn drop(&mut self) {
        unsafe { ffi::sfClock_destroy(self.clock) }
    }
}

impl Default for Clock {
    /// Equivalent to `Clock::start()`.
    fn default() -> Self {
        Clock::start()
    }
}
