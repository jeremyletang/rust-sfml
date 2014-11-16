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

//! Utility Class providing 3 dimensional vectors for f32.

/// Vector3f definition
#[repr(C)]
#[deriving(Clone, PartialOrd, PartialEq, Show)]
pub struct Vector3f {
    /// X coordinate of the vector.
    pub x: f32,
    /// Y coordinate of the vector.
    pub y: f32,
    /// Z coordinate of the vector.
    pub z: f32
}

impl Vector3f {
    /// Create a new Vector3f with the given values.
    pub fn new(x: f32, y: f32, z: f32) -> Vector3f {
        Vector3f{
            x: x,
            y: y,
            z: z
        }
    }
}

impl Add<f32, Vector3f> for Vector3f {
    fn add(&self, rhs: &f32) -> Vector3f {
        Vector3f {
            x: self.x + *rhs,
            y: self.y + *rhs,
            z: self.z + *rhs
        }
    }
}

impl Sub<f32, Vector3f> for Vector3f {
    fn sub(&self, rhs: &f32) -> Vector3f {
        Vector3f {
            x: self.x - *rhs,
            y: self.y - *rhs,
            z: self.z - *rhs
        }
    }
}

impl Mul<f32, Vector3f> for Vector3f {
    fn mul(&self, rhs: &f32) -> Vector3f {
        Vector3f {
            x: self.x * *rhs,
            y: self.y * *rhs,
            z: self.z * *rhs
        }
    }
}

impl Div<f32, Vector3f> for Vector3f {
    fn div(&self, rhs: &f32) -> Vector3f {
        Vector3f {
            x: self.x / *rhs,
            y: self.y / *rhs,
            z: self.z / *rhs
        }
    }
}


impl Add<Vector3f, Vector3f> for Vector3f {
    fn add(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub<Vector3f, Vector3f> for Vector3f {
    fn sub(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl Mul<Vector3f, Vector3f> for Vector3f {
    fn mul(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl Div<Vector3f, Vector3f> for Vector3f {
    fn div(&self, rhs: &Vector3f) -> Vector3f {
        Vector3f {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}
