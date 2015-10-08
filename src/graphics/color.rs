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

use csfml_graphics_sys as ffi;

/// Utility class for manpulating RGBA colors
///
/// sfColor is a simple color class composed of 4 components: Red, Green, Blue, Alpha
#[repr(C)]
pub struct Color(pub ffi::sfColor);

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
        Color(ffi::sfColor {
            red: red,
            green: green,
            blue: blue,
            alpha: 255
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
    pub fn new_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
        Color(ffi::sfColor {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        })
    }

    /// Add two colors
    ///
    /// # Arguments
    /// * color1 - The first color
    /// * color2 - The second color
    ///
    /// Return the component-wise saturated addition of the two colors
    pub fn add(color1: Color, color2: Color) -> Color {
        Color(unsafe {ffi::sfColor_add(color1.0, color2.0)})
    }

    /// Modulate two colors
    ///
    /// # Arguments
    /// * color1 - The first color
    /// * color2 - The second color
    ///
    /// Return the component-wise multiplication of the two colors
    pub fn modulate(color1: Color, color2: Color) -> Color {
        Color(unsafe {ffi::sfColor_modulate(color1.0, color2.0)})
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

    fn add(self, other: Color) -> Color {
        let r: i32 = self.0.red as i32 + other.0.red as i32;
        let g: i32 = self.0.green as i32 + other.0.green as i32;
        let b: i32 = self.0.blue as i32 + other.0.blue as i32;
        let a: i32 = self.0.alpha as i32 + other.0.alpha as i32;
        Color(ffi::sfColor {
            red: if r > 255 {255} else {r as u8},
            green: if g > 255 {255} else {g as u8},
            blue: if b > 255 {255} else {b as u8},
            alpha: if a > 255 {255} else {a as u8}
        })
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        let r: i32 = self.0.red as i32 * (other.0.red as i32);
        let g: i32 = self.0.green as i32 * (other.0.green as i32);
        let b: i32 = self.0.blue as i32 * (other.0.blue as i32);
        let a: i32 = self.0.alpha as i32 * (other.0.alpha as i32);
        Color(ffi::sfColor {
            red: if r > 255 {255} else {r as u8},
            green: if g > 255 {255} else {g as u8},
            blue: if b > 255 {255} else {b as u8},
            alpha: if a > 255 {255} else {a as u8}
        })
    }
}
