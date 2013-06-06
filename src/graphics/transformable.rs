/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
*
*
*
*
*/

use core::libc::{c_float};
use system::vector2;
use graphics::transform::Transform;
    
#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_float, c_void};
    use system::vector2;
    use graphics::transform::Transform;
    
    pub struct sfTransformable {
        This : *c_void,
        transform : Transform,
        inverseTransform : Transform
    }

    pub extern "C" {
        fn sfTransformable_create() -> *sfTransformable;
        fn sfTransformable_copy(transformable : *sfTransformable) -> *sfTransformable;
        fn sfTransformable_destroy(transformable : *sfTransformable) -> ();
        fn sfTransformable_setPosition(transformable : *sfTransformable, position : vector2::Vector2f) -> ();
        fn sfTransformable_setRotation(transformable : *sfTransformable, angle : c_float) -> ();
        fn sfTransformable_setScale(transformable : *sfTransformable, scale : vector2::Vector2f) -> ();
        fn sfTransformable_setOrigin(transformable : *sfTransformable, origin : vector2::Vector2f) -> ();
        fn sfTransformable_getPosition(transformable : *sfTransformable) -> vector2::Vector2f;
        fn sfTransformable_getRotation(transformable : *sfTransformable) -> c_float;
        fn sfTransformable_getScale(transformable : *sfTransformable) -> vector2::Vector2f;
        fn sfTransformable_getOrigin(transformable : *sfTransformable) -> vector2::Vector2f;
        fn sfTransformable_move(transformable : *sfTransformable, offset : vector2::Vector2f) -> ();
        fn sfTransformable_rotate(transformable : *sfTransformable, angle : c_float) -> ();
        fn sfTransformable_scale(transformable : *sfTransformable, factors : vector2::Vector2f) -> ();
        fn sfTransformable_getTransform(transformable : *sfTransformable) -> Transform;
        fn sfTransformable_getInverseTransform(transformable : *sfTransformable) -> Transform;
    }
}

#[doc(hidden)]
pub struct Transformable{
    priv trans : *csfml::sfTransformable
}

impl Transformable {
    pub fn new() -> Transformable {
        Transformable { trans : unsafe {csfml::sfTransformable_create()}}
    }
    
    pub fn new_copy(transformable : &Transformable) -> Transformable {
        Transformable { trans : unsafe {csfml::sfTransformable_copy(transformable.unwrap())} }
    }

    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfTransformable_setPosition(self.trans, *position)
        }
    }
    
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfTransformable_setRotation(self.trans, angle as c_float)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfTransformable_setScale(self.trans, *scale)
        }
    }

    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfTransformable_setOrigin(self.trans, *origin)
        }
    }
    
    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe {
            csfml::sfTransformable_getPosition(self.trans)
        }
    }

    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfTransformable_getRotation(self.trans) as float
        }
    }

    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe {
            csfml::sfTransformable_getScale(self.trans)
        }
    }

    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe {
            csfml::sfTransformable_getOrigin(self.trans)
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfTransformable_move(self.trans, *offset)
        }
    }

    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfTransformable_rotate(self.trans, angle as c_float)
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfTransformable_scale(self.trans, *factors)
        }
    }

    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfTransformable_getTransform(self.trans)
        }
    }

    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            csfml::sfTransformable_getInverseTransform(self.trans)
        }
    }

    #[doc(hidden)]
    pub fn wrap(transformable : *csfml::sfTransformable) -> Transformable {
        Transformable {trans : transformable}
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfTransformable {
        self.trans
    }
}

impl Drop for Transformable {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfTransformable_destroy(self.trans)
        }
    }
}

