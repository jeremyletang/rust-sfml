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

use libc::{c_uint, size_t};
use std::ffi::CString;

use graphics::{Glyph};

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Class for loading and manipulating character fonts
pub struct Font(Foreign<ffi::sfFont>);

impl Font {
    /// Create a new font from a file
    ///
    /// # Arguments
    /// * filename -  Path of the font file to load
    ///
    /// Return Some(Font) or None
    pub fn new_from_file(filename: &str) -> Option<Font> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfFont_createFromFile(c_str.as_ptr()))
        }.map(Font)
    }

    /// Create a new font from memory
    ///
    /// # Arguments
    /// * memory -  The in-memory font file
    ///
    /// Return Some(Font) or None
    pub fn new_from_memory(memory: &[u8]) -> Option<Font> {
        unsafe {
            Foreign::new(ffi::sfFont_createFromMemory(memory.as_ptr(), memory.len() as size_t))
        }.map(Font)
    }
	
	fn raw(&self) -> &ffi::sfFont { self.0.as_ref() }
	#[doc(hidden)]
	pub fn unwrap(&self) -> &ffi::sfFont { self.raw() }

    /// Create font from a existing one
    ///
    /// # Arguments
    /// * font - Font to copy
    ///
    /// Return Some(Font) or None
    pub fn clone_opt(&self) -> Option<Font> {
        unsafe {
			Foreign::new(ffi::sfFont_copy(self.raw()))
		}.map(Font)
    }

    /// Get the kerning value corresponding to a given pair of characters in a font
    ///
    /// # Arguments
    /// * first - Unicode code point of the first character
    /// * second - Unicode code point of the second character
    /// * characterSize - Character size, in pixels
    ///
    /// Return the kerning offset, in pixels
    pub fn get_kerning(&self,
                       first: u32,
                       second: u32,
                       character_size: u32) -> i32 {
        unsafe {
            ffi::sfFont_getKerning(self.raw(),
                                   first,
                                   second,
                                   character_size as c_uint) as i32
        }
    }

    /// Get the line spacing value
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the line spacing, in pixels
    pub fn get_line_spacing(&self, character_size: u32) -> i32 {
        unsafe {
            ffi::sfFont_getLineSpacing(self.raw(), character_size as c_uint) as i32
        }
    }

	/* TODO: return a reference to a Texture
    /// Get the texture containing the glyphs of a given size in a font
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the texture
    pub fn get_texture(&mut self, character_size: u32) -> Option<Texture> {
        let tex = unsafe {ffi::sfFont_getTexture(self.raw_mut(), character_size as c_uint)};
        if tex.is_null() {
            None
        } else {
            Some(Wrappable::wrap(tex))
        }
    }
	*/

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
                     bold: bool) -> Glyph {
        unsafe {
            ffi::sfFont_getGlyph(self.raw(), codepoint, character_size as c_uint, SfBool::from_bool(bold))
        }
    }
}

impl Clone for Font {
    fn clone(&self) -> Font {
		self.clone_opt().expect("Failed to clone Font")
    }
}

