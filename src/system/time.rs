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

//! Represents a time value.
//!
//! Time encapsulates a time value in a flexible way.

pub use libc::{c_long, c_float, c_int};

use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

use traits::Wrappable;

use csfml_system_sys as ffi;

/// Represents a time value.
///
/// Time encapsulates a time value in a flexible way.
#[derive(Copy, Clone)]
pub struct Time {
    time: ffi::sfTime
}

impl Time {
    /// Construct a time value from a number of seconds
    pub fn with_seconds(seconds: f32) -> Time {
        Time {
            time: unsafe { ffi::sfSeconds(seconds as c_float) }
        }
    }

    /// Construct a time value from a number of milliseconds
    pub fn with_milliseconds(milliseconds: i32) -> Time {
        Time {
            time: unsafe { ffi::sfMilliseconds(milliseconds as c_int) }
        }
    }

    /// Construct a time value from a number of microseconds
    pub fn with_microseconds(microseconds: i64) -> Time {
        Time {
            time: unsafe { ffi::sfMicroseconds(microseconds) }
        }
    }

    /// Return a time value as a number of seconds
    pub fn as_seconds(&self) -> f32 {
        unsafe {
            ffi::sfTime_asSeconds(self.time)
        }
    }

    /// Return a time value as a number of milliseconds
    pub fn as_milliseconds(&self) -> i32 {
        unsafe {
            ffi::sfTime_asMilliseconds(self.time)
        }
    }

    /// Return a time value as a number of microseconds
    pub fn as_microseconds(&self) -> i64 {
        unsafe {
            ffi::sfTime_asMicroseconds(self.time)
        }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.as_microseconds() == other.as_microseconds()
    }

    fn ne(&self, other: &Time) -> bool {
        self.as_microseconds() != other.as_microseconds()
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        self.as_microseconds().partial_cmp(&other.as_microseconds())
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
         Time::with_microseconds(self.as_microseconds() + other.as_microseconds())
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
         Time::with_microseconds(self.as_microseconds() - other.as_microseconds())
    }
}

impl Mul for Time {
    type Output = Time;

    fn mul(self, other: Time) -> Time {
         Time::with_microseconds(self.as_microseconds() * other.as_microseconds())
    }
}

impl Div for Time {
    type Output = Time;

    fn div(self, other: Time) -> Time {
         Time::with_microseconds(self.as_microseconds() / other.as_microseconds())
    }
}

impl Wrappable<ffi::sfTime> for Time {
    fn wrap(time: ffi::sfTime) -> Time {
        Time {
            time: time
        }
    }

    fn unwrap(&self) -> ffi::sfTime {
        self.time
    }
}
