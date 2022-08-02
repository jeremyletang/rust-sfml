use crate::{ffi::graphics as ffi, graphics::FloatRect, system::Vector2f};

/// Define a 3x3 transform matrix.
///
/// A `Transform` specifies how to translate,
/// rotate, scale, shear, project, whatever things.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub(crate) matrix: [f32; 16],
}

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
        Self {
            matrix: [
                a00, a10, 0., a20, a01, a11, 0., a21, 0., 0., 1., 0., a02, a12, 0., a22,
            ],
        }
    }

    /// Return the matrix
    #[must_use]
    pub fn get_matrix(&self) -> &[f32; 16] {
        &self.matrix
    }

    /// The identity transform (does nothing)
    pub const IDENTITY: Self = Self {
        matrix: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };

    /// Return the inverse of a transform
    ///
    /// If the inverse cannot be computed, a new identity transform
    /// is returned.
    ///
    /// Return the inverse matrix
    #[must_use]
    pub fn inverse(&self) -> Transform {
        // Directly translated from SFML source code.
        let det = self.matrix[0]
            * (self.matrix[15] * self.matrix[5] - self.matrix[7] * self.matrix[13])
            - self.matrix[1]
                * (self.matrix[15] * self.matrix[4] - self.matrix[7] * self.matrix[12])
            + self.matrix[3]
                * (self.matrix[13] * self.matrix[4] - self.matrix[5] * self.matrix[12]);

        // Compute the inverse if the determinant is not zero
        // (don't use an epsilon because the determinant may *really* be tiny)
        if det != 0. {
            Self::new(
                (self.matrix[15] * self.matrix[5] - self.matrix[7] * self.matrix[13]) / det,
                -(self.matrix[15] * self.matrix[4] - self.matrix[7] * self.matrix[12]) / det,
                (self.matrix[13] * self.matrix[4] - self.matrix[5] * self.matrix[12]) / det,
                -(self.matrix[15] * self.matrix[1] - self.matrix[3] * self.matrix[13]) / det,
                (self.matrix[15] * self.matrix[0] - self.matrix[3] * self.matrix[12]) / det,
                -(self.matrix[13] * self.matrix[0] - self.matrix[1] * self.matrix[12]) / det,
                (self.matrix[7] * self.matrix[1] - self.matrix[3] * self.matrix[5]) / det,
                -(self.matrix[7] * self.matrix[0] - self.matrix[3] * self.matrix[4]) / det,
                (self.matrix[5] * self.matrix[0] - self.matrix[1] * self.matrix[4]) / det,
            )
        } else {
            Self::IDENTITY
        }
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
        unsafe { ffi::sfTransform_combine(self, other) }
    }

    /// Combine a transform with a translation
    ///
    /// # Arguments
    /// * x - Offset to apply on X axis
    /// * y - Offset to apply on Y axis
    pub fn translate(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfTransform_translate(self, x, y) }
    }

    /// Combine the current transform with a rotation
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfTransform_rotate(self, angle) }
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
        unsafe { ffi::sfTransform_rotateWithCenter(self, angle, center_x, center_y) }
    }

    /// Combine the current transform with a scaling
    ///
    /// # Arguments
    /// * `scale_x` - Scaling factor on the X axis
    /// * `scale_y` - Scaling factor on the Y axis
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { ffi::sfTransform_scale(self, scale_x, scale_y) }
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
        unsafe { ffi::sfTransform_scaleWithCenter(self, scale_x, scale_y, center_x, center_y) }
    }

    /// Apply a transform to a 2D point
    ///
    /// # Arguments
    /// * point - Point to transform
    ///
    /// Return a transformed point
    #[must_use]
    pub fn transform_point(&self, point: Vector2f) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfTransform_transformPoint(self, point.raw())) }
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
        unsafe { FloatRect::from_raw(ffi::sfTransform_transformRect(self, rectangle.raw())) }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}
