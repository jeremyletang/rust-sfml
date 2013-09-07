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
use std::ptr;

use traits::wrappable::Wrappable;
use traits::drawable::Drawable;
use graphics::color::Color;
use graphics::texture::Texture;
use system::vector2::Vector2f;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::rect::{FloatRect, IntRect};
use graphics::transform::Transform;
use graphics::render_states::RenderStates;

#[doc(hidden)]
pub mod ffi {

    use std::libc::{c_uint, c_void, c_float};

    use system::vector2::Vector2f;
    use graphics::color::Color;
    use graphics::texture;
    use sfml_types::SfBool;
    use graphics::rect::{FloatRect, IntRect};
    use graphics::transform::Transform;
    
    pub struct sfConvexShape {
        This : *c_void,
        Texture : *texture::ffi::sfTexture,
        Transform : Transform,
        InverseTransform : Transform
    }

    extern "C" {
        pub fn sfConvexShape_create() -> *sfConvexShape;
        pub fn sfConvexShape_copy(shape : *sfConvexShape) -> *sfConvexShape;
        pub fn sfConvexShape_destroy(shape : *sfConvexShape) -> ();
        pub fn sfConvexShape_setPosition(shape : *sfConvexShape, position : Vector2f) -> ();
        pub fn sfConvexShape_setRotation(shape : *sfConvexShape, angle : c_float) -> ();
        pub fn sfConvexShape_setScale(shape : *sfConvexShape, scale : Vector2f) -> ();
        pub fn sfConvexShape_setOrigin(shape : *sfConvexShape, origin : Vector2f) -> ();
        pub fn sfConvexShape_getPosition(shape : *sfConvexShape) -> Vector2f;
        pub fn sfConvexShape_getRotation(shape : *sfConvexShape) -> c_float;
        pub fn sfConvexShape_getScale(shape : *sfConvexShape) -> Vector2f;
        pub fn sfConvexShape_getOrigin(shape : *sfConvexShape) -> Vector2f;
        pub fn sfConvexShape_move(shape : *sfConvexShape, offset : Vector2f) -> ();
        pub fn sfConvexShape_rotate(shape : *sfConvexShape, angle : c_float) -> ();
        pub fn sfConvexShape_scale(shape : *sfConvexShape, factors : Vector2f) -> ();
        pub fn sfConvexShape_getTransform(shape : *sfConvexShape) -> Transform;
        pub fn sfConvexShape_getInverseTransform(shape : *sfConvexShape) -> Transform;
        pub fn sfConvexShape_setTexture(shape : *sfConvexShape, texture : *texture::ffi::sfTexture, reset_rect : SfBool) -> ();
        pub fn sfConvexShape_setTextureRect(shape : *sfConvexShape, rect : IntRect) -> ();
        pub fn sfConvexShape_setFillColor(shape : *sfConvexShape, color : Color) -> ();
        pub fn sfConvexShape_setOutlineColor(shape : *sfConvexShape, color : Color) -> ();
        pub fn sfConvexShape_setOutlineThickness(shape : *sfConvexShape, thickness : c_float) -> ();
        pub fn sfConvexShape_getTexture(shape : *sfConvexShape) -> *texture::ffi::sfTexture;
        pub fn sfConvexShape_getTextureRect(shape : *sfConvexShape) -> IntRect;
        pub fn sfConvexShape_getFillColor(shape : *sfConvexShape) -> Color;
        pub fn sfConvexShape_getOutlineColor(shape : *sfConvexShape) -> Color;
        pub fn sfConvexShape_getOutlineThickness(shape : *sfConvexShape) -> c_float;
        pub fn sfConvexShape_getPointCount(shape : *sfConvexShape) -> c_uint;
        pub fn sfConvexShape_getPoint(shape : *sfConvexShape, index : c_uint) -> Vector2f;
        pub fn sfConvexShape_setPointCount(shape : *sfConvexShape, count : c_uint) -> ();
        pub fn sfConvexShape_setPoint(shape : *sfConvexShape, index : c_uint, point : Vector2f) -> ();
        pub fn sfConvexShape_getLocalBounds(shape : *sfConvexShape) -> FloatRect;
        pub fn sfConvexShape_getGlobalBounds(shape : *sfConvexShape) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct ConvexShape {
    priv convex_shape : *ffi::sfConvexShape,
    priv texture : Option<@mut Texture>
}

impl ConvexShape {
    /**
    * Create a new convex shape
    *
    * Return a new convexShape object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Option<ConvexShape> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        if ptr::is_null(shape) {
            None
        }
        else {
            Some(ConvexShape {
                convex_shape : shape,
                texture : None
            })
        } 
    }

    /**
    * Create a new convex shape with a texture
    *
    * Return a new convexShape object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_with_texture(texture : @mut Texture) -> Option<ConvexShape> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        if ptr::is_null(shape) {
            None
        }
        else {
            unsafe {
                ffi::sfConvexShape_setTexture(shape, texture.unwrap(), 1);
            }
            Some(ConvexShape {
                convex_shape : shape,
                texture : Some(texture)
            })
        } 

    }

    /**
    * Clone an existing convex shape
    *
    * Return the cloned object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn clone(&self) -> Option<ConvexShape> {
        let shape = unsafe { ffi::sfConvexShape_copy(self.convex_shape) };
        if ptr::is_null(shape) {
            None
        }
        else {
            Some(ConvexShape {
                convex_shape : shape,
                texture : self.texture
            })
        }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_position(&mut self, position : &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setPosition(self.convex_shape, *position)
        }
    }

    /**
    * Set the position of a convex shape
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a convex Shape object is (0, 0).
    *
    * # Arguments
    * * x - New x coordinate
    * * y - New y coordinate
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_position2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfConvexShape_setPosition(self.convex_shape, Vector2f::new(x, y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_scale(&mut self, scale : &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setScale(self.convex_shape, *scale)
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
    * * scale_x - New x scale factor
    * * scale_y - New y scale factor
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_scale2f(&mut self, scale_x : f32, scale_y : f32) -> () {
        unsafe {
            ffi::sfConvexShape_setScale(self.convex_shape, Vector2f::new(scale_x, scale_y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_origin(&mut self, origin : &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setOrigin(self.convex_shape, *origin)
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
    * * x - New x origin coordinate
    * * y - New y origin coordinate
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_origin2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfConvexShape_setOrigin(self.convex_shape, Vector2f::new(x, y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_move(self.convex_shape, *offset)
        }
    }

    /**
    * Move a convex shape by a given offset
    *
    * This function adds to the current position of the object,
    * unlike sfconvexShape_setPosition which overwrites it.
    *
    * # Arguments
    * * offsetX - Offset x
    * * offsetY - Offset y
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn move2f(&mut self, offset_x : f32, offset_y : f32) -> () {
        unsafe {
            ffi::sfConvexShape_move(self.convex_shape, Vector2f::new(offset_x, offset_y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn scale(&mut self, factors : &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_scale(self.convex_shape, *factors)
        }
    }

    /**
    * Scale a convex shape
    *
    * This function multiplies the current scale of the object,
    * unlike sfconvexShape_setScale which overwrites it.
    *
    * # Arguments
    * * factor_x - Scale factor x
    * * factor_y - Scale factor y
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn scale2f(&mut self, factor_x : f32, factor_y : f32) -> () {
        unsafe {
            ffi::sfConvexShape_scale(self.convex_shape, Vector2f::new(factor_x, factor_y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_point(&mut self, index : uint, point : &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setPoint(self.convex_shape, index as c_uint, *point)
        }
    }

    /**
    * Get the position of a convex shape
    *
    * Return the current position
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getPosition(self.convex_shape)
        }
    }
    
    /**
    * Get the current scale of a convex shape
    *
    * Return the current scale factors
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getScale(self.convex_shape)
        }
    }
    
    /**
    * Get the local origin of a convex shape
    *
    * return the current origin
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getOrigin(self.convex_shape)
        }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_point(&self, index : uint) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getPoint(self.convex_shape, index as c_uint)
        }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            ffi::sfConvexShape_setRotation(self.convex_shape, angle as c_float)
        }
    }

    /**
    * Get the orientation of a convex shape
    *
    * The rotation is always in the range [0, 360].
    *
    * Return the current rotation, in degrees
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfConvexShape_getRotation(self.convex_shape) as float
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfConvexShape_rotate(self.convex_shape, angle as c_float)
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
    * If reset_rect is true, the TextureRect property of
    * the shape is automatically adjusted to the size of the new
    * texture. If it is false, the texture rect is left unchanged.
    *
    * # Arguments
    * * texture - New texture
    * * reset_rect - Should the texture rect be reset to the size of the new texture?
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_texture(&mut self, texture : @mut Texture, reset_rect : bool) -> () {
        self.texture = Some(texture);
        unsafe {
            match reset_rect {
                true        => ffi::sfConvexShape_setTexture(self.convex_shape, texture.unwrap(), 1),
                false       => ffi::sfConvexShape_setTexture(self.convex_shape, texture.unwrap(), 0)
            }
        }
    }

    /**
    * Disable Texturing
    *
    * Disable the current texture and reset the texture rect
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn disable_texture(&mut self) -> () {
        self.texture = None;
        unsafe {
            ffi::sfConvexShape_setTexture(self.convex_shape, ptr::null(), 1)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_fill_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfConvexShape_setFillColor(self.convex_shape, *color)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_outline_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfConvexShape_setOutlineColor(self.convex_shape, *color)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_outline_thickness(&mut self, thickness : float) -> () {
        unsafe {
            ffi::sfConvexShape_setOutlineThickness(self.convex_shape, thickness as c_float)
        }
    }

    /**
    * Get the source texture of a convex shape
    *
    * You can't modify the texture when you retrieve it with this function.
    * 
    * Return the shape's texture
    */
    pub fn get_texture(&self) -> Option<@mut Texture> {
        self.texture
    }
    
    /**
    * Get the fill color of a convex shape
    *
    * Return the fill color of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfConvexShape_getFillColor(self.convex_shape)
        }
    }
    
    /**
    * Get the outline color of a convex shape
    *
    * Return the outline color of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfConvexShape_getOutlineColor(self.convex_shape)
        }
    }
    
    /**
    * Get the outline thickness of a convex shape
    *
    * Return the outline thickness of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            ffi::sfConvexShape_getOutlineThickness(self.convex_shape) as float
        }
    }

    /**
    * Get the total number of points of a convex shape
    *
    * Return the number of points of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_point_count(&self) -> uint {
        unsafe {
            ffi::sfConvexShape_getPointCount(self.convex_shape) as uint
        }
    }

    /**
    * Set the number of points of a convex
    *
    * # Arguments
    * * count - New number of points of the convex
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_point_count(&mut self, count : uint) -> () {
        unsafe {
            ffi::sfConvexShape_setPointCount(self.convex_shape, count as c_uint)
        }
    }

    /**
    * Get the sub-rectangle of the texture displayed by a convex shape
    *
    * Return the texture rectangle of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_texture_rect(&mut self, rect : &IntRect) -> () {
        unsafe {
            ffi::sfConvexShape_setTextureRect(self.convex_shape, *rect)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfConvexShape_getLocalBounds(self.convex_shape)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfConvexShape_getGlobalBounds(self.convex_shape)
        }
    }

    /**
    * Get the sub-rectangle of the texture displayed by a convex shape
    *
    * Return the texture rectangle of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfConvexShape_getTextureRect(self.convex_shape)
        }
    }
    
    /**
    * Get the combined transform of a convex shape
    *
    * Return transform combining the position/rotation/scale/origin of the object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfConvexShape_getTransform(self.convex_shape)
        }
    }

    /**
    * Get the inverse of the combined transform of a convex shape
    *
    * Return inverse of the combined transformations applied to the object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfConvexShape_getInverseTransform(self.convex_shape)
        }
    }
}

#[doc(hidden)]
impl Wrappable<*ffi::sfConvexShape> for ConvexShape {
    #[doc(hidden)]
    fn wrap(convex_shape : *ffi::sfConvexShape) -> ConvexShape {
        ConvexShape {
            convex_shape : convex_shape,
            texture : None

        }
    }
    
    #[doc(hidden)]
    fn unwrap(&self) -> *ffi::sfConvexShape {
        self.convex_shape
    }

}

impl Drawable for ConvexShape {
    fn draw_in_render_window(&self, render_window : &RenderWindow) -> () {
        render_window.draw_convex_shape(self)
    }

    fn draw_in_render_window_rs(&self, render_window : &RenderWindow, render_states : &mut RenderStates) -> () {
        render_window.draw_convex_shape_rs(self, render_states)
    }

    fn draw_in_render_texture(&self, render_texture : &RenderTexture) -> () {
        render_texture.draw_convex_shape(self)
    }

    fn draw_in_render_texture_rs(&self, render_texture : &RenderTexture, render_states : &mut RenderStates) -> () {
        render_texture.draw_convex_shape_rs(self, render_states)
    }
}

#[unsafe_destructor]
impl Drop for ConvexShape {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&self) -> () {
        unsafe {
            ffi::sfConvexShape_destroy(self.convex_shape)
        }
    }
}
