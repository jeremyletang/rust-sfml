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

#![allow(missing_copy_implementations)]

use std::ops::{Add, Sub, Mul, Div};

use traits::Wrappable;
use ffi::system::time as ffi;
use std::mem;

/// Represents a time duration in a flexible way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    microseconds: i64
}

impl Time {
    /// Construct a time value from a number of seconds
    pub fn with_seconds(seconds: f32) -> Time {
        Time {
            microseconds: (seconds * 1_000_000.) as i64
        }
    }

    /// Construct a time value from a number of milliseconds
    pub fn with_milliseconds(milliseconds: i32) -> Time {
        Time {
            microseconds: (milliseconds * 1_000) as i64
        }
    }

    /// Construct a time value from a number of microseconds
    pub fn with_microseconds(microseconds: i64) -> Time {
        Time {
            microseconds: microseconds
        }
    }

    /// Return a time value as a number of seconds
    pub fn as_seconds(&self) -> f32 {
        ((self.microseconds as f64) / 1_000_000.) as f32
    }

    /// Return a time value as a number of milliseconds
    pub fn as_milliseconds(&self) -> i32 {
        ((self.microseconds as f64) / 1_000.) as i32
    }

    /// Return a time value as a number of microseconds
    pub fn as_microseconds(&self) -> i64 {
        self.microseconds
    }
}

macro_rules! oper {
    ($t:ty, $f:ident, $p:path) => {
        impl $t for Time {
            type Output = Time;
            
            fn $f(self, other: Time) -> Time {
                Time { microseconds: $p ( self.microseconds, other.microseconds ) }
            }
        }
    }
}

oper!(Add, add, Add::add);
oper!(Sub, sub, Sub::sub);
oper!(Mul, mul, Mul::mul);
oper!(Div, div, Div::div);

#[doc(hidden)]
impl Wrappable<ffi::sfTime> for Time {
    fn wrap(time: ffi::sfTime) -> Time {
        unsafe { mem::transmute(time) }
    }

    fn unwrap(&self) -> ffi::sfTime {
        unsafe { mem::transmute(*self) }
    }
}
