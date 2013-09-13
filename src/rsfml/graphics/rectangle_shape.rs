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
use std::ptr;

use traits::drawable::Drawable;
use traits::wrappable::Wrappable;
use system::vector2::Vector2f;
use graphics::color::Color;
use graphics::texture::Texture;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::rect::{FloatRect, IntRect};
use graphics::transform::Transform;
use graphics::render_states::RenderStates;
use sfml_types::*;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_float, c_uint};

    use system::vector2::Vector2f;
    use graphics::color::Color;
    use graphics::texture;
    use sfml_types::SfBool;
    use graphics::rect::{FloatRect, IntRect};
    use graphics::transform::Transform;

    pub struct sfRectangleShape {
        This :              *c_void,
        Texture :           *texture::ffi::sfTexture,
        Transform :         Transform,
        InverseTransform :  Transform
    }
    
    extern "C" {
        pub fn sfRectangleShape_create() -> *sfRectangleShape;
        pub fn sfRectangleShape_copy(shape : *sfRectangleShape) -> *sfRectangleShape;
        pub fn sfRectangleShape_destroy(shape : *sfRectangleShape) -> ();
        pub fn sfRectangleShape_setPosition(shape : *sfRectangleShape, position : Vector2f) -> ();
        pub fn sfRectangleShape_setRotation(shape : *sfRectangleShape, angle : c_float) -> ();
        pub fn sfRectangleShape_setScale(shape : *sfRectangleShape, scale : Vector2f) -> ();
        pub fn sfRectangleShape_setOrigin(shape : *sfRectangleShape, origin : Vector2f) -> ();
        pub fn sfRectangleShape_getPosition(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_getRotation(shape : *sfRectangleShape) -> c_float;
        pub fn sfRectangleShape_getScale(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_getOrigin(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_move(shape : *sfRectangleShape, offset : Vector2f) -> ();
        pub fn sfRectangleShape_rotate(shape : *sfRectangleShape, angle : c_float) -> ();
        pub fn sfRectangleShape_scale(shape : *sfRectangleShape, factors : Vector2f) -> ();
        pub fn sfRectangleShape_getTransform(shape : *sfRectangleShape) -> Transform;
        pub fn sfRectangleShape_getInverseTransform(shape : *sfRectangleShape) -> Transform;
        pub fn sfRectangleShape_setTexture(shape : *sfRectangleShape, texture : *texture::ffi::sfTexture, reset_rect : SfBool) -> ();
        pub fn sfRectangleShape_setTextureRect(shape : *sfRectangleShape, rect : IntRect) -> ();
        pub fn sfRectangleShape_setFillColor(shape : *sfRectangleShape, color : Color) -> ();
        pub fn sfRectangleShape_setOutlineColor(shape : *sfRectangleShape, color : Color) -> ();
        pub fn sfRectangleShape_setOutlineThickness(shape : *sfRectangleShape, thickness : c_float) -> ();
        pub fn sfRectangleShape_getTexture(shape : *sfRectangleShape) -> *texture::ffi::sfTexture;
        pub fn sfRectangleShape_getTextureRect(shape : *sfRectangleShape) -> IntRect;
        pub fn sfRectangleShape_getFillColor(shape : *sfRectangleShape) -> Color;
        pub fn sfRectangleShape_getOutlineColor(shape : *sfRectangleShape) -> Color;
        pub fn sfRectangleShape_getOutlineThickness(shape : *sfRectangleShape) -> c_float;
        pub fn sfRectangleShape_getPointCount(shape : *sfRectangleShape) -> c_uint;
        pub fn sfRectangleShape_getPoint(shape : *sfRectangleShape, index : c_uint) -> Vector2f;
        pub fn sfRectangleShape_setSize(shape : *sfRectangleShape, size : Vector2f) -> ();
        pub fn sfRectangleShape_getSize(shape : *sfRectangleShape) -> Vector2f;
        pub fn sfRectangleShape_getLocalBounds(shape : *sfRectangleShape) -> FloatRect;
        pub fn sfRectangleShape_getGlobalBounds(shape : *sfRectangleShape) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct RectangleShape {
    priv rectangle_shape :  *ffi::sfRectangleShape,
    priv texture :          Option<@mut Texture>
}

impl RectangleShape {
    /**
    * Create a new rectangle shape
    *
    * Return a new option to a rectangleShape object or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Option<RectangleShape> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        if ptr::is_null(rectangle) {
            None
        }
        else {
            Some(RectangleShape {
                rectangle_shape :   rectangle,
                texture :           None
            })
        }
    }

    /**
    * Create a new rectangle shape with a texture
    *
    * Return a new option to a rectangleShape object or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_with_texture(texture : @mut Texture) -> Option<RectangleShape> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        if ptr::is_null(rectangle) {
            None
        }
        else {
            unsafe {
                ffi::sfRectangleShape_setTexture(rectangle, texture.unwrap(), SFTRUE);
            }
            Some(RectangleShape {
                rectangle_shape :   rectangle,
                texture :           Some(texture)
            })
        }
    }

    /**
    * Create a new rectangle shape initialized
    *
    * Default value on SFML is size = Vector2f(0, 0) 
    *
    * Return a new option to a rectangleShape object, or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_init(size : &Vector2f) -> Option<RectangleShape> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        if ptr::is_null(rectangle) {
            None
        }
        else {
            unsafe{
                ffi::sfRectangleShape_setSize(rectangle, *size);
            }
            Some(RectangleShape {
                rectangle_shape :   rectangle,
                texture :           None
            })
        }
    }

    /**
    * Clone an existing rectangle shape
    * 
    * Return the copied object on an option, or None
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn clone(&self) -> Option<RectangleShape> {
        let rectangle = unsafe { ffi::sfRectangleShape_copy(self.rectangle_shape) };
        if ptr::is_null(rectangle) {
            None
        }
        else {
            Some(RectangleShape {
                rectangle_shape :   rectangle,
                texture :           self.texture
            })
        }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_position(&mut self, position : &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setPosition(self.rectangle_shape, *position)
        }
    }
    
    /**
    * Set the position of a rectangle shape with 2 f32
    *
    * This function completely overwrites the previous position.
    * See move to apply an offset based on the previous position instead.
    * The default position of a rectangle Shape object is (0, 0).
    *
    * # Arguments
    * * x - X coordinate of the new position
    * * y - Y coordinate of the new position
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_position2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setPosition(self.rectangle_shape, Vector2f::new(x, y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfRectangleShape_setRotation(self.rectangle_shape, angle as c_float)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_scale(&mut self, scale : &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setScale(self.rectangle_shape, *scale)
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
    * * factor_x - New x scale factor
    * * factor_y - New y scale factor
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_scale2f(&mut self, factor_x : f32, factor_y: f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setScale(self.rectangle_shape, Vector2f::new(factor_x, factor_y))
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_origin(&mut self, origin : &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setOrigin(self.rectangle_shape, *origin)
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
    * * x - New coordinate x of the origin
    * * y - New coordinate y of the origin
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_origin2f(&mut self, x : f32, y : f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setOrigin(self.rectangle_shape, Vector2f::new(x, y))
        }
    }

    /**
    * Scale a rectangle shape
    *
    * This function multiplies the current scale of the object,
    * unlike set_scale which overwrites it.
    *
    * # Arguments
    * * factors - Scale factors
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn scale(&mut self, factors : &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_scale(self.rectangle_shape, *factors)
        }
    }

    /**
    * Scale a rectangle shape
    *
    * This function multiplies the current scale of the object,
    * unlike set_scale which overwrites it.
    *
    * # Arguments
    * * factor_x - Scale x factor
    * * factor_y - Scale y factor
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn scale2f(&mut self, factor_x : f32, factor_y : f32) -> () {
        unsafe {
            ffi::sfRectangleShape_scale(self.rectangle_shape, Vector2f::new(factor_x, factor_y))
        }
    }

    /**
    * Move a rectangle shape by a given offset
    *
    * This function adds to the current position of the object,
    * unlike set_position which overwrites it.
    *
    * # Arguments
    * * offset - Offset
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_move(self.rectangle_shape, *offset)
        }
    }

    /**
    * Move a rectangle shape by a given offset of 2 f32
    *
    * This function adds to the current position of the object,
    * unlike set_position which overwrites it.
    *
    * # Arguments
    * * offsetX - Offset in x
    * * offsetY - Offset in y
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn move2f(&mut self, offset_x : f32, offset_y : f32) -> () {
        unsafe {ffi::sfRectangleShape_move(self.rectangle_shape, Vector2f::new(offset_x, offset_y))}
    }

    /**
    * Get the orientation of a rectangle shape
    *
    * The rotation is always in the range [0, 360].
    *
    * Return the current rotation, in degrees
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfRectangleShape_getRotation(self.rectangle_shape) as float
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfRectangleShape_rotate(self.rectangle_shape, angle as c_float)
        }
    }

    /**
    * Get the position of a rectangle shape
    *
    * Return the current position
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfRectangleShape_getPosition(self.rectangle_shape) 
        }
    }

    /**
    * Get the current scale of a rectangle shape
    *
    * Return the current scale factors
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_scale(&self) -> Vector2f {
        unsafe { 
            ffi::sfRectangleShape_getScale(self.rectangle_shape) 
        }
    }

    /**
    * Get the local origin of a rectangle shape
    *
    * return the current origin
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_origin(&self) -> Vector2f {
        unsafe { 
            ffi::sfRectangleShape_getOrigin(self.rectangle_shape) 
        }
    }

    /**
    * Get the size of a rectangle shape
    *
    * Return the height Size of the rectangle
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_size(&self) -> Vector2f {
        unsafe { 
            ffi::sfRectangleShape_getSize(self.rectangle_shape) 
        }
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_point(&self, index : uint) -> Vector2f {
        unsafe { 
            ffi::sfRectangleShape_getPoint(self.rectangle_shape, index as c_uint) 
        }
    }

    /**
    * Set the size of a rectangle shape
    *
    * # Arguments
    * * size - The new size of the rectangle
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_size(&mut self, size : &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setSize(self.rectangle_shape, *size)
        }
    }

    /**
    * Set the size of a rectangle shape
    *
    * # Arguments
    * * size_x - The new size x of the rectangle
    * * size_y - The new size y of the rectangle
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_size2f(&mut self, size_x : f32, size_y : f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setSize(self.rectangle_shape, Vector2f::new(size_x, size_y))
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
                false       => ffi::sfRectangleShape_setTexture(self.rectangle_shape, texture.unwrap(), SFFALSE),
                true        => ffi::sfRectangleShape_setTexture(self.rectangle_shape, texture.unwrap(), SFTRUE)
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
            ffi::sfRectangleShape_setTexture(self.rectangle_shape, ptr::null(), SFTRUE)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_fill_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfRectangleShape_setFillColor(self.rectangle_shape, *color)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_outline_color(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfRectangleShape_setOutlineColor(self.rectangle_shape, *color)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_outline_thickness(&mut self, thickness : float) -> () {
        unsafe {
            ffi::sfRectangleShape_setOutlineThickness(self.rectangle_shape, thickness as c_float)
        }
    }

    /**
    * Get the source texture of a rectangle shape
    *
    * You can't modify the texture when you retrieve it with this function.
    * 
    * Return the shape's texture
    */
    pub fn get_texture(&self) -> Option<@mut Texture> {
        self.texture
    }

    /**
    * Get the fill color of a rectangle shape
    *
    * Return the fill color of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfRectangleShape_getFillColor(self.rectangle_shape)
        }
    }

    /**
    * Get the outline color of a rectangle shape
    *
    * Return the outline color of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfRectangleShape_getOutlineColor(self.rectangle_shape)
        }
    }

    /**
    * Get the outline thickness of a rectangle shape
    *
    * Return the outline thickness of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            ffi::sfRectangleShape_getOutlineThickness(self.rectangle_shape) as float 
        }
    }

    /**
    * Get the total number of points of a rectangle shape
    *
    * Return the number of points of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_point_count(&self) -> uint {
        unsafe {
            ffi::sfRectangleShape_getPointCount(self.rectangle_shape) as uint
        }
    }

    /**
    * Get the sub-rectangle of the texture displayed by a rectangle shape
    *
    * Return the texture rectangle of the shape
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfRectangleShape_getTextureRect(self.rectangle_shape)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_texture_rect(&mut self, rect : &IntRect) -> () {
        unsafe {
            ffi::sfRectangleShape_setTextureRect(self.rectangle_shape, *rect)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfRectangleShape_getGlobalBounds(self.rectangle_shape)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfRectangleShape_getLocalBounds(self.rectangle_shape)
        }
    }

    /**
    * Get the combined transform of a rectangle shape
    *
    * Return transform combining the position/rotation/scale/origin of the object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfRectangleShape_getTransform(self.rectangle_shape)
        }
    }

    /**
    * Get the inverse of the combined transform of a rectangle shape
    *
    * Return inverse of the combined transformations applied to the object
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfRectangleShape_getInverseTransform(self.rectangle_shape)
        }
    }
}

#[doc(hidden)]
impl Wrappable<*ffi::sfRectangleShape> for RectangleShape {
    fn wrap(rectangle_shape : *ffi::sfRectangleShape) -> RectangleShape {
        RectangleShape {
            rectangle_shape :   rectangle_shape,
            texture :           None
        }
    }
    
    fn unwrap(&self) -> *ffi::sfRectangleShape {
        self.rectangle_shape
    }
}

impl Drawable for RectangleShape {
    fn draw_in_render_window(&self, render_window : &RenderWindow) -> () {
        render_window.draw_rectangle_shape(self);
    }

    fn draw_in_render_window_rs(&self, render_window : &RenderWindow, render_states : &mut RenderStates) -> () {
        render_window.draw_rectangle_shape_rs(self, render_states);
    }

    fn draw_in_render_texture(&self, render_texture : &RenderTexture) -> () {
        render_texture.draw_rectangle_shape(self);
    }
 
    fn draw_in_render_texture_rs(&self, render_texture : &RenderTexture, render_states : &mut RenderStates) -> () {
        render_texture.draw_rectangle_shape_rs(self, render_states);
    }
}

#[unsafe_destructor]
impl Drop for RectangleShape {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&self) -> () {
        unsafe {
            ffi::sfRectangleShape_destroy(self.rectangle_shape)
        }
    }
}
