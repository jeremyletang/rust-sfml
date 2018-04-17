use graphics::csfml_graphics_sys as ffi;
use graphics::{FloatRect, IntRect};

/// Structure describing a glyph.
///
/// A glyph is the visual representation of a character.
///
/// The `Glyph` structure provides the information needed to handle the glyph:
///
/// - its coordinates in the font's texture
/// - its bounding rectangle
/// - the offset to apply to get the starting position of the next glyph
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Glyph {
    /// Offset to move horizontally to the next character.
    pub advance: f32,
    /// Bounding rectangle of the glyph, in coordinates relative to the baseline.
    pub bounds: FloatRect,
    /// Texture coordinates of the glyph inside the font's texture.
    pub texture_rect: IntRect,
}

impl Glyph {
    pub(super) unsafe fn from_raw(raw: ffi::sfGlyph) -> Self {
        ::std::mem::transmute(raw)
    }
}
