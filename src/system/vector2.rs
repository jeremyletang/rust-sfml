use num_traits::{AsPrimitive, CheckedDiv};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

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
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    /// Lossless conversion into `Vector2<U>`.
    pub fn into_other<U>(self) -> Vector2<U>
    where
        T: Into<U>,
    {
        Vector2 {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
    /// Fallible conversion into `Vector2<U>`
    pub fn try_into_other<U>(self) -> Result<Vector2<U>, T::Error>
    where
        T: TryInto<U>,
    {
        Ok(Vector2 {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
    }
    /// Lossy conversion into `Vector2<U>`
    pub fn as_other<U: 'static + Copy>(self) -> Vector2<U>
    where
        T: AsPrimitive<U>,
    {
        Vector2 {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector2<T> {
    /// Dot product of two 2D vectors.
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
    /// Square of vector's length.
    pub fn length_sq(self) -> T {
        self.dot(self)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vector2<T> {
    /// Z component of the cross product of two 2D vectors.
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T: Mul<Output = T>> Vector2<T> {
    /// Component-wise multiplication of self and rhs.
    pub fn cwise_mul(self, rhs: Self) -> Vector2<T> {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T>> Vector2<T> {
    /// Component-wise division of self and rhs. Panics on divide by zero
    pub fn cwise_div(self, rhs: Self) -> Vector2<T> {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Div<Output = T> + CheckedDiv> Vector2<T> {
    /// Component-wise checked division of self and rhs. Returns None on divide by zero
    pub fn cwise_checked_div(self, rhs: Self) -> Option<Vector2<T>> {
        let x = self.x.checked_div(&rhs.x)?;
        let y = self.y.checked_div(&rhs.y)?;
        Some(Vector2 { x, y })
    }
}

impl<T: Neg<Output = T>> Vector2<T> {
    /// Returns a perpendicular vector rotated +90 degrees
    pub fn perpendicular(self) -> Vector2<T> {
        Vector2::new(-self.y, self.x)
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    /// Constructs a `Vector2` from `(x, y)`.
    fn from(src: (T, T)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}

impl<T: Add> Add<Vector2<T>> for Vector2<T> {
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

impl<T: Sub> Sub<Vector2<T>> for Vector2<T> {
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

impl<T: Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn div(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: CheckedDiv> Vector2<T> {
    /// checked_div for scalar division
    pub fn checked_div(self, rhs: T) -> Option<Vector2<T>> {
        let x = self.x.checked_div(&rhs)?;
        let y = self.y.checked_div(&rhs)?;
        Some(Vector2 { x, y })
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
}
