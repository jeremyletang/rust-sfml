use csfml_system_sys::sfBool;
use graphics::{Glyph, TextureRef};
use graphics::csfml_graphics_sys as ffi;
use inputstream::InputStream;
use sf_bool_ext::SfBoolExt;
use std::borrow::{Borrow, ToOwned};
use std::ffi::{CStr, CString};
use std::io::{Read, Seek};
use std::ops::Deref;

/// Type for loading and manipulating character fonts
#[derive(Debug)]
pub struct Font {
    font: *mut ffi::sfFont,
}

impl Deref for Font {
    type Target = FontRef;

    fn deref(&self) -> &FontRef {
        unsafe { &*(self.font as *const FontRef) }
    }
}

/// A non-owning `Font`.
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum FontRef {}

impl FontRef {
    /// Get the kerning value corresponding to a given pair of characters in a font
    ///
    /// # Arguments
    /// * first - Unicode code point of the first character
    /// * second - Unicode code point of the second character
    /// * characterSize - Character size, in pixels
    ///
    /// Return the kerning offset, in pixels
    pub fn kerning(&self, first: u32, second: u32, character_size: u32) -> i32 {
        unsafe { ffi::sfFont_getKerning(self.raw(), first, second, character_size) as i32 }
    }

    /// Get the line spacing value
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the line spacing, in pixels
    pub fn line_spacing(&self, character_size: u32) -> i32 {
        unsafe { ffi::sfFont_getLineSpacing(self.raw(), character_size) as i32 }
    }

    /// Get a glyph in a font
    ///
    /// # Arguments
    /// * codePoint - Unicode code point of the character to get
    /// * characterSize - Character size, in pixels
    /// * bold - Retrieve the bold version or the regular one?
    ///
    /// Return the corresponding glyph
    pub fn glyph(&self,
                 codepoint: u32,
                 character_size: u32,
                 bold: bool,
                 outline_thickness: f32)
                 -> Glyph {
        unsafe {
            Glyph::from_raw(ffi::sfFont_getGlyph(self.raw(),
                                                 codepoint,
                                                 character_size,
                                                 sfBool::from_bool(bold),
                                                 outline_thickness))
        }
    }
    /// Returns the font information.
    pub fn info(&self) -> Info {
        unsafe {
            let raw = ffi::sfFont_getInfo(self.raw());
            let family = CStr::from_ptr(raw.family).to_string_lossy().into_owned();

            Info { family: family }
        }
    }
    /// Returns the position of the underline.
    pub fn underline_position(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlinePosition(self.raw(), character_size) }
    }
    /// Returns the thickness of the underline.
    pub fn underline_thickness(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlineThickness(self.raw(), character_size) }
    }
    pub fn raw(&self) -> *const ffi::sfFont {
        let ptr: *const Self = self;
        ptr as _
    }
}

/// Holds various information about a font.
#[derive(Debug)]
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

    /// Get the texture containing the glyphs of a given size in a font
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the texture
    pub fn texture(&mut self, character_size: u32) -> &TextureRef {
        let tex = unsafe { ffi::sfFont_getTexture(self.raw_mut(), character_size) };
        assert!(!tex.is_null(), "sfFont_getTexture failed");
        unsafe { &*(tex as *const TextureRef) }
    }
    pub fn raw_mut(&mut self) -> *mut ffi::sfFont {
        self.font
    }
}

#[test]
fn test_info() {
    let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    assert_eq!(font.info().family, "Sansation");
}

impl Borrow<FontRef> for Font {
    fn borrow(&self) -> &FontRef {
        &*self
    }
}

impl ToOwned for FontRef {
    type Owned = Font;
    fn to_owned(&self) -> Self::Owned {
        let fnt = unsafe { ffi::sfFont_copy(self.raw()) };
        if fnt.is_null() {
            panic!("Not enough memory to clone Font")
        } else {
            Font { font: fnt }
        }
    }
}

impl Clone for Font {
    /// Return a new Font or panic! if there is not enough memory
    fn clone(&self) -> Font {
        (**self).to_owned()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { ffi::sfFont_destroy(self.font) }
    }
}
