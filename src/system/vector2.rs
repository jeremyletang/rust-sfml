use crate::system::Angle;
use num_traits::Float;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use {
    num_traits::{AsPrimitive, CheckedDiv},
    std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
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
/// accessed directly (there are no accessors like `set_x()`, `get_x()`).
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
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2;
    /// let v: Vector2<isize> = Vector2::new(6969, 6969);
    /// ```
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Lossless conversion into `Vector2<U>`.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2;
    /// let vu: Vector2<u16> = Vector2::new(6969, 6969);
    /// let vi: Vector2<i32> = vu.into_other();
    /// assert_eq!(vu.x, vi.x.try_into().unwrap());
    /// assert_eq!(vu.y, vu.y.try_into().unwrap());
    /// ```
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
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2;
    /// // Passing case
    /// let vi: Vector2<i32> = Vector2::new(21, 21);
    /// let vu: Vector2<u32> = vi.try_into_other().unwrap(); // or any other Result resolution
    /// assert_eq!(u32::try_from(vi.x).unwrap(), vu.x);
    /// assert_eq!(u32::try_from(vi.y).unwrap(), vu.y);
    ///
    /// // Failing case
    /// let vi: Vector2<i32> = Vector2::new(-21, -21);
    /// let vu = vi.try_into_other::<u32>();
    /// assert!(vu.is_err());
    /// ```
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
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2;
    /// let vf: Vector2<f32> = Vector2::new(696969.6969, 6969.6969);
    /// let vi: Vector2<i32> = vf.as_other();
    /// assert_eq!(vf.x as i32, vi.x);
    /// assert_eq!(vf.y as i32, vi.y);
    /// ```
    pub fn as_other<U: 'static + Copy>(self) -> Vector2<U>
    where
        T: AsPrimitive<U>,
    {
        Vector2 {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }

    /// Dot product of two 2D vectors.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(16, 64);
    /// let b = Vector2i::new(2, 4);
    /// assert_eq!(a.dot(b), 288);
    /// ```
    pub fn dot(self, rhs: Self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy,
    {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Length of the vector
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2f;
    /// let a = Vector2f::new(3., 4.);
    /// assert!((a.length() - 5.).abs() <= f32::EPSILON);
    /// ```
    pub fn length(self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy + Float,
    {
        self.length_sq().sqrt()
    }

    /// Vector with the same direciton but length 1
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2f;
    /// let a = Vector2f::new(2., 2.).normalized();
    /// assert!((a.length() - 1.).abs() <= f32::EPSILON);
    /// assert!((a.x - 2f32.sqrt()/2.).abs() <= f32::EPSILON);
    /// ```
    pub fn normalized(self) -> Vector2<T>
    where
        T: Mul<Output = T> + Add<Output = T> + Copy + Float + Div<Output = T>,
    {
        self / self.length()
    }

    /// Square of vector's length.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(10, 9);
    /// assert_eq!(a.length_sq(), 181);
    /// ```
    pub fn length_sq(self) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Copy,
    {
        self.dot(self)
    }

    /// Z component of the cross product of two 2D vectors.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(69, 420);
    /// let b = Vector2i::new(21, 101);
    /// assert_eq!(a.cross(b), -1851);
    /// ```
    pub fn cross(self, rhs: Self) -> T
    where
        T: Mul<Output = T> + Sub<Output = T> + Copy,
    {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Component-wise multiplication of self and rhs.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(1, 1);
    /// let b = Vector2i::new(2, 2);
    /// assert_eq!(a.cwise_mul(b), Vector2i::new(2, 2));
    /// ```
    pub fn cwise_mul(self, rhs: Self) -> Vector2<T>
    where
        T: Mul<Output = T>,
    {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }

    /// Component-wise division of self and rhs. Panics on divide by zero
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(69, 69);
    /// let b = Vector2i::new(3, 3);
    /// assert_eq!(a.cwise_div(b), Vector2i::new(23, 23));
    /// ```
    pub fn cwise_div(self, rhs: Self) -> Vector2<T>
    where
        T: Div<Output = T>,
    {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }

    /// Component-wise checked division of self and rhs. Returns None on divide by zero
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// // Passing case
    /// let a = Vector2i::new(69, 69);
    /// let b = Vector2i::new(3, 3);
    /// assert_eq!(a.cwise_checked_div(b), Some(Vector2i::new(23, 23)));
    ///
    /// // Failing case
    /// let b = Vector2i::new(0, 3);
    /// assert_eq!(a.cwise_checked_div(b), None);
    /// ```
    pub fn cwise_checked_div(self, rhs: Self) -> Option<Vector2<T>>
    where
        T: Div<Output = T> + CheckedDiv,
    {
        let x = self.x.checked_div(&rhs.x)?;
        let y = self.y.checked_div(&rhs.y)?;
        Some(Vector2 { x, y })
    }

    /// Returns a perpendicular vector rotated +90 degrees
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(21, -21);
    /// assert_eq!(a.perpendicular(), Vector2i::new(21, 21));
    /// ```
    pub fn perpendicular(self) -> Vector2<T>
    where
        T: Neg<Output = T>,
    {
        Vector2::new(-self.y, self.x)
    }

    /// `checked_div` for scalar division
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// // Passing case
    /// let a = Vector2i::new(420, 69);
    /// assert_eq!(a.checked_div(1000), Some(Vector2i::new(0, 0)));
    ///
    /// // Failing case
    /// assert_eq!(a.checked_div(0), None);
    /// ```
    pub fn checked_div(self, rhs: T) -> Option<Vector2<T>>
    where
        T: CheckedDiv,
    {
        let x = self.x.checked_div(&rhs)?;
        let y = self.y.checked_div(&rhs)?;
        Some(Vector2 { x, y })
    }
    /// Creates a vector from polar coordinates (magnitude and angle)
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::{Vector2f, Angle};
    /// let v = Vector2f::from_polar(5.0, Angle::degrees(90.0)); // 5 units pointing up
    /// assert!((v.x - 0.0).abs() < 1e-6);
    /// assert!((v.y - 5.0).abs() < 1e-6);
    /// ```
    pub fn from_polar(r: T, phi: crate::system::Angle) -> Self
    where
        T: From<f32> + Copy + Mul<Output = T>,
    {
        let (sin, cos) = phi.as_radians().sin_cos();
        Self {
            x: r * T::from(cos),
            y: r * T::from(sin),
        }
    }

    /// Returns the angle of this vector from the positive X-axis
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::{Vector2f, Angle};
    /// let v = Vector2f::new(1.0, 1.0);
    /// let angle = v.angle();
    /// assert!((angle.as_degrees() - 45.0).abs() < 0.001);
    /// ```
    pub fn angle(self) -> crate::system::Angle
    where
        T: Float + Copy + Into<f32>,
    {
        debug_assert!(
            self.x != T::zero() || self.y != T::zero(),
            "Cannot calculate angle of zero vector"
        );
        Angle::radians(T::atan2(self.y, self.x).into())
    }

    /// Returns the signed angle from this vector to another vector
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::{Vector2f, Angle};
    /// let v1 = Vector2f::new(1.0, 0.0);  // pointing right
    /// let v2 = Vector2f::new(0.0, 1.0);  // pointing up
    /// let angle = v1.angle_to(v2);
    /// assert!((angle.as_degrees() - 90.0).abs() < 0.001);
    /// ```
    pub fn angle_to(self, rhs: Self) -> crate::system::Angle
    where
        T: Float + Copy + Into<f32>,
    {
        debug_assert!(
            self.x != T::zero() || self.y != T::zero(),
            "Cannot calculate angle from zero vector"
        );
        debug_assert!(
            rhs.x != T::zero() || rhs.y != T::zero(),
            "Cannot calculate angle to zero vector"
        );

        let cross = self.cross(rhs);
        let dot = self.dot(rhs);
        Angle::radians(T::atan2(cross, dot).into())
    }

    /// Returns this vector rotated by the specified angle
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::{Vector2f, Angle};
    /// let v = Vector2f::new(1.0, 0.0);  // pointing right
    /// let rotated = v.rotated_by(Angle::degrees(90.0));  // rotate 90 degrees
    /// assert!((rotated.x - 0.0).abs() < 0.001);
    /// assert!((rotated.y - 1.0).abs() < 0.001);
    /// ```
    pub fn rotated_by(self, phi: crate::system::Angle) -> Self
    where
        T: Float + Copy + From<f32>,
    {
        let (sin, cos) = phi.as_radians().sin_cos();
        let (sin, cos) = (sin.into(), cos.into());

        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Returns the projection of this vector onto the given axis
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2f;
    /// let v = Vector2f::new(3.0, 4.0);
    /// let axis = Vector2f::new(1.0, 0.0);  // X-axis
    /// let projected = v.projected_onto(axis);
    /// assert!((projected.x - 3.0).abs() < f32::EPSILON);
    /// assert!((projected.y - 0.0).abs() < f32::EPSILON);
    /// ```
    pub fn projected_onto(self, axis: Self) -> Self
    where
        T: Float + Copy,
    {
        debug_assert!(
            axis.x != T::zero() || axis.y != T::zero(),
            "Cannot project onto zero vector"
        );

        let axis_length_sq = axis.length_sq();
        let projection_length = self.dot(axis) / axis_length_sq;
        axis * projection_length
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    /// Constructs a `Vector2` from `(x, y)`.
    ///
    /// # Usage example
    ///
    /// ```
    /// # use sfml::system::Vector2;
    /// let a: Vector2<u16> = Vector2::from((69u16, 420u16));
    /// assert_eq!(a.x, 69u16);
    /// assert_eq!(a.y, 420u16);
    /// ```
    fn from(src: (T, T)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}

impl<T> From<Vector2<T>> for (T, T) {
    /// Constructs `(x, y)` from a `Vector2`
    fn from(vec: Vector2<T>) -> Self {
        (vec.x, vec.y)
    }
}

impl<T> From<[T; 2]> for Vector2<T> {
    /// Constructs a `Vector2` from `[x, y]`.
    fn from([x, y]: [T; 2]) -> Self {
        Self { x, y }
    }
}

impl<T> From<Vector2<T>> for [T; 2] {
    /// Constructs `[x, y]` from a `Vector2`
    fn from(vec: Vector2<T>) -> Self {
        [vec.x, vec.y]
    }
}

/// Create a `Vector2` with both fields initialized to the same value
impl<T: Clone> From<T> for Vector2<T> {
    fn from(src: T) -> Self {
        Self {
            x: src.clone(),
            y: src,
        }
    }
}

impl<T: Add> Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T::Output>;

    /// Performs component wise addition
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(9, 10);
    /// let b = Vector2i::new(10, 9);
    /// assert_ne!(a + b, Vector2i::new(21, 21));
    /// ```
    fn add(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    /// Performs component wise addition assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let mut a = Vector2i::new(9, 10);
    /// let b = Vector2i::new(10, 9);
    /// a += b;
    /// assert_eq!(a, Vector2i::new(19, 19));
    /// ```
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub> Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T::Output>;

    /// Performs component wise subtraction
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(9, 10);
    /// let b = Vector2i::new(10, 9);
    /// assert_eq!(a - b, Vector2i::new(-1, 1));
    /// ```
    fn sub(self, rhs: Vector2<T>) -> Vector2<T::Output> {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    /// Performs component wise subtraction assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let mut a = Vector2i::new(9, 10);
    /// let b = Vector2i::new(10, 9);
    /// a -= b;
    /// assert_eq!(a, Vector2i::new(-1, 1));
    /// ```
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    /// Performs scalar multiplication
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(9, 10);
    /// assert_eq!(a * 420, Vector2i::new(3780, 4200));
    /// ```
    fn mul(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    /// Performs scalar multiplication assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let mut a = Vector2i::new(9, 10);
    /// a *= 420;
    /// assert_eq!(a, Vector2i::new(3780, 4200));
    /// ```
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div + Copy> Div<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    /// Performs scalar division
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let a = Vector2i::new(9, 10);
    /// assert_eq!(a / 3, Vector2i::new(3, 3));
    /// ```
    fn div(self, rhs: T) -> Vector2<T::Output> {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    /// Performs scalar division assignment
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// let mut a = Vector2i::new(9, 10);
    /// a /= 3;
    /// assert_eq!(a, Vector2i::new(3, 3));
    /// ```
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    /// Negates the vector
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::system::Vector2i;
    /// use std::ops::Neg;
    /// let a = Vector2i::new(21, 21);
    /// assert_eq!(a.neg(), Vector2i::new(-21, -21));
    /// ```
    fn neg(self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
