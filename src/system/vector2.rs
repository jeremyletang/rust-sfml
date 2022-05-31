use std::{
    convert::{TryFrom, TryInto},
    num::TryFromIntError,
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
        impl LossyFrom<Vector2<$from>> for Vector2<$to> {
            #[inline]
            fn lossy_from(v: Vector2<$from>) -> Vector2<$to> {
                Vector2 {
                    x: v.x as $to,
                    y: v.y as $to
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
        impl From<Vector2<$from>> for Vector2<$to> {
            #[inline]
            fn from(v: Vector2<$from>) -> Vector2<$to> {
                Vector2{ x:v.x.into(), y: v.y.into() }
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
        impl TryFrom<Vector2<$from>> for Vector2<$to> {
            type Error = $from_err;

            #[inline]
            fn try_from(v: Vector2<$from>) -> Result<Vector2<$to>, Self::Error> {
                Ok(Vector2{ x: v.x.try_into()?, y: v.y.try_into()? })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from() {
        macro_rules! test_try_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Vector2<$from> = Vector2 { x: 0, y: 0 };
                let to: Result<Vector2<$to>, TryFromIntError> = from.try_into();
                assert!(to.is_ok());

                let from: Vector2<$from> = Vector2 { x: <$from>::MAX, y: <$from>::MIN };
                let to: Result<Vector2<$to>, TryFromIntError> = from.try_into();
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
                let from: Vector2<$from> = Vector2 { x: 0, y: 0 };
                let _to: Vector2<$to> = from.into();
                let from: Vector2<$from> = Vector2 { x: <$from>::MAX, y: <$from>::MIN };
                let _to: Vector2<$to> = from.into();
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
                let from: Vector2<$from> = Vector2{ x: <$from>::MAX, y: <$from>::MIN};
                let _to: Vector2<$to> = from.lossy_into();
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
