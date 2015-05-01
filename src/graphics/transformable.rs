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

use graphics::Transform;
use system::Vector2f;

use ffi::Foreign;
use ffi::graphics as ffi;

/// Reusable implementation of `Transformable`.
pub struct BasicTransformable(Foreign<ffi::sfTransformable>);

impl BasicTransformable {
    /// Create a new transformable
    ///
    /// Return Some(Transformable) or None
    pub fn new() -> Option<BasicTransformable> {
		unsafe {
			Foreign::new(ffi::sfTransformable_create())
		}.map(BasicTransformable)
    }

    /// Copy an existing transformable
    ///
    /// Return Some(Transformable) or None
    pub fn clone_opt(&self) -> Option<BasicTransformable> {
        unsafe {
			Foreign::new(ffi::sfTransformable_copy(self.raw()))
		}.map(BasicTransformable)
    }

	fn raw(&self) -> &ffi::sfTransformable { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfTransformable { self.0.as_mut() }
}

impl Clone for BasicTransformable {
    fn clone(&self) -> BasicTransformable {
		self.clone_opt().expect("Not enough memory to clone Transformable")
    }
}

/// Holder of a decomposed transform defined by a position, a rotation and a
/// scale.
pub trait Transformable {
    /// Set the position of the object.
    ///
    /// This function completely overwrites the previous position.
    /// See `move` to apply an offset based on the previous position instead.
    /// The default position is (0, 0).
    fn set_position(&mut self, position: &Vector2f);

    /// Set the position of the object.
    ///
    /// This function completely overwrites the previous position.
    /// See `move` to apply an offset based on the previous position instead.
    /// The default position is (0, 0).
	fn set_position2f(&mut self, x: f32, y: f32) {
		self.set_position(&Vector2f::new(x, y));
	}

    /// Set the orientation of the object.
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a transformable Transformable object is 0.
    ///
    /// # Arguments
    /// * angle - The new rotation, in degrees
    fn set_rotation(&mut self, angle: f32);

    /// Set the scale factors of the object.
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale is (1, 1).
    ///
    /// # Arguments
    /// * scale - New scale factors
    fn set_scale(&mut self, scale: &Vector2f);

    /// Set the scale factors of the object.
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale is (1, 1).
	fn set_scale2f(&mut self, x: f32, y: f32) {
		self.set_scale(&Vector2f::new(x, y));
	}

    /// Set the local origin of a transformable
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin is (0, 0).
    ///
    /// # Arguments
    /// * origin - New origin
    fn set_origin(&mut self, origin: &Vector2f);

    /// Set the local origin of a transformable
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin is (0, 0).
	fn set_origin2f(&mut self, x: f32, y: f32) {
		self.set_origin(&Vector2f::new(x, y))
	}

    /// Get the position of the object.
    fn get_position(&self) -> Vector2f;

    /// Get the orientation of the object, in degrees.
    ///
    /// The rotation is always in the range [0, 360].
    fn get_rotation(&self) -> f32;

    /// Get the current scale of the object.
    fn get_scale(&self) -> Vector2f;

    /// Get the local origin of the object.
    fn get_origin(&self) -> Vector2f;

    /// Move the object by a given offset.
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    fn move_(&mut self, offset: &Vector2f);

    /// Move the object by a given offset.
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
	fn move2f(&mut self, x: f32, y: f32) {
		self.move_(&Vector2f::new(x, y));
	}

    /// Rotate the object.
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - Angle of rotation, in degrees
    fn rotate(&mut self, angle: f32);

    /// Scale a transformable
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    fn scale(&mut self, factors: &Vector2f);

    /// Scale a transformable
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
	fn scale2f(&mut self, x: f32, y: f32) {
		self.scale(&Vector2f::new(x, y));
	}

    /// Get the combined transform of the object.
    ///
    /// Return the transform combining the
    /// position/rotation/scale/origin of the object
    fn get_transform(&self) -> Transform;

    /// Get the inverse of the combined transform of the object.
    ///
    /// Return the inverse of the combined transformations applied to the object
    fn get_inverse_transform(&self) -> Transform;
}

impl Transformable for BasicTransformable {
    fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setPosition(self.raw_mut(), *position)
        }
    }

    fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfTransformable_setRotation(self.raw_mut(), angle as c_float)
        }
    }

    fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setScale(self.raw_mut(), *scale)
        }
    }

    fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setOrigin(self.raw_mut(), *origin)
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getPosition(self.raw())
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfTransformable_getRotation(self.raw()) as f32
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getScale(self.raw())
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getOrigin(self.raw())
        }
    }

    fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_move(self.raw_mut(), *offset)
        }
    }

    fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfTransformable_rotate(self.raw_mut(), angle as c_float)
        }
    }

    fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_scale(self.raw_mut(), *factors)
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfTransformable_getTransform(self.raw())
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfTransformable_getInverseTransform(self.raw())
        }
    }
}
