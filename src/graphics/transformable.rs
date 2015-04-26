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

//use libc::c_float;

use traits::Wrappable;
use graphics::Transform;
use system::vector2::Vector2f;
//use std::convert::{AsRef, AsMut};

use ffi::graphics::transformable as ffi;

/// Data container for the `Transformable` trait.
pub struct TransformableData {
    raw: *mut ffi::sfTransformable
}

impl TransformableData {
    /// Create a new transformable
    ///
    /// Return Some(Transformable) or None
    pub fn new() -> Option<TransformableData> {
        let tran = unsafe { ffi::sfTransformable_create() };
        if tran.is_null() {
            None
        } else {
            Some(TransformableData {
				raw: tran
			})
        }
    }

    /// Copy an existing transformable
    ///
    /// Return Some(Transformable) or None
    pub fn clone_opt(&self) -> Option<TransformableData> {
        let tran = unsafe { ffi::sfTransformable_copy(self.raw) };
        if tran.is_null() {
            None
        } else {
            Some(TransformableData {
				raw: tran
			})
        }
    }
}

impl Clone for TransformableData {
    /// Return a new Transformable or panic! if there is not enough memory
    fn clone(&self) -> TransformableData {
		self.clone_opt().expect("Not enough memory to clone Transformable")
    }
}

impl Wrappable<*mut ffi::sfTransformable> for TransformableData {
    fn wrap(transformable: *mut ffi::sfTransformable) -> TransformableData {
        TransformableData {
            raw: transformable
        }
    }

    fn unwrap(&self) -> *mut ffi::sfTransformable {
        self.raw
    }
}

impl Drop for TransformableData {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfTransformable_destroy(self.raw)
        }
    }
}

/// Decomposed transform defined by a position, a rotation and a scale.
pub trait Transformable {
    /// Set the position of a transformable
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a transformable Transformable object is (0, 0).
    ///
    /// # Arguments
    /// * position - The new position
    fn set_position(&mut self, position: &Vector2f) -> ();

    /// Set the orientation of a transformable
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a transformable Transformable object is 0.
    ///
    /// # Arguments
    /// * angle - The new rotation, in degrees
    fn set_rotation(&mut self, angle: f32) -> ();

    /// Set the scale factors of a transformable
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a transformable Transformable object is (1, 1).
    ///
    /// # Arguments
    /// * scale - New scale factors
    fn set_scale(&mut self, scale: &Vector2f) -> ();

    /// Set the local origin of a transformable
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a transformable Transformable object is (0, 0).
    ///
    /// # Arguments
    /// * origin - New origin
    fn set_origin(&mut self, origin: &Vector2f) -> ();

    /// Get the position of a transformable
    ///
    /// Return the current position
    fn get_position(&self) -> Vector2f;

    /// Get the orientation of a transformable
    ///
    /// The rotation is always in the range [0, 360].
    ///
    /// Return the current rotation, in degrees
    fn get_rotation(&self) -> f32;

    /// Get the current scale of a transformable
    ///
    /// Return the current scale factors
    fn get_scale(&self) -> Vector2f;

    /// Get the local origin of a transformable
    ///
    /// Return the current origin
    fn get_origin(&self) -> Vector2f;

    /// Move a transformable by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    fn move_(&mut self, offset: &Vector2f) -> ();

    /// Rotate a transformable
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - Angle of rotation, in degrees
    fn rotate(&mut self, angle: f32) -> ();

    /// Scale a transformable
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    fn scale(&mut self, factors: &Vector2f) -> ();

    /// Get the combined transform of a transformable
    ///
    /// Return the transform combining the
    /// position/rotation/scale/origin of the object
    fn get_transform(&self) -> Transform;

    /// Get the inverse of the combined transform of a transformable
    ///
    /// Return the inverse of the combined transformations applied to the object
    fn get_inverse_transform(&self) -> Transform;
}

/*impl<T: Upcast<TransformableData>> Transformable for T {
    fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setPosition(self.upcast_mut().raw, *position)
        }
    }

    fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfTransformable_setRotation(self.upcast_mut().raw, angle as c_float)
        }
    }

    fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setScale(self.upcast_mut().raw, *scale)
        }
    }

    fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setOrigin(self.upcast_mut().raw, *origin)
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getPosition(self.upcast().raw)
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfTransformable_getRotation(self.upcast().raw) as f32
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getScale(self.upcast().raw)
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getOrigin(self.upcast().raw)
        }
    }

    fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_move(self.upcast_mut().raw, *offset)
        }
    }

    fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfTransformable_rotate(self.upcast_mut().raw, angle as c_float)
        }
    }

    fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_scale(self.upcast_mut().raw, *factors)
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfTransformable_getTransform(self.upcast().raw)
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfTransformable_getInverseTransform(self.upcast().raw)
        }
    }
}*/

