use num_traits::AsPrimitive;

use crate::system::Vector2;
use std::{
    convert::TryInto,
    ops::{Add, Sub},
};

use crate::ffi;

/// Utility type for manipulating 2D axis-aligned rectangles.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug, Copy, Default)]
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
    pub const fn new(left: T, top: T, width: T, height: T) -> Self {
        Rect {
            left,
            top,
            width,
            height,
        }
    }

    /// Construct a rectangle from its position and size.
    pub fn from_vecs(pos: Vector2<T>, size: Vector2<T>) -> Rect<T> {
        Rect {
            left: pos.x,
            top: pos.y,
            width: size.x,
            height: size.y,
        }
    }

    /// Lossless conversion into `Rect<U>`.
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
    pub fn position(self) -> Vector2<T> {
        Vector2::new(self.left, self.top)
    }

    /// Get the size of the rectangle
    pub fn size(self) -> Vector2<T> {
        Vector2::new(self.width, self.height)
    }
}

impl<T: PartialOrd + Add<Output = T> + Sub<Output = T> + Copy> Rect<T> {
    /// Check if a point is inside the rectangle's area.
    #[inline]
    pub fn contains(self, point: Vector2<T>) -> bool {
        self.contains2(point.x, point.y)
    }

    /// Check if a point is inside the rectangle's area.
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
    pub(super) fn raw(&self) -> ffi::sfIntRect {
        ffi::sfIntRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
    pub(super) fn from_raw(raw: ffi::sfIntRect) -> Self {
        Self {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}

impl FloatRect {
    pub(super) fn raw(&self) -> ffi::sfFloatRect {
        ffi::sfFloatRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
    pub(super) fn from_raw(raw: ffi::sfFloatRect) -> Self {
        Self {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}
