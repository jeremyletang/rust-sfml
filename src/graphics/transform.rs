/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

#![allow(missing_doc)]

//! Define a 3x3 transform matrix.
//!
//! A Transform specifies how to translate,
//! rotate, scale, shear, project, whatever things.

use libc::c_float;

use system::vector2::Vector2f;
use graphics::FloatRect;

use ffi = ffi::graphics::transform;

/// Define a 3x3 transform matrix.
///
/// A Transform specifies how to translate,
/// rotate, scale, shear, project, whatever things.
pub struct Transform {
    pub a00: f32,
    pub a01: f32,
    pub a02: f32,
    pub a10: f32,
    pub a11: f32,
    pub a12: f32,
    pub a20: f32,
    pub a21: f32,
    pub a22: f32
}

impl Transform {
    /// Create a new transform from a matrix
    ///
    /// # Arguments
    /// * a00 - Element (0, 0) of the matrix
    /// * a01 - Element (0, 1) of the matrix
    /// * a02 - Element (0, 2) of the matrix
    /// * a10 - Element (1, 0) of the matrix
    /// * a11 - Element (1, 1) of the matrix
    /// * a12 - Element (1, 2) of the matrix
    /// * a20 - Element (2, 0) of the matrix
    /// * a21 - Element (2, 1) of the matrix
    /// * a22 - Element (2, 2) of the matrix
    ///
    /// Return a new Transform
    pub fn new(a00: f32, a01: f32, a02: f32,
               b10: f32, b11: f32, b12: f32,
               c20: f32, c21: f32, c22: f32) -> Transform {
        unsafe {
            ffi::sfTransform_fromMatrix(a00, a01, a02,
                                        b10, b11, b12,
                                        c20, c21, c22)
        }
    }

    /// Return the matrix
    pub fn get_matrix(&mut self) -> [f32, ..16] {
        unsafe {
            let matrix: [f32, ..16] =
                [0.,0.,0.,0.,
                 0.,0.,0.,0.,
                 0.,0.,0.,0.,
                 0.,0.,0.,0.];
            ffi::sfTransform_getMatrix(self, matrix.as_ptr() as *mut f32);
            matrix
        }
    }

    /// Create a new identity transform
    ///
    /// Return a new Transform initialized at 1, 0, 0, 0, 1, 0, 0, 0, 1
    pub fn new_identity() -> Transform {
        unsafe {
            ffi::sfTransform_fromMatrix(1., 0., 0., 0., 1., 0., 0., 0., 1.)
        }
    }

    /// Return the inverse of a transform
    ///
    /// If the inverse cannot be computed, a new identity transform
    /// is returned.
    ///
    /// Return the inverse matrix
    pub fn get_inverse(&mut self) -> Transform {
        unsafe {
            ffi::sfTransform_getInverse(self)
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
    pub fn combine(&mut self, other: &mut Transform) -> () {
        unsafe {
            ffi::sfTransform_combine(self, other)
        }
    }

    /// Combine a transform with a translation
    ///
    /// # Arguments
    /// * x - Offset to apply on X axis
    /// * y - Offset to apply on Y axis
    pub fn translate(&mut self, x: f32, y: f32) -> () {
        unsafe {
            ffi::sfTransform_translate(self, x as c_float, y as c_float)
        }
    }

    /// Combine the current transform with a rotation
    ///
    /// # Arguments
    /// * angle - Rotation angle, in degrees
    pub fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfTransform_rotate(self, angle as c_float)
        }
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
    /// * center_x - X coordinate of the center of rotation
    /// * center_y - Y coordinate of the center of rotation
    pub fn rotate_with_center(&mut self,
                              angle: f32,
                              center_x: f32,
                              center_y: f32) -> () {
        unsafe {
            ffi::sfTransform_rotateWithCenter(self,
                                              angle as c_float,
                                              center_x as c_float,
                                              center_y as c_float)
        }
    }

    /// Combine the current transform with a scaling
    ///
    /// # Arguments
    /// * scale_x - Scaling factor on the X axis
    /// * scale_y - Scaling factor on the Y axis
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) -> () {
        unsafe {
            ffi::sfTransform_scale(self, scale_x as c_float, scale_y as c_float)
        }
    }

    /// Combine the current transform with a scaling
    ///
    /// The center of scaling is provided for convenience as a second
    /// argument, so that you can build scaling around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), scale(factors), translate(center)]
    ///
    /// # Arguments
    /// * scale_x - Scaling factor on X axis
    /// * scale_y - Scaling factor on Y axis
    /// * center_x - X coordinate of the center of scaling
    /// * center_y - Y coordinate of the center of scaling
    pub fn scale_with_center(&mut self,
                             scale_x: f32,
                             scale_y: f32,
                             center_x: f32,
                             center_y: f32) -> () {
        unsafe {
            ffi::sfTransform_scaleWithCenter(self,
                                             scale_x,
                                             scale_y,
                                             center_x,
                                             center_y)
        }
    }

    /// Apply a transform to a 2D point
    ///
    /// # Arguments
    /// * point - Point to transform
    ///
    /// Return a transformed point
    pub fn transform_point(&mut self, point: &Vector2f) -> Vector2f {
        unsafe {
            ffi::sfTransform_transformPoint(self, *point)
        }
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
    pub fn transform_rect(&mut self, rectangle: &FloatRect) -> FloatRect {
        unsafe {
            ffi::sfTransform_transformRect(self, *rectangle)
        }
    }
}
