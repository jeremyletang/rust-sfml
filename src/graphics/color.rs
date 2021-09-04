use crate::ffi;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Utility type for manpulating RGBA colors
///
/// `Color` is a simple color type composed of 4 components: Red, Green, Blue, Alpha
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color(pub(super) ffi::sfColor);

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
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self(ffi::sfColor {
            r: red,
            g: green,
            b: blue,
            a: 255,
        })
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
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(ffi::sfColor {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        })
    }

    /// The red component of this color
    #[must_use]
    pub const fn red(&self) -> u8 {
        self.0.r
    }

    /// Mutable reference to the red component
    #[must_use]
    pub fn red_mut(&mut self) -> &mut u8 {
        &mut self.0.r
    }

    /// The green component of this color
    #[must_use]
    pub const fn green(&self) -> u8 {
        self.0.g
    }

    /// Mutable reference to the green component
    #[must_use]
    pub fn green_mut(&mut self) -> &mut u8 {
        &mut self.0.g
    }

    /// The blue component of this color
    #[must_use]
    pub const fn blue(&self) -> u8 {
        self.0.b
    }

    /// Mutable reference to the blue component
    #[must_use]
    pub fn blue_mut(&mut self) -> &mut u8 {
        &mut self.0.b
    }

    /// The alpha component of this color
    #[must_use]
    pub const fn alpha(&self) -> u8 {
        self.0.a
    }

    /// Mutable reference to the alpha component
    #[must_use]
    pub fn alpha_mut(&mut self) -> &mut u8 {
        &mut self.0.a
    }

    /// Black predefined color
    pub const BLACK: Self = Self::rgb(0, 0, 0);

    /// White predefined color
    pub const WHITE: Self = Self::rgb(255, 255, 255);

    /// Red predefined color
    pub const RED: Self = Self::rgb(255, 0, 0);

    /// Green predefined color
    pub const GREEN: Self = Self::rgb(0, 255, 0);

    /// Blue predefined color
    pub const BLUE: Self = Self::rgb(0, 0, 255);

    /// Yellow predefined color
    pub const YELLOW: Self = Self::rgb(255, 255, 0);

    /// Magenta predefined color
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);

    /// Cyan predifined color
    pub const CYAN: Self = Self::rgb(0, 255, 255);

    /// Tranparent predefined color
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
}

impl From<u32> for Color {
    /// Construct the color from 32-bit unsigned integer.
    ///
    /// The number should contain the components in RGBA order.
    #[allow(clippy::cast_possible_truncation)]
    fn from(src: u32) -> Self {
        Self(ffi::sfColor {
            r: ((src & 0xff000000) >> 24) as u8,
            g: ((src & 0x00ff0000) >> 16) as u8,
            b: ((src & 0x0000ff00) >> 8) as u8,
            a: (src & 0x000000ff) as u8,
        })
    }
}

impl From<Color> for u32 {
    fn from(src: Color) -> Self {
        ((src.0.r as u32) << 24)
            | ((src.0.g as u32) << 16)
            | ((src.0.b as u32) << 8)
            | (src.0.a as u32)
    }
}

impl Add for Color {
    type Output = Color;

    /// Calculate the component-wise saturated addition of two colors.
    fn add(self, other: Color) -> Color {
        Color(ffi::sfColor {
            r: self.0.r.saturating_add(other.0.r),
            g: self.0.g.saturating_add(other.0.g),
            b: self.0.b.saturating_add(other.0.b),
            a: self.0.a.saturating_add(other.0.a),
        })
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
        Color(ffi::sfColor {
            r: self.0.r.saturating_sub(other.0.r),
            g: self.0.g.saturating_sub(other.0.g),
            b: self.0.b.saturating_sub(other.0.b),
            a: self.0.a.saturating_sub(other.0.a),
        })
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
    #[allow(clippy::cast_possible_truncation)]
    fn mul(self, other: Color) -> Color {
        let (r1, r2) = (self.0.r as u16, other.0.r as u16);
        let (g1, g2) = (self.0.g as u16, other.0.g as u16);
        let (b1, b2) = (self.0.b as u16, other.0.b as u16);
        let (a1, a2) = (self.0.a as u16, other.0.a as u16);
        Self(ffi::sfColor {
            r: (r1 * r2 / 255) as u8,
            g: (g1 * g2 / 255) as u8,
            b: (b1 * b2 / 255) as u8,
            a: (a1 * a2 / 255) as u8,
        })
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
