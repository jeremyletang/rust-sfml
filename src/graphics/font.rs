use {
    crate::{
        cpp::FBox,
        ffi::graphics as ffi,
        graphics::{Glyph, Texture},
        system::InputStream,
        IntoSfResult, SfResult,
    },
    std::{
        ffi::{CStr, CString},
        io::{Read, Seek},
    },
};

decl_opaque! {
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
pub Font;
}

/// Creation and loading
impl Font {
    /// Creates a new (empty) font.
    pub fn new() -> SfResult<FBox<Self>> {
        FBox::new(unsafe { ffi::sfFont_new() }).into_sf_result()
    }
    /// Creates a new `Font` from a file on the filesystem.
    ///
    /// See [`Self::load_from_file`].
    pub fn from_file(path: &str) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_file(path)?;
        Ok(new)
    }
    /// Creates a new `Font` from font file data in memory.
    ///
    /// See [`Self::load_from_memory`].
    ///
    /// # Safety
    ///
    /// Also see [`Self::load_from_memory`].
    pub unsafe fn from_memory(data: &[u8]) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        unsafe {
            new.load_from_memory(data)?;
        }
        Ok(new)
    }
    /// Creates a new `Font` from static font file data in memory.
    ///
    /// See [`Self::load_from_memory_static`].
    pub fn from_memory_static(data: &'static [u8]) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_memory_static(data)?;
        Ok(new)
    }
    /// Creates a new `Font` from a streamable source.
    ///
    /// See [`Self::load_from_stream`].
    ///
    /// # Safety
    ///
    /// Also see [`Self::load_from_stream`].
    pub unsafe fn from_stream<T: Read + Seek>(stream: &mut T) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        unsafe {
            new.load_from_stream(stream)?;
        }
        Ok(new)
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
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// let font = match Font::from_file("examples/resources/sansation.ttf") {
    ///     Ok(font) => font,
    ///     Err(e) => {
    ///         panic!("Failed to read font file: {e}");
    ///     }
    /// };
    /// ```
    pub fn load_from_file(&mut self, path: &str) -> SfResult<()> {
        let c_str = CString::new(path)?;
        unsafe { ffi::sfFont_loadFromFile(self, c_str.as_ptr()) }.into_sf_result()
    }

    /// Load the font from a custom stream.
    ///
    /// The supported font formats are: TrueType, Type 1, CFF, OpenType, SFNT, X11 PCF,
    /// Windows FNT, BDF, PFR and Type 42.
    ///
    /// # Safety
    /// SFML cannot preload all the font data in this function, so the stream has to remain
    /// accessible until the `Font` object loads a new font or is destroyed.
    ///
    /// # See also
    /// [`Font::from_file`], [`Font::from_memory`]
    pub unsafe fn load_from_stream<T: Read + Seek>(&mut self, stream: &mut T) -> SfResult<()> {
        let mut input_stream = InputStream::new(stream);
        unsafe { ffi::sfFont_loadFromStream(self, &mut *input_stream.stream) }.into_sf_result()
    }

    /// Load the font from a file in memory.
    ///
    /// The supported font formats are: TrueType, Type 1, CFF, OpenType, SFNT, X11 PCF,
    /// Windows FNT, BDF, PFR and Type 42.
    ///
    /// # Safety
    /// SFML cannot preload all the font data in this function, so the buffer pointed by `memory`
    /// has to remain valid until the `Font` object loads a new font or is destroyed.
    ///
    /// For a safe version, see [`Font::from_memory_static`].
    ///
    /// # See also
    ///
    /// [`Font::from_file`], [`Font::from_stream`]
    pub unsafe fn load_from_memory(&mut self, data: &[u8]) -> SfResult<()> {
        unsafe { ffi::sfFont_loadFromMemory(self, data.as_ptr(), data.len()) }.into_sf_result()
    }
    /// Load the font from a file in static memory.
    ///
    /// This function is safe because the font will stay in memory as long as required.
    ///
    /// See [`Self::load_from_memory`].
    pub fn load_from_memory_static(&mut self, data: &'static [u8]) -> SfResult<()> {
        unsafe { self.load_from_memory(data) }
    }
}

