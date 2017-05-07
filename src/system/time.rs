use csfml_system_sys::*;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use system::raw_conv::{FromRaw, Raw};

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
#[derive(Copy, Clone, Debug)]
pub struct Time(sfTime);

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.0.microseconds == other.0.microseconds
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
        self.0.microseconds.partial_cmp(&other.0.microseconds)
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> ::std::cmp::Ordering {
        self.0.microseconds.cmp(&other.0.microseconds)
    }
}

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

impl Neg for Time {
    type Output = Self;
    fn neg(self) -> Self {
        Time(sfTime { microseconds: -self.0.microseconds })
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Time::microseconds(self.0.microseconds + rhs.0.microseconds)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0.microseconds += rhs.0.microseconds;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Time::microseconds(self.0.microseconds - rhs.0.microseconds)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.microseconds -= rhs.0.microseconds;
    }
}

impl Mul<f32> for Time {
    type Output = Self;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: f32) -> Self {
        Time::seconds(self.as_seconds() * rhs)
    }
}

impl Mul<i64> for Time {
    type Output = Self;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: i64) -> Self {
        Time::microseconds(self.as_microseconds() * rhs)
    }
}

impl Mul<Time> for f32 {
    type Output = Time;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: Time) -> Time {
        rhs * self
    }
}

impl Mul<Time> for i64 {
    type Output = Time;

    /// Overload of binary * operator to scale a time value.
    fn mul(self, rhs: Time) -> Time {
        rhs * self
    }
}

impl MulAssign<f32> for Time {
    /// Overload of binary *= operator to scale/assign a time value.
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl MulAssign<i64> for Time {
    /// Overload of binary *= operator to scale/assign a time value.
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl Div<f32> for Time {
    type Output = Self;

    /// Overload of binary / operator to scale a time value.
    fn div(self, rhs: f32) -> Self {
        Time::seconds(self.as_seconds() / rhs)
    }
}

impl Div<i64> for Time {
    type Output = Self;

    /// Overload of binary / operator to scale a time value.
    fn div(self, rhs: i64) -> Self {
        Time::microseconds(self.as_microseconds() / rhs)
    }
}

impl Div for Time {
    type Output = f32;

    /// Overload of binary / operator to compute the ratio of two time values.
    fn div(self, rhs: Self) -> f32 {
        self.as_seconds() / rhs.as_seconds()
    }
}

impl DivAssign<f32> for Time {
    /// Overload of binary /= operator to scale/assign a time value.
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

impl DivAssign<i64> for Time {
    /// Overload of binary /= operator to scale/assign a time value.
    fn div_assign(&mut self, rhs: i64) {
        *self = *self / rhs;
    }
}

impl Rem for Time {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Time::microseconds(self.0.microseconds % rhs.0.microseconds)
    }
}

impl RemAssign for Time {
    fn rem_assign(&mut self, rhs: Self) {
        self.0.microseconds %= rhs.0.microseconds
    }
}

impl Raw for Time {
    type Raw = sfTime;
    fn raw(&self) -> Self::Raw {
        self.0
    }
}

impl FromRaw for Time {
    type RawFrom = sfTime;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        Time(raw)
    }
}

/// Predefined "zero" time value.
pub const ZERO: Time = Time(sfTime { microseconds: 0 });
