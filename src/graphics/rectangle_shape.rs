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

use std::libc::{c_float, c_uint};
use system::vector2;
use graphics::color;
use graphics::texture;
use graphics::drawable;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::rect::{FloatRect, IntRect};
use graphics::transform::Transform;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_void, c_float, c_uint};
    use system::vector2;
    use graphics::color;
    use graphics::texture;
    use rsfml::sfTypes::sfBool;
    use graphics::rect::{FloatRect, IntRect};
    use graphics::transform::Transform;

    pub struct sfRectangleShape {
        This : *c_void,
        Texture : *texture::csfml::sfTexture,
        Transform : Transform,
        InverseTransform : Transform
    }
    
    pub extern "C" {
        fn sfRectangleShape_create() -> *sfRectangleShape;
        fn sfRectangleShape_copy(shape : *sfRectangleShape) -> *sfRectangleShape;
        fn sfRectangleShape_destroy(shape : *sfRectangleShape) -> ();
        fn sfRectangleShape_setPosition(shape : *sfRectangleShape, position : vector2::Vector2f) -> ();
        fn sfRectangleShape_setRotation(shape : *sfRectangleShape, angle : c_float) -> ();
        fn sfRectangleShape_setScale(shape : *sfRectangleShape, scale : vector2::Vector2f) -> ();
        fn sfRectangleShape_setOrigin(shape : *sfRectangleShape, origin : vector2::Vector2f) -> ();
        fn sfRectangleShape_getPosition(shape : *sfRectangleShape) -> vector2::Vector2f;
        fn sfRectangleShape_getRotation(shape : *sfRectangleShape) -> c_float;
        fn sfRectangleShape_getScale(shape : *sfRectangleShape) -> vector2::Vector2f;
        fn sfRectangleShape_getOrigin(shape : *sfRectangleShape) -> vector2::Vector2f;
        fn sfRectangleShape_move(shape : *sfRectangleShape, offset : vector2::Vector2f) -> ();
        fn sfRectangleShape_rotate(shape : *sfRectangleShape, angle : c_float) -> ();
        fn sfRectangleShape_scale(shape : *sfRectangleShape, factors : vector2::Vector2f) -> ();
        fn sfRectangleShape_getTransform(shape : *sfRectangleShape) -> Transform;
        fn sfRectangleShape_getInverseTransform(shape : *sfRectangleShape) -> Transform;
        fn sfRectangleShape_setTexture(shape : *sfRectangleShape, texture : *texture::csfml::sfTexture, resetRect : sfBool) -> ();
        fn sfRectangleShape_setTextureRect(shape : *sfRectangleShape, rect : IntRect) -> ();
        fn sfRectangleShape_setFillColor(shape : *sfRectangleShape, color : color::Color) -> ();
        fn sfRectangleShape_setOutlineColor(shape : *sfRectangleShape, color : color::Color) -> ();
        fn sfRectangleShape_setOutlineThickness(shape : *sfRectangleShape, thickness : c_float) -> ();
        fn sfRectangleShape_getTexture(shape : *sfRectangleShape) -> *texture::csfml::sfTexture;
        fn sfRectangleShape_getTextureRect(shape : *sfRectangleShape) -> IntRect;
        fn sfRectangleShape_getFillColor(shape : *sfRectangleShape) -> color::Color;
        fn sfRectangleShape_getOutlineColor(shape : *sfRectangleShape) -> color::Color;
        fn sfRectangleShape_getOutlineThickness(shape : *sfRectangleShape) -> c_float;
        fn sfRectangleShape_getPointCount(shape : *sfRectangleShape) -> c_uint;
        fn sfRectangleShape_getPoint(shape : *sfRectangleShape, index : c_uint) -> vector2::Vector2f;
        fn sfRectangleShape_setSize(shape : *sfRectangleShape, size : vector2::Vector2f) -> ();
        fn sfRectangleShape_getSize(shape : *sfRectangleShape) -> vector2::Vector2f;
        fn sfRectangleShape_getLocalBounds(shape : *sfRectangleShape) -> FloatRect;
        fn sfRectangleShape_getGlobalBounds(shape : *sfRectangleShape) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct RectangleShape {
    priv rectangleShape : *csfml::sfRectangleShape
}

impl RectangleShape {
    pub fn new() -> RectangleShape {
        RectangleShape { rectangleShape : unsafe {csfml::sfRectangleShape_create()} }
    }

    pub fn new_copy(rectangleShape : &RectangleShape) -> RectangleShape {
        RectangleShape {rectangleShape : unsafe {csfml::sfRectangleShape_copy(rectangleShape.unwrap())} }
    }
    
    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setPosition(self.rectangleShape, *position)
        }
    }
    
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfRectangleShape_setRotation(self.rectangleShape, angle as c_float)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setScale(self.rectangleShape, *scale)
        }
    }

    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setOrigin(self.rectangleShape, *origin)
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_scale(self.rectangleShape, *factors)
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_move(self.rectangleShape, *offset)
        }
    }

    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfRectangleShape_getRotation(self.rectangleShape) as float
        }
    }
    
    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfRectangleShape_rotate(self.rectangleShape, angle as c_float)
        }
    }

    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getPosition(self.rectangleShape) }
    }

    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getScale(self.rectangleShape) }
    }

    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getOrigin(self.rectangleShape) }
    }

    pub fn get_size(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getSize(self.rectangleShape) }
    }

    pub fn get_point(&self, index : uint) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getPoint(self.rectangleShape, index as c_uint) }
    }

    pub fn set_size(&self, size : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setSize(self.rectangleShape, *size)
        }
    }

    pub fn set_texture(&self, texture : &texture::Texture, resetRect : bool) -> () {
        match resetRect {
            false       => unsafe { csfml::sfRectangleShape_setTexture(self.rectangleShape, texture.unwrap(), 0) },
            true        => unsafe { csfml::sfRectangleShape_setTexture(self.rectangleShape, texture.unwrap(), 1) }
        }
    }

    pub fn set_fill_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfRectangleShape_setFillColor(self.rectangleShape, *color)
        }
    }
    
    pub fn set_outline_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfRectangleShape_setOutlineColor(self.rectangleShape, *color)
        }
    }

    pub fn set_outline_thickness(&self, thickness : float) -> () {
        unsafe {
            csfml::sfRectangleShape_setOutlineThickness(self.rectangleShape, thickness as c_float)
        }
    }

    pub fn get_texture(&self) -> texture::Texture {
        unsafe {
            texture::Texture::wrap(csfml::sfRectangleShape_getTexture(self.rectangleShape))
        }
    }

    pub fn get_fill_color(&self) -> color::Color {
        unsafe { csfml::sfRectangleShape_getFillColor(self.rectangleShape) }
    }

    pub fn get_outline_color(&self) -> color::Color {
        unsafe { csfml::sfRectangleShape_getOutlineColor(self.rectangleShape) }
    }

    pub fn get_outline_thickness(&self) -> float {
        unsafe { csfml::sfRectangleShape_getOutlineThickness(self.rectangleShape) as float }
    }

    pub fn get_point_count(&self) -> uint {
        unsafe {
            csfml::sfRectangleShape_getPointCount(self.rectangleShape) as uint
        }
    }

    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            csfml::sfRectangleShape_getTextureRect(self.rectangleShape)
        }
    }

    pub fn set_texture_rect(&self, rect : &IntRect) -> () {
        unsafe {
            csfml::sfRectangleShape_setTextureRect(self.rectangleShape, *rect)
        }
    }

    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfRectangleShape_getGlobalBounds(self.rectangleShape)
        }
    }

    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfRectangleShape_getLocalBounds(self.rectangleShape)
        }
    }

    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfRectangleShape_getTransform(self.rectangleShape)
        }
    }

    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            csfml::sfRectangleShape_getInverseTransform(self.rectangleShape)
        }
    }

    #[doc(hidden)]
    pub fn wrap(rectangleShape : *csfml::sfRectangleShape) -> RectangleShape {
        RectangleShape { rectangleShape : rectangleShape }
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfRectangleShape {
        self.rectangleShape
    }
}

impl drawable::Drawable for RectangleShape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_rectangle_shape(self);
    }

    pub fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_rectangle_shape(self);
    }
}

impl Drop for RectangleShape {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfRectangleShape_destroy(self.rectangleShape)
        }
    }
}