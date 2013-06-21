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
* Specialized shape representing a rectangle
*
*
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
    /**
    * Create a new rectangle shape
    *
    * Return a new rectangleShape object
    */
    pub fn new() -> RectangleShape {
        RectangleShape { rectangleShape : unsafe {csfml::sfRectangleShape_create()} }
    }
    
    /**
    * Set the position of a rectangle shape
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a rectangle Shape object is (0, 0).
    *
    * # Arguments
    * * position - New position
    */
    pub fn set_position(&mut self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setPosition(self.rectangleShape, *position)
        }
    }
    
    /**
    * Set the orientation of a rectangle shape
    *
    * This function completely overwrites the previous rotation.
    * See rotate to add an angle based on the previous rotation instead.
    * The default rotation of a rectangle Shape object is 0.
    *
    * # Arguments
    * * rotation - New rotation
    */
    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            csfml::sfRectangleShape_setRotation(self.rectangleShape, angle as c_float)
        }
    }

    /**
    * Set the scale factors of a rectangle shape
    *
    * This function completely overwrites the previous scale.
    * See scale to add a factor based on the previous scale instead.
    * The default scale of a rectangle Shape object is (1, 1).
    *
    * # Arguments
    * * scale - New scale factors
    */
    pub fn set_scale(&mut self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setScale(self.rectangleShape, *scale)
        }
    }

    /**
    * Set the local origin of a rectangle shape
    *
    * The origin of an object defines the center point for
    * all transformations (position, scale, rotation).
    * The coordinates of this point must be relative to the
    * top-left corner of the object, and ignore all
    * transformations (position, scale, rotation).
    * The default origin of a rectangle Shape object is (0, 0).
    *
    * # Arguments
    * * origin - New origin
    */
    pub fn set_origin(&mut self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setOrigin(self.rectangleShape, *origin)
        }
    }

    /**
    * Scale a rectangle shape
    *
    * This function multiplies the current scale of the object,
    * unlike sfrectangleShape_setScale which overwrites it.
    *
    * # Arguments
    * * factors - Scale factors
    */
    pub fn scale(&mut self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_scale(self.rectangleShape, *factors)
        }
    }

    /**
    * Move a rectangle shape by a given offset
    *
    * This function adds to the current position of the object,
    * unlike sset_position which overwrites it.
    *
    * # Arguments
    * * offset - Offset
    */
    pub fn move(&mut self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_move(self.rectangleShape, *offset)
        }
    }

    /**
    * Get the orientation of a rectangle shape
    *
    * The rotation is always in the range [0, 360].
    *
    * Return the current rotation, in degrees
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfRectangleShape_getRotation(self.rectangleShape) as float
        }
    }
    
    /**
    * Rotate a rectangle shape
    *
    * This function adds to the current rotation of the object,
    * unlike set_rotation which overwrites it.
    *
    * # Arguments
    * * angle - Angle of rotation, in degrees
    */
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            csfml::sfRectangleShape_rotate(self.rectangleShape, angle as c_float)
        }
    }

    /**
    * Get the position of a rectangle shape
    *
    * Return the current position
    */
    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getPosition(self.rectangleShape) }
    }

    /**
    * Get the current scale of a rectangle shape
    *
    * Return the current scale factors
    */
    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getScale(self.rectangleShape) }
    }

    /**
    * Get the local origin of a rectangle shape
    *
    * return the current origin
    */
    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getOrigin(self.rectangleShape) }
    }

    /**
    * Get the size of a rectangle shape
    *
    * Return the height Size of the rectangle
    */
    pub fn get_size(&self) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getSize(self.rectangleShape) }
    }

    /**
    * Get a point of a rectangle shape
    *
    * The result is undefined if index is out of the valid range.
    *
    * # Arguments
    * * index- Index of the point to get, in range [0 .. getPointCount() - 1]
    *
    * Return the index-th point of the shape
    */
    pub fn get_point(&self, index : uint) -> vector2::Vector2f {
        unsafe { csfml::sfRectangleShape_getPoint(self.rectangleShape, index as c_uint) }
    }

    /**
    * Set the size of a rectangle shape
    *
    * # Arguments
    * * size - The new size of the rectangle
    */
    pub fn set_size(&mut self, size : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfRectangleShape_setSize(self.rectangleShape, *size)
        }
    }

    /**
    * Change the source texture of a rectangle shape
    *
    * The texture argument refers to a texture that must
    * exist as long as the shape uses it. Indeed, the shape
    * doesn't store its own copy of the texture, but rather keeps
    * a pointer to the one that you passed to this function.
    * If the source texture is destroyed and the shape tries to
    * use it, the behaviour is undefined.
    * texture can be NULL to disable texturing.
    * If resetRect is true, the TextureRect property of
    * the shape is automatically adjusted to the size of the new
    * texture. If it is false, the texture rect is left unchanged.
    *
    * # Arguments
    * * texture - New texture
    * * resetRect - Should the texture rect be reset to the size of the new texture?
    */
    pub fn set_texture(&mut self, texture : &texture::Texture, resetRect : bool) -> () {
        match resetRect {
            false       => unsafe { csfml::sfRectangleShape_setTexture(self.rectangleShape, texture.unwrap(), 0) },
            true        => unsafe { csfml::sfRectangleShape_setTexture(self.rectangleShape, texture.unwrap(), 1) }
        }
    }

    /**
    * Set the fill color of a rectangle shape
    *
    * This color is modulated (multiplied) with the shape's
    * texture if any. It can be used to colorize the shape,
    * or change its global opacity.
    * You can use sfTransparent to make the inside of
    * the shape transparent, and have the outline alone.
    * By default, the shape's fill color is opaque white.
    *
    * # Arguments
    * * color - New color of the shape
    */
    pub fn set_fill_color(&mut self, color : &color::Color) -> () {
        unsafe {
            csfml::sfRectangleShape_setFillColor(self.rectangleShape, *color)
        }
    }
    
    /**
    * Set the outline color of a rectangle shape
    *
    * You can use Transparent to disable the outline.
    * By default, the shape's outline color is opaque white.
    *
    * # Arguments
    * * color - New outline color of the shape
    */
    pub fn set_outline_color(&mut self, color : &color::Color) -> () {
        unsafe {
            csfml::sfRectangleShape_setOutlineColor(self.rectangleShape, *color)
        }
    }

    /**
    * Set the thickness of a rectangle shape's outline
    *
    * This number cannot be negative. Using zero disables
    * the outline.
    * By default, the outline thickness is 0.
    *
    * # Arguments
    * * thickness - New outline thickness
    */
    pub fn set_outline_thickness(&mut self, thickness : float) -> () {
        unsafe {
            csfml::sfRectangleShape_setOutlineThickness(self.rectangleShape, thickness as c_float)
        }
    }

    /**
    * Get the source texture of a rectangle shape
    *
    * You can't modify the texture when you retrieve it with this function.
    * 
    * Return the shape's texture
    */
    pub fn get_texture(&self) -> texture::Texture {
        unsafe {
            texture::Texture::wrap(csfml::sfRectangleShape_getTexture(self.rectangleShape))
        }
    }

    /**
    * Get the fill color of a rectangle shape
    *
    * Return the fill color of the shape
    */
    pub fn get_fill_color(&self) -> color::Color {
        unsafe { csfml::sfRectangleShape_getFillColor(self.rectangleShape) }
    }

    /**
    * Get the outline color of a rectangle shape
    *
    * Return the outline color of the shape
    */
    pub fn get_outline_color(&self) -> color::Color {
        unsafe { csfml::sfRectangleShape_getOutlineColor(self.rectangleShape) }
    }

    /**
    * Get the outline thickness of a rectangle shape
    *
    * Return the outline thickness of the shape
    */
    pub fn get_outline_thickness(&self) -> float {
        unsafe { csfml::sfRectangleShape_getOutlineThickness(self.rectangleShape) as float }
    }

    /**
    * Get the total number of points of a rectangle shape
    *
    * Return the number of points of the shape
    */
    pub fn get_point_count(&self) -> uint {
        unsafe {
            csfml::sfRectangleShape_getPointCount(self.rectangleShape) as uint
        }
    }

    /**
    * Get the sub-rectangle of the texture displayed by a rectangle shape
    *
    * Return the texture rectangle of the shape
    */
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            csfml::sfRectangleShape_getTextureRect(self.rectangleShape)
        }
    }

    /**
    * Set the sub-rectangle of the texture that a rectangle shape will display
    *
    * The texture rect is useful when you don't want to display
    * the whole texture, but rather a part of it.
    * By default, the texture rect covers the entire texture.
    *
    * # Arguments
    * * rec - Rectangle defining the region of the texture to display
    */
    pub fn set_texture_rect(&mut self, rect : &IntRect) -> () {
        unsafe {
            csfml::sfRectangleShape_setTextureRect(self.rectangleShape, *rect)
        }
    }

    /**
    * Get the global bounding rectangle of a rectangle shape
    *
    * The returned rectangle is in global coordinates, which means
    * that it takes in account the transformations (translation,
    * rotation, scale, ...) that are applied to the entity.
    * In other words, this function returns the bounds of the
    * sprite in the global 2D world's coordinate system.
    *
    * Return the global bounding rectangle of the entity
    */
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfRectangleShape_getGlobalBounds(self.rectangleShape)
        }
    }

    /**
    * Get the local bounding rectangle of a rectangle shape
    *
    * The returned rectangle is in local coordinates, which means
    * that it ignores the transformations (translation, rotation,
    * scale, ...) that are applied to the entity.
    * In other words, this function returns the bounds of the
    * entity in the entity's coordinate system.
    *
    * Return the local bounding rectangle of the entity
    */
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfRectangleShape_getLocalBounds(self.rectangleShape)
        }
    }

    /**
    * Get the combined transform of a rectangle shape
    *
    * Return transform combining the position/rotation/scale/origin of the object
    */
    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfRectangleShape_getTransform(self.rectangleShape)
        }
    }

    /**
    * Get the inverse of the combined transform of a rectangle shape
    *
    * Return inverse of the combined transformations applied to the object
    */
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
    pub fn draw_in_render_window(&self, renderWindow : &mut RenderWindow) -> () {
        renderWindow.draw_rectangle_shape(self);
    }

    pub fn draw_in_render_texture(&self, renderTexture : &mut RenderTexture) -> () {
        renderTexture.draw_rectangle_shape(self);
    }
}

impl Clone for RectangleShape {
    fn clone(&self) -> RectangleShape {
        unsafe {
            RectangleShape {
                rectangleShape: csfml::sfRectangleShape_copy(self.rectangleShape)
            }
        }
    }
}

impl Drop for RectangleShape {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfRectangleShape_destroy(self.rectangleShape)
        }
    }
}