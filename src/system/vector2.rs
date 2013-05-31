/*!
* Utility Class providing 2 dimensional vectors for int, uint, and float.
*
* Create your own by implementing the Trait Vector2
*
*/

pub use core::libc::{c_uint, c_int, c_float};

#[doc(hidden)]
pub mod csfml {
    pub use core::libc::{c_uint, c_int, c_float};
    
    /**
    * C implemention of Vector2i
    */
    pub struct sfVector2i {
        x : c_int,
        y : c_int
    }
    
    /**
    * C implemention of Vector2u
    */
    pub struct sfVector2u {
        x : c_uint,
        y : c_uint
    }

    /**
    * C implemention of Vector2f
    */
    pub struct sfVector2f {
        x : c_float, 
        y : c_float
    }
}

/**
* Implemention of Vector2i
*/
pub struct Vector2i {
    x : int,
    y : int
}

/**
* Implemention of Vector2u
*/
pub struct Vector2u {
    x : uint,
    y : uint
}

/**
* Implemention of Vector2f
*/
pub struct Vector2f {
    x : float,
    y : float
}

pub trait Vector2<T> {
    pub fn new(x : T, y : T) -> Self;
}

impl Vector2<int> for Vector2i {
    pub fn new(x : int, y: int) -> Vector2i {
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

impl Vector2<uint> for Vector2u {
    fn new(x : uint, y: uint) -> Vector2u {
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

impl Vector2<float> for Vector2f {
    fn new(xa : float, ya: float) -> Vector2f {
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

/**
* Function to convert C sfVector2i to Vector2i
*/
#[doc(hidden)]
pub fn wrap_vector2i(vec : csfml::sfVector2i) -> Vector2i {
    Vector2i {x : vec.x as int, y : vec.y as int}
}

/**
* Function to convert C sfVector2u to Vector2u
*/
#[doc(hidden)]
pub fn wrap_vector2u(vec : csfml::sfVector2u) -> Vector2u {
    Vector2u {x : vec.x as uint, y : vec.y as uint}
}

/**
* Function to convert C sfVector2f to Vector2f
*/
#[doc(hidden)]
pub fn wrap_vector2f(vec : csfml::sfVector2f) -> Vector2f {
    Vector2f {x : vec.x as float, y : vec.y as float}
}

/**
* Function to convert Vector2i to C sfVector2i
*/
#[doc(hidden)]
pub fn unwrap_vector2i(vec : Vector2i) -> csfml::sfVector2i {
    csfml::sfVector2i {x : vec.x as c_int, y : vec.y as c_int}
}

/**
* Function to convert Vector2i to C sfVector2i
*/
#[doc(hidden)]
pub fn unwrap_vector2u(vec : Vector2u) -> csfml::sfVector2u {
    csfml::sfVector2u {x : vec.x as c_uint, y : vec.y as c_uint}
}

/**
* Function to convert Vector2i to C sfVector2i
*/
#[doc(hidden)]
pub fn unwrap_vector2f(vec : Vector2f) -> csfml::sfVector2f {
    csfml::sfVector2f {x : vec.x as c_float, y : vec.y as c_float}
}
