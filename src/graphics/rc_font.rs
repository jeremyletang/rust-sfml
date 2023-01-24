use crate::{
    graphics::{font::Info, Font, Glyph, Texture},
    sf_box::{Dispose, SfBox},
};
use std::{
    cell::RefCell,
    io::{Read, Seek},
    rc::Rc,
};

/// Type for loading and manipulating character fonts.
///
/// Fonts can be loaded from a file, from memory or from a custom stream,
/// and supports the most common types of fonts.
///
/// See the [`from_file`] function for the complete list of supported formats.
///
/// [`from_file`]: RcFont::from_file
///
/// Once it is loaded, a `RcFont` instance provides three types of information about the font:
///
/// - Global metrics, such as the line spacing
/// - Per-glyph metrics, such as bounding box or kerning
/// - Pixel representation of glyphs
///
/// Fonts alone are not very useful: they hold the font data but cannot make anything useful of it.
/// To do so you need to use the [`RcText`] type, which is able to properly output text with
/// several options such as character size, style, color, position, rotation, etc.
/// This separation allows more flexibility and better performances:
/// indeed a `RcFont` is a heavy resource, and any operation on it is
/// slow (often too slow for real-time applications).
/// On the other side, a [`RcText`] is a lightweight object which can combine the
/// glyphs data and metrics of a `RcFont` to display any text on a render target.
/// Note that it is also possible to bind several [`RcText`] instances to the same `RcFont`.
///
/// It is important to note that the [`RcText`] instance doesn't copy the font that it uses,
/// it only keeps a reference to it.
/// Thus, a `RcFont` must not be destructed while it is used by a
/// [`RcText`] (i.e. never write a function that uses a local `RcFont` instance for creating a text).
///
/// Apart from loading font files, and passing them to instances of [`RcText`],
/// you should normally not have to deal directly with this type.
/// However, it may be useful to access the font metrics or rasterized glyphs for advanced usage.
///
/// Note that if the font is a bitmap font, it is not scalable,
/// thus not all requested sizes will be available to use.
/// This needs to be taken into consideration when using [`RcText`].
/// If you need to display text of a certain size, make sure the corresponding bitmap font that
/// supports that size is used.
///
/// [`RcText`]: crate::graphics::RcText
#[derive(Debug)]
pub struct RcFont {
    font: Rc<RefCell<SfBox<Font>>>,
}

