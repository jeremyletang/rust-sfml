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

use libc::c_float;

use system::Vector2f;
use graphics::FloatRect;

use ffi::graphics as ffi;

/// A 3x3 transformation matrix.
///
/// A `Transform` specifies how to translate, rotate, scale, shear, project,
/// etc. things. In mathematical terms, it defines how to transform a coordinate
/// system into another.
///
/// For example, if you apply a rotation transform to a sprite, the result will
/// be a rotated sprite. Anything that is transformed by this rotation transform
/// will be rotated the same way, according to its initial position.
///
/// Transforms are typically used for drawing, but they can also be used for any
/// computation that requires to transform points between the local and global
/// coordinate systems of an entity (such as collision detection).
#[repr(C)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
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
    /// Create a new transform from the given 3x3 matrix.
    pub fn new(a00: f32, a01: f32, a02: f32,
               a10: f32, a11: f32, a12: f32,
               a20: f32, a21: f32, a22: f32) -> Transform {
		Transform {
			a00: a00, a01: a01, a02: a02,
			a10: a10, a11: a11, a12: a12,
			a20: a20, a21: a21, a22: a22
		}
    }

    /// Create a new identity transform (one that does nothing).
    ///
	/// The initial values will be [[1, 0, 0], [0, 1, 0], [0, 0, 1]].
    pub fn new_identity() -> Transform {
		Transform::new(1., 0., 0., 0., 1., 0., 0., 0., 1.)
    }

    /// Return the transform as a 4x4 matrix.
    pub fn get_matrix(&self) -> [f32; 16] {
		let mut matrix = [0.; 16];
        unsafe { ffi::sfTransform_getMatrix(self, matrix.as_mut_ptr()) }
		matrix
    }

    /// Return the inverse of the transform.
	///
	/// If the inverse cannot be computed, an identity transform is returned.
    pub fn get_inverse(&self) -> Transform {
        unsafe { ffi::sfTransform_getInverse(self) }
    }

    /// Combine this transform with another one.
    ///
    /// The result is a transform that is equivalent to applying `self`
    /// followed by `other`. Mathematically, it is equivalent to a matrix
	/// multiplication.
    pub fn combine(&mut self, other: &Transform) {
        unsafe { ffi::sfTransform_combine(self, other) }
    }

    /// Combine this transform with a translation.
    pub fn translate(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfTransform_translate(self, x as c_float, y as c_float) }
    }

    /// Combine this transform with a rotation.
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfTransform_rotate(self, angle as c_float) }
    }

	/// Combine this transform with a rotation around a center.
    ///
    /// The center of rotation is provided for convenience as a second
    /// argument, so that you can build rotations around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), rotate(angle), translate(center)].
    pub fn rotate_with_center(&mut self,
                              angle: f32,
                              center_x: f32,
                              center_y: f32) {
        unsafe {
            ffi::sfTransform_rotateWithCenter(self,
                                              angle as c_float,
                                              center_x as c_float,
                                              center_y as c_float)
        }
    }

    /// Combine the current transform with a scale.
    pub fn scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfTransform_scale(self, scale_x as c_float, scale_y as c_float)
        }
    }

    /// Combine the current transform with a scale around a center.
    ///
    /// The center of scaling is provided for convenience as a second
    /// argument, so that you can build scaling around arbitrary points
    /// more easily (and efficiently) than the usual
    /// [translate(-center), scale(factors), translate(center)].
    pub fn scale_with_center(&mut self,
                             scale_x: f32,
                             scale_y: f32,
                             center_x: f32,
                             center_y: f32) {
        unsafe {
            ffi::sfTransform_scaleWithCenter(self,
                                             scale_x,
                                             scale_y,
                                             center_x,
                                             center_y)
        }
    }

	/// Apply this transform to a 2D point.
    pub fn transform_point(&self, point: Vector2f) -> Vector2f {
        unsafe { ffi::sfTransform_transformPoint(self, point) }
    }

    /// Apply this transform to a rectangle.
    ///
    /// Since SFML doesn't provide support for oriented rectangles,
    /// the result of this function is always an axis-aligned
    /// rectangle. This means that if the transform contains a
    /// rotation, the bounding rectangle of the transformed rectangle
    /// is returned.
    pub fn transform_rect(&self, rectangle: FloatRect) -> FloatRect {
        unsafe { ffi::sfTransform_transformRect(self, rectangle) }
    }
}

impl Default for Transform {
	fn default() -> Transform {
		Transform::new_identity()
	}
}
