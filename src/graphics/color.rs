/*!
* Utility class for manpulating RGBA colors
*
* Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha
*
*/

use rsfml::Wrapper;

#[doc(hidden)]
pub mod csfml {
    
    pub struct sfColor {
        red : u8,
        green : u8,
        blue : u8,
        alpha : u8
    }
    
    pub extern "C" {
        fn sfColor_fromRGB(red : u8, green : u8, blue : u8) -> sfColor;
        fn sfColor_fromRGBA(red : u8, green : u8, blue : u8, alpha : u8) -> sfColor;
        fn sfColor_add(color1 : sfColor, color2 : sfColor) -> sfColor;
        fn sfColor_modulate(color1 : sfColor, color2 : sfColor) -> sfColor;
    }
}

#[doc(hidden)]
pub struct Color {
    priv color : csfml::sfColor
}

impl Color {

    /**
    * Construct a color from its 3 RGB components
    */
    pub fn new_from_RGB(red : u8, green : u8, blue : u8) -> Color {
        Color { color : csfml::sfColor{red : red, green : green, blue : blue, alpha : 255}}
    }

    /**
    * Construct a color from its 4 RGBA components
    */
    pub fn new_from_RGBA(red : u8, green : u8, blue : u8, alpha : u8) -> Color {
        Color { color : csfml::sfColor {red : red, green : green, blue : blue, alpha : alpha}}
    }

    /**
    * Add two colors
    */
    pub fn add(color1 : Color, color2 : Color) -> Color {
        Color { color : unsafe {csfml::sfColor_add(color1.unwrap_color(), color2.unwrap_color())}}
    }

    /**
    * Modulate two colors
    */
    pub fn modulate(color1 : Color, color2 : Color) -> Color {
        Color { color : unsafe {csfml::sfColor_modulate(color1.unwrap_color(), color2.unwrap_color())}}
    }
    
    /// Black predefined color
    pub fn black() -> Color {
        Color { color : csfml::sfColor {red : 0, green : 0, blue : 0, alpha : 255}}
    }
    
    /// White predefined color
    pub fn white() -> Color {
        Color { color : csfml::sfColor {red : 255, green : 255, blue : 255, alpha : 255}}
    }
    
    /// Red predefined color
    pub fn red() -> Color {
        //Color { color : unsafe {csfml::sfColor_fromRGB(255, 0, 0)}}
        Color { color : csfml::sfColor {red : 255, green : 0, blue : 0, alpha : 255}}
    }
    
    /// Green predefined color
    pub fn green() -> Color {
        Color { color : csfml::sfColor {red : 0, green : 255, blue : 0, alpha : 255}}
    }
    
    /// Blue predefined color
    pub fn blue() -> Color {
        Color { color : csfml::sfColor {red : 0, green : 0, blue : 255, alpha : 255}}
    }
    
    #[doc(hidden)]
    pub fn wrap_color(color : csfml::sfColor) -> Color {
        Color { color : color}
    }

    #[doc(hidden)]
    pub fn unwrap_color(&self) -> csfml::sfColor {
        self.color
    }
}