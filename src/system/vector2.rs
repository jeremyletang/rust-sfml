/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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

/*!
* Utility Class providing 2 dimensional vectors for int, uint, and float.
*
* Create your own by implementing the Trait Vector2
*
*/

pub use std::libc::{c_uint, c_int, c_float};

/**
* Implemention of Vector2i
*/
pub struct Vector2i {
    x : i32,
    y : i32
}

/**
* Implemention of Vector2u
*/
pub struct Vector2u {
    x : u32,
    y : u32
}

/**
* Implemention of Vector2f
*/
pub struct Vector2f {
    x : f32,
    y : f32
}

pub trait Vector2<T> {
    pub fn new(x : T, y : T) -> Self;
}

impl Vector2<i32> for Vector2i {
    pub fn new(x : i32, y: i32) -> Vector2i {
        Vector2i{x : x, y : y}
    }
}

impl Sub<Vector2i, Vector2i> for Vector2i {
    fn sub(&self, rhs : &Vector2i) -> Vector2i {
        Vector2i { x : self.x - rhs.x, y : self.y - rhs.y }
    }
}

impl Add<Vector2i, Vector2i> for Vector2i {
    fn add(&self, rhs : &Vector2i) -> Vector2i {
        Vector2i { x : self.x + rhs.x, y : self.y + rhs.y }
    }
}

impl Mul<Vector2i, Vector2i> for Vector2i {
    fn mul(&self, rhs : &Vector2i) -> Vector2i {
        Vector2i { x : self.x * rhs.x, y : self.y * rhs.y }
    }
}

impl Div<Vector2i, Vector2i> for Vector2i {
    fn div(&self, rhs : &Vector2i) -> Vector2i {
        Vector2i { x : self.x / rhs.x, y : self.y / rhs.y }
    }
}

impl Eq for Vector2i {
    fn eq(&self, rhs : &Vector2i) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
    fn ne(&self, rhs : &Vector2i) -> bool {
        self.x != rhs.x && self.y != rhs.y 
    }
}

impl Vector2<u32> for Vector2u {
    fn new(x : u32, y: u32) -> Vector2u {
        Vector2u{x : x, y : y}
    }
}

impl Sub<Vector2u, Vector2u> for Vector2u {
    fn sub(&self, rhs : &Vector2u) -> Vector2u {
        Vector2u { x : self.x - rhs.x, y : self.y - rhs.y }
    }
}

impl Add<Vector2u, Vector2u> for Vector2u {
    fn add(&self, rhs : &Vector2u) -> Vector2u {
        Vector2u { x : self.x + rhs.x, y : self.y + rhs.y }
    }
}

impl Mul<Vector2u, Vector2u> for Vector2u {
    fn mul(&self, rhs : &Vector2u) -> Vector2u {
        Vector2u { x : self.x * rhs.x, y : self.y * rhs.y }
    }
}

impl Div<Vector2u, Vector2u> for Vector2u {
    fn div(&self, rhs : &Vector2u) -> Vector2u {
        Vector2u { x : self.x / rhs.x, y : self.y / rhs.y }
    }
}

impl Eq for Vector2u {
    fn eq(&self, rhs : &Vector2u) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
    fn ne(&self, rhs : &Vector2u) -> bool {
        self.x != rhs.x && self.y != rhs.y 
    }
}

impl Vector2<f32> for Vector2f {
    fn new(xa : f32, ya: f32) -> Vector2f {
        Vector2f{x : xa, y : ya}
    }
}

impl Sub<Vector2f, Vector2f> for Vector2f {
    fn sub(&self, rhs : &Vector2f) -> Vector2f {
        Vector2f { x : self.x - rhs.x, y : self.y - rhs.y }
    }
}

impl Add<Vector2f, Vector2f> for Vector2f {
    fn add(&self, rhs : &Vector2f) -> Vector2f {
        Vector2f { x : self.x + rhs.x, y : self.y + rhs.y }
    }
}

impl Mul<Vector2f, Vector2f> for Vector2f {
    fn mul(&self, rhs : &Vector2f) -> Vector2f {
        Vector2f { x : self.x * rhs.x, y : self.y * rhs.y }
    }
}

impl Div<Vector2f, Vector2f> for Vector2f {
    fn div(&self, rhs : &Vector2f) -> Vector2f {
        Vector2f { x : self.x / rhs.x, y : self.y / rhs.y }
    }
}

impl Eq for Vector2f {
    fn eq(&self, rhs : &Vector2f) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
    fn ne(&self, rhs : &Vector2f) -> bool {
        self.x != rhs.x && self.y != rhs.y 
    }
}
