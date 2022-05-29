use std::{
    convert::{TryFrom, TryInto},
    num::TryFromIntError,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

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
#[derive(Clone, PartialOrd, PartialEq, Debug, Copy, Default)]
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
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

/// [`Vector3`] with `f32` coordinates.
pub type Vector3f = Vector3<f32>;
/// [`Vector3`] with `i32` coordinates.
pub type Vector3i = Vector3<i32>;

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
        impl LossyFrom<Vector3<$from>> for Vector3<$to> {
            #[inline]
            fn lossy_from(v: Vector3<$from>) -> Vector3<$to> {
                Vector3 {
                    x: v.x as $to,
                    y: v.y as $to,
                    z: v.z as $to
                }
            }
        }
    )*}
}

lossy_from_impls!(f32, f64; f64, f32);

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

macro_rules! from_impls {
    ($( $from:ty, $to:ty );*) => {$(
        impl From<Vector3<$from>> for Vector3<$to> {
            #[inline]
            fn from(v: Vector3<$from>) -> Vector3<$to> {
                Vector3{ x:v.x.into(), y: v.y.into(), z: v.z.into() }
            }
        }
    )*}
}

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

macro_rules! try_from_impls {
    ($( $from:ty, $to:ty, $from_err: ty );*) => {$(
        impl TryFrom<Vector3<$from>> for Vector3<$to> {
            type Error = $from_err;

            #[inline]
            fn try_from(v: Vector3<$from>) -> Result<Vector3<$to>, Self::Error> {
                Ok(Vector3{ x: v.x.try_into()?, y: v.y.try_into()?, z: v.z.try_into()? })
            }
        }
    )*}
}

try_from_impls!(i8, u8, TryFromIntError;
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

macro_rules! impl_ops {
    ( $_trait:ident, $_func:ident, $( $_type:ty ),+ ) => {
        impl<T: $_trait + Copy> $_trait<T> for Vector3<T> {
            type Output = Vector3<T::Output>;

            fn $_func(self, rhs: T) -> Vector3<T::Output> {
                Vector3 {
                    x: $_trait::$_func(self.x, rhs),
                    y: $_trait::$_func(self.y, rhs),
                    z: $_trait::$_func(self.z, rhs)
                }
            }
        }

        $(
            impl $_trait<Vector3<$_type>> for $_type {
                type Output = Vector3<$_type>;

                fn $_func(self, rhs: Vector3<$_type>) -> Vector3<$_type> {
                    Vector3 {
                        x: $_trait::$_func(self, rhs.x),
                        y: $_trait::$_func(self, rhs.y),
                        z: $_trait::$_func(self, rhs.z)
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

impl<T: Add> Add for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn add(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign for Vector3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.x += rhs.y;
        self.x += rhs.z;
    }
}

impl<T: Sub> Sub for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn sub(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign for Vector3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.x -= rhs.y;
        self.x -= rhs.z;
    }
}

impl<T: Mul> Mul for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn mul(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T: Div> Div for Vector3<T> {
    type Output = Vector3<T::Output>;

    fn div(self, rhs: Vector3<T>) -> Vector3<T::Output> {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_from() {
        macro_rules! test_try_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Vector3<$from> = Vector3 { x: 0, y: 0, z: 0 };
                let to: Result<Vector3<$to>, TryFromIntError> = from.try_into();
                assert!(to.is_ok());

                let from: Vector3<$from> = Vector3 { x: <$from>::MAX, y: <$from>::MIN, z: 0 };
                let to: Result<Vector3<$to>, TryFromIntError> = from.try_into();
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
    }

    #[test]
    fn from() {
        macro_rules! test_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Vector3<$from> = Vector3 { x: 0, y: 0, z: 0 };
                let _to: Vector3<$to> = from.into();
                let from: Vector3<$from> = Vector3 { x: <$from>::MAX, y: <$from>::MIN, z: 0 };
                let _to: Vector3<$to> = from.into();
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
    }

    #[test]
    fn lossy_from() {
        macro_rules! test_lossy_froms {
            ($( $from:ty, $to:ty);*) => {$(
                let from: Vector3<$from> = Vector3{ x: <$from>::MAX, y: <$from>::MIN, z: <$from>::MAX};
                let _to: Vector3<$to> = from.lossy_into();
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
