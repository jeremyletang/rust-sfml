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


//! Class for loading and manipulating character fonts

use libc::c_uint;
use std::ptr;

use traits::Wrappable;
use graphics::{Texture, Glyph};

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi = ffi::graphics::font;

/// Class for loading and manipulating character fonts
pub struct Font {
    #[doc(hidden)]
    font :     *mut ffi::sfFont,
    #[doc(hidden)]
    dropable : bool
}

impl Font {
    /**
     * Create a new font from a file
     *
     * # Arguments
     * * filename -  Path of the font file to load
     *
     * Return Some(Font) or None
     */
    pub fn new_from_file(filename : &str) -> Option<Font> {
        let mut fnt = ptr::mut_null();
        unsafe {
            filename.with_c_str(|c_str| {
                    fnt = ffi::sfFont_createFromFile(c_str)
                });
        }
        if fnt.is_null() {
            None
        } else {
            Some(Font {
                    font :      fnt,
                    dropable :  true
                })
        }
    }

    /**
     * Create font from a existing one
     *
     * # Arguments
     * * font - Font to copy
     *
     * Return Some(Font) or None
     */
    pub fn clone_opt(&self) -> Option<Font> {
        let fnt = unsafe {ffi::sfFont_copy(self.font)};
        if fnt.is_null() {
            None
        } else {
            Some(Font {
                    font :      fnt,
                    dropable :  true
                })
        }
    }

    /**
     * Get the kerning value corresponding to a given pair of characters in a font
     *
     * # Arguments
     * * first - Unicode code point of the first character
     * * second - Unicode code point of the second character
     * * characterSize - Character size, in pixels
     *
     * Return the kerning offset, in pixels
     */
    pub fn get_kerning(&self,
                       first : u32,
                       second : u32,
                       characterSize : uint) -> int {
        unsafe {
            ffi::sfFont_getKerning(self.font,
                                   first,
                                   second,
                                   characterSize as c_uint) as int
        }
    }

    /**
     * Get the line spacing value
     *
     * # Arguments
     * * characterSize - Character size, in pixels
     *
     * Return the line spacing, in pixels
     */
    pub fn get_line_spacing(&self, character_size : uint) -> int {
        unsafe {
            ffi::sfFont_getLineSpacing(self.font,
                                       character_size as c_uint) as int
        }
    }

    /**
     * Get the texture containing the glyphs of a given size in a font
     *
     * # Arguments
     * * characterSize - Character size, in pixels
     *
     * Return the texture
     */
    pub fn get_texture(&self, character_size : uint) -> Option<Texture> {
        let tex = unsafe {ffi::sfFont_getTexture(self.font,
                                                 character_size as c_uint)};
        if tex.is_null() {
            None
        } else {
            Some(Wrappable::wrap(tex))
        }
    }

    /**
     * Get a glyph in a font
     *
     * # Arguments
     * * codePoint - Unicode code point of the character to get
     * * characterSize - Character size, in pixels
     * * bold - Retrieve the bold version or the regular one?
     *
     * Return the corresponding glyph
     */
    pub fn get_glyph(&self,
                     codepoint : u32,
                     character_size : uint,
                     bold : bool) -> Glyph {
        unsafe {
            match bold {
                true        => ffi::sfFont_getGlyph(self.font,
                                                    codepoint,
                                                    character_size as c_uint,
                                                    SFFALSE),
                false       => ffi::sfFont_getGlyph(self.font,
                                                    codepoint,
                                                    character_size as c_uint,
                                                    SFTRUE)
            }
        }
    }
}

impl Clone for Font {
    /// Return a new Font or fail! if there is not enough memory
    fn clone(&self) -> Font {
        let fnt = unsafe {ffi::sfFont_copy(self.font)};
        if fnt.is_null() {
            fail!("Not enough memory to clone Font")
        } else {
            Font {
                font :      fnt,
                dropable :  true
            }
        }
    }
}

impl Wrappable<*mut ffi::sfFont> for Font {
    fn wrap(font : *mut ffi::sfFont) -> Font {
        Font {
            font :      font,
            dropable :  false
        }
    }
    fn unwrap(&self) -> *mut ffi::sfFont {
        self.font
    }
}

impl Drop for Font {
    /// Destroy an existing font
    fn drop(&mut self) -> () {
        if self.dropable {
            unsafe {
                ffi::sfFont_destroy(self.font)
            }
        }
    }
}
