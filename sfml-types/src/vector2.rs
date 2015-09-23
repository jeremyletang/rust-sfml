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

//! Utility Class providing 2 dimensional vectors for i32, u32, and f32.

use std::ops::{Add, Sub, Mul, Div};

/// Implementation of Vector2i
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T
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
        Vector2 {
            x: x,
            y: y
        }
    }
}

impl<T: Add + Copy> Add<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl<T: Sub + Copy> Sub<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<T: Div + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}


impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: Mul> Mul for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl<T: Div> Div for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y
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
        self.clone()
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

impl ToVec for Vector2i {
    fn to_vector2f(&self) -> Vector2f {
        Vector2f {
            x: self.x as f32,
            y: self.y as f32
        }
    }

    fn to_vector2i(&self) -> Vector2i {
         self.clone()
    }

    fn to_vector2u(&self) -> Vector2u {
        Vector2u {
            x: self.x as u32,
            y: self.y as u32
        }
    }
}

impl ToVec for Vector2u {
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
        self.clone()
    }
}
