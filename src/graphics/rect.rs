use std::ops::{Add, Sub};
use system::Vector2;
use system::raw_conv::{FromRaw, Raw};

/// Utility type for manipulating 2D axis-aligned rectangles.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Default)]
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

/// A `Rect` of `i32`.
pub type IntRect = Rect<i32>;
/// A `Rect` of `f32`.
pub type FloatRect = Rect<f32>;

impl<T> Rect<T> {
    /// Construct a rectangle from its coordinates.
    pub fn new(left: T, top: T, width: T, height: T) -> Rect<T> {
        Rect {
            left: left,
            top: top,
            width: width,
            height: height,
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

impl Raw for IntRect {
    type Raw = ::csfml_graphics_sys::sfIntRect;

    fn raw(&self) -> Self::Raw {
        ::csfml_graphics_sys::sfIntRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
}

impl FromRaw for IntRect {
    type RawFrom = ::csfml_graphics_sys::sfIntRect;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        IntRect {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}

impl Raw for FloatRect {
    type Raw = ::csfml_graphics_sys::sfFloatRect;

    fn raw(&self) -> Self::Raw {
        ::csfml_graphics_sys::sfFloatRect {
            top: self.top,
            left: self.left,
            width: self.width,
            height: self.height,
        }
    }
}

impl FromRaw for FloatRect {
    type RawFrom = ::csfml_graphics_sys::sfFloatRect;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        FloatRect {
            top: raw.top,
            left: raw.left,
            width: raw.width,
            height: raw.height,
        }
    }
}
