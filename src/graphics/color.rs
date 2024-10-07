use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Utility type for manpulating RGBA colors
///
/// `Color` is a simple color type composed of 4 components: Red, Green, Blue, Alpha
///
/// # Example
///
/// There are 3 basic ways to construct a color
///
/// ```
/// use sfml::graphics::Color;
/// let color1 = Color::rgb(255, 0, 0); // from red/green/blue values
/// let color2 = Color::rgba(255, 255, 255, 128); // from red/green/blue/alpha (transparency)
/// let color3 = Color::GREEN; // from one of the associated color constants
/// ```
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component (transparency)
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
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
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
    pub const fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
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
    fn from(src: u32) -> Self {
        Self {
            r: ((src & 0xff00_0000) >> 24) as u8,
            g: ((src & 0x00ff_0000) >> 16) as u8,
            b: ((src & 0x0000_ff00) >> 8) as u8,
            a: (src & 0x0000_00ff) as u8,
        }
    }
}

impl From<Color> for u32 {
    fn from(src: Color) -> Self {
        (u32::from(src.r) << 24)
            | (u32::from(src.g) << 16)
            | (u32::from(src.b) << 8)
            | u32::from(src.a)
    }
}

impl Add for Color {
    type Output = Self;

    /// Calculate the component-wise saturated addition of two colors.
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
            a: self.a.saturating_add(other.a),
        }
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
        Self {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
            a: self.a.saturating_sub(other.a),
        }
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
    #[expect(clippy::cast_possible_truncation)]
    fn mul(self, other: Color) -> Color {
        let (r1, r2) = (u16::from(self.r), u16::from(other.r));
        let (g1, g2) = (u16::from(self.g), u16::from(other.g));
        let (b1, b2) = (u16::from(self.b), u16::from(other.b));
        let (a1, a2) = (u16::from(self.a), u16::from(other.a));
        Self {
            r: (r1 * r2 / 255) as u8,
            g: (g1 * g2 / 255) as u8,
            b: (b1 * b2 / 255) as u8,
            a: (a1 * a2 / 255) as u8,
        }
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
