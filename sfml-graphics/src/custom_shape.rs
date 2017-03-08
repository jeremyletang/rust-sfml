// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Base class for textured shapes with outline

use std::os::raw::c_void;
use std::ptr;

use sfml::system::raw_conv::{Raw, FromRaw};
use shape::ShapeImpl;
use {Drawable, Transformable, RenderTarget, RenderStates, Texture, Color, Transform, IntRect,
     FloatRect};
use sfml::system::Vector2f;
use csfml_graphics_sys as ffi;
use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use sfml::system::SfBoolExt;

/// Base class for textured shapes with outline
pub struct CustomShape<'s> {
    shape: *mut ffi::sfShape,
    texture: Option<&'s Texture>,
    shape_impl: *mut Box<ShapeImpl + Send>,
}

unsafe extern "C" fn get_point_count_callback(obj: *mut c_void) -> usize {
    let shape = obj as *mut Box<ShapeImpl + Send>;
    let ret = (*shape).get_point_count();
    ret as usize
}

unsafe extern "C" fn get_point_callback(point: usize, obj: *mut c_void) -> sfVector2f {
    let shape = obj as *mut Box<ShapeImpl + Send>;
    let ret = (*shape).get_point(point as u32);
    ret.raw()
}


impl<'s> CustomShape<'s> {
    /// Create a new CustomShape
    ///
    /// # Arguments
    /// * shape_impl - Implementation of ShapeImpl
    pub fn new(shape_impl: Box<ShapeImpl + Send>) -> CustomShape<'s> {
        let raw_impl = Box::into_raw(Box::new(shape_impl));
        let sp = unsafe {
            ffi::sfShape_create(Some(get_point_count_callback),
                                Some(get_point_callback),
                                raw_impl as *mut _)
        };
        if sp.is_null() {
            panic!("sfShape_create returned null.")
        } else {
            CustomShape {
                shape: sp,
                texture: None,
                shape_impl: raw_impl,
            }
        }
    }

    /// Create a new CustomShape with a texture
    ///
    /// # Arguments
    /// * shape_impl - Implementation of ShapeImpl trait
    /// * texture - The texture to bind to the CustomShape
    pub fn with_texture(shape_impl: Box<ShapeImpl + Send>,
                        texture: &'s Texture)
                        -> CustomShape<'s> {
        let raw_impl = Box::into_raw(Box::new(shape_impl));
        let sp = unsafe {
            ffi::sfShape_create(Some(get_point_count_callback),
                                Some(get_point_callback),
                                raw_impl as *mut _)
        };
        if sp.is_null() {
            panic!("sfShape_create returned null.")
        } else {
            unsafe {
                ffi::sfShape_setTexture(sp, texture.raw(), sfTrue);
            }
            CustomShape {
                shape: sp,
                texture: Some(texture),
                shape_impl: raw_impl,
            }
        }
    }

    /// Change the source texture of a shape
    ///
    /// The texture argument refers to a texture that must
    /// exist as long as the shape uses it. Indeed, the shape
    /// doesn't store its own copy of the texture, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If reset_rect is true, the TextureRect property of
    /// the shape is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// # Arguments
    /// * texture - The new texture
    /// * reset_rect - Should the texture rect be reset to the size of the new texture?
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe { ffi::sfShape_setTexture(self.shape, texture.raw(), sfBool::from_bool(reset_rect)) }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) {
        self.texture = None;
        unsafe { ffi::sfShape_setTexture(self.shape, ptr::null_mut(), sfTrue) }
    }

    /// Set the sub-rectangle of the texture that a shape will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rect - The rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfShape_setTextureRect(self.shape, rect.raw()) }
    }

    /// Set the fill color of a shape
    ///
    /// This color is modulated (multiplied) with the shape's
    /// texture if any. It can be used to colorize the shape,
    /// or change its global opacity.
    /// You can use sfTransparent to make the inside of
    /// the shape transparent, and have the outline alone.
    /// By default, the shape's fill color is opaque white.
    ///
    /// # Arguments
    /// * color - The new color of the Shape
    pub fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfShape_setFillColor(self.shape, color.raw()) }
    }

    /// Set the outline color of a shape
    ///
    /// You can use Transparent to disable the outline.
    /// By default, the Shape's outline color is opaque white.
    ///
    /// # Arguments
    /// * color - The new outline color of the shape
    pub fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfShape_setOutlineColor(self.shape, color.raw()) }
    }

    /// Set the thickness of a shape's outline
    ///
    /// This number cannot be negative. Using zero disables
    /// the outline.
    /// By default, the outline thickness is 0.
    ///
    /// # Arguments
    /// * thickness - The new outline thickness
    pub fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfShape_setOutlineThickness(self.shape, thickness) }
    }

    /// Get the source texture of a shape
    ///
    /// If the shape has no source texture, a None is returned.
    /// The returned pointer is const, which means that you can't
    /// modify the texture when you retrieve it with this function.
    ///
    /// Return the pointer to the Shape's texture
    pub fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Get the sub-rectangle of the texture displayed by a shape
    ///
    /// Return the texture rectangle of the shape
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfShape_getTextureRect(self.shape)) }
    }

    /// Get the fill color of a shape
    ///
    /// Return the fill color of the shape
    pub fn get_fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfShape_getFillColor(self.shape)) }
    }

    /// Get the outline color of a shape
    ///
    /// Return the outline color of the shape
    pub fn get_outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfShape_getOutlineColor(self.shape)) }
    }

    /// Get the outline thickness of a shape
    ///
    /// Return the outline thickness of the shape
    pub fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfShape_getOutlineThickness(self.shape) as f32 }
    }

    /// Get the total number of points of a shape
    ///
    /// Return the number of points of the shape
    pub fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfShape_getPointCount(self.shape) as u32 }
    }

    /// Get a point of a shape
    ///
    /// The result is undefined if index is out of the valid range.
    ///
    /// # Arguments
    /// * index - The index of the point to get, in range [0 .. getPointCount() - 1]
    ///
    /// Return the index-th point of the shape
    pub fn get_point(&self, index: u32) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getPoint(self.shape, index as usize)) }
    }

    /// Get the local bounding rectangle of a shape
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    ///
    /// Return the local bounding rectangle of the entity
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfShape_getLocalBounds(self.shape)) }
    }

    /// Get the global bounding rectangle of a shape
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    ///
    /// Return the global bounding rectangle of the entity
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfShape_getGlobalBounds(self.shape)) }
    }

    /// Recompute the internal geometry of a shape
    ///
    /// This function must be called by specialized shape objects
    /// everytime their points change (ie. the result of either
    /// the getPointCount or getPoint callbacks is different).
    pub fn update(&mut self) {
        unsafe { ffi::sfShape_update(self.shape) }
    }
}

