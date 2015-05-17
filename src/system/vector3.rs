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

use std::ops::{Add, Sub, Mul, Div};

/// Generic three-dimensional vector.
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy)]
pub struct Vector3<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
    /// Z coordinate of the vector.
    pub z: T
}

/// Three-dimensional float vector.
pub type Vector3f = Vector3<f32>;

impl<T> Vector3<T> {
    /// Create a new `Vector3` with the given values.
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 {
            x: x,
            y: y,
            z: z
        }
    }
}

impl<T: Add> Add for Vector3<T> {
    type Output = Vector3<T::Output>;

	/// Component-wise addition.
    fn add(self, other: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
			z: self.z + other.z
        }
    }
}

impl<T: Sub> Sub for Vector3<T> {
    type Output = Vector3<T::Output>;

	/// Component-wise subtraction.
    fn sub(self, other: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
			z: self.z - other.z
        }
    }
}

impl<T: Copy + Mul> Mul<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

	/// Scalar multiplication.
    fn mul(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
			z: self.z * rhs
        }
    }
}

impl<T: Mul> Mul for Vector3<T> {
    type Output = Vector3<T::Output>;

	/// Component-wise multiplication.
    fn mul(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
			z: self.z * rhs.z
        }
    }
}

impl<T: Copy + Div> Div<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

	/// Scalar division.
    fn div(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
			z: self.z / rhs
        }
    }
}

impl<T: Div> Div for Vector3<T> {
    type Output = Vector3<T::Output>;

	/// Component-wise division.
    fn div(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
			z: self.z / rhs.z
        }
    }
}

