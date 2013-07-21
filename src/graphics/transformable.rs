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
* Target for off-screen 2D rendering into a texture
*
*
*
*/

use std::libc::{c_float};
use std::ptr;

use traits::wrappable::Wrappable;
use system::vector2::Vector2f;
use graphics::transform::Transform;
    
#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_float, c_void};

    use system::vector2::Vector2f;
    use graphics::transform::Transform;
    
    pub struct sfTransformable {
        This : *c_void,
        transform : Transform,
        inverseTransform : Transform
    }

    extern "C" {
        pub fn sfTransformable_create() -> *sfTransformable;
        pub fn sfTransformable_copy(transformable : *sfTransformable) -> *sfTransformable;
        pub fn sfTransformable_destroy(transformable : *sfTransformable) -> ();
        pub fn sfTransformable_setPosition(transformable : *sfTransformable, position : Vector2f) -> ();
        pub fn sfTransformable_setRotation(transformable : *sfTransformable, angle : c_float) -> ();
        pub fn sfTransformable_setScale(transformable : *sfTransformable, scale : Vector2f) -> ();
        pub fn sfTransformable_setOrigin(transformable : *sfTransformable, origin : Vector2f) -> ();
        pub fn sfTransformable_getPosition(transformable : *sfTransformable) -> Vector2f;
        pub fn sfTransformable_getRotation(transformable : *sfTransformable) -> c_float;
        pub fn sfTransformable_getScale(transformable : *sfTransformable) -> Vector2f;
        pub fn sfTransformable_getOrigin(transformable : *sfTransformable) -> Vector2f;
        pub fn sfTransformable_move(transformable : *sfTransformable, offset : Vector2f) -> ();
        pub fn sfTransformable_rotate(transformable : *sfTransformable, angle : c_float) -> ();
        pub fn sfTransformable_scale(transformable : *sfTransformable, factors : Vector2f) -> ();
        pub fn sfTransformable_getTransform(transformable : *sfTransformable) -> Transform;
        pub fn sfTransformable_getInverseTransform(transformable : *sfTransformable) -> Transform;
    }
}

#[doc(hidden)]
pub struct Transformable{
    priv trans : *ffi::sfTransformable
}

impl Transformable {
    /**
    * Create a new transformable
    *
    * Return a new Transformable object
    */
    pub fn new() -> Option<Transformable> {
        let tran = unsafe { ffi::sfTransformable_create() };
        if ptr::is_null(tran) {
            None
        }
        else {
            Some(Transformable {
                trans : tran
            })
        }
    }
    
    /**
    * Copy an existing transformable
    *
    * Return the copied object
    */
    pub fn clone(&self) -> Option<Transformable> {
        let tran = unsafe { ffi::sfTransformable_copy(self.trans) };
        if ptr::is_null(tran) {
            None
        }
        else {
            Some(Transformable {
                trans :tran
            }) 
        }
    }

    /**
    * Set the position of a transformable
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a transformable Transformable object is (0, 0).
    * 
    * # Arguments
    * * position - The new position
    */
    pub fn set_position(&mut self, position : &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setPosition(self.trans, *position)
        }
    }
    
    /**
    * Set the orientation of a transformable
    *
    * This function completely overwrites the previous rotation.
    * See rotate to add an angle based on the previous rotation instead.
    * The default rotation of a transformable Transformable object is 0.
    *
    * # Arguments
    * * angle - The new rotation, in degrees
    */
    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfTransformable_setRotation(self.trans, angle as c_float)
        }
    }

    /**
    * Set the scale factors of a transformable
    *
    * This function completely overwrites the previous scale.
    * See scale to add a factor based on the previous scale instead.
    * The default scale of a transformable Transformable object is (1, 1).
    * 
    * # Arguments
    * * scale - New scale factors
    */
    pub fn set_scale(&mut self, scale : &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setScale(self.trans, *scale)
        }
    }

    /**
    * Set the local origin of a transformable
    *
    * The origin of an object defines the center point for
    * all transformations (position, scale, rotation).
    * The coordinates of this point must be relative to the
    * top-left corner of the object, and ignore all
    * transformations (position, scale, rotation).
    * The default origin of a transformable Transformable object is (0, 0).
    *
    * # Arguments
    * * origin - New origin
    */
    pub fn set_origin(&mut self, origin : &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_setOrigin(self.trans, *origin)
        }
    }
    
    /**
    * Get the position of a transformable
    *
    * Return the current position
    */
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getPosition(self.trans)
        }
    }

    /**
    * Get the orientation of a transformable
    *
    * The rotation is always in the range [0, 360].
    * 
    * Return the current rotation, in degrees
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfTransformable_getRotation(self.trans) as float
        }
    }

    /**
    * Get the current scale of a transformable
    *
    * Return the current scale factors
    */
    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getScale(self.trans)
        }
    }

    /**
    * Get the local origin of a transformable
    *
    * Return the current origin
    */
    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfTransformable_getOrigin(self.trans)
        }
    }

    /**
    * Move a transformable by a given offset
    *
    * This function adds to the current position of the object,
    * unlike set_position which overwrites it.
    *
    * # Arguments
    * * offset - Offset
    */
    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_move(self.trans, *offset)
        }
    }

    /**
    * Rotate a transformable
    *
    * This function adds to the current rotation of the object,
    * unlike set_rotation which overwrites it.
    *
    * # Arguments
    * * angle - Angle of rotation, in degrees
    */
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfTransformable_rotate(self.trans, angle as c_float)
        }
    }

    /**
    * Scale a transformable
    *
    * This function multiplies the current scale of the object,
    * unlike set_scale which overwrites it.
    *
    * # Arguments
    * * factors - Scale factors
    */
    pub fn scale(&mut self, factors : &Vector2f) -> () {
        unsafe {
            ffi::sfTransformable_scale(self.trans, *factors)
        }
    }

    /**
    * Get the combined transform of a transformable
    *
    * Return the transform combining the position/rotation/scale/origin of the object
    */
    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfTransformable_getTransform(self.trans)
        }
    }

    /**
    * Get the inverse of the combined transform of a transformable
    *
    * Return the inverse of the combined transformations applied to the object
    */
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfTransformable_getInverseTransform(self.trans)
        }
    }
}

#[doc(hidden)]
impl Wrappable<*ffi::sfTransformable> for Transformable {
    pub fn wrap(transformable : *ffi::sfTransformable) -> Transformable {
        Transformable {
            trans : transformable
        }
    }

    pub fn unwrap(&self) -> *ffi::sfTransformable {
        self.trans
    }
}

impl Drop for Transformable {
    fn drop(&self) -> () {
        unsafe {
            ffi::sfTransformable_destroy(self.trans)
        }
    }
}

