#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num_traits::{AsPrimitive, CheckedDiv};

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
/// - `Vector3<f32>` is [`Vector3f`]
/// - `Vector3<i32>` is [`Vector3i`]
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
/// Note: for 2-dimensional vectors, see [`Vector2`].
///
/// [`Vector2`]: crate::system::Vector2
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vector3<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
    /// Z coordinate of the vector.
    pub z: T,
}

impl<T> Vector3<T> {
    /// Create a new `Vector3` with the given values.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector3;
    /// let v: Vector3<isize> = Vector3::new(1, 2, 3);
    /// ```
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    /// Lossless conversion into `Vector3<U>`.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector3;
    /// let vu: Vector3<u16> = Vector3::new(1, 2, 3);
    /// let vi: Vector3<i32> = vu.into_other();
    /// assert_eq!(i32::from(vu.x), vi.x);
    /// assert_eq!(i32::from(vu.y), vi.y);
    /// assert_eq!(i32::from(vu.z), vi.z);
    /// ```
    pub fn into_other<U>(self) -> Vector3<U>
    where
        T: Into<U>,
    {
        Vector3 {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
        }
    }
    /// Fallible conversion into `Vector3<U>`
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector3;
    /// let vi: Vector3<i32> = Vector3::new(1, 2, 3);
    /// let vu: Vector3<u32> = vi.try_into_other().unwrap(); // or any other Result resolution
    /// assert_eq!(u32::try_from(vi.x).unwrap(), vu.x);
    /// assert_eq!(u32::try_from(vi.y).unwrap(), vu.y);
    /// assert_eq!(u32::try_from(vi.z).unwrap(), vu.z);
    ///
    /// let vi: Vector3<i32> = Vector3::new(-1, -2, -3);
    /// let vu = vi.try_into_other::<u32>();
    /// assert!(vu.is_err());
    /// ```
    pub fn try_into_other<U>(self) -> Result<Vector3<U>, T::Error>
    where
        T: TryInto<U>,
    {
        Ok(Vector3 {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
            z: self.z.try_into()?,
        })
    }
    /// Lossy conversion into `Vector3<U>`
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector3;
    /// let vf: Vector3<f32> = Vector3::new(1., 2.5, 3.3);
    /// let vi: Vector3<i32> = vf.as_other();
    /// assert_eq!(vi, Vector3::new(1, 2, 3));
    /// ```
    pub fn as_other<U: 'static + Copy>(self) -> Vector3<U>
    where
        T: AsPrimitive<U>,
    {
        Vector3 {
            x: self.x.as_(),
            y: self.y.as_(),
            z: self.z.as_(),
        }
    }
}

