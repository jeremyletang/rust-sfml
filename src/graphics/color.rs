use crate::graphics::csfml_graphics_sys as ffi;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Utility type for manpulating RGBA colors
///
/// `Color` is a simple color type composed of 4 components: Red, Green, Blue, Alpha
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color {
    /// Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
    /// Alpha (opacity) component.
    pub a: u8,
}

impl Color {
    /// Construct a color from its 3 RGB components
    ///
    /// # Arguments
    /// * red - Red component   (0 .. 255)
    /// * green - -Green component (0 .. 255)
    /// * blue - Blue component  (0 .. 255)
    ///
    /// Return Color object constructed from the components
    #[must_use]
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: 255,
        }
    }

    /// Construct a color from its 4 RGBA components
    ///
    /// # Arguments
    /// * red - Red component   (0 .. 255)
    /// * green - -Green component (0 .. 255)
    /// * blue - Blue component  (0 .. 255)
    /// * alpha - Alpha component  (0 .. 255)
    ///
    /// Return Color object constructed from the components
    #[must_use]
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    pub(super) fn raw(self) -> ffi::sfColor {
        unsafe { ::std::mem::transmute(self) }
    }

    pub(super) unsafe fn from_raw(raw: ffi::sfColor) -> Self {
        ::std::mem::transmute(raw)
    }

    /// Black predefined color
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    /// White predefined color
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    /// Red predefined color
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    /// Green predefined color
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };

    /// Blue predefined color
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    /// Yellow predefined color
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };

    /// Magenta predefined color
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };

    /// Cyan predifined color
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };

    /// Tranparent predefined color
    pub const TRANSPARENT: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
}

impl From<u32> for Color {
    /// Construct the color from 32-bit unsigned integer.
    ///
    /// The number should contain the components in RGBA order.
    fn from(src: u32) -> Self {
        unsafe { Color::from_raw(ffi::sfColor_fromInteger(src)) }
    }
}

impl Into<u32> for Color {
    /// Retrieve the color as a 32-bit unsigned integer.
    fn into(self) -> u32 {
        unsafe { ffi::sfColor_toInteger(self.raw()) }
    }
}

impl Add for Color {
    type Output = Color;

    /// Calculate the component-wise saturated addition of two colors.
    fn add(self, other: Color) -> Color {
        unsafe { Color::from_raw(ffi::sfColor_add(self.raw(), other.raw())) }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Color {
    type Output = Self;

    /// Component-wise subtraction of two colors. Components below 0 are clamped to 0.
    fn sub(self, other: Self) -> Self {
        unsafe { Self::from_raw(ffi::sfColor_subtract(self.raw(), other.raw())) }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for Color {
    type Output = Color;

    /// Calculate the component-wise modulated multiplication of two colors.
    ///
    /// For each `X` in `rgba`, `result.X = a.X * b.X / 255`.
    fn mul(self, other: Color) -> Color {
        unsafe { Color::from_raw(ffi::sfColor_modulate(self.raw(), other.raw())) }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
