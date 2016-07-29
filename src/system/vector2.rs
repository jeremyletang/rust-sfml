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

//! Utility Class providing 2 dimensional vectors for i32, u32, and f32.

use std::ops::{Add, Div, Mul, Sub};
use raw_conv::{FromRaw, Raw};

/// Implementation of Vector2i
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
}

/// export Vector2<i32> as Vector2i
pub type Vector2i = Vector2<i32>;
/// export Vector2<u32> as Vector2u
pub type Vector2u = Vector2<u32>;
/// export Vector2<f32> as Vector2f
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {
    /// Build a new Vector2<T>
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x: x, y: y }
    }
}

macro_rules! impl_ops {
    ( $_trait:ident, $_func:ident, $( $_type:ty ),+ ) => {
        impl<T: $_trait + Copy> $_trait<T> for Vector2<T> {
            type Output = Vector2<T::Output>;

            fn $_func(self, rhs: T) -> Vector2<T::Output> {
                Vector2 {
                    x: $_trait::$_func(self.x, rhs),
                    y: $_trait::$_func(self.y, rhs)
                }
            }
        }

        $(
            impl $_trait<Vector2<$_type>> for $_type {
                type Output = Vector2<$_type>;

                fn $_func(self, rhs: Vector2<$_type>) -> Vector2<$_type> {
                    Vector2 {
                        x: $_trait::$_func(self, rhs.x),
                        y: $_trait::$_func(self, rhs.y)
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

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul> Mul for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div> Div for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}


/// Utility trait to convert a Vector2 on another type
pub trait ToVec {
    /// Convert the current Vector2 to a Vector2f
    fn to_vector2f(&self) -> Vector2f;
    /// Convert the current Vector2 to a Vector2i
    fn to_vector2i(&self) -> Vector2i;
    /// Convert the current Vector2f to a Vector2u
    fn to_vector2u(&self) -> Vector2u;
}

impl ToVec for Vector2f {
    fn to_vector2f(&self) -> Vector2f {
        *self
    }

    fn to_vector2i(&self) -> Vector2i {
        Vector2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    fn to_vector2u(&self) -> Vector2u {
        Vector2u {
            x: self.x as u32,
            y: self.y as u32,
        }
    }
}

impl ToVec for Vector2i {
    fn to_vector2f(&self) -> Vector2f {
        Vector2f {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    fn to_vector2i(&self) -> Vector2i {
        *self
    }

    fn to_vector2u(&self) -> Vector2u {
        Vector2u {
            x: self.x as u32,
            y: self.y as u32,
        }
    }
}

impl ToVec for Vector2u {
    fn to_vector2f(&self) -> Vector2f {
        Vector2f {
            x: self.x as f32,
            y: self.y as f32,
        }
    }

    fn to_vector2i(&self) -> Vector2i {
        Vector2i {
            x: self.x as i32,
            y: self.y as i32,
        }
    }

    fn to_vector2u(&self) -> Vector2u {
        *self
    }
}

impl Raw for Vector2i {
    type Raw = ::csfml_system_sys::sfVector2i;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector2i {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromRaw for Vector2i {
    fn from_raw(raw: Self::Raw) -> Self {
        Vector2i {
            x: raw.x,
            y: raw.y,
        }
    }
}

impl Raw for Vector2u {
    type Raw = ::csfml_system_sys::sfVector2u;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector2u {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromRaw for Vector2u {
    fn from_raw(raw: Self::Raw) -> Self {
        Vector2u {
            x: raw.x,
            y: raw.y,
        }
    }
}

impl Raw for Vector2f {
    type Raw = ::csfml_system_sys::sfVector2f;

    fn raw(&self) -> Self::Raw {
        ::csfml_system_sys::sfVector2f {
            x: self.x,
            y: self.y,
        }
    }
}

impl FromRaw for Vector2f {
    fn from_raw(raw: Self::Raw) -> Self {
        Vector2f {
            x: raw.x,
            y: raw.y,
        }
    }
}
