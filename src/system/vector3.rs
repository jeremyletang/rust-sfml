/*!
* Utility Class providing 3 dimensional vectors for float.
* 
* Create your own by implementing the Trait Vector3
*
*/

pub use core::libc::{c_float};

pub struct Vector3f {
    x : f32,
    y : f32,
    z : f32
}

trait Vector3<T> {
    fn new(x : T, y : T, z : T) -> Self;
}

impl Vector3<f32> for Vector3f {
    fn new(x : f32, y : f32, z : f32) -> Vector3f {
        Vector3f{x : x, y : y, z : z}
    }
}

impl Sub<Vector3f, Vector3f> for Vector3f {
    fn sub(&self, rhs : &Vector3f) -> Vector3f {
        Vector3f { x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z}
    }
}

impl Add<Vector3f, Vector3f> for Vector3f {
    fn add(&self, rhs : &Vector3f) -> Vector3f {
        Vector3f { x : self.x + rhs.x, y : self.y + rhs.y, z : self.z + rhs.z}
    }
}

impl Mul<Vector3f, Vector3f> for Vector3f {
    fn mul(&self, rhs : &Vector3f) -> Vector3f {
        Vector3f { x : self.x * rhs.x, y : self.y * rhs.y, z : self.z * rhs.z}
    }
}

impl Div<Vector3f, Vector3f> for Vector3f {
    fn div(&self, rhs : &Vector3f) -> Vector3f {
        Vector3f { x : self.x / rhs.x, y : self.y / rhs.y, z : self.z / rhs.z}
    }
}

impl Eq for Vector3f {
    fn eq(&self, rhs : &Vector3f) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
    fn ne(&self, rhs : &Vector3f) -> bool {
        self.x != rhs.x && self.y != rhs.y && self.z != rhs.z
    }
}
