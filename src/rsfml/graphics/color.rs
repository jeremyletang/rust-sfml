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

/*!
 * Utility class for manpulating RGBA colors
 *
 * Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha
 */

use ffi = ffi::graphics::color;

/**
 * Utility class for manpulating RGBA colors
 *
 * Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha
 */
#[deriving(Clone, Ord, Show, ToStr)]
pub struct Color {
    /// The red composant of the color
    red :   u8,
    /// The green composant of the color
    green : u8,
    /// The blue composant of the color
    blue :  u8,
    /// The alpha composant of the color
    alpha : u8
}

impl Color {

    /**
     * Construct a color from its 3 RGB components
     *
     * # Arguments
     * * red - Red component   (0 .. 255)
     * * green - -Green component (0 .. 255)
     * * blue - Blue component  (0 .. 255)
     *
     * Return Color object constructed from the components
     */
    pub fn new_RGB(red : u8, green : u8, blue : u8) -> Color {
        Color {
            red :   red,
            green : green,
            blue :  blue,
            alpha : 255
        }
    }

    /**
     * Construct a color from its 4 RGBA components
     *
     * # Arguments
     * * red - Red component   (0 .. 255)
     * * green - -Green component (0 .. 255)
     * * blue - Blue component  (0 .. 255)
     * * alpha - Alpha component  (0 .. 255)
     *
     * Return Color object constructed from the components
     */
    pub fn new_RGBA(red : u8, green : u8, blue : u8, alpha : u8) -> Color {
        Color {
            red :   red,
            green : green,
            blue :  blue,
            alpha : alpha
        }
    }

    /**
     * Add two colors
     *
     * # Arguments
     * * color1 - The first color
     * * color2 - The second color
     *
     * Return the component-wise saturated addition of the two colors
     */
    pub fn add(color1 : Color, color2 : Color) -> Color {
        unsafe {ffi::sfColor_add(color1, color2)}
    }

    /**
     * Modulate two colors
     *
     * # Arguments
     * * color1 - The first color
     * * color2 - The second color
     *
     * Return the component-wise multiplication of the two colors
     */
    pub fn modulate(color1 : Color, color2 : Color) -> Color {
        unsafe {ffi::sfColor_modulate(color1, color2)}
    }

    /// Black predefined color
    pub fn black() -> Color {
        Color::new_RGB(0, 0, 0)
    }

    /// White predefined color
    pub fn white() -> Color {
        Color::new_RGB(255, 255, 255)
    }

    /// Red predefined color
    pub fn red() -> Color {
        Color::new_RGB(255, 0, 0)
    }

    /// Green predefined color
    pub fn green() -> Color {
        Color::new_RGB(0, 255, 0)
    }

    /// Blue predefined color
    pub fn blue() -> Color {
        Color::new_RGB(0, 0, 255)
    }

    /// Yellow predefined color
    pub fn yellow() -> Color {
        Color::new_RGB(255, 255, 0)
    }

    /// Magenta predefined color
    pub fn magenta() -> Color {
        Color::new_RGB(255, 0, 255)
    }

    /// Cyan predifined color
    pub fn cyan() -> Color {
        Color::new_RGB(0, 255, 255)
    }

    /// Tranparent predefined color
    pub fn transparent() -> Color {
        Color::new_RGBA(0, 0, 0, 0)
    }

}

impl Add<Color, Color> for Color {
    fn add(&self, other : &Color) -> Color {
        let r : i32 = self.red as i32 + other.red as i32;
        let g : i32 = self.green as i32 + other.green as i32;
        let b : i32 = self.blue as i32 + other.blue as i32;
        let a : i32 = self.alpha as i32 + other.alpha as i32;
        Color {
            red :   if r > 255 {255} else {r as u8},
            green : if g > 255 {255} else {g as u8},
            blue :  if b > 255 {255} else {b as u8},
            alpha : if a > 255 {255} else {a as u8}
        }
    }
}

impl Mul<Color, Color> for Color {
    fn mul(&self, other : &Color) -> Color {
        let r : i32 = self.red as i32 * (other.red as i32);
        let g : i32 = self.green as i32 * (other.green as i32);
        let b : i32 = self.blue as i32 * (other.blue as i32);
        let a : i32 = self.alpha as i32 * (other.alpha as i32);
        Color {
            red :   if r > 255 {255} else {r as u8},
            green : if g > 255 {255} else {g as u8},
            blue :  if b > 255 {255} else {b as u8},
            alpha : if a > 255 {255} else {a as u8}
        }
    }
}

impl Eq for Color {
    fn eq(&self, other : &Color) -> bool {
        self.red == other.red &&
            self.green == other.green &&
            self.blue == other.blue &&
            self.alpha == other.alpha
    }
    fn ne(&self, other : &Color) -> bool {
        self.red != other.red &&
            self.green != other.green &&
            self.blue != other.blue &&
            self.alpha != other.alpha
    }
}
