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

use libc::{c_uint, size_t};
use std::ffi::CString;
use std::io::{Read, Seek};

use system::InputStream;
use graphics::{Glyph, Texture};

use ffi::{SfBool, Foreign, Ref, from_c_str};
use ffi::graphics as ffi;

/// Type for loading and manipulating character fonts.
///
/// Fonts can be loaded from a file, from memory or from a custom stream, and
/// supports the most common types of fonts. The supported font formats are:
/// TrueType, Type 1, CFF, OpenType, SFNT, X11 PCF, Windows FNT, BDF, PFR and
/// Type 42.
///
/// Once it is loaded, a `Font` instance provides three types of information
/// about the font:
///
/// * Global metrics, such as the line spacing
/// * Per-glyph metrics, such as bounding box or kerning
/// * Pixel representation of glyphs
///
/// Fonts alone are not very useful: they hold the font data but cannot make
/// anything useful of it. To do so you need to use the `Text` type, which is
/// able to properly output text with several options such as character size,
/// style, color, position, rotation, etc. This separation allows more
/// flexibility and better performance: a `Font` is a heavy resource, and any
/// operation on it is slow (often too slow for real-time applications). On the
/// other hand, a `Text` is a lightweight object which can combine the glyph
/// data and metrics of a `Font` to display any text on a render target. Note
/// that it is also possible to bind several `Text`s to the same `Font`.
///
/// Note that if the font is a bitmap font, it is not scalable, thus not all
/// requested sizes will be available to use. This needs to be taken into
/// consideration when using `Text`. If you need to display text of a certain
/// size, make sure the corresponding bitmap font that supports that size is
/// used.
pub struct Font(Foreign<ffi::sfFont>);

impl Font {
    /// Create a new font from a file.
    ///
	/// The file must be in one of the supported formats. Note that the standard
	/// fonts installed on a user's system cannot be loaded directly.
	///
    /// Returns Some(Font) or None on failure.
    pub fn new_from_file(filename: &str) -> Option<Font> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfFont_createFromFile(c_str.as_ptr()))
        }.map(Font)
    }

    /// Create a new font from a file contained in memory.
    ///
	/// The buffer must be in one of the supported formats.
	///
    /// Returns Some(Font) or None on failure.
    pub fn new_from_memory(memory: &[u8]) -> Option<Font> {
        unsafe {
            Foreign::new(ffi::sfFont_createFromMemory(memory.as_ptr(), memory.len() as size_t))
        }.map(Font)
    }

	/// Create a new font from an input stream.
    ///
	/// The stream must be in one of the supported formats.
	///
	/// Returns Some(Font) or None on failure.
	pub fn new_from_stream<T: Read + Seek>(stream: &mut T) -> Option<Font> {
		unsafe {
			Foreign::new(ffi::sfFont_createFromStream(&mut InputStream::new(stream)))
		}.map(Font)
	}
	
	fn raw(&self) -> &ffi::sfFont { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfFont { self.0.as_mut() }
	#[doc(hidden)]
	pub fn unwrap(&self) -> &ffi::sfFont { self.raw() }

    /// Create a font from an existing one.
    ///
    /// Returns Some(Font) or None on failure.
    pub fn clone_opt(&self) -> Option<Font> {
        unsafe {
			Foreign::new(ffi::sfFont_copy(self.raw()))
		}.map(Font)
    }

	/// Get the font information.
	pub fn get_info(&self) -> FontInfo {
		unsafe {
			let raw = ffi::sfFont_getInfo(self.raw());
			FontInfo {
				family: from_c_str(raw.family)
			}
		}
	}

    /// Get the kerning offset of two glyphs.
	///
	/// The kerning is an extra offset (negative) to apply between two glyphs
	/// when rendering them, to make the pair look more "natural". For example,
	/// the pair "AV" have a special kerning to make them closer than other
	/// characters. Most glyph pairs have a kerning offset of zero.
    ///
    /// Returns the kerning offset, in pixels.
    pub fn get_kerning(&self, first: char, second: char, character_size: u32) -> f32 {
        unsafe {
            ffi::sfFont_getKerning(self.raw(), first as u32, second as u32, character_size as c_uint) as f32
        }
    }

    /// Get the line spacing.
	///
	/// Line spacing is the vertical offset to apply between two consecutive
	/// lines of text.
    ///
    /// Returns the line spacing, in pixels.
    pub fn get_line_spacing(&self, character_size: u32) -> f32 {
        unsafe {
            ffi::sfFont_getLineSpacing(self.raw(), character_size as c_uint) as f32
        }
    }

	/// Retrieve the texture containing the loaded glyphs of a certain size.
	///
	/// The contents of the returned texture changes as more glyphs are
	/// requested, thus it is not very relevant. It is mainly used internally by
	/// `Text`.
    ///
    /// Returns a `Texture` containing glyphs of the requested size.
	// Only mut because of an apparent CSFML deficiency? Const in SFML.
    pub fn get_texture(&mut self, character_size: u32) -> Option<Ref<Texture>> {
		unsafe {
			Ref::new(ffi::sfFont_getTexture(self.raw_mut(), character_size as c_uint))
		}
    }

    /// Retrieve a glyph of the font.
	///
	/// If the font is a bitmap font, not all character sizes might be
	/// available. If the glyph is not available at the requested size, an empty
	/// glyph is returned.
    ///
	/// The character, character size in pixels, and bold status must be
	/// specified.
    ///
    /// Returns the glyph corresponding to `codepoint` and `character_size`.
    pub fn get_glyph(&self, codepoint: char, character_size: u32, bold: bool) -> Glyph {
        unsafe {
            ffi::sfFont_getGlyph(self.raw(), codepoint as u32, character_size as c_uint, SfBool::from_bool(bold))
        }
    }

	/// Get the position of the underline for a character size.
	///
	/// Underline position is the vertical offset to apply between the baseline
	/// and the underline.
	pub fn get_underline_position(&self, character_size: u32) -> f32 {
		unsafe { ffi::sfFont_getUnderlinePosition(self.raw(), character_size as c_uint) as f32 }
	}

	/// Get the thickness of the underline for a character size.
	///
	/// Underline thickness is the vertical size of the underline.
	pub fn get_underline_thickness(&self, character_size: u32) -> f32 {
		unsafe { ffi::sfFont_getUnderlineThickness(self.raw(), character_size as c_uint) as f32 }
	}
}

impl Clone for Font {
    fn clone(&self) -> Font {
		self.clone_opt().expect("Failed to clone Font")
    }
}

/// Holds various information about a font.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontInfo {
	/// The font family.
	pub family: String
}
