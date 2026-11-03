use crate::{
    cpp::{CppVector, CppVectorItem, CppVectorSliceable},
    ffi::graphics as ffi,
    graphics::{Glyph, TextDirection},
    system::Vector2f,
};

/// Glyph after shaping, i.e. positioned based on the properties of the font and the string
///
/// Obtained from [`Text::shaped_glyphs`](crate::graphics::Text::shaped_glyphs) and
/// [`RcText::shaped_glyphs`](crate::graphics::RcText::shaped_glyphs).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ShapedGlyph {
    /// The glyph itself
    pub glyph: Glyph,
    /// Position of the glyph within the text, in local coordinates
    pub position: Vector2f,
    /// Cluster ID, maps the glyph back to the characters of the string it was shaped from
    pub cluster: u32,
    /// Text direction of the glyph
    pub text_direction: TextDirection,
    /// Baseline position of the line this glyph is part of
    pub baseline: f32,
    /// Starting offset of the vertex data belonging to this glyph
    pub vertex_offset: usize,
    /// Number of vertices belonging to this glyph
    pub vertex_count: usize,
}

unsafe impl CppVectorSliceable for ShapedGlyph {}

unsafe impl CppVectorItem for ShapedGlyph {
    fn get_data(vec: &CppVector<Self>) -> *const Self {
        unsafe { ffi::sfShapedGlyphVector_getData(vec) }
    }

    fn get_len(vec: &CppVector<Self>) -> usize {
        unsafe { ffi::sfShapedGlyphVector_getLength(vec) }
    }

    // The vector is owned by the text, so there is nothing to delete
    fn del(_: &mut CppVector<Self>) {}
}
