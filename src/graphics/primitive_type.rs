use graphics::csfml_graphics_sys::sfPrimitiveType;
use system::raw_conv::{FromRaw, Raw};

/// Types of primitives that a `VertexArray` can render.
///
/// Points and lines have no area, therefore their thickness will always be 1 pixel,
/// regardless the current transform and view.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    /// List of individual points.
    Points = 0,
    /// List of individual lines.
    Lines = 1,
    /// List of connected lines, a point uses the previous point to form a line.
    LineStrip = 2,
    /// List of individual triangles.
    Triangles = 3,
    /// List of connected triangles, a point uses the two previous points to form a triangle.
    TriangleStrip = 4,
    /// List of connected triangles, a point uses the common center
    /// and the previous point to form a triangle.
    TriangleFan = 5,
    /// List of individual quads (deprecated, don't work with OpenGL ES)
    Quads = 6,
}

impl Raw for PrimitiveType {
    type Raw = sfPrimitiveType;

    fn raw(&self) -> sfPrimitiveType {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for PrimitiveType {
    type RawFrom = sfPrimitiveType;

    unsafe fn from_raw(raw: sfPrimitiveType) -> Self {
        ::std::mem::transmute(raw)
    }
}
