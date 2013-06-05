/*!
* Graphical text
*
* Text is a drawable class that allows to easily display some text with custom style and color on a render target.
*
*/

use graphics::drawable::*;
use graphics::render_window;
use system::vector2;
use graphics::font;
use graphics::color::Color;
use core::libc::{c_float, c_uint, size_t};
use graphics::rect::FloatRect;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_uint, c_float, c_void, c_char, size_t};
    use graphics::transform;
    use system::vector2;
    use graphics::font;
    use graphics::color;
    use graphics::rect::FloatRect;

    pub struct sfText {
        This : c_void,
        font : *c_void,
        transform : transform::csfml::sfTransform,
        transform2 : transform::csfml::sfTransform
    }

    pub extern "C" {
        fn sfText_create() -> *sfText;
        fn sfText_copy(text : *sfText) -> *sfText;
        fn sfText_destroy(text : *sfText) -> ();
        fn sfText_setPosition(text : *sfText, position : vector2::Vector2f) -> ();
        fn sfText_setRotation(text : *sfText, angle : c_float) -> ();
        fn sfText_setScale(text : *sfText, scale : vector2::Vector2f) -> ();
        fn sfText_setOrigin(text : *sfText, origin : vector2::Vector2f) -> ();
        fn sfText_getPosition(text : *sfText) -> vector2::Vector2f;
        fn sfText_getRotation(text : *sfText) -> c_float;
        fn sfText_getScale(text : *sfText) -> vector2::Vector2f;
        fn sfText_getOrigin(text : *sfText) -> vector2::Vector2f;
        fn sfText_move(text : *sfText, offset : vector2::Vector2f) -> ();
        fn sfText_rotate(text : *sfText, angle : c_float) -> ();
        fn sfText_scale(text : *sfText, factors : vector2::Vector2f) -> ();
        // fn sfText_getTransform(text : *sfText) -> sfTransform;
        // fn sfText_getInverseTransform(text : *sfText) -> sfTransform;
        fn sfText_setString(text : *sfText, string : *c_char) -> ();
        fn sfText_setUnicodeString(text : *sfText, string : *u32 ) -> ();
        fn sfText_setFont(text : *sfText, font : *font::csfml::sfFont) -> ();
        fn sfText_setCharacterSize(text : *sfText, size : c_uint) -> ();
        fn sfText_setStyle(text : *sfText, style : u32) -> ();
        fn sfText_setColor(text : *sfText, color : color::Color) -> ();
        fn sfText_getString(text : *sfText) -> *c_char;
        //fn sfText_getUnicodeString(text : *sfText) -> *u32;
        fn sfText_getFont(text : *sfText) -> *font::csfml::sfFont;
        fn sfText_getCharacterSize(text : *sfText) -> c_uint;
        fn sfText_getStyle(text : *sfText) -> u32;
        fn sfText_getColor(text : *sfText) -> color::Color;
        fn sfText_findCharacterPos(text : *sfText, index : size_t) -> vector2::Vector2f;
        fn sfText_getLocalBounds(text : *sfText) -> FloatRect;
        fn sfText_getGlobalBounds(text : *sfText) -> FloatRect;
    }
}

pub enum Style {
    Regular = 0,
    Bold = 1,
    Italic = 2,
    Underlined = 4
}

pub struct Text {
    priv text : *csfml::sfText
}

impl Text {
    /**
    * Create a new text
    */
    pub fn new() -> Option<Text> {
        let text : *csfml::sfText;
        unsafe {text = csfml::sfText_create()};
        if text == ptr::null() {
            None
        }
        else {
            Some(Text {text : text})
        }
    }
    
    /**
    * Copy an existing text
    */
    pub fn set_string(&self, string : ~str) -> () {
        do str::as_c_str(string) |cstring| {
            unsafe {csfml::sfText_setString(self.text, cstring)}
        };
    }

    /**
    * Get the string of a text (returns an ANSI string)
    */
    pub fn get_string(&self) -> ~str {
        unsafe {
            str::raw::from_c_str(csfml::sfText_getString(self.text))
        }
    }

    /**
    * Set the size of the characters
    */
    pub fn get_character_size(&self) -> uint {
        unsafe { csfml::sfText_getCharacterSize(self.text) as uint}
    }
    
    /**
    * Set the font of the text
    */
    pub fn set_font(&self, font : &font::Font) -> () {
        unsafe {
            csfml::sfText_setFont(self.text, font.unwrap())
        }
    }
    
    /**
    * Set the orientation of a text
    */
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfText_setRotation(self.text, angle as c_float)
        }
    }
    
    /**
    * Get the orientation of a text
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfText_getRotation(self.text) as float
        }
    }
    
    /**
    * Rotate a text
    */
    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfText_rotate(self.text, angle as c_float)
        }
    }

    /**
    * Set the style of a text
    */
    pub fn set_style(&self, style : Style) -> () {
        unsafe {
            csfml::sfText_setStyle(self.text, style as u32)
        }
    }
    
    /**
    * Get the size of the characters of a text
    */
    pub fn set_character_size(&self, size : uint) -> () {
        unsafe {
            csfml::sfText_setCharacterSize(self.text, size as c_uint)
        }
    }

    /**
    * Get the style of a text
    */
    pub fn get_style(&self) -> Style {
        match unsafe {csfml::sfText_getStyle(self.text)} {
            0 => Regular,
            1 => Bold,
            2 => Italic,
            _ => Underlined
        }
    }
    
    /**
    * Get the font of a text
    */
    pub fn get_font(&self) -> font::Font {
        unsafe {
            font::Font::wrap(csfml::sfText_getFont(self.text))
        }
    }
    
    /**
    * Set the global color of a text
    */
    pub fn set_color(&self, color : &Color) -> () {
        unsafe {
            csfml::sfText_setColor(self.text, *color)
        }
    }
    
    /**
    * Get the global color of a text
    */
    pub fn get_color(&self) -> Color {
        unsafe {
            csfml::sfText_getColor(self.text)
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfText_scale(self.text, *factors)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfText_setScale(self.text, *scale)
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfText_move(self.text, *offset)
        }
    }

    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfText_setPosition(self.text, *position)
        }
    }

    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfText_setOrigin(self.text, *origin)
        }
    }
    
    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe {csfml::sfText_getScale(self.text)}
    }

    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe {csfml::sfText_getOrigin(self.text)}
    }

    pub fn find_character_pos(&self, index : u64) -> vector2::Vector2f {
        unsafe {csfml::sfText_findCharacterPos(self.text, index as size_t)}
    }

    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe {csfml::sfText_getPosition(self.text)}
    }

    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfText_getLocalBounds(self.text)
        }
    }

    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfText_getGlobalBounds(self.text)
        }
    }

    pub fn set_unicode_string(&self, string : ~[u32]) -> () {
        unsafe {
            csfml::sfText_setUnicodeString(self.text, vec::raw::to_ptr(string) )
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfText {
        self.text
    }
    
}

#[doc(hidden)]
impl Drawable for Text {
    pub fn draw_in_render_window(&self, renderWindow : &render_window::RenderWindow) -> () {
        renderWindow.draw_text(self)
    }
}

impl Drop for Text {
    /**
    *   Destructor for class Text. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfText_destroy(self.text);
        }
    }
}
