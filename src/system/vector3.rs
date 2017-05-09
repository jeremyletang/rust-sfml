use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Utility type for manipulating 3-dimensional vectors.
///
/// `Vector3` is a simple type that defines a mathematical vector with
/// three coordinates (x, y and z).
///
/// It can be used to represent anything that has three dimensions:
/// a size, a point, a velocity, etc.
///
/// The type parameter T is the type of the coordinates.
///
/// You generally don't have to care about the generic form (`Vector3<T>`),
/// the most common specializations have special type aliases:
///
/// - `Vector3<f32>` is `Vector3f`
/// - `Vector3<i32>` is `Vector3i`
///
/// The `Vector3` type has a small and simple interface, its x and y members can be
/// accessed directly (there are no accessors like `set_x()`, `get_x()`) and it contains no
/// mathematical function like dot product, cross product, length, etc.
///
/// # Usage example
/// ```
/// # use sfml::system::Vector3f;
/// let mut v1 = Vector3f::new(16.5, 24.0, -8.2);
/// v1.x = 18.2;
/// let y = v1.y;
/// let z = v1.z;
///
/// let v2 = v1 * 5.0;
/// let v3 = v1 + v2;
///
/// assert_ne!(v2, v3);
/// ```
///
/// Note: for 2-dimensional vectors, see `Vector2`.
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

impl<T: AddAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.x += rhs.y;
        self.x += rhs.z;
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

impl<T: SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.x -= rhs.y;
        self.x -= rhs.z;
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

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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

impl<T: DivAssign + Copy> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> From<(T, T, T)> for Vector3<T> {
    /// Constructs a `Vector3` from `(x, y, z)`.
    fn from(src: (T, T, T)) -> Self {
        Self {
            x: src.0,
            y: src.1,
            z: src.2,
        }
    }
}

impl Vector3f {
    pub fn raw(&self) -> ::csfml_system_sys::sfVector3f {
        ::csfml_system_sys::sfVector3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
    pub unsafe fn from_raw(raw: ::csfml_system_sys::sfVector3f) -> Self {
        Self {
            x: raw.x,
            y: raw.y,
            z: raw.z,
        }
    }
}