/// Font information, properties, glyph fetch
impl Font {
    /// Get the kerning value corresponding to a given pair of characters in a font
    ///
    /// # Arguments
    /// * first - Unicode code point of the first character
    /// * second - Unicode code point of the second character
    /// * characterSize - Character size, in pixels
    ///
    /// Return the kerning offset, in pixels
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// # let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    /// let kerning = font.kerning(0, 0, 32);
    /// assert_eq!(kerning, 0.);
    /// ```
    #[must_use]
    pub fn kerning(&self, first: u32, second: u32, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getKerning(self, first, second, character_size) }
    }

    /// Get the bold kerning value corresponding to a given pair of characters in a font
    ///
    /// # Arguments
    /// * first - Unicode code point of the first character
    /// * second - Unicode code point of the second character
    /// * characterSize - Character size, in pixels
    ///
    /// Return the bold kerning offset, in pixels
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// # let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    /// let kerning = font.bold_kerning(0, 0, 32);
    /// assert_eq!(kerning, 0.);
    /// ```
    #[must_use]
    pub fn bold_kerning(&self, first: u32, second: u32, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getBoldKerning(self, first, second, character_size) }
    }

    /// Get the line spacing value
    ///
    /// # Arguments
    /// * characterSize - Character size, in pixels
    ///
    /// Return the line spacing, in pixels
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// # let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    /// let line_spacing = font.line_spacing(32);
    /// assert_eq!(line_spacing, 35.);
    /// ```
    #[must_use]
    pub fn line_spacing(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getLineSpacing(self, character_size) }
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
                self,
                codepoint,
                character_size,
                bold,
                outline_thickness,
            ))
        }
    }
    /// Returns the font information.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    /// let font_info = font.info();
    /// assert_eq!(font_info.family, "Sansation");
    /// ```
    #[must_use]
    pub fn info(&self) -> Info {
        unsafe {
            let raw = ffi::sfFont_getInfo(self);
            let family = CStr::from_ptr(raw.family).to_string_lossy().into_owned();

            Info { family }
        }
    }
    /// Returns the position of the underline.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// # let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    /// let underline_position = font.underline_position(32);
    /// assert_eq!(underline_position, 3.734375);
    /// ```
    #[must_use]
    pub fn underline_position(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlinePosition(self, character_size) }
    }
    /// Returns the thickness of the underline.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::Font;
    /// # let font = Font::from_file("examples/resources/sansation.ttf").unwrap();
    /// let underline_thickness = font.underline_thickness(32);
    /// assert_eq!(underline_thickness, 2.34375);
    /// ```
    #[must_use]
    pub fn underline_thickness(&self, character_size: u32) -> f32 {
        unsafe { ffi::sfFont_getUnderlineThickness(self, character_size) }
    }
}

/// Texture atlas and smoothing
impl Font {
    /// Get the texture containing the glyphs of a given size in a font
    ///
    /// # Arguments
    /// * `character_size` - Character size, in pixels
    #[must_use]
    pub fn texture(&self, character_size: u32) -> &Texture {
        // # Safety
        //
        // `getTexture` returns a reference, which is never null or dangling.
        unsafe { &*ffi::sfFont_getTexture(self, character_size) }
    }

    /// Tell whether the smooth filter is enabled or not.
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfFont_isSmooth(self) }
    }

    /// Enable or disable the smooth filter.
    ///
    /// When the filter is activated, the font appears smoother so that pixels are less noticeable.
    /// However if you want the font to look exactly the same as its source file,
    /// you should disable it. The smooth filter is enabled by default.
    ///
    /// # Arguments
    /// * `smooth` - True to enable smoothing, false to disable smoothing
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfFont_setSmooth(self, smooth) }
    }
}

impl ToOwned for Font {
    type Owned = FBox<Font>;
    fn to_owned(&self) -> Self::Owned {
        let fnt = unsafe { ffi::sfFont_cpy(self) };
        FBox::new(fnt).expect("Failed to copy Font")
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { ffi::sfFont_del(self) }
    }
}

/// Holds various information about a font.
#[derive(Debug)]
pub struct Info {
    /// The font family.
    pub family: String,
}
