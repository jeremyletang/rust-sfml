/*!
* Utility Class providing 3 dimensional vectors for float.
* 
* Create your own by implementing the Trait Vector3
*
*/

pub use core::libc::{c_float};
#[doc(hidden)]
pub mod csfml {

    pub use core::libc::{c_float};

    /*
    * C implemention of Vector3f
    */
    pub struct sfVector3f {
        x : c_float,
        y : c_float,
        z : c_float
    }
}


pub struct Vector3f {
    x : float,
    y : float,
    z : float
}

trait Vector3<T> {
    fn new(x : T, y : T, z : T) -> Self;
}

impl Vector3<float> for Vector3f {
    fn new(x : float, y : float, z : float) -> Vector3f {
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

/**
* Function to convert C sfVector3f to Vector3f
*/
#[doc(hidden)]
pub fn wrap_vector3f(vec : csfml::sfVector3f) -> Vector3f {
    Vector3f {x : vec.x as float, y : vec.y as float, z : vec.z as float}
}

/**
* Function to convert Vector3f to C sfVector3f
*/
#[doc(hidden)]
pub fn unwrap_vector3f(vec : Vector3f) -> csfml::sfVector3f {
        csfml::sfVector3f {x : vec.x as c_float, y : vec.y as c_float, z : vec.z as c_float}
}