impl<'s> Raw for CustomShape<'s> {
    type Raw = *mut ffi::sfShape;
    fn raw(&self) -> Self::Raw {
        self.shape
    }
}

impl<'s> Drawable for CustomShape<'s> {
    fn draw(&self, render_target: &mut RenderTarget, render_states: &mut RenderStates) {
        render_target.draw_shape(self, render_states)
    }
}

impl<'s> Transformable for CustomShape<'s> {
    /// Set the position of a shape
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a Shape object is (0, 0).
    ///
    /// # Arguments
    /// * position - The new position of the Shape
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfShape_setPosition(self.shape, position.raw()) }
    }

    /// Set the position of a shape
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a Shape object is (0, 0).
    ///
    /// # Arguments
    /// * x - The new x position of the Shape
    /// * y - The new y position of the Shape
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfShape_setPosition(self.shape, sfVector2f { x: x, y: y }) }
    }

    /// Set the orientation of a shape
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a Shape object is 0.
    ///
    /// # Arguments
    /// * angle - The new rotation, in degrees
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfShape_setRotation(self.shape, angle) }
    }

    /// Set the scale factors of a shape
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a Shape object is (1, 1).
    ///
    /// # Arguments
    /// scale - The new scale factors
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfShape_setScale(self.shape, scale.raw()) }
    }

    /// Set the scale factors of a shape
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a Shape object is (1, 1).
    ///
    /// # Arguments
    /// scale_x - The new x scale factors
    /// scale_y - The new y scale factors
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfShape_setScale(self.shape,
                                  sfVector2f {
                                      x: scale_x,
                                      y: scale_y,
                                  })
        }
    }

    /// Set the local origin of a shape
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a Shape object is (0, 0).
    ///
    /// # Arguments
    /// * origin - The new origin
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfShape_setOrigin(self.shape, origin.raw()) }
    }

    /// Set the local origin of a shape
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a Shape object is (0, 0).
    ///
    /// # Arguments
    /// * x - The new x origin
    /// * y - The new y origin
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfShape_setOrigin(self.shape, sfVector2f { x: x, y: y }) }
    }

    /// Get the position of a shape
    ///
    /// Return the current position
    fn get_position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getPosition(self.shape)) }
    }

    /// Get the orientation of a shape
    ///
    /// The rotation is always in the range [0, 360].
    ///
    /// Return the current rotation, in degrees
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfShape_getRotation(self.shape) as f32 }
    }

    /// Get the current scale of a shape
    ///
    /// Return the current scale factors
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getScale(self.shape)) }
    }

    /// Get the local origin of a shape
    ///
    /// Return the current origin
    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getOrigin(self.shape)) }
    }

    /// Move a shape by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfShape_move(self.shape, offset.raw()) }
    }

    /// Move a shape by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike set_position which overwrites it.
    ///
    /// # Arguments
    /// * offset_x - Offset x
    /// * offset_y - Offset y
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfShape_move(self.shape,
                              sfVector2f {
                                  x: offset_x,
                                  y: offset_y,
                              })
        }
    }

    /// Rotate a shape
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - The angle of rotation, in degrees
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfShape_rotate(self.shape, angle) }
    }

    /// Scale a shape
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfShape_scale(self.shape, factors.raw()) }
    }

    /// Scale a shape
    ///
    /// This function multiplies the current scale of the object,
    /// unlike set_scale which overwrites it.
    ///
    /// # Arguments
    /// * factor_x - x Scale factors
    /// * factor_y - y Scale factors
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfShape_scale(self.shape,
                               sfVector2f {
                                   x: factor_x,
                                   y: factor_y,
                               })
        }
    }

    /// Get the combined transform of a shape
    ///
    /// Return the transform combining the position/rotation/scale/origin
    /// of the object
    fn get_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfShape_getTransform(self.shape)) }
    }

    /// Get the inverse of the combined transform of a shape
    ///
    /// Return the inverse of the combined transformations applied to the object
    fn get_inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfShape_getInverseTransform(self.shape)) }
    }
}

impl<'s> Drop for CustomShape<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfShape_destroy(self.shape);
            Box::from_raw(self.shape_impl);
        }
    }
}
