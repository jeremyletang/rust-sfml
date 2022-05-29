use crate::system::Vector2;
use std::{
    convert::{TryFrom, TryInto},
    num::TryFromIntError,
    ops::{Add, Sub},
};

use crate::ffi;

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

/// A [`Rect`] of `i32`.
pub type IntRect = Rect<i32>;
/// A [`Rect`] of `f32`.
pub type FloatRect = Rect<f32>;

pub trait LossyFrom<T> {
    fn lossy_from(v: T) -> Self;
}

pub trait LossyInto<T> {
    fn lossy_into(self) -> T;
}

impl<T, U> LossyInto<U> for T
where
    U: LossyFrom<T>,
{
    fn lossy_into(self) -> U {
        U::lossy_from(self)
    }
}

macro_rules! lossy_from_impls {
    ($( $from:ty, $to:ty );*) => {$(
        impl LossyFrom<Rect<$from>> for Rect<$to> {
            #[inline]
            fn lossy_from(r: Rect<$from>) -> Rect<$to> {
                Rect {
                    top: r.top as $to,
                    left: r.left as $to,
                    width: r.width as $to,
                    height: r.height as $to,
                }
            }
        }
    )*}
}
// float -> float (differently size)
lossy_from_impls!(f32, f64; f64, f32);
// f32 -> integer
lossy_from_impls!(
f32, i8;
f32, i16;
f32, i32;
f32, i64;
f32, i128;
f32, isize;
f32, u8;
f32, u16;
f32, u32;
f32, u64;
f32, u128;
f32, usize
);
// integer -> f32
lossy_from_impls!(
i8, f32;
i16, f32;
i32, f32;
i64, f32;
i128, f32;
isize, f32;
u8, f32;
u16, f32;
u32, f32;
u64, f32;
u128, f32;
usize, f32
);
// f64 -> integer
lossy_from_impls!(
f64, i8;
f64, i16;
f64, i32;
f64, i64;
f64, i128;
f64, isize;
f64, u8;
f64, u16;
f64, u32;
f64, u64;
f64, u128;
f64, usize
);
// integer -> f64
lossy_from_impls!(
i8, f64;
i16, f64;
i32, f64;
i64, f64;
i128, f64;
isize, f64;
u8, f64;
u16, f64;
u32, f64;
u64, f64;
u128, f64;
usize, f64
);

macro_rules! from_impls {
    ($( $from:ty, $to:ty );*) => {$(
        impl From<Rect<$from>> for Rect<$to> {
            #[inline]
            fn from(r: Rect<$from>) -> Rect<$to> {
                Rect { top:r.top.into(), left: r.left.into(), width: r.width.into(), height: r.height.into() }
            }
        }
    )*}
}
// unsigned -> signed
from_impls!(
u8, i16;
u8, i32;
u8, i64;
u8, i128;
u16, i32;
u16, i64;
u16, i128;
u32, i64;
u32, i128;
u64, i128
);
// unsigned -> unsigned (different unsize)
from_impls!(
u8, u16;
u8, u32;
u8, u64;
u8, u128;
u16, u32;
u16, u64;
u16, u128;
u32, u64;
u32, u128;
u64, u128
);
// signed -> signed (different size)
from_impls!(
i8, i16;
i8, i32;
i8, i64;
i8, i128;
i16, i32;
i16, i64;
i16, i128;
i32, i64;
i32, i128;
i64, i128
);

