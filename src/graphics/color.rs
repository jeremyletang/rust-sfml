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
    */
    pub fn new_from_RGB(red : u8, green : u8, blue : u8) -> Color {
        Color { red : red, green : green, blue : blue, alpha : 255}
    }

    /**
    * Construct a color from its 4 RGBA components
    */
    pub fn new_from_RGBA(red : u8, green : u8, blue : u8, alpha : u8) -> Color {
        Color { red : red, green : green, blue : blue, alpha : alpha}
    }

    /**
    * Add two colors
    */
    pub fn add(color1 : Color, color2 : Color) -> Color {
        unsafe {csfml::sfColor_add(color1, color2)}
    }

    /**
    * Modulate two colors
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