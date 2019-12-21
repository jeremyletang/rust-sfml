use crate::graphics::csfml_graphics_sys as ffi;
use crate::graphics::FloatRect;
use crate::system::Vector2f;

/// Define a 3x3 transform matrix.
///
/// A `Transform` specifies how to translate,
/// rotate, scale, shear, project, whatever things.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform(pub ffi::sfTransform);

impl Transform {
    /// Creates a new transform from a 3x3 matrix.
    ///
    /// # Arguments
    ///
    /// - *a00* : Element (0, 0) of the matrix
    /// - *a01* : Element (0, 1) of the matrix
    /// - *a02* : Element (0, 2) of the matrix
    /// - *a10* : Element (1, 0) of the matrix
    /// - *a11* : Element (1, 1) of the matrix
    /// - *a12* : Element (1, 2) of the matrix
    /// - *a20* : Element (2, 0) of the matrix
    /// - *a21* : Element (2, 1) of the matrix
    /// - *a22* : Element (2, 2) of the matrix
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn new(
        a00: f32,
        a01: f32,
        a02: f32,
        a10: f32,
        a11: f32,
        a12: f32,
        a20: f32,
        a21: f32,
        a22: f32,
    ) -> Transform {
        unsafe {
            Transform(ffi::sfTransform_fromMatrix(
                a00, a01, a02, a10, a11, a12, a20, a21, a22,
            ))
        }
    }

    /// Return the matrix
    pub fn get_matrix(&self, matrix: &mut [f32; 16]) {
        unsafe {
            ffi::sfTransform_getMatrix(&self.0, matrix.as_mut_ptr());
        }
    }

    /// The identity transform (does nothing)
    pub const IDENTITY: Self = Transform(ffi::sfTransform {
        matrix: [1., 0., 0., 0., 1., 0., 0., 0., 1.],
    });

    /// Return the inverse of a transform
    ///
    /// If the inverse cannot be computed, a new identity transform
    /// is returned.
    ///
    /// Return the inverse matrix
    #[must_use]
    pub fn inverse(&self) -> Transform {
        unsafe { Transform(ffi::sfTransform_getInverse(&self.0)) }
    }

    /// Combine two transforms
    ///
    /// The result is a transform that is equivalent to applying
    /// transform followed by other. Mathematically, it is
    /// equivalent to a matrix multiplication.
    ///
    /// # Arguments
    /// * other - Transform to combine to transform
    pub fn combine(&mut self, other: &Transform) {
        unsafe { ffi::sfTransform_combine(&mut self.0, &other.0) }
    }

    /// Combine a transform with a translation
    ///
    /// # Arguments
    /// * x - Offset to apply on X axis
    /// * y - Offset to apply on Y axis
    pub fn translate(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfTransform_translate(&mut self.0, x, y) }
    }

    /// Combine the current transform with a rotation
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfTransform_rotate(&mut self.0, angle) }
    }

    /// Combine the current transform with a rotation
    ///
    /// The center of rotation is provided for convenience as a second
    /// argument, so that you can build rotations around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), rotate(angle), translate(center)].
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    /// * `center_x` - X coordinate of the center of rotation
    /// * `center_y` - Y coordinate of the center of rotation
    pub fn rotate_with_center(&mut self, angle: f32, center_x: f32, center_y: f32) {
        unsafe { ffi::sfTransform_rotateWithCenter(&mut self.0, angle, center_x, center_y) }
    }

    /// Combine the current transform with a scaling
    ///
    /// # Arguments
    /// * `scale_x` - Scaling factor on the X axis
    /// * `scale_y` - Scaling factor on the Y axis
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { ffi::sfTransform_scale(&mut self.0, scale_x, scale_y) }
    }

    /// Combine the current transform with a scaling
    ///
    /// The center of scaling is provided for convenience as a second
    /// argument, so that you can build scaling around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), scale(factors), translate(center)]
    ///
    /// # Arguments
    /// * `scale_x` - Scaling factor on X axis
    /// * `scale_y` - Scaling factor on Y axis
    /// * `center_x` - X coordinate of the center of scaling
    /// * `center_y` - Y coordinate of the center of scaling
    pub fn scale_with_center(&mut self, scale_x: f32, scale_y: f32, center_x: f32, center_y: f32) {
        unsafe {
            ffi::sfTransform_scaleWithCenter(&mut self.0, scale_x, scale_y, center_x, center_y)
        }
    }

    /// Apply a transform to a 2D point
    ///
    /// # Arguments
    /// * point - Point to transform
    ///
    /// Return a transformed point
    #[must_use]
    pub fn transform_point(&self, point: Vector2f) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfTransform_transformPoint(&self.0, point.raw())) }
    }

    /// Apply a transform to a rectangle
    ///
    /// Since SFML doesn't provide support for oriented rectangles,
    /// the result of this function is always an axis-aligned
    /// rectangle. Which means that if the transform contains a
    /// rotation, the bounding rectangle of the transformed rectangle
    /// is returned.
    ///
    /// # Arguments
    /// rectangle - Rectangle to transform
    ///
    /// Return the transformed rectangle
    #[must_use]
    pub fn transform_rect(&self, rectangle: &FloatRect) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfTransform_transformRect(&self.0, rectangle.raw())) }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}