macro_rules! try_from_impls {
    ($( $from:ty, $to:ty, $from_err: ty );*) => {$(
        impl TryFrom<Rect<$from>> for Rect<$to> {
            type Error = $from_err;

            #[inline]
            fn try_from(r: Rect<$from>) -> Result<Rect<$to>, Self::Error> {
                Ok(Rect{ top: r.top.try_into()?, left: r.left.try_into()?, width: r.width.try_into()?, height: r.height.try_into()? })
            }
        }
    )*}
}
// signed -> unsigned
try_from_impls!(
i8, u8, TryFromIntError;
i8, u16, TryFromIntError;
i8, u32, TryFromIntError;
i8, u64, TryFromIntError;
i8, u128, TryFromIntError;
i8, usize, TryFromIntError;
i16, u8, TryFromIntError;
i16, u16, TryFromIntError;
i16, u32, TryFromIntError;
i16, u64, TryFromIntError;
i16, u128, TryFromIntError;
i16, usize, TryFromIntError;
i32, u8, TryFromIntError;
i32, u16, TryFromIntError;
i32, u32, TryFromIntError;
i32, u64, TryFromIntError;
i32, u128, TryFromIntError;
i32, usize, TryFromIntError;
i64, u8, TryFromIntError;
i64, u16, TryFromIntError;
i64, u32, TryFromIntError;
i64, u64, TryFromIntError;
i64, u128, TryFromIntError;
i64, usize, TryFromIntError;
i128, u8, TryFromIntError;
i128, u16, TryFromIntError;
i128, u32, TryFromIntError;
i128, u64, TryFromIntError;
i128, u128, TryFromIntError;
i128, usize, TryFromIntError;
isize, u8, TryFromIntError;
isize, u16, TryFromIntError;
isize, u32, TryFromIntError;
isize, u64, TryFromIntError;
isize, u128, TryFromIntError;
isize, usize, TryFromIntError
);
// unsigned -> signed
try_from_impls!(
u8, i8, TryFromIntError;
u8, isize, TryFromIntError;
u16, i8, TryFromIntError;
u16, i16, TryFromIntError;
u16, isize, TryFromIntError;
u32, i8, TryFromIntError;
u32, i16, TryFromIntError;
u32, i32, TryFromIntError;
u32, isize, TryFromIntError;
u64, i8, TryFromIntError;
u64, i16, TryFromIntError;
u64, i32, TryFromIntError;
u64, i64, TryFromIntError;
u64, isize, TryFromIntError;
u128, i8, TryFromIntError;
u128, i16, TryFromIntError;
u128, i32, TryFromIntError;
u128, i64, TryFromIntError;
u128, i128, TryFromIntError;
u128, isize, TryFromIntError;
usize, i8, TryFromIntError;
usize, i16, TryFromIntError;
usize, i32, TryFromIntError;
usize, i64, TryFromIntError;
usize, i128, TryFromIntError;
usize, isize, TryFromIntError
);
// unsigned -> unsigned (different size)
try_from_impls!(
u16, u8, TryFromIntError;
u32, u16, TryFromIntError;
u32, u8, TryFromIntError;
u64, u32, TryFromIntError;
u64, u16, TryFromIntError;
u64, u8, TryFromIntError;
u128, u64, TryFromIntError;
u128, u32, TryFromIntError;
u128, u16, TryFromIntError;
u128, u8, TryFromIntError;
usize, u8, TryFromIntError;
usize, u16, TryFromIntError;
usize, u32, TryFromIntError;
usize, u64, TryFromIntError;
usize, u128, TryFromIntError
);
// signed -> signed (different size)
try_from_impls!(
i16, i8, TryFromIntError;
i32, i16, TryFromIntError;
i32, i8, TryFromIntError;
i64, i32, TryFromIntError;
i64, i16, TryFromIntError;
i64, i8, TryFromIntError;
i128, i64, TryFromIntError;
i128, i32, TryFromIntError;
i128, i16, TryFromIntError;
i128, i8, TryFromIntError;
isize, i8, TryFromIntError;
isize, i16, TryFromIntError;
isize, i32, TryFromIntError;
isize, i64, TryFromIntError;
isize, i128, TryFromIntError
);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from() {
        macro_rules! test_try_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Rect<$from> = Rect { top: 0, left: 0, width: 0, height: 0 };
                let to: Result<Rect<$to>, TryFromIntError> = from.try_into();
                assert!(to.is_ok());

                let from: Rect<$from> = Rect { top: <$from>::MAX, left: <$from>::MIN, width: 0, height: 0 };
                let to: Result<Rect<$to>, TryFromIntError> = from.try_into();
                assert!(to.is_err());
            )*}
        }

        test_try_froms!(
        i8, u8;
        i8, u16;
        i8, u32;
        i8, u64;
        i8, u128;
        i16, u8;
        i16, u16;
        i16, u32;
        i16, u64;
        i16, u128;
        i32, u8;
        i32, u16;
        i32, u32;
        i32, u64;
        i32, u128;
        i64, u8;
        i64, u16;
        i64, u32;
        i64, u64;
        i64, u128;
        i128, u8;
        i128, u16;
        i128, u32;
        i128, u64;
        i128, u128
        );

        test_try_froms!(
        u8, i8;
        u16, i8;
        u16, i16;
        u32, i8;
        u32, i16;
        u32, i32;
        u64, i8;
        u64, i16;
        u64, i32;
        u64, i64;
        u128, i8;
        u128, i16;
        u128, i32;
        u128, i64;
        u128, i128
        );

        test_try_froms!(
        i16, i8;
        i32, i16;
        i32, i8;
        i64, i32;
        i64, i16;
        i64, i8;
        i128, i64;
        i128, i32;
        i128, i16;
        i128, i8
        );

        test_try_froms!(
        u16, u8;
        u32, u16;
        u32, u8;
        u64, u32;
        u64, u16;
        u64, u8;
        u128, u64;
        u128, u32;
        u128, u16;
        u128, u8
        );
    }

    #[test]
    fn from() {
        macro_rules! test_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Rect<$from> = Rect { top: 0, left: 0, width: 0, height: 0 };
                let _to: Rect<$to> = from.into();
                let from: Rect<$from> = Rect { top: <$from>::MAX, left: <$from>::MIN, width: 0, height: 0 };
                let _to: Rect<$to> = from.into();
            )*}
        }

        test_froms!(
        u8, i16;
        u8, i32;
        u8, i64;
        u8, i128;
        u16, i32;
        u16, i64;
        u16, i128;
        u32, i64;
        u32, i128;
        u64, i128
                );

        test_froms!(
        u8, u16;
        u8, u32;
        u8, u64;
        u8, u128;
        u16, u32;
        u16, u64;
        u16, u128;
        u32, u64;
        u32, u128;
        u64, u128
        );

        test_froms!(
        i8, i16;
        i8, i32;
        i8, i64;
        i8, i128;
        i16, i32;
        i16, i64;
        i16, i128;
        i32, i64;
        i32, i128;
        i64, i128
        );
    }

    #[test]
    fn lossy_from() {
        macro_rules! test_lossy_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Rect<$from> = Rect{ top: <$from>::MAX, left: <$from>::MIN, width: <$from>::MAX, height: <$from>::MIN};
                let _to: Rect<$to> = from.lossy_into();
            )*}
        }

        test_lossy_froms!(
        f32, i8;
        f32, i16;
        f32, i32;
        f32, i64;
        f32, i128;
        f32, isize;
        f32, u8;
        f32, u16;
        f32, u32;
        f32, u64;
        f32, u128;
        f32, usize;
        i8, f32;
        i16, f32;
        i32, f32;
        i64, f32;
        i128, f32;
        isize, f32;
        u8, f32;
        u16, f32;
        u32, f32;
        u64, f32;
        u128, f32;
        usize, f32
        );
        test_lossy_froms!(
        f64, i8;
        f64, i16;
        f64, i32;
        f64, i64;
        f64, i128;
        f64, isize;
        f64, u8;
        f64, u16;
        f64, u32;
        f64, u64;
        f64, u128;
        f64, usize;
        i8, f64;
        i16, f64;
        i32, f64;
        i64, f64;
        i128, f64;
        isize, f64;
        u8, f64;
        u16, f64;
        u32, f64;
        u64, f64;
        u128, f64;
        usize, f64
        );
        test_lossy_froms!(f32, f64; f64, f32);
    }
}
