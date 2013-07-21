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

use std::libc::{c_float, c_uint};
use std::ptr;

use traits::drawable::Drawable;
use traits::wrappable::Wrappable;
use graphics::color::Color;
use graphics::rect::{IntRect, FloatRect};
use graphics::texture::Texture;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use system::vector2::Vector2f;
use graphics::transform::Transform;
use graphics::render_states::RenderStates;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_float, c_uint};

    use graphics::texture;
    use rsfml::sfTypes::{sfBool};
    use graphics::rect::{IntRect, FloatRect};
    use system::vector2::Vector2f;
    use graphics::color::Color;
    use graphics::transform::Transform;

    pub struct sfCircleShape {
        This : *c_void,
        Texture : *texture::ffi::sfTexture,
        Transform : Transform,
        InverseTransform : Transform
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
        pub fn sfCircleShape_setTexture(shape : *sfCircleShape, texture : *texture::ffi::sfTexture, resetRect : sfBool) -> ();
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

#[doc(hidden)]
pub struct CircleShape {
    priv circleShape : *ffi::sfCircleShape,
    priv texture : Option<@mut Texture>

}

impl CircleShape {
    /**
    * Create a new circle shape
    *
    * Return a new option to CircleShape object or None
    */
    pub fn new() -> Option<CircleShape> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if ptr::is_null(circle) {
            None
        }
        else {
            Some(CircleShape {
                circleShape : circle,
                texture : None
            })
        }
    }
    
    pub fn new_with_texture(texture : @mut Texture) -> Option<CircleShape> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if ptr::is_null(circle) {
            None
        }
        else {
            unsafe {
                ffi::sfCircleShape_setTexture(circle, texture.unwrap(), 1);
            }
            Some(CircleShape {
                circleShape : circle,
                texture : Some(texture)
            })
        }

    }

    /**
    * Create a new CircleShape and initialize it.
    *
    * Default value on SFML are radius = 0 / pointCount = 30
    *
    * Return a new initialized option to CircleShape or None
    */
    pub fn new_init(radius : float, pointCount : uint) -> Option<CircleShape> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if ptr::is_null(circle) {
           None
        }
        else {
            unsafe {
                ffi::sfCircleShape_setRadius(circle, radius as c_float);
                ffi::sfCircleShape_setPointCount(circle, pointCount as c_uint);
            }
            Some(CircleShape {
                circleShape : circle,
                texture : None
            })
        }
    }

    /**
    * Copy an existing circle shape
    *
    * # Arguments
    * * shape - Shape to copy
    * 
    * Return the copied object on option or None
    */
    pub fn clone(&self) -> Option<CircleShape> {
        let circle = unsafe { ffi::sfCircleShape_copy(self.circleShape) };
        if ptr::is_null(circle) {
            None
        }
        else {
            Some(CircleShape {
                circleShape : circle,
                texture : self.texture
            })
        }
    }

    /**
    * Set the orientation of a circle shape
    *
    * This function completely overwrites the previous rotation.
    * See rotate to add an angle based on the previous rotation instead.
    * The default rotation of a circle Shape object is 0.
    *
    * # Arguments
    * * rotation - New rotation
    */
    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfCircleShape_setRotation(self.circleShape, angle as c_float)
        }
    }

    /**
    * Get the orientation of a circle shape
    *
    * The rotation is always in the range [0, 360].
    *
    * Return the current rotation, in degrees
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfCircleShape_getRotation(self.circleShape) as float
        }
    } 

    /**
    * Rotate a circle shape
    *
    * This function adds to the current rotation of the object,
    * unlike set_rotation which overwrites it.
    *
    * # Arguments
    * * angle - Angle of rotation, in degrees
    */
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfCircleShape_rotate(self.circleShape, angle as c_float)
        }
    }

    /**
    * Change the source texture of a circle shape
    *
    * The texture argument refers to a texture that must
    * exist as long as the shape uses it. Indeed, the shape
    * doesn't store its own copy of the texture, but rather keeps
    * a pointer to the one that you passed to this function.
    * If the source texture is destroyed and the shape tries to
    * use it, the behaviour is undefined.
    * If resetRect is true, the TextureRect property of
    * the shape is automatically adjusted to the size of the new
    * texture. If it is false, the texture rect is left unchanged.
    *
    * # Arguments
    * * texture - New texture
    * * resetRect - Should the texture rect be reset to the size of the new texture?
    */
    pub fn set_texture(&mut self, texture : @mut Texture, resetRect : bool) -> () {
        self.texture = Some(texture);
        unsafe {
            match resetRect {
                true        => ffi::sfCircleShape_setTexture(self.circleShape, texture.unwrap(), 1),
                false       => ffi::sfCircleShape_setTexture(self.circleShape, texture.unwrap(), 0),
            }
        }
    }

    /**
    * Disable the current texture
    *
    * Disable the current texture on the CircleShape and reset the texture rect
    */
    pub fn disable_texture(&self) -> () {
        unsafe {
            ffi::sfCircleShape_setTexture(self.circleShape, ptr::null(), 1)
        }
    }
    
    /**
    * Set the sub-rectangle of the texture that a circle shape will display
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
            ffi::sfCircleShape_setTextureRect(self.circleShape, *rect)
        }
    }
    
    /**
    * Set the fill color of a circle shape
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
    pub fn set_fill_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfCircleShape_setFillColor(self.circleShape, *color)
        }
    }

    /**
    * Set the outline color of a circle shape
    *
    * You can use Transparent to disable the outline.
    * By default, the shape's outline color is opaque white.
    *
    * # Arguments
    * * color - New outline color of the shape
    */
    pub fn set_outline_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfCircleShape_setOutlineColor(self.circleShape, *color)
        }
    }

    /**
    * Set the thickness of a circle shape's outline
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
            ffi::sfCircleShape_setOutlineThickness(self.circleShape, thickness as c_float)
        }
    }

    /**
    * Get the source texture of a circle shape
    *
    * You can't modify the texture when you retrieve it with this function.
    * 
    * Return the shape's texture
    */
    pub fn get_texture(&self) -> Option<@mut Texture> {
        self.texture
    }

    /**
    * Get the sub-rectangle of the texture displayed by a circle shape
    *
    * Return the texture rectangle of the shape
    */
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfCircleShape_getTextureRect(self.circleShape)
        }
    }
    
    /**
    * Get the fill color of a circle shape
    *
    * Return the fill color of the shape
    */
    pub fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfCircleShape_getFillColor(self.circleShape)
        }
    }

    /**
    * Get the outline color of a circle shape
    *
    * Return the outline color of the shape
    */
    pub fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfCircleShape_getOutlineColor(self.circleShape)
        }
    }

    /**
    * Get the outline thickness of a circle shape
    *
    * Return the outline thickness of the shape
    */
    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            ffi::sfCircleShape_getOutlineThickness(self.circleShape) as float
        }
    }

    /**
    * Get the total number of points of a circle shape
    *
    * Return the number of points of the shape
    */
    pub fn get_point_count(&self) -> uint {
        unsafe {
            ffi::sfCircleShape_getPointCount(self.circleShape) as uint
        }
    }

    /**
    * Get a point of a circle shape
    *
    * The result is undefined if index is out of the valid range.
    *
    * # Arguments
    * * index- Index of the point to get, in range [0 .. getPointCount() - 1]
    *
    * Return the index-th point of the shape
    */
    pub fn get_point(&self, index : uint) -> () {
        unsafe {
            ffi::sfCircleShape_getPoint(self.circleShape, index as c_uint)
        }
    }
    
    /**
    * Set the radius of a circle
    *
    * # Arguments
    * * radius - New radius of the circle
    */
    pub fn set_radius(&self, radius : float) -> () {
        unsafe {
            ffi::sfCircleShape_setRadius(self.circleShape, radius as c_float)
        }
    }

    /**
    * Set the radius of a circle
    *
    * Return the radius of the circle
    */
    pub fn get_radius(&self) -> float {
        unsafe {
            ffi::sfCircleShape_getRadius(self.circleShape) as float
        }
    }
    
    /**
    * Set the number of points of a circle
    *
    * # Arguments
    * * count - New number of points of the circle
    */
    pub fn set_point_count(&mut self, count : uint) -> () {
        unsafe {
            ffi::sfCircleShape_setPointCount(self.circleShape, count as c_uint)
        }
    }

    /**
    * Get the position of a circle shape
    *
    * Return the current position
    */
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfCircleShape_getPosition(self.circleShape)
        }
    }

    /**
    * Get the current scale of a circle shape
    *
    * Return the current scale factors
    */
    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfCircleShape_getScale(self.circleShape)
        }
    }

    /**
    * Get the local origin of a circle shape
    *
    * return the current origin
    */
    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfCircleShape_getOrigin(self.circleShape)
        }
    }

    /**
    * Move a circle shape by a given offset
    *
    * This function adds to the current position of the object,
    * unlike sset_position which overwrites it.
    *
    * # Arguments
    * * offset - Offset
    */
    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_move(self.circleShape, *offset)
        }
    }

    /**
    * Move a circle shape by a given offset
    *
    * This function adds to the current position of the object,
    * unlike sset_position which overwrites it.
    *
    * # Arguments
    * * offsetX - Offset x
    * * offsetY - Offset y
    */
    pub fn move2f(&mut self, offsetX : f32, offsetY : f32) -> () {
        unsafe {
            ffi::sfCircleShape_move(self.circleShape, Vector2f::new(offsetX, offsetY))
        }
    }

    /**
    * Scale a circle shape
    *
    * This function multiplies the current scale of the object,
    * unlike sfCircleShape_setScale which overwrites it.
    *
    * # Arguments
    * * factors - Scale factors
    */
    pub fn scale(&mut self, factors : &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_scale(self.circleShape, *factors)
        }
    }

    /**
    * Scale a circle shape
    *
    * This function multiplies the current scale of the object,
    * unlike sfCircleShape_setScale which overwrites it.
    *
    * # Arguments
    * * factorX - Scale x factor
    * * factorY - Scale y factor
    */
    pub fn scale2f(&mut self, factorX : f32, factorY : f32) -> () {
        unsafe {
            ffi::sfCircleShape_scale(self.circleShape, Vector2f::new(factorX, factorY))
        }
    }

    /**
    * Set the position of a circle shape
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a circle Shape object is (0, 0).
    *
    * # Arguments
    * * position - New position
    */
    pub fn set_position(&mut self, position : &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_setPosition(self.circleShape, *position)
        }
    }

    /**
    * Set the position of a circle shape
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a circle Shape object is (0, 0).
    *
    * # Arguments
    * * x - New x coordinate
    * * y - New y coordinate
    */
    pub fn set_position2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfCircleShape_setPosition(self.circleShape, Vector2f::new(x, y))
        }
    }

    /**
    * Set the scale factors of a circle shape
    *
    * This function completely overwrites the previous scale.
    * See scale to add a factor based on the previous scale instead.
    * The default scale of a circle Shape object is (1, 1).
    *
    * # Arguments
    * * scale - New scale factors
    */
    pub fn set_scale(&mut self, scale : &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_setScale(self.circleShape, *scale)
        }
    }
    
    /**
    * Set the scale factors of a circle shape
    *
    * This function completely overwrites the previous scale.
    * See scale to add a factor based on the previous scale instead.
    * The default scale of a circle Shape object is (1, 1).
    *
    * # Arguments
    * * scaleX - New x scale factor
    * * scaleY - New y scale factor
    */
    pub fn set_scale2f(&mut self, scaleX : f32, scaleY : f32) -> () {
        unsafe {
            ffi::sfCircleShape_setScale(self.circleShape, Vector2f::new(scaleX, scaleY))
        }
    }
    
    /**
    * Set the local origin of a circle shape
    *
    * The origin of an object defines the center point for
    * all transformations (position, scale, rotation).
    * The coordinates of this point must be relative to the
    * top-left corner of the object, and ignore all
    * transformations (position, scale, rotation).
    * The default origin of a circle Shape object is (0, 0).
    *
    * # Arguments
    * * origin - New origin
    */
    pub fn set_origin(&mut self, origin : &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_setOrigin(self.circleShape, *origin)
        }
    }

    /**
    * Set the local origin of a circle shape
    *
    * The origin of an object defines the center point for
    * all transformations (position, scale, rotation).
    * The coordinates of this point must be relative to the
    * top-left corner of the object, and ignore all
    * transformations (position, scale, rotation).
    * The default origin of a circle Shape object is (0, 0).
    *
    * # Arguments
    * * x - New x origin coordinate
    * * y - New y origin coordinate
    */
    pub fn set_origin2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfCircleShape_setOrigin(self.circleShape, Vector2f::new(x, y))
        }
    }

    /**
    * Get the local bounding rectangle of a circle shape
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
            ffi::sfCircleShape_getLocalBounds(self.circleShape)
        }
    }
    
    /**
    * Get the global bounding rectangle of a circle shape
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
            ffi::sfCircleShape_getGlobalBounds(self.circleShape)
        }
    }

    /**
    * Get the combined transform of a circle shape
    *
    * Return transform combining the position/rotation/scale/origin of the object
    */
    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfCircleShape_getTransform(self.circleShape)
        }
    }

    /**
    * Get the inverse of the combined transform of a circle shape
    *
    * Return inverse of the combined transformations applied to the object
    */
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfCircleShape_getInverseTransform(self.circleShape)
        }
    }

}

#[doc(hidden)]
impl Wrappable<*ffi::sfCircleShape> for CircleShape {
    #[doc(hidden)]
    pub fn wrap(circleShape : *ffi::sfCircleShape) -> CircleShape {
        CircleShape {
            circleShape : circleShape,
            texture : None
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *ffi::sfCircleShape {
        self.circleShape
    }
}

impl Drawable for CircleShape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_circle_shape(self)
    }

    pub fn draw_in_render_window_rs(&self, renderWindow : &RenderWindow, renderStates : &mut RenderStates) -> () {
        renderWindow.draw_circle_shape_rs(self, renderStates)
    }
    
    pub fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_circle_shape(self)
    }

    pub fn draw_in_render_texture_rs(&self, renderTexture : &RenderTexture, renderStates : &mut RenderStates) -> () {
        renderTexture.draw_circle_shape_rs(self, renderStates)
    }
}

#[unsafe_destructor]
impl Drop for CircleShape {
    /**
    * Destroy an existing CircleShape
    */
    fn drop(&self) {
        unsafe {
            ffi::sfCircleShape_destroy(self.circleShape)
        }
    }
}
