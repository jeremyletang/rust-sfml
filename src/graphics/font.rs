/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
* Class for loading and manipulating character fonts
*
* 
*
*/

use std::libc::{c_uint};
use graphics::texture::Texture;
use graphics::glyph::Glyph;
use std::str;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void, c_char, c_uint, c_int};
    use graphics::texture;
    use rsfml::sfTypes::sfBool;
    use graphics::glyph::Glyph;

    pub struct sfFont {
        This : *c_void
    }

    pub extern "C" {
        fn sfFont_createFromFile(filename : *c_char) -> *sfFont;
        fn sfFont_copy(font : *sfFont) -> *sfFont;
        // fn sfFont_createFromMemory(data : *c_void, sizeInBytes : size_t) -> *sfFont;
        // fn sfFont_createFromStream(stream : *sfInputStream) -> *sfFont;
        fn sfFont_destroy(font : *sfFont) -> ();
        fn sfFont_getGlyph(font : *sfFont, codepoint : u32, characterSize : c_uint, bold :sfBool) -> Glyph;
        fn sfFont_getKerning(font : *sfFont, first : u32, second : u32, characterSize : c_uint) -> c_int;
        fn sfFont_getLineSpacing(font : *sfFont, characterSize : c_uint) -> c_int;
        fn sfFont_getTexture(font : *sfFont, characterSize : c_uint) -> *texture::csfml::sfTexture;
    }
}

#[doc(hidden)]
pub struct Font {
    priv font : *csfml::sfFont
}

impl Font {
    /**
    * Create a new font from a file
    */
    pub fn new_from_file(filename : ~str) -> Font {
        do str::as_c_str(filename) |filenamebuf| {
            unsafe {
            Font { font : csfml::sfFont_createFromFile(filenamebuf)}
            }
        }
    }
    
    /**
    * Create font from a existing one
    */
    pub fn new_copy(font : &Font) -> Font {
        unsafe {
            Font { font : csfml::sfFont_copy(font.unwrap())}
        }
    }    
    /**
    * Get the kerning value corresponding to a given pair of characters in a font
    */
    pub fn get_kerning(&self, first : u32, second : u32, characterSize : uint) -> int {
        unsafe {
            csfml::sfFont_getKerning(self.font, first, second, characterSize as c_uint) as int
        }
    }

    /**
    * Get the line spacing value
    */
    pub fn get_line_spacing(&self, characterSize : uint) -> int {
        unsafe {
            csfml::sfFont_getLineSpacing(self.font, characterSize as c_uint) as int
        }
    }

    /**
    * Get the texture containing the glyphs of a given size in a font
    */
    pub fn get_texture(&self, characterSize : uint) -> Texture {
        unsafe {
            Texture::wrap(csfml::sfFont_getTexture(self.font, characterSize as c_uint))
        }
    }
    
    pub fn get_glyph(&self, codepoint : u32, characterSize : uint, bold : bool) -> Glyph {
        match bold {
            true        => unsafe {csfml::sfFont_getGlyph(self.font, codepoint, characterSize as c_uint, 1)},
            false       => unsafe {csfml::sfFont_getGlyph(self.font, codepoint, characterSize as c_uint, 0)}
        }
    }

    #[doc(hidden)]
    pub fn wrap(font : *csfml::sfFont) -> Font {
        Font {font : font}
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfFont {
        self.font
    } 

}

impl Drop for Font {
    /**
    * Destroy an existing font
    */
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfFont_destroy(self.font)
        }
    }
}