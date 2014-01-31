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
* Specialized shape representing a circle.
*
* 
*
*/

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_float, c_uint};

    use graphics::texture;
    use sfml_types::SfBool;
    use graphics::rect::{IntRect, FloatRect};
    use system::vector2::Vector2f;
    use graphics::color::Color;
    use graphics::transform::Transform;

    pub struct sfCircleShape {
        This :              *c_void,
        Texture :           *texture::ffi::sfTexture,
        Transform :         Transform,
        InverseTransform :  Transform
    }
    
    extern "C" {
        pub fn sfCircleShape_create() -> *sfCircleShape;
        pub fn sfCircleShape_copy(shape : *sfCircleShape) -> *sfCircleShape;
        pub fn sfCircleShape_destroy(shape : *sfCircleShape) -> ();
        pub fn sfCircleShape_setPosition(shape : *sfCircleShape, position : Vector2f) -> ();
        pub fn sfCircleShape_setRotation(shape : *sfCircleShape, angle : c_float) -> ();
        pub fn sfCircleShape_setScale(shape : *sfCircleShape, scale : Vector2f) -> ();
        pub fn sfCircleShape_setOrigin(shape : *sfCircleShape, origin : Vector2f) -> ();
        pub fn sfCircleShape_getPosition(shape : *sfCircleShape) -> Vector2f;
        pub fn sfCircleShape_getRotation(shape : *sfCircleShape) -> c_float;
        pub fn sfCircleShape_getScale(shape : *sfCircleShape) -> Vector2f;
        pub fn sfCircleShape_getOrigin(shape : *sfCircleShape) -> Vector2f;
        pub fn sfCircleShape_move(shape : *sfCircleShape, offset : Vector2f) -> ();
        pub fn sfCircleShape_rotate(shape : *sfCircleShape, angle : c_float) -> ();
        pub fn sfCircleShape_scale(shape : *sfCircleShape, factors : Vector2f) -> ();
        pub fn sfCircleShape_getTransform(shape : *sfCircleShape) -> Transform;
        pub fn sfCircleShape_getInverseTransform(shape : *sfCircleShape) -> Transform;
        pub fn sfCircleShape_setTexture(shape : *sfCircleShape, texture : *texture::ffi::sfTexture, reset_rect : SfBool) -> ();
        pub fn sfCircleShape_setTextureRect(shape : *sfCircleShape, rect : IntRect) -> ();
        pub fn sfCircleShape_setFillColor(shape : *sfCircleShape, color : Color) -> ();
        pub fn sfCircleShape_setOutlineColor(shape : *sfCircleShape, color : Color) -> ();
        pub fn sfCircleShape_setOutlineThickness(shape : *sfCircleShape, thickness : c_float) -> ();
        pub fn sfCircleShape_getTexture(shape : *sfCircleShape) -> *texture::ffi::sfTexture;
        pub fn sfCircleShape_getTextureRect(shape : *sfCircleShape) -> IntRect;
        pub fn sfCircleShape_getFillColor(shape : *sfCircleShape) -> Color;
        pub fn sfCircleShape_getOutlineColor(shape : *sfCircleShape) -> Color;
        pub fn sfCircleShape_getOutlineThickness(shape : *sfCircleShape) -> c_float;
        pub fn sfCircleShape_getPointCount(shape : *sfCircleShape) -> c_uint;
        pub fn sfCircleShape_getPoint(shape : *sfCircleShape, index : c_uint) -> ();
        pub fn sfCircleShape_setRadius(shape : *sfCircleShape, radius : c_float) -> ();
        pub fn sfCircleShape_getRadius(shape : *sfCircleShape) -> c_float;
        pub fn sfCircleShape_setPointCount(shape : *sfCircleShape, count : c_uint) -> ();
        pub fn sfCircleShape_getLocalBounds(shape : *sfCircleShape) -> FloatRect;
        pub fn sfCircleShape_getGlobalBounds(shape : *sfCircleShape) -> FloatRect;
    }
}

pub mod rc;
pub mod borrow;

