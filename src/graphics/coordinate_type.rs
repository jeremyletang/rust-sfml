use crate::ffi::graphics::sfCoordinateType;

/// Types of texture coordinates that can be used for rendering.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CoordinateType(pub(super) sfCoordinateType);

impl CoordinateType {
    /// Texutre coordinates in range [0 .. 1].
    pub const NORMALIZED: Self = Self(sfCoordinateType::sfCoordinateTypeNormalized);
    /// Texture coordinates in range [0 .. size].
    pub const PIXELS: Self = Self(sfCoordinateType::sfCoordinateTypePixels);
}
