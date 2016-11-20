// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Utility class for manpulating RGBA colors
//!
//! Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha

use std::ops::{Add, Mul};

use csfml_graphics_sys as ffi;

use raw_conv::{Raw, FromRaw};

/// Utility class for manpulating RGBA colors
///
/// sfColor is a simple color class composed of 4 components: Red, Green, Blue, Alpha
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color {
    /// 	Red component.
    pub r: u8,
    /// Green component.
    pub g: u8,
    /// Blue component.
    pub b: u8,
    /// Alpha (opacity) component.
    pub a: u8,
}

impl Raw for Color {
    type Raw = ffi::sfColor;

    fn raw(&self) -> Self::Raw {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for Color {
    fn from_raw(src: Self::Raw) -> Self {
        unsafe { ::std::mem::transmute(src) }
    }
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
    pub fn new_rgb(red: u8, green: u8, blue: u8) -> Color {
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
    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color {
            r: red,
            g: green,
            b: blue,
            a: alpha,
        }
    }

    /// Black predefined color
    pub fn black() -> Color {
        Color::new_rgb(0, 0, 0)
    }

    /// White predefined color
    pub fn white() -> Color {
        Color::new_rgb(255, 255, 255)
    }

    /// Red predefined color
    pub fn red() -> Color {
        Color::new_rgb(255, 0, 0)
    }

    /// Green predefined color
    pub fn green() -> Color {
        Color::new_rgb(0, 255, 0)
    }

    /// Blue predefined color
    pub fn blue() -> Color {
        Color::new_rgb(0, 0, 255)
    }

    /// Yellow predefined color
    pub fn yellow() -> Color {
        Color::new_rgb(255, 255, 0)
    }

    /// Magenta predefined color
    pub fn magenta() -> Color {
        Color::new_rgb(255, 0, 255)
    }

    /// Cyan predifined color
    pub fn cyan() -> Color {
        Color::new_rgb(0, 255, 255)
    }

    /// Tranparent predefined color
    pub fn transparent() -> Color {
        Color::new_rgba(0, 0, 0, 0)
    }
}

impl Add for Color {
    type Output = Color;

    /// Calculate the component-wise saturated addition of two colors.
    fn add(self, other: Color) -> Color {
        Color::from_raw(unsafe { ffi::sfColor_add(self.raw(), other.raw()) })
    }
}

impl Mul for Color {
    type Output = Color;

    /// Calculate the component-wise modulated multiplication of two colors.
    ///
    /// For each `X` in `rgba`, `result.X = a.X * b.X / 255`.
    fn mul(self, other: Color) -> Color {
        Color::from_raw(unsafe { ffi::sfColor_modulate(self.raw(), other.raw()) })
    }
}
