use crate::{
    ffi::{graphics as ffi, system::sfBool},
    graphics::{Glyph, Texture},
    inputstream::InputStream,
    sf_bool_ext::SfBoolExt,
    sf_box::{Dispose, SfBox},
};
use std::{
    borrow::ToOwned,
    ffi::{CStr, CString},
    io::{Read, Seek},
};

/// Type for loading and manipulating character fonts.
///
/// Fonts can be loaded from a file, from memory or from a custom stream,
/// and supports the most common types of fonts.
///
/// See the [`from_file`] function for the complete list of supported formats.
///
/// [`from_file`]: Font::from_file
///
/// Once it is loaded, a `Font` instance provides three types of information about the font:
///
/// - Global metrics, such as the line spacing
/// - Per-glyph metrics, such as bounding box or kerning
/// - Pixel representation of glyphs
///
/// Fonts alone are not very useful: they hold the font data but cannot make anything useful of it.
/// To do so you need to use the [`Text`] type, which is able to properly output text with
/// several options such as character size, style, color, position, rotation, etc.
/// This separation allows more flexibility and better performances:
/// indeed a `Font` is a heavy resource, and any operation on it is
/// slow (often too slow for real-time applications).
/// On the other side, a [`Text`] is a lightweight object which can combine the
/// glyphs data and metrics of a `Font` to display any text on a render target.
/// Note that it is also possible to bind several [`Text`] instances to the same `Font`.
///
/// It is important to note that the [`Text`] instance doesn't copy the font that it uses,
/// it only keeps a reference to it.
/// Thus, a `Font` must not be destructed while it is used by a
/// [`Text`] (i.e. never write a function that uses a local `Font` instance for creating a text).
///
/// Apart from loading font files, and passing them to instances of [`Text`],
/// you should normally not have to deal directly with this type.
/// However, it may be useful to access the font metrics or rasterized glyphs for advanced usage.
///
/// Note that if the font is a bitmap font, it is not scalable,
/// thus not all requested sizes will be available to use.
/// This needs to be taken into consideration when using [`Text`].
/// If you need to display text of a certain size, make sure the corresponding bitmap font that
/// supports that size is used.
///
/// [`Text`]: crate::graphics::Text
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Font {
    _opaque: [u8; 0],
}

impl Font {
    /// Get the kerning value corresponding to a given pair of characters in a font
    ///
    /// # Arguments
    /// * first - Unicode code point of the first character
    /// * second - Unicode code point of the second character
    /// * characterSize - Character size, in pixels
    ///
    /// Return the kerning offset, in pixels
    #[must_use]
    pub fn kerning(&self, first: u32, second: u32, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getKerning(self.raw(), first, second, character_size) }
    }

    /// Get the line spacing value
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the line spacing, in pixels
    #[must_use]
    pub fn line_spacing(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getLineSpacing(self.raw(), character_size) }
    }

    /// Get a glyph in a font
    ///
    /// # Arguments
    /// * codePoint - Unicode code point of the character to get
    /// * characterSize - Character size, in pixels
    /// * bold - Retrieve the bold version or the regular one?
    ///
    /// Return the corresponding glyph
    #[must_use]
    pub fn glyph(
        &self,
        codepoint: u32,
        character_size: u32,
        bold: bool,
        outline_thickness: f32,
    ) -> Glyph {
        unsafe {
            Glyph(ffi::sfFont_getGlyph(
                self.raw(),
                codepoint,
                character_size,
                sfBool::from_bool(bold),
                outline_thickness,
            ))
        }
    }
    /// Returns the font information.
    #[must_use]
    pub fn info(&self) -> Info {
        unsafe {
            let raw = ffi::sfFont_getInfo(self.raw());
            let family = CStr::from_ptr(raw.family).to_string_lossy().into_owned();

            Info { family }
        }
    }
    /// Returns the position of the underline.
    #[must_use]
    pub fn underline_position(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlinePosition(self.raw(), character_size) }
    }
    /// Returns the thickness of the underline.
    #[must_use]
    pub fn underline_thickness(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlineThickness(self.raw(), character_size) }
    }
    /// Load the font from a file.
    ///
    /// The supported font formats are: TrueType, Type 1, CFF, OpenType, SFNT, X11 PCF,
    /// Windows FNT, BDF, PFR and Type 42.
    /// Note that this function know nothing about the standard fonts installed on the
    /// user's system, thus you can't load them directly.
    ///
    /// # Warning
    /// SFML cannot preload all the font data in this function,
    /// so the file has to remain accessible until the `Font` object loads a new font or
    /// is destroyed.
    #[must_use]
    pub fn from_file(filename: &str) -> Option<SfBox<Self>> {
        let c_str = CString::new(filename).unwrap();
        let fnt = unsafe { ffi::sfFont_createFromFile(c_str.as_ptr()) };
        SfBox::new(fnt as *mut Self)
    }

    /// Create a new font from a stream (a struct implementing Read and Seek)
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    ///
    /// Returns `None` on failure.
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<SfBox<Self>> {
        let mut input_stream = InputStream::new(stream);
        let fnt = unsafe { ffi::sfFont_createFromStream(&mut input_stream.0) };
        SfBox::new(fnt as *mut Self)
    }

    /// Create a new font from memory
    ///
    /// # Arguments
    /// * memory -  The in-memory font file
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn from_memory(memory: &[u8]) -> Option<SfBox<Self>> {
        let fnt =
            unsafe { ffi::sfFont_createFromMemory(memory.as_ptr() as *const _, memory.len()) };
        SfBox::new(fnt as *mut Self)
    }

    /// Get the texture containing the glyphs of a given size in a font
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the texture
    ///
    /// Note: Unfortunately, this method requires mutable access, because CSFML
    /// uses a texture cache or something that it must update every time this function
    /// is called.
    pub fn texture(&mut self, character_size: u32) -> &Texture {
        let tex = unsafe { ffi::sfFont_getTexture(self.raw_mut(), character_size) };
        assert!(!tex.is_null(), "sfFont_getTexture failed");
        unsafe { &*(tex as *const Texture) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfFont {
        let ptr: *const Self = self;
        ptr as _
    }
    fn raw_mut(&mut self) -> *mut ffi::sfFont {
        let ptr: *mut Self = self;
        ptr as _
    }
}

impl ToOwned for Font {
    type Owned = SfBox<Font>;
    fn to_owned(&self) -> Self::Owned {
        let fnt = unsafe { ffi::sfFont_copy(self.raw()) };
        SfBox::new(fnt as *mut Self).expect("Failed to copy Font")
    }
}

impl Dispose for Font {
    unsafe fn dispose(&mut self) {
        let ptr: *mut Self = self;
        ffi::sfFont_destroy(ptr as _)
    }
}

/// Holds various information about a font.
#[derive(Debug)]
pub struct Info {
    /// The font family.
    pub family: String,
}

#[cfg_attr(not(feature = "ci-headless"), test)]
#[test]
fn test_info() {
    let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    assert_eq!(font.info().family, "Sansation");
}
