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

use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

use raw_conv::{Raw, FromRaw};

use csfml_system_sys::*;

/// Represents a time value.
///
/// Time encapsulates a time value in a flexible way.
///
/// It allows to define a time value either as a number of seconds, milliseconds or microseconds.
/// It also works the other way round: you can read a time value as either a number of seconds,
/// milliseconds or microseconds.
///
/// By using such a flexible interface, the API doesn't impose any fixed type or resolution for
/// time values, and let the user choose its own favorite representation.
///
/// Time values support the usual mathematical operations: you can add or subtract two times,
/// multiply or divide a time by a number, compare two times, etc.
///
/// Since they represent a time span and not an absolute time value, times can also be negative.
///
/// # Usage example
/// ```
/// # use sfml::system::Time;
/// let t1 = Time::seconds(0.1);
/// assert_eq!(t1.as_milliseconds(), 100);
///
/// let t2 = Time::milliseconds(30);
/// assert_eq!(t2.as_microseconds(), 30_000);
///
/// let t3 = Time::microseconds(-800_000);
/// assert_eq!(t3.as_seconds(), -0.8);
/// ```
///
/// # See also
/// - `Clock`
#[derive(Copy, Clone)]
pub struct Time(sfTime);

impl Time {
    /// Constructs a time value from a number of seconds.
    pub fn seconds(seconds: f32) -> Self {
        Time(unsafe { sfSeconds(seconds) })
    }

    /// Constructs a time value from a number of milliseconds.
    pub fn milliseconds(milliseconds: i32) -> Self {
        Time(unsafe { sfMilliseconds(milliseconds) })
    }

    /// Constructs a time value from a number of microseconds.
    pub fn microseconds(microseconds: i64) -> Self {
        Time(sfTime { microseconds: microseconds })
    }

    /// Returns the time value as a number of seconds.
    pub fn as_seconds(&self) -> f32 {
        unsafe { sfTime_asSeconds(self.0) }
    }

    /// Returns the time value as a number of milliseconds.
    pub fn as_milliseconds(&self) -> i32 {
        unsafe { sfTime_asMilliseconds(self.0) }
    }

    /// Returns the time value as a number of microseconds.
    pub fn as_microseconds(&self) -> i64 {
        unsafe { sfTime_asMicroseconds(self.0) }
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Time) -> bool {
        self.0.microseconds == other.0.microseconds
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        self.0.microseconds.partial_cmp(&other.0.microseconds)
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, other: Time) -> Time {
        Time::microseconds(self.0.microseconds + other.0.microseconds)
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, other: Time) -> Time {
        Time::microseconds(self.0.microseconds - other.0.microseconds)
    }
}

impl Mul for Time {
    type Output = Time;

    fn mul(self, other: Time) -> Time {
        Time::microseconds(self.0.microseconds * other.0.microseconds)
    }
}

impl Div for Time {
    type Output = Time;

    fn div(self, other: Time) -> Time {
        Time::microseconds(self.0.microseconds / other.0.microseconds)
    }
}

impl Raw for Time {
    type Raw = sfTime;
    fn raw(&self) -> Self::Raw {
        self.0
    }
}

impl FromRaw for Time {
    fn from_raw(raw: Self::Raw) -> Self {
        Time(raw)
    }
}

/// Predefined "zero" time value.
pub const ZERO: Time = Time(sfTime { microseconds: 0 });
