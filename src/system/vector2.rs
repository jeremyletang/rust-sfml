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
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Show, Copy)]
pub struct Vector2i {
    /// X coordinate of the vector.
    pub x: i32,
    /// Y coordinate of the vector.
    pub y: i32
}

/// Implementation of Vector2u
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Show, Copy)]
pub struct Vector2u {
    /// X coordinate of the vector.
    pub x: u32,
    /// Y coordinate of the vector.
    pub y: u32
}

/// Implementation of Vector2f
#[repr(C)]
#[derive(Clone, PartialOrd, PartialEq, Show, Copy)]
pub struct Vector2f {
    /// X coordinate of the vector.
    pub x: f32,
    /// Y coordinate of the vector.
    pub y: f32
}

impl Vector2i {
    /// Create a new Vector2i with the given values.
    pub fn new(x: i32, y: i32) -> Vector2i {
        Vector2i{
            x: x,
            y: y
        }
    }
}

impl Add<i32, Vector2i> for Vector2i {
    fn add(self, rhs: i32) -> Vector2i {
        Vector2i {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Sub<i32, Vector2i> for Vector2i {
    fn sub(self, rhs: i32) -> Vector2i {
        Vector2i {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl Mul<i32, Vector2i> for Vector2i {
    fn mul(self, rhs: i32) -> Vector2i {
        Vector2i {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<i32, Vector2i> for Vector2i {
    fn div(self, rhs: i32) -> Vector2i {
        Vector2i {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}


impl Add<Vector2i, Vector2i> for Vector2i {
    fn add(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Vector2i, Vector2i> for Vector2i {
    fn sub(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul<Vector2i, Vector2i> for Vector2i {
    fn mul(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Div<Vector2i, Vector2i> for Vector2i {
    fn div(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Vector2u {
    /// Create a new Vector2u with the given values.
    pub fn new(x: u32, y: u32) -> Vector2u {
        Vector2u{
            x: x,
            y: y
        }
    }
}

impl Add<u32, Vector2u> for Vector2u {
    fn add(self, rhs: u32) -> Vector2u {
        Vector2u {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Sub<u32, Vector2u> for Vector2u {
    fn sub(self, rhs: u32) -> Vector2u {
        Vector2u {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl Mul<u32, Vector2u> for Vector2u {
    fn mul(self, rhs: u32) -> Vector2u {
        Vector2u {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<u32, Vector2u> for Vector2u {
    fn div(self, rhs: u32) -> Vector2u {
        Vector2u {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}


impl Add<Vector2u, Vector2u> for Vector2u {
    fn add(self, rhs: Vector2u) -> Vector2u {
        Vector2u {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Vector2u, Vector2u> for Vector2u {
    fn sub(self, rhs: Vector2u) -> Vector2u {
        Vector2u {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul<Vector2u, Vector2u> for Vector2u {
    fn mul(self, rhs: Vector2u) -> Vector2u {
        Vector2u {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Div<Vector2u, Vector2u> for Vector2u {
    fn div(self, rhs: Vector2u) -> Vector2u {
        Vector2u {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }
}

impl Vector2f {
    /// Create a new Vector2f with the given values.
    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f{
            x: x,
            y: y
        }
    }
}

impl Add<f32, Vector2f> for Vector2f {
    fn add(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Sub<f32, Vector2f> for Vector2f {
    fn sub(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl Mul<f32, Vector2f> for Vector2f {
    fn mul(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<f32, Vector2f> for Vector2f {
    fn div(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}


impl Add<Vector2f, Vector2f> for Vector2f {
    fn add(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Sub<Vector2f, Vector2f> for Vector2f {
    fn sub(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul<Vector2f, Vector2f> for Vector2f {
    fn mul(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Div<Vector2f, Vector2f> for Vector2f {
    fn div(self, rhs: Vector2f) -> Vector2f {
        Vector2f {
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
