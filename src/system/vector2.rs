use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
/// - `Vector2<f32>` is `Vector2f`
/// - `Vector2<i32>` is `Vector2i`
/// - `Vector2<u32>` is `Vector2u`
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
/// Note: for 3-dimensional vectors, see `Vector3`.
#[repr(C)]
#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Copy, Default)]
pub struct Vector2<T> {
    /// X coordinate of the vector.
    pub x: T,
    /// Y coordinate of the vector.
    pub y: T,
}

/// `Vector2` with `i32` coordinates.
pub type Vector2i = Vector2<i32>;
/// `Vector2` with `u32` coordinates.
pub type Vector2u = Vector2<u32>;
/// `Vector2` with `f32` coordinates.
pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> {
    /// Creates a new vector from its coordinates.
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x: x, y: y }
    }
}

impl<T> From<(T, T)> for Vector2<T> {
    /// Constructs a `Vector2` from `(x, y)`.
    fn from(src: (T, T)) -> Self {
        Self { x: src.0, y: src.1 }
    }
}

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

impl<T: MulAssign + Copy> MulAssign<Vector2<T>> for Vector2<T> {
    fn mul_assign(&mut self, rhs: Vector2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
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

impl<T: DivAssign + Copy> DivAssign<Vector2<T>> for Vector2<T> {
    fn div_assign(&mut self, rhs: Vector2<T>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
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

impl Vector2i {
    pub(crate) fn raw(&self) -> ::csfml_system_sys::sfVector2i {
        ::csfml_system_sys::sfVector2i {
            x: self.x,
            y: self.y,
        }
    }
    pub(crate) fn from_raw(raw: ::csfml_system_sys::sfVector2i) -> Self {
        Self { x: raw.x, y: raw.y }
    }
}

impl Vector2u {
    pub(crate) fn raw(&self) -> ::csfml_system_sys::sfVector2u {
        ::csfml_system_sys::sfVector2u {
            x: self.x,
            y: self.y,
        }
    }
    pub(crate) fn from_raw(raw: ::csfml_system_sys::sfVector2u) -> Self {
        Self { x: raw.x, y: raw.y }
    }
}

impl Vector2f {
    pub(crate) fn raw(&self) -> ::csfml_system_sys::sfVector2f {
        ::csfml_system_sys::sfVector2f {
            x: self.x,
            y: self.y,
        }
    }
    pub(crate) fn from_raw(raw: ::csfml_system_sys::sfVector2f) -> Self {
        Self { x: raw.x, y: raw.y }
    }
}
