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
*
*/

#[doc(hidden)]
pub mod csfml {
    
    use graphics::color::Color;
    
    pub extern "C" {
        fn sfColor_fromRGB(red : u8, green : u8, blue : u8) -> Color;
        fn sfColor_fromRGBA(red : u8, green : u8, blue : u8, alpha : u8) -> Color;
        fn sfColor_add(color1 : Color, color2 : Color) -> Color;
        fn sfColor_modulate(color1 : Color, color2 : Color) -> Color;
    }
}

#[doc(hidden)]
pub struct Color {
    red : u8,
    green : u8,
    blue : u8,
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
    pub fn new_from_RGB(red : u8, green : u8, blue : u8) -> Color {
        Color { red : red, green : green, blue : blue, alpha : 255}
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
    pub fn new_from_RGBA(red : u8, green : u8, blue : u8, alpha : u8) -> Color {
        Color { red : red, green : green, blue : blue, alpha : alpha}
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
        unsafe {csfml::sfColor_add(color1, color2)}
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
        unsafe {csfml::sfColor_modulate(color1, color2)}
    }
    
    /// Black predefined color
    pub fn black() -> Color {
        Color::new_from_RGB(0, 0, 0)
    }
    
    /// White predefined color
    pub fn white() -> Color {
        Color::new_from_RGB(255, 255, 255)
    }
    
    /// Red predefined color
    pub fn red() -> Color {
        Color::new_from_RGB(255, 0, 0)
    }
    
    /// Green predefined color
    pub fn green() -> Color {
        //Color { color : csfml::sfColor {red : 0, green : 255, blue : 0, alpha : 255}}
        Color::new_from_RGB(0, 255, 0)
    }
   
    /// Blue predefined color
    pub fn blue() -> Color {
        Color::new_from_RGB(0, 0, 255)
    }
}