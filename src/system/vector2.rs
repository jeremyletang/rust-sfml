use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Utility type for manipulating 2-dimensional vectors.
///
/// `Vector2` is a simple type that defines
/// a mathematical vector with two coordinates (x and y).
///
/// It can be used to represent anything that has two dimensions: a size, a point, a velocity, etc.
///
/// The type parameter T is the type of the coordinates.
///
/// You generally don't have to care about the generic form (`Vector2<T>`), the most common
/// specializations have special type aliases:
///
/// - `Vector2<f32>` is [`Vector2f`]
/// - `Vector2<i32>` is [`Vector2i`]
/// - `Vector2<u32>` is [`Vector2u`]
/// 
/// The `Vector2` type has a small and simple interface, its x and y members can be
/// accessed directly (there are no accessors like `set_x()`, `get_x()`) and it contains no
/// mathematical function like dot product, cross product, length, etc.
///
/// # Usage example
///
/// ```
/// # use sfml::system::Vector2f;
/// let mut v1 = Vector2f::new(16.5, 24.0);
/// v1.x = 18.2;
/// let y = v1.y;
///
/// let v2 = v1 * 5.0;
/// let v3 = v1 + v2;
/// assert_ne!(v2, v3);
/// ```
///
/// Note: for 3-dimensional vectors, see [`Vector3`].
///
/// [`Vector3`]: crate::system::Vector3
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Default)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
}

/// [`Vector2`] with `i32` coordinates.
pub type Vector2i = Vector2<i32>;
/// [`Vector2`] with `u32` coordinates.
pub type Vector2u = Vector2<u32>;
/// [`Vector2`] with `f32` coordinates.
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {
    /// Creates a new vector from its coordinates.
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    /// Constructs a `Vector2` from `(x, y)`.
    fn from(src: (T, T)) -> Self {
        Self { x: src.0, y: src.1 }
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

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
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

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(feature = "window")]
impl Vector2i {
    pub(crate) fn raw(self) -> crate::ffi::sfVector2i {
        crate::ffi::sfVector2i {
            x: self.x,
            y: self.y,
        }
    }
    pub(crate) fn from_raw(raw: crate::ffi::sfVector2i) -> Self {
        Self { x: raw.x, y: raw.y }
    }

    /// Creates a Vector2i from a Vector2f. Conversion issues may arise from integer overflow and
    /// floating point truncation.
    pub fn from_vector2f(vec: Vector2f) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }

    /// Creates a Vector2i from a Vector2f. Conversion issues may arise from integer overflow.
    pub fn from_vector2u(vec: Vector2u) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }
}

#[cfg(feature = "window")]
impl Vector2u {
    pub(crate) fn raw(self) -> crate::ffi::sfVector2u {
        crate::ffi::sfVector2u {
            x: self.x,
            y: self.y,
        }
    }
    pub(crate) fn from_raw(raw: crate::ffi::sfVector2u) -> Self {
        Self { x: raw.x, y: raw.y }
    }

    /// Creates a Vector2u from a Vector2f. Conversion issues may arise from negative numbers and
    /// floating point truncation.
    pub fn from_vector2f(vec: Vector2f) -> Self {
        Self {
            x: vec.x as u32,
            y: vec.y as u32,
        }
    }

    /// Creates a Vector2u from a Vector2i. Conversion issues may arise from negative integers.
    pub fn from_vector2i(vec: Vector2i) -> Self {
        Self {
            x: vec.x as u32,
            y: vec.y as u32,
        }
    }
}

#[cfg(feature = "graphics")]
impl Vector2f {
    pub(crate) fn raw(self) -> crate::ffi::sfVector2f {
        crate::ffi::sfVector2f {
            x: self.x,
            y: self.y,
        }
    }
    pub(crate) fn from_raw(raw: crate::ffi::sfVector2f) -> Self {
        Self { x: raw.x, y: raw.y }
    }

    /// Converts Vector2i to Vector2f.
    pub fn from_vector2i(vec: Vector2i) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }

    /// Converts Vector2u to Vector2f.
    pub fn from_vector2u(vec: Vector2u) -> Self {
        Self {
            x: vec.x as f32,
            y: vec.y as f32,
        }
    }
}

// We just want to ensure non-crashing behavior
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_vector2u() {
        let vu = Vector2u::new(10, 10);
        let _vi = Vector2i::from_vector2u(vu);
        let _vf = Vector2f::from_vector2u(vu);

        let ovu = Vector2u::new(u32::MAX, u32::MIN);
        let _ovi = Vector2i::from_vector2u(ovu);
        let _ovf = Vector2f::from_vector2u(ovu);
    }

    #[test]
    fn from_vector2i() {
        let vi = Vector2i::new(10, 10);
        let _vu = Vector2u::from_vector2i(vi);
        let _vf = Vector2f::from_vector2i(vi);

        let ovi = Vector2i::new(i32::MAX, i32::MIN);
        let _vu = Vector2u::from_vector2i(ovi);
        let _vf = Vector2f::from_vector2i(ovi);
    }

    #[test]
    fn from_vector2f() {
        let vf = Vector2f::new(10., 10.);
        let _vu = Vector2u::from_vector2f(vf);
        let _vi = Vector2i::from_vector2f(vf);

        let ovf = Vector2f::new(f32::MAX, f32::MIN);
        let _vu = Vector2u::from_vector2f(ovf);
        let _vi = Vector2i::from_vector2f(ovf);

        let fovf = Vector2f::new(-1.00303030303030, 1.08080808080808);
        let _fovu = Vector2u::from_vector2f(fovf);
        let _fovi = Vector2i::from_vector2f(fovf);
    }
}
