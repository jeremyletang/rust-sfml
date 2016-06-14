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

//! Utility Class providing 3 dimensional vectors for f32.

use std::ops::{Add, Div, Mul, Sub};
use raw_conv::{FromRaw, Raw};

/// Vector3f definition
#[repr(C)]
#[derive(Clone, PartialOrd, PartialEq, Debug, Copy)]
pub struct Vector3<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
    /// Z coordinate of the vector.
    pub z: T,
}

impl<T> Vector3<T> {
    /// Create a new Vector3f with the given values.
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x: x, y: y, z: z }
    }
}

/// export Vector3<f32> as Vector3f
pub type Vector3f = Vector3<f32>;
/// export Vector3<i32> as Vector3i
pub type Vector3i = Vector3<i32>;
/// export Vector3<u32> as Vector3u
pub type Vector3u = Vector3<u32>;

impl<T: Add + Copy> Add<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn add(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<T: Sub + Copy> Sub<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn sub(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<T: Mul + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn mul(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Div + Copy> Div<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn div(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}


impl<T: Add> Add for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn add(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub> Sub for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Mul> Mul for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn mul(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Div> Div for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn div(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Raw for Vector3f {
    type Raw = ::csfml_system_sys::sfVector3f;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl FromRaw for Vector3f {
    fn from_raw(raw: Self::Raw) -> Self {
        Vector3f {
            x: raw.x,
            y: raw.y,
            z: raw.z,
        }
    }
}
