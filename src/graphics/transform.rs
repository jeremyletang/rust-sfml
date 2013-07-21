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

/*!
* Define a 3x3 transform matrix.
*
* A Transform specifies how to translate, rotate, scale, shear, project, whatever things.
*
*/

extern mod std;
pub use extra::c_vec::{CVec, len, get};
use std::libc::{c_float};
use std::vec;

use system::vector2::Vector2f;
use graphics::rect::FloatRect;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_float};

    use system::vector2::Vector2f;
    use graphics::rect::FloatRect;
    use graphics::transform::Transform;

    extern "C" {
        pub fn sfTransform_fromMatrix(a01 : f32, a02 : f32, a03 : f32, b01 : f32, b02 : f32, b03 : f32, c01 : f32, c02 : f32, c03 : f32) -> Transform;
        pub fn sfTransform_getMatrix(tranform : *Transform, matrix : *f32) -> ();
        pub fn sfTransform_getInverse(transform : *Transform) -> Transform;
        pub fn sfTransform_transformPoint(transform : *Transform, point : Vector2f) -> Vector2f;
        pub fn sfTransform_transformRect(transform : *Transform, rectangle : FloatRect) -> FloatRect;
        pub fn sfTransform_combine(transform : *Transform, other : *Transform) -> ();
        pub fn sfTransform_translate(transform : *Transform, x : c_float, y : c_float) -> ();
        pub fn sfTransform_rotate(transform : *Transform, angle : c_float) -> ();
        pub fn sfTransform_rotateWithCenter(transform : *Transform, angle : c_float, centerX : c_float, centerY : c_float) -> ();
        pub fn sfTransform_scale(transform : *Transform, scaleX : c_float, scaleY : c_float) -> ();
        pub fn sfTransform_scaleWithCenter(transform: *Transform, scaleX : c_float, scaleY : c_float, centerX : c_float, centerY : c_float) -> ();
    }
}

#[doc(hidden)]
pub struct Transform {
        a00 : f32,
        a01 : f32,
        a02 : f32,
        a10 : f32,
        a11 : f32,
        a12 : f32,
        a20 : f32,
        a21 : f32,
        a22 : f32
}

impl Transform {
    /**
    * Create a new transform from a matrix
    *
    * # Arguments
    * * a00 - Element (0, 0) of the matrix
    * * a01 - Element (0, 1) of the matrix
    * * a02 - Element (0, 2) of the matrix
    * * a10 - Element (1, 0) of the matrix
    * * a11 - Element (1, 1) of the matrix
    * * a12 - Element (1, 2) of the matrix
    * * a20 - Element (2, 0) of the matrix
    * * a21 - Element (2, 1) of the matrix
    * * a22 - Element (2, 2) of the matrix
    *
    * Return a new Transform object
    */
    pub fn new(a00 : f32, a01 : f32, a02 : f32, b10 : f32, b11 : f32, b12 : f32, c20 : f32, c21 : f32, c22 : f32) -> Transform {
        unsafe {
            ffi::sfTransform_fromMatrix(a00, a01, a02, b10, b11, b12, c20, c21, c22)
        }
    }

    pub fn get_matrix(&self) -> [f32, ..16] {
        unsafe {
            let matrix : [f32, ..16] = [0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.,0.];
            ffi::sfTransform_getMatrix(self, vec::raw::to_ptr(matrix));
            matrix
        }
    }
    
    /**
    * Create a new identity transform
    *
    * Return a new Transform object initialized at 1, 0, 0, 0, 1, 0, 0, 0, 1
    */
    pub fn new_identity() -> Transform {
        unsafe {
            ffi::sfTransform_fromMatrix(1., 0., 0., 0., 1., 0., 0., 0., 1.)
        }
    }

    /**
    * Return the inverse of a transform
    *
    * If the inverse cannot be computed, a new identity transform
    * is returned.
    * 
    * Return the inverse matrix
    */
    pub fn get_inverse(&self) -> Transform {
        unsafe {
            ffi::sfTransform_getInverse(self)
        }
    }
    
    /**
    * Combine two transforms
    *
    * The result is a transform that is equivalent to applying
    * transform followed by other. Mathematically, it is
    * equivalent to a matrix multiplication.
    *
    * # Arguments
    * * other - Transform to combine to transform
    */
    pub fn combine(&self, other : &Transform) -> () {
        unsafe {
            ffi::sfTransform_combine(self, other)
        }
    }
    
    /**
    * Combine a transform with a translation
    *
    * # Arguments
    * * x - Offset to apply on X axis
    * * y - Offset to apply on Y axis
    */
    pub fn translate(&self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfTransform_translate(self, x as c_float, y as c_float)
        }
    }

    /**
    * Combine the current transform with a rotation
    *
    * # Arguments
    * * angle - Rotation angle, in degrees
    */
    pub fn rotate(&self, angle : f32) -> () {
        unsafe {
            ffi::sfTransform_rotate(self, angle as c_float)
        }
    }
    
    /**
    * Combine the current transform with a rotation
    *
    * The center of rotation is provided for convenience as a second
    * argument, so that you can build rotations around arbitrary points
    * more easily (and efficiently) than the usual
    * [translate(-center), rotate(angle), translate(center)].
    *
    * # Arguments
    * * angle - Rotation angle, in degrees
    * * centerX - X coordinate of the center of rotation
    * * centerY - Y coordinate of the center of rotation
    */
    pub fn rotate_with_center(&self, angle : f32, centerX : f32, centerY : f32) -> () {
        unsafe {
            ffi::sfTransform_rotateWithCenter(self, angle as c_float, centerX as c_float, centerY as c_float)
        }
    }

    /**
    * Combine the current transform with a scaling
    *
    * # Arguments
    * * scaleX - Scaling factor on the X axis
    * * scaleY - Scaling factor on the Y axis
    */
    pub fn scale(&self, scaleX : f32, scaleY : f32) -> () {
        unsafe {
            ffi::sfTransform_scale(self, scaleX as c_float, scaleY as c_float)
        }
    }
    
    /**
    * Combine the current transform with a scaling
    *
    * The center of scaling is provided for convenience as a second
    * argument, so that you can build scaling around arbitrary points
    * more easily (and efficiently) than the usual
    * [translate(-center), scale(factors), translate(center)]
    *
    * # Arguments
    * * scaleX - Scaling factor on X axis
    * * scaleY - Scaling factor on Y axis
    * * centerX - X coordinate of the center of scaling
    * * centerY - Y coordinate of the center of scaling
    */
    pub fn scale_with_center(&self, scaleX : f32, scaleY : f32, centerX : f32, centerY : f32) -> () {
        unsafe {
            ffi::sfTransform_scaleWithCenter(self, scaleX, scaleY, centerX, centerY)
        }
    }

    /**
    * Apply a transform to a 2D point
    *
    * # Arguments
    * * point - Point to transform
    *
    * Return a transformed point
    */
    pub fn transform_point(&self, point : &Vector2f) -> Vector2f {
        unsafe {
            ffi::sfTransform_transformPoint(self, *point)
        }
    }

    /**
    * Apply a transform to a rectangle
    * 
    * Since SFML doesn't provide support for oriented rectangles,
    * the result of this function is always an axis-aligned
    * rectangle. Which means that if the transform contains a
    * rotation, the bounding rectangle of the transformed rectangle
    * is returned.
    *
    * # Arguments
    * rectangle - Rectangle to transform
    *
    * Return the transformed rectangle
    */
    pub fn transform_rect(&self, rectangle : &FloatRect) -> FloatRect {
        unsafe {
            ffi::sfTransform_transformRect(self, *rectangle)
        }
    }
    
}
