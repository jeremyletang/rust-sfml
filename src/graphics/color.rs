/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Utility class for manpulating RGBA colors
//!
//! Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha

use std::ops::{Add, Mul};

/// Utility class for manpulating RGBA colors
///
/// Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct Color {
    /// The red composant of the color
    pub red: u8,
    /// The green composant of the color
    pub green: u8,
    /// The blue composant of the color
    pub blue: u8,
    /// The alpha composant of the color
    pub alpha: u8
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
            red: red,
            green: green,
            blue: blue,
            alpha: 255
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
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
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
        let red = self.red as u16 + other.red as u16;
		let green = self.green as u16 + other.green as u16;
		let blue = self.blue as u16 + other.blue as u16;
		let alpha = self.alpha as u16 + other.alpha as u16;
		Color::new_rgba(
			if red > 255 { 255 } else { red as u8 },
			if green > 255 { 255 } else { green as u8 },
			if blue > 255 { 255 } else { blue as u8 },
			if alpha > 255 { 255 } else { alpha as u8 },
		)
    }
}

impl Mul for Color {
    type Output = Color;

    /// Calculate the component-wise modulated multiplication of two colors.
    ///
    /// For each `X` in `rgba`, `result.X = a.X * b.X / 255`.
    fn mul(self, other: Color) -> Color {
        let red = self.red as u16 * other.red as u16 / 255;
		let green = self.green as u16 * other.green as u16 / 255;
		let blue = self.blue as u16 * other.blue as u16 / 255;
		let alpha = self.alpha as u16 * other.alpha as u16 / 255;
		Color::new_rgba(red as u8, green as u8, blue as u8, alpha as u8)
    }
}
