use crate::ffi;

/// Types of primitives of which vertex arrays can be rendered.
///
/// Points and lines have no area, therefore their thickness will always be 1 pixel,
/// regardless the current transform and view.
///
/// See: [`RenderWindow::draw_primitives`](crate::graphics::RenderTarget::draw_primitives)
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimitiveType(pub(super) ffi::graphics::sfPrimitiveType);

impl PrimitiveType {
    /// List of individual points.
    pub const POINTS: Self = Self(ffi::graphics::sfPrimitiveType::Points);
    /// List of individual lines.
    pub const LINES: Self = Self(ffi::graphics::sfPrimitiveType::Lines);
    /// List of connected lines, a point uses the previous point to form a line.
    pub const LINE_STRIP: Self = Self(ffi::graphics::sfPrimitiveType::LineStrip);
    /// List of individual triangles.
    pub const TRIANGLES: Self = Self(ffi::graphics::sfPrimitiveType::Triangles);
    /// List of connected triangles, a point uses the two previous points to form a triangle.
    pub const TRIANGLE_STRIP: Self = Self(ffi::graphics::sfPrimitiveType::TriangleStrip);
    /// List of connected triangles, a point uses the common center
    /// and the previous point to form a triangle.
    pub const TRIANGLE_FAN: Self = Self(ffi::graphics::sfPrimitiveType::TriangleFan);
}
