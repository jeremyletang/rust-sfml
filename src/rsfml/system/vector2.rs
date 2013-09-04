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

trait Vector2iOp {
    fn add_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i;
    fn div_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i;
    fn mul_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i;
    fn sub_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i;
}

trait Vector2fOp {
    fn add_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f;
    fn div_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f;
    fn mul_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f;
    fn sub_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f;
}

trait Vector2uOp {
    fn add_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u;
    fn div_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u;
    fn mul_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u;
    fn sub_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u;
}

impl Vector2i {
    pub fn new(x : i32, y: i32) -> Vector2i {
        Vector2i{
            x : x, 
            y : y
        }
    }
}

impl<R : Vector2iOp> Sub<R, Vector2i> for Vector2i {
    fn sub(&self, rhs : &R) -> Vector2i {
        rhs.sub_to_Vector2i(self)
    }
}

impl<R: Vector2iOp> Add<R, Vector2i> for Vector2i {
    fn add(&self, rhs: &R) -> Vector2i {
         rhs.add_to_Vector2i(self)
    }
}

impl<R: Vector2iOp> Mul<R, Vector2i> for Vector2i {
    fn mul(&self, rhs: &R) -> Vector2i {
         rhs.mul_to_Vector2i(self)
    }
}

impl<R: Vector2iOp> Div<R, Vector2i> for Vector2i {
    fn div(&self, rhs: &R) -> Vector2i {
         rhs.div_to_Vector2i(self)
    }
}

impl Vector2iOp for Vector2i {
    fn add_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x + self.x,
            y : lhs.y + self.y
        }
    }

    fn sub_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i { 
            x : lhs.x - self.x, 
            y : lhs.y - self.y 
        }
    }

    fn div_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x / self.x,
            y : lhs.y / self.y 
        }
    }

    fn mul_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x * self.x,
            y : lhs.y * self.y 
        }
    }
}
 
impl Vector2iOp for int {
    fn add_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x + *self as i32,
            y : lhs.y + *self as i32
        }
    }

    fn sub_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x - *self as i32,
            y : lhs.y - *self as i32
        }
    }

    fn mul_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x * (*self as i32),
            y : lhs.y * (*self as i32)
        }
    }

    fn div_to_Vector2i(&self, lhs: &Vector2i) -> Vector2i {
        Vector2i {
            x : lhs.x / (*self as i32),
            y : lhs.y / (*self as i32)
        }
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

impl Vector2u {
    pub fn new(x : u32, y: u32) -> Vector2u {
        Vector2u{
            x : x,
            y : y
        }
    }
}

impl<R : Vector2uOp> Sub<R, Vector2u> for Vector2u {
    fn sub(&self, rhs : &R) -> Vector2u {
        rhs.sub_to_Vector2u(self)
    }
}

impl<R: Vector2uOp> Add<R, Vector2u> for Vector2u {
    fn add(&self, rhs: &R) -> Vector2u {
         rhs.add_to_Vector2u(self)
    }
}

impl<R: Vector2uOp> Mul<R, Vector2u> for Vector2u {
    fn mul(&self, rhs: &R) -> Vector2u {
         rhs.mul_to_Vector2u(self)
    }
}

impl<R: Vector2uOp> Div<R, Vector2u> for Vector2u {
    fn div(&self, rhs: &R) -> Vector2u {
         rhs.div_to_Vector2u(self)
    }
}

impl Vector2uOp for Vector2u {
    fn add_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x + self.x,
            y : lhs.y + self.y
        }
    }

    fn sub_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x - self.x,
            y : lhs.y - self.y
        }
    }

    fn div_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x / self.x,
            y : lhs.y / self.y
        }
    }

    fn mul_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x * self.x,
            y : lhs.y * self.y 
        }
    }
}
 
impl Vector2uOp for uint {
    fn add_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x + *self as u32,
            y : lhs.y + *self as u32
        }
    }

    fn sub_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x - *self as u32,
            y : lhs.y - *self as u32
        }
    }

    fn mul_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u {
            x : lhs.x * (*self as u32),
            y : lhs.y * (*self as u32)
        }
    }

    fn div_to_Vector2u(&self, lhs: &Vector2u) -> Vector2u {
        Vector2u { 
            x : lhs.x / (*self as u32),
            y : lhs.y / (*self as u32)
        }
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

impl Vector2f {
    pub fn new(x : f32, y: f32) -> Vector2f {
        Vector2f{
            x : x,
            y : y
        }
    }
}

impl<R: Vector2fOp> Add<R, Vector2f> for Vector2f {
    fn add(&self, rhs: &R) -> Vector2f {
         rhs.add_to_Vector2f(self)
    }
}

impl<R: Vector2fOp> Sub<R, Vector2f> for Vector2f {
    fn sub(&self, rhs: &R) -> Vector2f {
         rhs.sub_to_Vector2f(self)
    }
}

impl<R: Vector2fOp> Mul<R, Vector2f> for Vector2f {
    fn mul(&self, rhs: &R) -> Vector2f {
         rhs.mul_to_Vector2f(self)
    }
}

impl<R: Vector2fOp> Div<R, Vector2f> for Vector2f {
    fn div(&self, rhs: &R) -> Vector2f {
         rhs.div_to_Vector2f(self)
    }
}

impl Vector2fOp for Vector2f {
    fn add_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f {
            x : lhs.x + self.x,
            y : lhs.y + self.y
        }
    }

    fn sub_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f {
            x : lhs.x - self.x,
            y : lhs.y - self.y
        }
    }

    fn div_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f {
            x : lhs.x / self.x,
            y : lhs.y / self.y
        }
    }

    fn mul_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f {
            x : lhs.x * self.x,
            y : lhs.y * self.y 
        }
    }
}
 
impl Vector2fOp for float {
    fn add_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f { 
            x : lhs.x + *self as f32,
            y : lhs.y + *self as f32
        }
    }

    fn sub_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f {
            x : lhs.x - *self as f32,
            y : lhs.y - *self as f32
        }
    }

    fn mul_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f { 
            x : lhs.x * (*self as f32),
            y : lhs.y * (*self as f32)
        }
    }

    fn div_to_Vector2f(&self, lhs: &Vector2f) -> Vector2f {
        Vector2f {
            x : lhs.x / (*self as f32),
            y : lhs.y / (*self as f32)
        }
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
