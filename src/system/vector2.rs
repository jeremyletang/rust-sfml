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

/// Generic two-dimensional vector.
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T
}

/// Two-dimensional integer vector.
pub type Vector2i = Vector2<i32>;
/// Two-dimensional unsigned integer vector.
pub type Vector2u = Vector2<u32>;
/// Two-dimensional float vector.
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {
    /// Create a new `Vector2` with the given values.
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 {
            x: x,
            y: y
        }
    }
}

impl<T: Add<Output=T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Sub<Output=T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, other: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: Copy + Mul<Output=T>> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Vector2<T> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Copy + Div<Output=T>> Div<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn div(self, rhs: T) -> Vector2<T> {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

/// Utility trait for converting between different `Vector2`s.
pub trait ToVec {
    /// Convert the current Vector2 to a `Vector2f`.
    fn to_vector2f(&self) -> Vector2f;
    /// Convert the current Vector2 to a `Vector2i`.
    fn to_vector2i(&self) -> Vector2i;
    /// Convert the current Vector2f to a `Vector2u`.
    fn to_vector2u(&self) -> Vector2u;
}

macro_rules! to_vec {
    (vec; $vec:ty) => (
        impl ToVec for $vec {
            fn to_vector2f(&self) -> Vector2f {
                Vector2f {
                    x: self.x as f32,
                    y: self.y as f32
                }
            }

            fn to_vector2i(&self) -> Vector2i {
                Vector2i {
                    x: self.x as i32,
                    y: self.y as i32
                }
            }

            fn to_vector2u(&self) -> Vector2u {
                Vector2u {
                    x: self.x as u32,
                    y: self.y as u32
                }
            }
        }
    );
    (tup; $elem:ty) => (
        impl ToVec for ($elem, $elem) {
            fn to_vector2f(&self) -> Vector2f {
                Vector2f {
                    x: self.0 as f32,
                    y: self.1 as f32
                }
            }

            fn to_vector2i(&self) -> Vector2i {
                Vector2i {
                    x: self.0 as i32,
                    y: self.1 as i32
                }
            }

            fn to_vector2u(&self) -> Vector2u {
                Vector2u {
                    x: self.0 as u32,
                    y: self.1 as u32
                }
            }
        }
    )
}

to_vec!(vec; Vector2f);
to_vec!(vec; Vector2i);
to_vec!(vec; Vector2u);
to_vec!(tup; f32);
to_vec!(tup; i32);
to_vec!(tup; u32);
