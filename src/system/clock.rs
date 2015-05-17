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

use system::Time;

/// Simple clock capable of measuring elapsed time.
///
/// It provides the most precise time that the underlying OS can achieve
/// (generally microseconds or nanoseconds). It also ensures monotonicity, which
/// means that the returned time can never go backward, even if the system time
/// is changed.
#[derive(Debug, Clone, Copy)]
pub struct Clock {
    start_time: Time
}

impl Clock {
    /// Create a new Clock and start it.
    pub fn new() -> Clock {
        Clock {
            start_time: now()
        }
    }

    /// Get the time elapsed since the Clock was last restarted.
    pub fn get_elapsed_time(&self) -> Time {
        now() - self.start_time
    }

    /// Restart the Clock and get the time since it was last restarted.
    pub fn restart(&mut self) -> Time {
        let now = now();
        let elapsed = now - self.start_time;
        self.start_time = now;
        elapsed
    }
}

fn now() -> Time {
    unsafe { platform::now() }
}

// These implementations are derived from the original SFML implementations.

#[cfg(not(target_os = "macos"))]
#[cfg(not(target_os = "windows"))]
mod platform {
    use system::Time;
    use libc::{c_int, timespec, CLOCK_MONOTONIC};

    extern "C" {
        fn clock_gettime(clk_id: c_int, tp: *mut timespec) -> c_int;
    }

    pub unsafe fn now() -> Time {
        let mut time = timespec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(CLOCK_MONOTONIC, &mut time);
        let us = time.tv_sec as i64 * 1_000_000 + time.tv_nsec as i64 / 1_000;
        Time::with_microseconds(us)
    }
}

#[cfg(target_os = "macos")]
mod platform {
    use system::Time;

    pub unsafe fn now() -> Time {
        unimplemented!()
        /*
        // Mac OS X specific implementation (it doesn't support clock_gettime)
        static mach_timebase_info_data_t frequency = {0, 0};
        if (frequency.denom == 0)
            mach_timebase_info(&frequency);
        Uint64 nanoseconds = mach_absolute_time() * frequency.numer / frequency.denom;
        return sf::microseconds(nanoseconds / 1000);
        */
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use system::Time;
    use libc::{HANDLE, DWORD, LARGE_INTEGER};
    use libc::{QueryPerformanceFrequency, QueryPerformanceCounter};

    extern "system" {
        fn GetCurrentThread() -> HANDLE;
        fn SetThreadAffinityMask(thread: HANDLE, mask: DWORD) -> DWORD;
    }

    static mut FREQUENCY: Option<LARGE_INTEGER> = None;

    pub unsafe fn now() -> Time {
        // Force the following code to run on first core
        // (see http://msdn.microsoft.com/en-us/library/windows/desktop/ms644904(v=vs.85).aspx)
        let thread = GetCurrentThread();
        let previous_mask = SetThreadAffinityMask(thread, 1);

        // Get the frequency of the performance counter
        // (it is constant across the program lifetime)
        // NB: a static mut is being accessed here
        let frequency = match FREQUENCY {
            Some(value) => value,
            None => {
                let mut value: LARGE_INTEGER = 0;
                QueryPerformanceFrequency(&mut value);
                FREQUENCY = Some(value);
                value
            }
        };

        let mut time: LARGE_INTEGER = 0;
        QueryPerformanceCounter(&mut time);

        // Restore the thread affinity
        SetThreadAffinityMask(thread, previous_mask);

        // Return the current time as microseconds
        Time::with_microseconds(1_000_000 * time / frequency)
    }
}
