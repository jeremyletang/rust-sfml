use std::ops::{Mul, MulAssign};

use crate::{
    ffi::graphics as ffi,
    graphics::FloatRect,
    system::{Angle, Vector2f},
};

/// Define a 3x3 transform matrix.
///
/// A `Transform` specifies how to translate,
/// rotate, scale, shear, project, whatever things.
///
/// Internally the matrix is stored as 16 floats in the layout used by SFML
/// and OpenGL, not as a compact 9-float array.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub(crate) matrix: [f32; 16],
}

impl Transform {
    /// Creates a new transform from a 3x3 matrix.
    ///
    /// The matrix is stored in the crate's 16-float internal layout; the
    /// additional elements hold the homogeneous row and column.
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
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// let transform = Transform::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
    /// let matrix = transform.matrix();
    /// assert_eq!(matrix[0], 1.);
    /// assert_eq!(matrix[1], 4.);
    /// assert_eq!(matrix[4], 2.);
    /// ```
    #[expect(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        a00: f32,
        a01: f32,
        a02: f32,
        a10: f32,
        a11: f32,
        a12: f32,
        a20: f32,
        a21: f32,
        a22: f32,
    ) -> Self {
        Self {
            matrix: [
                a00, a10, 0., a20, a01, a11, 0., a21, 0., 0., 1., 0., a02, a12, 0., a22,
            ],
        }
    }

    /// Return the matrix
    #[must_use]
    pub const fn matrix(&self) -> &[f32; 16] {
        &self.matrix
    }

    /// The identity transform (does nothing)
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::Vector2f;
    /// assert_eq!(
    ///     Transform::IDENTITY.transform_point(Vector2f::new(3., 4.)),
    ///     Vector2f::new(3., 4.)
    /// );
    /// ```
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
    pub const fn inverse(&self) -> Self {
        // Directly translated from SFML source code.
        let det = self.matrix[0]
            * (self.matrix[15] * self.matrix[5] - self.matrix[7] * self.matrix[13])
            - self.matrix[1]
                * (self.matrix[15] * self.matrix[4] - self.matrix[7] * self.matrix[12])
            + self.matrix[3]
                * (self.matrix[13] * self.matrix[4] - self.matrix[5] * self.matrix[12]);

        // Compute the inverse if the determinant is not zero
        // (don't use an epsilon because the determinant may *really* be tiny)
        if det == 0. {
            Self::IDENTITY
        } else {
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
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::Vector2f;
    /// let mut transform = Transform::IDENTITY;
    /// transform.translate(Vector2f::new(10., 0.));
    ///
    /// let mut other = Transform::IDENTITY;
    /// other.translate(Vector2f::new(0., 5.));
    ///
    /// transform.combine(&other);
    /// assert_eq!(
    ///     transform.transform_point(Vector2f::new(0., 0.)),
    ///     Vector2f::new(10., 5.)
    /// );
    /// ```
    pub fn combine(&mut self, other: &Transform) {
        unsafe { ffi::sfTransform_combine(self, other) }
    }

    /// Combine a transform with a translation
    ///
    /// # Arguments
    /// * x - Offset to apply on X axis
    /// * y - Offset to apply on Y axis
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::Vector2f;
    /// let mut transform = Transform::IDENTITY;
    /// transform.translate(Vector2f::new(10., 20.));
    /// assert_eq!(
    ///     transform.transform_point(Vector2f::new(0., 0.)),
    ///     Vector2f::new(10., 20.)
    /// );
    /// ```
    pub fn translate(&mut self, translate: Vector2f) {
        unsafe { ffi::sfTransform_translate(self, translate) }
    }

    /// Combine the current transform with a rotation
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::{Angle, Vector2f};
    /// let mut transform = Transform::IDENTITY;
    /// transform.rotate(Angle::degrees(90.));
    /// let point = transform.transform_point(Vector2f::new(1., 0.));
    /// assert!((point.x - 0.).abs() < 1e-5);
    /// assert!((point.y - 1.).abs() < 1e-5);
    /// ```
    pub fn rotate(&mut self, angle: Angle) {
        unsafe { ffi::sfTransform_rotate(self, angle.as_degrees()) }
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
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::{Angle, Vector2f};
    /// let mut transform = Transform::IDENTITY;
    /// transform.rotate_with_center(Angle::degrees(180.), Vector2f::new(1., 0.));
    /// let point = transform.transform_point(Vector2f::new(0., 0.));
    /// assert!((point.x - 2.).abs() < 1e-5);
    /// assert!(point.y.abs() < 1e-5);
    /// ```
    pub fn rotate_with_center(&mut self, angle: Angle, center: Vector2f) {
        unsafe { ffi::sfTransform_rotateWithCenter(self, angle.as_degrees(), center) }
    }

    /// Combine the current transform with a scaling
    ///
    /// # Arguments
    /// * `scale_x` - Scaling factor on the X axis
    /// * `scale_y` - Scaling factor on the Y axis
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::Vector2f;
    /// let mut transform = Transform::IDENTITY;
    /// transform.scale(Vector2f::new(2., 3.));
    /// assert_eq!(
    ///     transform.transform_point(Vector2f::new(1., 1.)),
    ///     Vector2f::new(2., 3.)
    /// );
    /// ```
    pub fn scale(&mut self, scale: Vector2f) {
        unsafe { ffi::sfTransform_scale(self, scale) }
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
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::Vector2f;
    /// let mut transform = Transform::IDENTITY;
    /// transform.scale_with_center(Vector2f::new(2., 2.), Vector2f::new(1., 1.));
    /// assert_eq!(
    ///     transform.transform_point(Vector2f::new(2., 1.)),
    ///     Vector2f::new(3., 1.)
    /// );
    /// ```
    pub fn scale_with_center(&mut self, scale: Vector2f, center: Vector2f) {
        unsafe { ffi::sfTransform_scaleWithCenter(self, scale, center) }
    }

    /// Apply a transform to a 2D point
    ///
    /// # Arguments
    /// * point - Point to transform
    ///
    /// Return a transformed point
    ///
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::{Angle, Vector2f};
    /// let mut transform = Transform::IDENTITY;
    /// transform.translate(Vector2f::new(5., 5.));
    /// transform.scale(Vector2f::new(2., 2.));
    /// assert_eq!(
    ///     transform.transform_point(Vector2f::new(1., 0.)),
    ///     Vector2f::new(7., 5.)
    /// );
    /// ```
    #[must_use]
    pub fn transform_point(&self, point: Vector2f) -> Vector2f {
        unsafe { ffi::sfTransform_transformPoint(self, point) }
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
    ///
    /// ```
    /// # use sfml::graphics::{FloatRect, Transform};
    /// # use sfml::system::Vector2f;
    /// let mut transform = Transform::IDENTITY;
    /// transform.translate(Vector2f::new(5., 10.));
    /// let rect = FloatRect::new(Vector2f::new(0., 0.), Vector2f::new(2., 2.));
    /// assert_eq!(
    ///     transform.transform_rect(rect),
    ///     FloatRect::new(Vector2f::new(5., 10.), Vector2f::new(2., 2.))
    /// );
    /// ```
    #[must_use]
    pub fn transform_rect(&self, rectangle: FloatRect) -> FloatRect {
        unsafe { ffi::sfTransform_transformRect(self, rectangle) }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Transform {
    type Output = Self;

    /// Combines two transformations
    ///
    /// Equivalent to calling `left.combine(&right)`
    ///
    /// Returns a new combined Transform
    /// ```
    /// # use sfml::graphics::Transform;
    /// let test_transform = Transform::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
    /// let result = Transform::new(30., 36., 42., 66., 81., 96., 102., 126., 150.);
    /// assert_eq!(test_transform * test_transform, result);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        let mut clone_self = self;
        clone_self.combine(&rhs);
        clone_self
    }
}

impl MulAssign for Transform {
    /// Combines two transformations
    ///
    /// Equivalent to calling `left.combine(&right)`
    ///
    /// Combines the other transform into this one in place.
    /// ```
    /// # use sfml::graphics::Transform;
    /// let mut test_transform = Transform::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
    /// let result = Transform::new(30., 36., 42., 66., 81., 96., 102., 126., 150.);
    /// test_transform *= test_transform;
    /// assert_eq!(test_transform, result);
    /// ```
    fn mul_assign(&mut self, rhs: Self) {
        self.combine(&rhs);
    }
}

impl Mul<Vector2f> for Transform {
    type Output = Vector2f;

    /// Transforms a point
    ///
    /// Equivalent to calling `left.transform_point(right)`
    ///
    /// Returns a new transformed points
    /// ```
    /// # use sfml::graphics::Transform;
    /// # use sfml::system::Vector2f;
    /// let test_transform = Transform::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
    /// let test_point = Vector2f::new(2., 3.);
    /// let result = Vector2f::new(11., 29.);
    /// assert_eq!(test_transform * test_point, result);
    /// ```
    fn mul(self, rhs: Vector2f) -> Self::Output {
        self.transform_point(rhs)
    }
}
