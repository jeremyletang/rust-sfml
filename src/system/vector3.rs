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
use raw_conv::{Raw, FromRaw};

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

/// `Vector3` with `f32` coordinates.
pub type Vector3f = Vector3<f32>;
/// `Vector3` with `i32` coordinates.
pub type Vector3i = Vector3<i32>;

macro_rules! impl_ops {
    ( $_trait:ident, $_func:ident, $( $_type:ty ),+ ) => {
        impl<T: $_trait + Copy> $_trait<T> for Vector3<T> {
            type Output = Vector3<T::Output>;

            fn $_func(self, rhs: T) -> Vector3<T::Output> {
                Vector3 {
                    x: $_trait::$_func(self.x, rhs),
                    y: $_trait::$_func(self.y, rhs),
                    z: $_trait::$_func(self.z, rhs)
                }
            }
        }

        $(
            impl $_trait<Vector3<$_type>> for $_type {
                type Output = Vector3<$_type>;

                fn $_func(self, rhs: Vector3<$_type>) -> Vector3<$_type> {
                    Vector3 {
                        x: $_trait::$_func(self, rhs.x),
                        y: $_trait::$_func(self, rhs.y),
                        z: $_trait::$_func(self, rhs.z)
                    }
                }
            }
        )+
    }
}

impl_ops!(Add, add, i32, u32, f32);
impl_ops!(Sub, sub, i32, u32, f32);
impl_ops!(Mul, mul, i32, u32, f32);
impl_ops!(Div, div, i32, u32, f32);


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
