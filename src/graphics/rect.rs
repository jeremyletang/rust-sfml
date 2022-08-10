use crate::system::Vector2;
use num_traits::AsPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    convert::TryInto,
    ops::{Add, Sub},
};

use crate::ffi;

/// Utility type for manipulating 2D axis-aligned rectangles.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T> {
    /// Left coordinate of the rectangle.
    pub left: T,
    /// Top coordinate of the rectangle.
    pub top: T,
    /// Width of the rectangle.
    pub width: T,
    /// Height of the rectangle.
    pub height: T,
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
    /// let rect = Rect::new(10, 10, 10, 10);
    /// ```
    pub const fn new(left: T, top: T, width: T, height: T) -> Self {
        Rect {
            left,
            top,
            width,
            height,
        }
    }

    /// Construct a rectangle from its position and size.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Vector2::new(10, 20);
    /// let b = Vector2::new(30, 40);
    /// let rect = Rect::from_vecs(a, b);
    /// assert_eq!(rect, Rect::new(10, 20, 30, 40));
    /// ```
    pub fn from_vecs(pos: Vector2<T>, size: Vector2<T>) -> Rect<T> {
        Rect {
            left: pos.x,
            top: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    /// Lossless conversion into `Rect<U>`.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// let a = Rect::new(1u8, 2u8, 3u8, 4u8);
    /// let b: Rect<u32> = a.into_other();
    /// assert_eq!(u8::try_from(b.top).unwrap(), a.top);
    /// assert_eq!(u8::try_from(b.height).unwrap(), a.height);
    /// assert_eq!(u8::try_from(b.left).unwrap(), a.left);
    /// assert_eq!(u8::try_from(b.width).unwrap(), a.width);
    /// ```
    pub fn into_other<U>(self) -> Rect<U>
    where
        T: Into<U>,
    {
        Rect {
            top: self.top.into(),
            left: self.left.into(),
            width: self.width.into(),
            height: self.height.into(),
        }
    }
    /// Fallible conversion into `Rect<U>`
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// let a = Rect::new(1i16, 2i16, 3i16, 4i16);
    /// let b: Rect<u8> = a.try_into_other().unwrap();
    /// assert_eq!(i16::from(b.top), a.top);
    /// assert_eq!(i16::from(b.height), a.height);
    /// assert_eq!(i16::from(b.left), a.left);
    /// assert_eq!(i16::from(b.width), a.width);
    ///
    /// let a = Rect::new(-1i16, -2i16, -3i16, -4i16);
    /// let b = a.try_into_other::<u8>();
    /// assert!(b.is_err());
    /// ```
    pub fn try_into_other<U>(self) -> Result<Rect<U>, T::Error>
    where
        T: TryInto<U>,
    {
        Ok(Rect {
            left: self.left.try_into()?,
            top: self.top.try_into()?,
            width: self.width.try_into()?,
            height: self.height.try_into()?,
        })
    }
    /// Lossy conversion into `Rect<U>`
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// let a = Rect::new(2., 32.32, 3.34, 1.443);
    /// let b: Rect<u8> = a.as_other();
    /// assert_eq!(b.top, 32);
    /// assert_eq!(b.left, 2);
    /// assert_eq!(b.width, 3);
    /// assert_eq!(b.height, 1);
    /// ```
    pub fn as_other<U: 'static + Copy>(self) -> Rect<U>
    where
        T: AsPrimitive<U>,
    {
        Rect {
            left: self.left.as_(),
            top: self.top.as_(),
            width: self.width.as_(),
            height: self.height.as_(),
        }
    }

    /// Get the position of the rectangle's top-left corner
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Rect::new(1, 2, 3, 4);
    /// assert_eq!(a.position(), Vector2::new(1, 2));
    /// ```
    pub fn position(self) -> Vector2<T> {
        Vector2::new(self.left, self.top)
    }

    /// Get the size of the rectangle
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// # use sfml::system::Vector2;
    /// let a = Rect::new(1, 2, 3, 4);
    /// assert_eq!(a.size(), Vector2::new(3, 4));
    /// ```
    pub fn size(self) -> Vector2<T> {
        Vector2::new(self.width, self.height)
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
    /// let a = Rect::new(0, 0, 4, 4);
    /// let b = Vector2::new(2, 2);
    /// assert!(a.contains(b));
    ///
    /// // Failing case
    /// let a = Rect::new(0, 0, 1, 1);
    /// let b = Vector2::new(20, 10);
    /// assert!(!a.contains(b));
    /// ```
    #[inline]
    pub fn contains(self, point: Vector2<T>) -> bool {
        self.contains2(point.x, point.y)
    }

    /// Check if a point is inside the rectangle's area.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Rect;
    /// // Passing case
    /// let a = Rect::new(0, 0, 4, 4);
    /// assert!(a.contains2(2, 2));
    ///
    /// // Failing case
    /// let a = Rect::new(0, 0, 1, 1);
    /// assert!(!a.contains2(20, 10));
    /// ```
    pub fn contains2(self, x: T, y: T) -> bool {
        // Based on SFML's implementation.
        // Rectangles with negative dimensions are allowed.
        let (min_x, max_x) = min_max(self.left, self.left + self.width);
        let (min_y, max_y) = min_max(self.top, self.top + self.height);
        x >= min_x && x < max_x && y >= min_y && y < max_y
    }

    /// Returns the intersection between two rectangles, if any.
    ///
    /// If the rectangles intersect, returns Some filled with the intersection
    /// of the two rectangles. Otherwise, returns None.
    ///
    /// # Usage Example
    /// ```
    /// # use sfml::graphics::Rect;
    /// // Passing case
    /// let a = Rect::new(0, 0, 2, 2);
    /// let b = Rect::new(1, 1, 2, 2);
    /// assert_eq!(a.intersection(&b), Some(Rect::new(1, 1, 1, 1)));
    ///
    /// // Failing case
    /// let a = Rect::new(0, 0, 2, 2);
    /// let b = Rect::new(2, 2, 2, 2);
    /// assert_eq!(a.intersection(&b), None);
    /// ```
    pub fn intersection(self, other: &Rect<T>) -> Option<Rect<T>> {
        // Based on SFML's implementation.
        // Compute the min and max coordinates on various axes.
        let (r1_min_x, r1_max_x) = min_max(self.left, self.left + self.width);
        let (r1_min_y, r1_max_y) = min_max(self.top, self.top + self.height);
        let (r2_min_x, r2_max_x) = min_max(other.left, other.left + other.width);
        let (r2_min_y, r2_max_y) = min_max(other.top, other.top + other.height);
        // Compute the intersection.
        let left = max(r1_min_x, r2_min_x);
        let top = max(r1_min_y, r2_min_y);
        let right = min(r1_max_x, r2_max_x);
        let bottom = min(r1_max_y, r2_max_y);
        // Return the result.
        if left < right && top < bottom {
            Some(Rect::new(left, top, right - left, bottom - top))
        } else {
            None
        }
    }
}

#[inline]
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[inline]
fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[inline]
fn min_max<T: PartialOrd + Copy>(a: T, b: T) -> (T, T) {
    (min(a, b), max(a, b))
}

impl IntRect {
    pub(super) fn raw(&self) -> ffi::graphics::sfIntRect {
        ffi::graphics::sfIntRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
    pub(super) fn from_raw(raw: ffi::graphics::sfIntRect) -> Self {
        Self {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}

impl FloatRect {
    pub(super) fn raw(&self) -> ffi::graphics::sfFloatRect {
        ffi::graphics::sfFloatRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
    pub(super) fn from_raw(raw: ffi::graphics::sfFloatRect) -> Self {
        Self {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}
