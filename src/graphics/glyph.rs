use crate::graphics::{csfml_graphics_sys as ffi, FloatRect, IntRect};

/// Structure describing a glyph.
///
/// A glyph is the visual representation of a character.
///
/// The `Glyph` structure provides the information needed to handle the glyph:
///
/// - its coordinates in the font's texture
/// - its bounding rectangle
/// - the offset to apply to get the starting position of the next glyph
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Glyph(pub(super) ffi::sfGlyph);

impl Glyph {
    /// Offset to move horizontally to the next character.
    #[must_use]
    pub fn advance(&self) -> f32 {
        self.0.advance
    }
    /// Bounding rectangle of the glyph, in coordinates relative to the baseline.
    #[must_use]
    pub fn bounds(&self) -> FloatRect {
        FloatRect::from_raw(self.0.bounds)
    }
    /// Texture coordinates of the glyph inside the font's texture.
    #[must_use]
    pub fn texture_rect(&self) -> IntRect {
        IntRect::from_raw(self.0.textureRect)
    }
}
