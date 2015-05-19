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

pub mod clock {
    use ffi::system::time::sfTime;

    #[repr(C)]
    pub struct sfClock;

    extern "C" {
        pub fn sfClock_create() -> *mut sfClock;
        pub fn sfClock_copy(clock: *mut sfClock) -> *mut sfClock;
        pub fn sfClock_destroy(clock: *mut sfClock) -> ();
        pub fn sfClock_getElapsedTime(clock: *mut sfClock) -> sfTime;
        pub fn sfClock_restart(clock: *mut sfClock) -> sfTime;
    }
}

pub mod sleep {
    use ffi::system::time::sfTime;

    extern "C" {
        pub fn sfSleep(duration: sfTime) -> ();
    }
}

pub mod time {
    pub use libc::{c_longlong, c_float, c_int};

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct sfTime {
        microseconds: c_longlong
    }

    extern "C" {
        pub fn sfTime_asSeconds(time: sfTime) -> c_float;
        pub fn sfTime_asMilliseconds(time: sfTime) -> c_int;
        pub fn sfTime_asMicroseconds(time: sfTime) -> c_longlong;
        pub fn sfSeconds(amount: c_float) -> sfTime;
        pub fn sfMilliseconds(amount: c_int) -> sfTime;
        pub fn sfMicroseconds(amount: c_longlong) -> sfTime;
    }
}
