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


//! Type for loading and manipulating character fonts

use std::ffi::{CStr, CString};

use system::raw_conv::{Raw, FromRaw};
use graphics::{TextureRef, Glyph};

use csfml_system_sys::sfBool;
use csfml_graphics_sys as ffi;

use std::io::{Read, Seek};
use inputstream::InputStream;
use ext::sf_bool_ext::SfBoolExt;

/// Type for loading and manipulating character fonts
pub struct Font {
    font: *mut ffi::sfFont,
}

/// Holds various information about a font.
pub struct Info {
    /// The font family.
    pub family: String,
}

impl Font {
    /// Create a new font from a file
    ///
    /// # Arguments
    /// * filename -  Path of the font file to load
    ///
    /// Return Some(Font) or None
    pub fn from_file(filename: &str) -> Option<Font> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let fnt = unsafe { ffi::sfFont_createFromFile(c_str.as_ptr()) };
        if fnt.is_null() {
            None
        } else {
            Some(Font { font: fnt })
        }
    }

    /// Create a new font from a stream (a struct implementing Read and Seek)
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    ///
    /// Return Some(Font) or None
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<Font> {
        let mut input_stream = InputStream::new(stream);
        let fnt = unsafe { ffi::sfFont_createFromStream(&mut input_stream.0) };
        if fnt.is_null() {
            None
        } else {
            Some(Font { font: fnt })
        }
    }

    /// Create a new font from memory
    ///
    /// # Arguments
    /// * memory -  The in-memory font file
    ///
    /// Return Some(Font) or None
    pub fn from_memory(memory: &[u8]) -> Option<Font> {
        let fnt =
            unsafe { ffi::sfFont_createFromMemory(memory.as_ptr() as *const _, memory.len()) };
        if fnt.is_null() {
            None
        } else {
            Some(Font { font: fnt })
        }
    }

    /// Get the kerning value corresponding to a given pair of characters in a font
    ///
    /// # Arguments
    /// * first - Unicode code point of the first character
    /// * second - Unicode code point of the second character
    /// * characterSize - Character size, in pixels
    ///
    /// Return the kerning offset, in pixels
    pub fn get_kerning(&self, first: u32, second: u32, character_size: u32) -> i32 {
        unsafe { ffi::sfFont_getKerning(self.font, first, second, character_size) as i32 }
    }

    /// Get the line spacing value
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the line spacing, in pixels
    pub fn get_line_spacing(&self, character_size: u32) -> i32 {
        unsafe { ffi::sfFont_getLineSpacing(self.font, character_size) as i32 }
    }

    /// Get the texture containing the glyphs of a given size in a font
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the texture
    pub fn get_texture(&self, character_size: u32) -> &TextureRef {
        let tex = unsafe { ffi::sfFont_getTexture(self.font, character_size) };
        if tex.is_null() {
            panic!("Font::get_texture: texture is null");
        } else {
            unsafe { &*(tex as *const TextureRef) }
        }
    }

    /// Get a glyph in a font
    ///
    /// # Arguments
    /// * codePoint - Unicode code point of the character to get
    /// * characterSize - Character size, in pixels
    /// * bold - Retrieve the bold version or the regular one?
    ///
    /// Return the corresponding glyph
    pub fn get_glyph(&self,
                     codepoint: u32,
                     character_size: u32,
                     bold: bool,
                     outline_thickness: f32)
                     -> Glyph {
        unsafe {
            Glyph::from_raw(ffi::sfFont_getGlyph(self.font,
                                                 codepoint,
                                                 character_size,
                                                 sfBool::from_bool(bold),
                                                 outline_thickness))
        }
    }
    /// Returns the font information.
    pub fn info(&self) -> Info {
        unsafe {
            let raw = ffi::sfFont_getInfo(self.font);
            let family = CStr::from_ptr(raw.family).to_string_lossy().into_owned();

            Info { family: family }
        }
    }
    /// Returns the position of the underline.
    pub fn underline_position(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlinePosition(self.font, character_size) }
    }
    /// Returns the thickness of the underline.
    pub fn underline_thickness(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlineThickness(self.font, character_size) }
    }
}

#[test]
fn test_info() {
    let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    assert_eq!(font.info().family, "Sansation");
}

impl Clone for Font {
    /// Return a new Font or panic! if there is not enough memory
    fn clone(&self) -> Font {
        let fnt = unsafe { ffi::sfFont_copy(self.font) };
        if fnt.is_null() {
            panic!("Not enough memory to clone Font")
        } else {
            Font { font: fnt }
        }
    }
}

impl Raw for Font {
    type Raw = *mut ffi::sfFont;
    fn raw(&self) -> Self::Raw {
        self.font
    }
}

impl Drop for Font {
    /// Destroy an existing font
    fn drop(&mut self) {
        unsafe { ffi::sfFont_destroy(self.font) }
    }
}