impl RcFont {
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
    /// # use sfml::graphics::RcFont;
    /// # let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let kerning = font.kerning(0, 0, 32);
    /// assert_eq!(kerning, 0.);
    /// ```
    #[must_use]
    pub fn kerning(&self, first: u32, second: u32, character_size: u32) -> f32 {
        self.font.borrow().kerning(first, second, character_size)
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
    /// # use sfml::graphics::RcFont;
    /// # let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let line_spacing = font.line_spacing(32);
    /// assert_eq!(line_spacing, 35.);
    /// ```
    #[must_use]
    pub fn line_spacing(&self, character_size: u32) -> f32 {
        self.font.borrow().line_spacing(character_size)
    }

    /// Get a glyph in a font
    ///
    /// # Arguments
    /// * codePoint - Unicode code point of the character to get
    /// * characterSize - Character size, in pixels
    /// * bold - Retrieve the bold version or the regular one?
    ///
    /// Return the corresponding glyph
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::graphics::RcFont;
    /// # let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let glyph = font.glyph(41, 32, false, 5.);
    /// # use sfml::graphics::Rect;
    /// assert_eq!(glyph.bounds(), Rect::new(0., -23., 17., 39.));
    /// ```
    #[must_use]
    pub fn glyph(
        &self,
        codepoint: u32,
        character_size: u32,
        bold: bool,
        outline_thickness: f32,
    ) -> Glyph {
        self.font
            .borrow()
            .glyph(codepoint, character_size, bold, outline_thickness)
    }

    /// Returns the font information.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::RcFont;
    /// let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let font_info = font.info();
    /// assert_eq!(font_info.family, "Sansation");
    /// ```
    #[must_use]
    pub fn info(&self) -> Info {
        self.font.borrow().info()
    }

    /// Returns the position of the underline.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::RcFont;
    /// # let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let underline_position = font.underline_position(32);
    /// assert_eq!(underline_position, 3.734375);
    /// ```
    #[must_use]
    pub fn underline_position(&self, character_size: u32) -> f32 {
        self.font.borrow().underline_position(character_size)
    }

    /// Returns the thickness of the underline.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::RcFont;
    /// # let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let underline_thickness = font.underline_thickness(32);
    /// assert_eq!(underline_thickness, 2.34375);
    /// ```
    #[must_use]
    pub fn underline_thickness(&self, character_size: u32) -> f32 {
        self.font.borrow().underline_thickness(character_size)
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
    /// so the file has to remain accessible until the `RcFont` object loads a new font or
    /// is destroyed.
    ///
    /// # Usage Example
    ///
    /// ```
    /// # use sfml::graphics::RcFont;
    /// let font = match RcFont::from_file("examples/resources/sansation.ttf") {
    ///     Some(font) => font,
    ///     None => {
    ///         panic!("Failed to read font file!");
    ///     }
    /// };
    /// ```
    #[must_use]
    pub fn from_file(filename: &str) -> Option<Self> {
        Some(RcFont {
            font: Rc::new(RefCell::new(Font::from_file(filename)?)),
        })
    }

    /// Load the font from a custom stream.
    ///
    /// The supported font formats are: TrueType, Type 1, CFF, OpenType, SFNT, X11 PCF,
    /// Windows FNT, BDF, PFR and Type 42.
    ///
    /// # Safety
    /// SFML cannot preload all the font data in this function, so the stream has to remain
    /// accessible until the `RcFont` object loads a new font or is destroyed.
    ///
    /// # Returns
    /// True if loading succeeded, false if it failed
    ///
    /// # See also
    /// [`RcFont::from_file`], [`RcFont::from_memory`]
    pub unsafe fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<Self> {
        Some(RcFont {
            font: Rc::new(RefCell::new(Font::from_stream(stream)?)),
        })
    }

    /// Load the font from a file in memory.
    ///
    /// The supported font formats are: TrueType, Type 1, CFF, OpenType, SFNT, X11 PCF,
    /// Windows FNT, BDF, PFR and Type 42.
    ///
    /// # Safety
    /// SFML cannot preload all the font data in this function, so the buffer pointed by `memory`
    /// has to remain valid until the `RcFont` object loads a new font or is destroyed.
    ///
    /// # Returns
    /// True if loading succeeded, false if it failed
    ///
    /// See also
    /// [`RcFont::from_file`], [`RcFont::from_stream`]
    #[must_use]
    pub unsafe fn from_memory(memory: &[u8]) -> Option<Self> {
        Some(RcFont {
            font: Rc::new(RefCell::new(Font::from_memory(memory)?)),
        })
    }

    /// Get the texture containing the glyphs of a given size in a font
    ///
    /// # Arguments
    /// * `character_size` - Character size, in pixels
    ///
    /// # Usage Example
    ///
    /// ```no_run
    /// # use sfml::graphics::RcFont;
    /// # let font = RcFont::from_file("examples/resources/sansation.ttf").unwrap();
    /// let texture = font.texture(32);
    /// # use sfml::system::Vector2;
    /// assert_eq!(texture.size(), Vector2::new(128, 128));
    /// ```
    #[must_use]
    pub fn texture(&self, character_size: u32) -> &Texture {
        unsafe { (**self.font.as_ptr()).texture(character_size) }
    }

    /// INTERNAL FUNCTION ONLY
    /// Allows other rc variants to request a weak pointer to the texture
    pub(super) fn downgrade(&self) -> std::rc::Weak<RefCell<SfBox<Font>>> {
        Rc::downgrade(&self.font)
    }
}

impl ToOwned for RcFont {
    type Owned = RcFont;
    fn to_owned(&self) -> Self {
        RcFont {
            font: Rc::new(RefCell::new(self.font.borrow().to_owned())),
        }
    }
}

impl Dispose for RcFont {
    unsafe fn dispose(&mut self) {
        self.font.borrow_mut().dispose()
    }
}
