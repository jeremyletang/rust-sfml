use std::ops::Div;

use num_traits::One;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use {
    crate::system::Vector2,
    num_traits::AsPrimitive,
    std::ops::{Add, Sub},
};

/// Utility type for manipulating 2D axis-aligned rectangles.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T> {
    /// Position of the top left corner of the rectangle
    pub position: Vector2<T>,
    /// Size of the rectangle
    pub size: Vector2<T>,
}

/// A [`Rect`] of `i32`.
pub type IntRect = Rect<i32>;
/// A [`Rect`] of `f32`.
pub type FloatRect = Rect<f32>;

impl<T> Rect<T> {
    /// Construct a rectangle from its coordinates.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let rect = Rect::new(Vector2::new(10, 10), Vector2::new(10, 10));
    /// ```
    pub const fn new(position: Vector2<T>, size: Vector2<T>) -> Self {
        Rect { position, size }
    }

    /// Lossless conversion into `Rect<U>`.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Rect::new(Vector2::new(1u8, 2u8), Vector2::new(3u8, 4u8));
    /// let b: Rect<u32> = a.into_other();
    /// assert_eq!(u8::try_from(b.position.y).unwrap(), a.position.y);
    /// assert_eq!(u8::try_from(b.size.y).unwrap(), a.size.y);
    /// assert_eq!(u8::try_from(b.position.x).unwrap(), a.position.x);
    /// assert_eq!(u8::try_from(b.size.x).unwrap(), a.size.x);
    /// ```
    pub fn into_other<U>(self) -> Rect<U>
    where
        T: Into<U>,
    {
        Rect {
            position: self.position.into_other(),
            size: self.size.into_other(),
        }
    }
    /// Fallible conversion into `Rect<U>`
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Rect::new(Vector2::new(1i16, 2i16), Vector2::new(3i16, 4i16));
    /// let b: Rect<u8> = a.try_into_other().unwrap();
    /// assert_eq!(i16::from(b.position.y), a.position.y);
    /// assert_eq!(i16::from(b.size.y), a.size.y);
    /// assert_eq!(i16::from(b.position.x), a.position.x);
    /// assert_eq!(i16::from(b.size.x), a.size.x);
    ///
    /// let a = Rect::new(Vector2::new(-1i16, -2i16), Vector2::new(-3i16, -4i16));
    /// let b = a.try_into_other::<u8>();
    /// assert!(b.is_err());
    /// ```
    pub fn try_into_other<U>(self) -> Result<Rect<U>, T::Error>
    where
        T: TryInto<U>,
    {
        Ok(Rect {
            position: self.position.try_into_other()?,
            size: self.size.try_into_other()?,
        })
    }
    /// Lossy conversion into `Rect<U>`
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Rect::new(Vector2::new(2., 32.32), Vector2::new(3.34, 1.443));
    /// let b: Rect<u8> = a.as_other();
    /// assert_eq!(b.position.y, 32);
    /// assert_eq!(b.position.x, 2);
    /// assert_eq!(b.size.x, 3);
    /// assert_eq!(b.size.y, 1);
    /// ```
    pub fn as_other<U: 'static + Copy>(self) -> Rect<U>
    where
        T: AsPrimitive<U>,
    {
        Rect {
            position: self.position.as_other(),
            size: self.size.as_other(),
        }
    }
}

impl<T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy> Rect<T> {
    /// Check if a point is inside the rectangle's area.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// // Passing case
    /// let a = Rect::new(Vector2::new(0, 0), Vector2::new(4, 4));
    /// let b = Vector2::new(2, 2);
    /// assert!(a.contains(b));
    ///
    /// // Failing case
    /// let a = Rect::new(Vector2::new(0, 0), Vector2::new(1, 1));
    /// let b = Vector2::new(20, 10);
    /// assert!(!a.contains(b));
    /// ```
    #[inline]
    pub fn contains(self, point: Vector2<T>) -> bool {
        let (min_x, max_x) = min_max(self.position.x, self.position.x + self.size.x);
        let (min_y, max_y) = min_max(self.position.y, self.position.y + self.size.y);
        point.x >= min_x && point.x < max_x && point.y >= min_y && point.y < max_y
    }

    /// Returns the intersection between two rectangles, if any.
    ///
    /// If the rectangles intersect, returns Some filled with the intersection
    /// of the two rectangles. Otherwise, returns None.
    ///
    /// # Usage Example
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// // Passing case
    /// let a = Rect::new(Vector2::new(0, 0), Vector2::new(2, 2));
    /// let b = Rect::new(Vector2::new(1, 1), Vector2::new(2, 2));
    /// assert_eq!(a.intersection(&b), Some(Rect::new(Vector2::new(1, 1), Vector2::new(1, 1))));
    ///
    /// // Failing case
    /// let a = Rect::new(Vector2::new(0, 0), Vector2::new(2, 2));
    /// let b = Rect::new(Vector2::new(2, 2), Vector2::new(2, 2));
    /// assert_eq!(a.intersection(&b), None);
    /// ```
    pub fn intersection(self, other: &Rect<T>) -> Option<Rect<T>> {
        // Based on SFML's implementation.
        // Compute the min and max coordinates on various axes.
        let (r1_min_x, r1_max_x) = min_max(self.position.x, self.position.x + self.size.x);
        let (r1_min_y, r1_max_y) = min_max(self.position.y, self.position.y + self.size.y);
        let (r2_min_x, r2_max_x) = min_max(other.position.x, other.position.x + other.size.x);
        let (r2_min_y, r2_max_y) = min_max(other.position.y, other.position.y + other.size.y);
        // Compute the intersection.
        let left = max(r1_min_x, r2_min_x);
        let top = max(r1_min_y, r2_min_y);
        let right = min(r1_max_x, r2_max_x);
        let bottom = min(r1_max_y, r2_max_y);
        // Return the result.
        if left < right && top < bottom {
            Some(Rect::new(
                Vector2::new(left, top),
                Vector2::new(right - left, bottom - top),
            ))
        } else {
            None
        }
    }
}

impl<T> Rect<T>
where
    T: Div<Output = T> + One + Add<Output = T> + Copy,
{
    /// Return the position of the center of the rectangle
    ///
    /// # Usage Example
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Rect::new(Vector2::new(0, 0), Vector2::new(2, 2)).center();
    /// assert_eq!(a, Vector2::new(1, 1));
    /// ````
    pub fn center(self) -> Vector2<T> {
        let two = T::one() + T::one();
        self.position + self.size / two
    }
}

#[inline]
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

#[inline]
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

#[inline]
fn min_max<T: PartialOrd + Copy>(a: T, b: T) -> (T, T) {
    (min(a, b), max(a, b))
}
