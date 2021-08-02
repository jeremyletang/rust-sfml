use csfml_graphics_sys as ffi;

/// Types of primitives of which vertex arrays can be rendered.
///
/// Points and lines have no area, therefore their thickness will always be 1 pixel,
/// regardless the current transform and view.
///
/// See: [`RenderWindow::draw_primitives`](crate::graphics::RenderTarget::draw_primitives)
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimitiveType(pub(super) ffi::sfPrimitiveType);

impl PrimitiveType {
    /// List of individual points.
    pub const POINTS: Self = Self(ffi::sfPrimitiveType_sfPoints);
    /// List of individual lines.
    pub const LINES: Self = Self(ffi::sfPrimitiveType_sfLines);
    /// List of connected lines, a point uses the previous point to form a line.
    pub const LINE_STRIP: Self = Self(ffi::sfPrimitiveType_sfLineStrip);
    /// List of individual triangles.
    pub const TRIANGLES: Self = Self(ffi::sfPrimitiveType_sfTriangles);
    /// List of connected triangles, a point uses the two previous points to form a triangle.
    pub const TRIANGLE_STRIP: Self = Self(ffi::sfPrimitiveType_sfTriangleStrip);
    /// List of connected triangles, a point uses the common center
    /// and the previous point to form a triangle.
    pub const TRIANGLE_FAN: Self = Self(ffi::sfPrimitiveType_sfTriangleFan);
    /// List of individual quads (deprecated, don't work with OpenGL ES)
    pub const QUADS: Self = Self(ffi::sfPrimitiveType_sfQuads);
}
