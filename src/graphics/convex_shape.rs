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
* Specialized shape representing a convex polygon
*
* It is important to keep in mind that a convex shape must always be... convex, otherwise it may not be drawn correctly. 
* Moreover, the points must be defined in order; using a random order would result in an incorrect shape.
*/

use std::libc::{c_float, c_uint};

use graphics::color;
use graphics::texture;
use system::vector2;
use graphics::drawable;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::rect::{FloatRect, IntRect};
use graphics::transform::Transform;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_uint, c_void, c_float};

    use system::vector2;
    use graphics::color;
    use graphics::texture;
    use rsfml::sfTypes::sfBool;
    use graphics::rect::{FloatRect, IntRect};
    use graphics::transform::Transform;
    
    pub struct sfConvexShape {
        This : *c_void,
        Texture : *texture::csfml::sfTexture,
        Transform : Transform,
        InverseTransform : Transform
    }

    pub extern "C" {
        fn sfConvexShape_create() -> *sfConvexShape;
        fn sfConvexShape_copy(shape : *sfConvexShape) -> *sfConvexShape;
        fn sfConvexShape_destroy(shape : *sfConvexShape) -> ();
        fn sfConvexShape_setPosition(shape : *sfConvexShape, position : vector2::Vector2f) -> ();
        fn sfConvexShape_setRotation(shape : *sfConvexShape, angle : c_float) -> ();
        fn sfConvexShape_setScale(shape : *sfConvexShape, scale : vector2::Vector2f) -> ();
        fn sfConvexShape_setOrigin(shape : *sfConvexShape, origin : vector2::Vector2f) -> ();
        fn sfConvexShape_getPosition(shape : *sfConvexShape) -> vector2::Vector2f;
        fn sfConvexShape_getRotation(shape : *sfConvexShape) -> c_float;
        fn sfConvexShape_getScale(shape : *sfConvexShape) -> vector2::Vector2f;
        fn sfConvexShape_getOrigin(shape : *sfConvexShape) -> vector2::Vector2f;
        fn sfConvexShape_move(shape : *sfConvexShape, offset : vector2::Vector2f) -> ();
        fn sfConvexShape_rotate(shape : *sfConvexShape, angle : c_float) -> ();
        fn sfConvexShape_scale(shape : *sfConvexShape, factors : vector2::Vector2f) -> ();
        fn sfConvexShape_getTransform(shape : *sfConvexShape) -> Transform;
        fn sfConvexShape_getInverseTransform(shape : *sfConvexShape) -> Transform;
        fn sfConvexShape_setTexture(shape : *sfConvexShape, texture : *texture::csfml::sfTexture, resetRect : sfBool) -> ();
        fn sfConvexShape_setTextureRect(shape : *sfConvexShape, rect : IntRect) -> ();
        fn sfConvexShape_setFillColor(shape : *sfConvexShape, color : color::Color) -> ();
        fn sfConvexShape_setOutlineColor(shape : *sfConvexShape, color : color::Color) -> ();
        fn sfConvexShape_setOutlineThickness(shape : *sfConvexShape, thickness : c_float) -> ();
        fn sfConvexShape_getTexture(shape : *sfConvexShape) -> *texture::csfml::sfTexture;
        fn sfConvexShape_getTextureRect(shape : *sfConvexShape) -> IntRect;
        fn sfConvexShape_getFillColor(shape : *sfConvexShape) -> color::Color;
        fn sfConvexShape_getOutlineColor(shape : *sfConvexShape) -> color::Color;
        fn sfConvexShape_getOutlineThickness(shape : *sfConvexShape) -> c_float;
        fn sfConvexShape_getPointCount(shape : *sfConvexShape) -> c_uint;
        fn sfConvexShape_getPoint(shape : *sfConvexShape, index : c_uint) -> vector2::Vector2f;
        fn sfConvexShape_setPointCount(shape : *sfConvexShape, count : c_uint) -> ();
        fn sfConvexShape_setPoint(shape : *sfConvexShape, index : c_uint, point : vector2::Vector2f) -> ();
        fn sfConvexShape_getLocalBounds(shape : *sfConvexShape) -> FloatRect;
        fn sfConvexShape_getGlobalBounds(shape : *sfConvexShape) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct ConvexShape {
    priv convexShape : *csfml::sfConvexShape
}

impl ConvexShape {
    /**
    * Create a new convex shape
    *
    * Return a new convexShape object
    */
    pub fn new() -> ConvexShape {
        ConvexShape { convexShape : unsafe {csfml::sfConvexShape_create()} }
    }

    /**
    * Copy an existing convex shape
    *
    * # Arguments
    * * shape - Shape to copy
    * 
    * Return the copied object
    */
    pub fn new_copy(shape : &ConvexShape) -> ConvexShape {
        ConvexShape { convexShape : unsafe {csfml::sfConvexShape_copy(shape.unwrap())} }
    }

    /**
    * Set the position of a convex shape
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a convex Shape object is (0, 0).
    *
    * # Arguments
    * * position - New position
    */
    pub fn set_position(&mut self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setPosition(self.convexShape, *position)
        }
    }

    /**
    * Set the scale factors of a convex shape
    *
    * This function completely overwrites the previous scale.
    * See scale to add a factor based on the previous scale instead.
    * The default scale of a convex Shape object is (1, 1).
    *
    * # Arguments
    * * scale - New scale factors
    */
    pub fn set_scale(&mut self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setScale(self.convexShape, *scale)
        }
    }

    /**
    * Set the local origin of a convex shape
    *
    * The origin of an object defines the center point for
    * all transformations (position, scale, rotation).
    * The coordinates of this point must be relative to the
    * top-left corner of the object, and ignore all
    * transformations (position, scale, rotation).
    * The default origin of a convex Shape object is (0, 0).
    *
    * # Arguments
    * * origin - New origin
    */
    pub fn set_origin(&mut self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setOrigin(self.convexShape, *origin)
        }
    }

    /**
    * Move a convex shape by a given offset
    *
    * This function adds to the current position of the object,
    * unlike sfconvexShape_setPosition which overwrites it.
    *
    * # Arguments
    * * offset - Offset
    */
    pub fn move(&mut self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_move(self.convexShape, *offset)
        }
    }

    /**
    * Scale a convex shape
    *
    * This function multiplies the current scale of the object,
    * unlike sfconvexShape_setScale which overwrites it.
    *
    * # Arguments
    * * factors - Scale factors
    */
    pub fn scale(&mut self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_scale(self.convexShape, *factors)
        }
    }

    /**
    * Set the number of points of a convex shape
    * 
    * count must be greater than 2 to define a valid shape.
    *
    * # Arguments
    * * count - New number of points of the shape
    */
    pub fn set_point(&mut self, index : uint, point : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setPoint(self.convexShape, index as c_uint, *point)
        }
    }

    /**
    * Get the position of a convex shape
    *
    * Return the current position
    */
    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getPosition(self.convexShape)}
    }
    
    /**
    * Get the current scale of a convex shape
    *
    * Return the current scale factors
    */
    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getScale(self.convexShape)}
    }
    
    /**
    * Get the local origin of a convex shape
    *
    * return the current origin
    */
    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getOrigin(self.convexShape)}
    }

    /**
    * Get a point of a convex shape
    *
    * The result is undefined if index is out of the valid range.
    *
    * # Arguments
    * * index- Index of the point to get, in range [0 .. getPointCount() - 1]
    *
    * Return the index-th point of the shape
    */
    pub fn get_point(&self, index : uint) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getPoint(self.convexShape, index as c_uint)}
    }
    
    /**
    * Set the orientation of a convex shape
    *
    * This function completely overwrites the previous rotation.
    * See rotate to add an angle based on the previous rotation instead.
    * The default rotation of a convex Shape object is 0.
    *
    * # Arguments
    * * rotation - New rotation
    */
    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            csfml::sfConvexShape_setRotation(self.convexShape, angle as c_float)
        }
    }

    /**
    * Get the orientation of a convex shape
    *
    * The rotation is always in the range [0, 360].
    *
    * Return the current rotation, in degrees
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfConvexShape_getRotation(self.convexShape) as float
        }
    }

    /**
    * Rotate a convex shape
    *
    * This function adds to the current rotation of the object,
    * unlike set_rotation which overwrites it.
    *
    * # Arguments
    * * angle - Angle of rotation, in degrees
    */
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            csfml::sfConvexShape_rotate(self.convexShape, angle as c_float)
        }
    }

    /**
    * Change the source texture of a convex shape
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
            true        => unsafe {csfml::sfConvexShape_setTexture(self.convexShape, texture.unwrap(), 1)},
            false       => unsafe {csfml::sfConvexShape_setTexture(self.convexShape, texture.unwrap(), 0)}
        }
    }

    /**
    * Set the fill color of a convex shape
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
            csfml::sfConvexShape_setFillColor(self.convexShape, *color)
        }
    }

    /**
    * Set the outline color of a convex shape
    *
    * You can use Transparent to disable the outline.
    * By default, the shape's outline color is opaque white.
    *
    * # Arguments
    * * color - New outline color of the shape
    */
    pub fn set_outline_color(&mut self, color : &color::Color) -> () {
        unsafe {
            csfml::sfConvexShape_setOutlineColor(self.convexShape, *color)
        }
    }

    /**
    * Set the thickness of a convex shape's outline
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
            csfml::sfConvexShape_setOutlineThickness(self.convexShape, thickness as c_float)
        }
    }

    /**
    * Get the source texture of a convex shape
    *
    * You can't modify the texture when you retrieve it with this function.
    * 
    * Return the shape's texture
    */
    pub fn get_texture(&self) -> texture::Texture {
            texture::Texture::wrap(unsafe {csfml::sfConvexShape_getTexture(self.convexShape)})
    }
    
    /**
    * Get the fill color of a convex shape
    *
    * Return the fill color of the shape
    */
    pub fn get_fill_color(&self) -> color::Color {
        unsafe {csfml::sfConvexShape_getFillColor(self.convexShape)}
    }
    
    /**
    * Get the outline color of a convex shape
    *
    * Return the outline color of the shape
    */
    pub fn get_outline_color(&self) -> color::Color {
        unsafe {csfml::sfConvexShape_getOutlineColor(self.convexShape)}
    }
    
    /**
    * Get the outline thickness of a convex shape
    *
    * Return the outline thickness of the shape
    */
    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            csfml::sfConvexShape_getOutlineThickness(self.convexShape) as float
        }
    }

    /**
    * Get the total number of points of a convex shape
    *
    * Return the number of points of the shape
    */
    pub fn get_point_count(&self) -> uint {
        unsafe {
            csfml::sfConvexShape_getPointCount(self.convexShape) as uint
        }
    }

    /**
    * Set the number of points of a convex
    *
    * # Arguments
    * * count - New number of points of the convex
    */
    pub fn set_point_count(&mut self, count : uint) -> () {
        unsafe {
            csfml::sfConvexShape_setPointCount(self.convexShape, count as c_uint)
        }
    }

    /**
    * Get the sub-rectangle of the texture displayed by a convex shape
    *
    * Return the texture rectangle of the shape
    */
    pub fn set_texture_rect(&mut self, rect : &IntRect) -> () {
        unsafe {
            csfml::sfConvexShape_setTextureRect(self.convexShape, *rect)
        }
    }

    /**
    * Get the local bounding rectangle of a convex shape
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
            csfml::sfConvexShape_getLocalBounds(self.convexShape)
        }
    }

    /**
    * Get the global bounding rectangle of a convex shape
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
            csfml::sfConvexShape_getGlobalBounds(self.convexShape)
        }
    }

    /**
    * Get the sub-rectangle of the texture displayed by a convex shape
    *
    * Return the texture rectangle of the shape
    */
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            csfml::sfConvexShape_getTextureRect(self.convexShape)
        }
    }
    
    /**
    * Get the combined transform of a convex shape
    *
    * Return transform combining the position/rotation/scale/origin of the object
    */
    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfConvexShape_getTransform(self.convexShape)
        }
    }

    /**
    * Get the inverse of the combined transform of a convex shape
    *
    * Return inverse of the combined transformations applied to the object
    */
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            csfml::sfConvexShape_getInverseTransform(self.convexShape)
        }
    }

    #[doc(hidden)]
    pub fn wrap(convexShape : *csfml::sfConvexShape) -> ConvexShape {
        ConvexShape { convexShape : convexShape}
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfConvexShape {
        self.convexShape
    }
}

impl drawable::Drawable for ConvexShape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_convex_shape(self)
    }

    pub fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_convex_shape(self)
    }
}

impl Drop for ConvexShape {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfConvexShape_destroy(self.convexShape)
        }
    }
}