/// [`Vector3`] with `f32` coordinates.
pub type Vector3f = Vector3<f32>;
/// [`Vector3`] with `i32` coordinates.
pub type Vector3i = Vector3<i32>;

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Vector3<T> {
    /// Dot product of two 3D vectors.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(9, 10, 21);
    /// let b = Vector3i::new(69, 420, 101);
    ///
    /// assert_eq!(a.dot(b), 6942);
    /// ```
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    /// Square of vector's length.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(9, 10, 21);
    /// assert_eq!(a.length_sq(), 622);
    /// ```
    pub fn length_sq(self) -> T {
        self.dot(self)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Copy> Vector3<T> {
    /// Cross product of two 3D vectors.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(9, 10, 21);
    /// let b = Vector3i::new(69, 420, 101);
    /// assert_eq!(a.cross(b), Vector3i::new(-7810, 540, 3090));
    /// ```
    pub fn cross(self, rhs: Self) -> Vector3<T> {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl<T: Mul<Output = T>> Vector3<T> {
    /// Component-wise multiplication of self and rhs.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(1, 1, 1);
    /// let b = Vector3i::new(2, 2, 2);
    /// assert_eq!(a.cwise_mul(b), Vector3i::new(2, 2, 2));
    /// ```
    pub fn cwise_mul(self, rhs: Self) -> Vector3<T> {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Div<Output = T>> Vector3<T> {
    /// Component-wise division of self and rhs. Panics on divide by zero
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(10, 10, 10);
    /// let b = Vector3i::new(3, 3, 3);
    /// assert_eq!(a.cwise_div(b), Vector3i::new(3, 3, 3));
    /// ```
    pub fn cwise_div(self, rhs: Self) -> Vector3<T> {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Div<Output = T> + CheckedDiv> Vector3<T> {
    /// Component-wise checked division of self and rhs. Returns None on divide by zero
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(10, 10, 10);
    /// let b = Vector3i::new(3, 3, 3);
    /// assert_eq!(a.cwise_checked_div(b), Some(Vector3i::new(3, 3, 3)));
    ///
    /// let a = Vector3i::new(10, 10, 10);
    /// let b = Vector3i::new(3, 0, 3);
    /// assert_eq!(a.cwise_checked_div(b), None);
    /// ```
    pub fn cwise_checked_div(self, rhs: Self) -> Option<Vector3<T>> {
        let x = self.x.checked_div(&rhs.x)?;
        let y = self.y.checked_div(&rhs.y)?;
        let z = self.z.checked_div(&rhs.z)?;
        Some(Vector3 { x, y, z })
    }
}

impl<T: Add> Add for Vector3<T> {
    type Output = Vector3<T::Output>;

    /// Component-wise addition
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(10, 10, 10);
    /// let b = Vector3i::new(10, 10, 10);
    /// assert_eq!(a + b, Vector3i::new(20, 20, 20));
    /// ```
    fn add(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
    /// Component-wise addition assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let mut a = Vector3i::new(10, 10, 10);
    /// a += Vector3i::new(10, 10, 10);
    /// assert_eq!(a, Vector3i::new(20, 20, 20));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub> Sub for Vector3<T> {
    type Output = Vector3<T::Output>;

    /// Component-wise subtraction
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(10, 10, 10);
    /// let b = Vector3i::new(10, 10, 10);
    /// assert_eq!(a - b, Vector3i::new(0, 0, 0));
    /// ```
    fn sub(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
    /// Component-wise subtraction assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let mut a = Vector3i::new(10, 10, 10);
    /// a -= Vector3i::new(10, 10, 10);
    /// assert_eq!(a, Vector3i::new(0, 0, 0));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Mul + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

    /// scalar multiplication
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(69, 21, 19);
    /// assert_eq!(a * 10, Vector3i::new(690, 210, 190));
    /// ```
    fn mul(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    /// scalar multiplication assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let mut a = Vector3i::new(69, 21, 19);
    /// a *= 10;
    /// assert_eq!(a, Vector3i::new(690, 210, 190));
    /// ```
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Div + Copy> Div<T> for Vector3<T> {
    type Output = Vector3<T::Output>;

    /// Scalar division
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::new(10, 10, 10);
    /// assert_eq!(a / 3, Vector3i::new(3, 3, 3));
    /// ```
    fn div(self, rhs: T) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector3<T> {
    /// Scalar division assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let mut a = Vector3i::new(10, 10, 10);
    /// a /= 3;
    /// assert_eq!(a, Vector3i::new(3, 3, 3));
    /// ```
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: CheckedDiv> Vector3<T> {
    /// `checked_div` for scalar division
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// // Passing case
    /// let a = Vector3i::new(10, 10, 10);
    /// assert_eq!(a.checked_div(3), Some(Vector3i::new(3, 3, 3)));
    ///
    /// // Failing case
    /// assert_eq!(a.checked_div(0), None);
    /// ```
    pub fn checked_div(self, rhs: T) -> Option<Vector3<T>> {
        let x = self.x.checked_div(&rhs)?;
        let y = self.y.checked_div(&rhs)?;
        let z = self.z.checked_div(&rhs)?;
        Some(Vector3 { x, y, z })
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

    /// Negates the vector
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// use std::ops::Neg;
    /// let a = Vector3i::new(-69, 21, -10);
    /// assert_eq!(a.neg(), Vector3i::new(69, -21, 10));
    /// ```
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
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector3i;
    /// let a = Vector3i::from((1, 2, 3));
    /// assert_eq!(a, Vector3i::new(1, 2, 3));
    /// ```
    fn from(src: (T, T, T)) -> Self {
        Self {
            x: src.0,
            y: src.1,
            z: src.2,
        }
    }
}

impl Vector3f {
    #[cfg(any(feature = "audio", feature = "graphics"))]
    pub(crate) fn raw(&self) -> crate::ffi::sfVector3f {
        crate::ffi::sfVector3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
    #[cfg(any(feature = "window", feature = "audio"))]
    pub(crate) fn from_raw(raw: crate::ffi::sfVector3f) -> Self {
        Self {
            x: raw.x,
            y: raw.y,
            z: raw.z,
        }
    }
}
