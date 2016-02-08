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

//! Specialized shape representing a convex polygon
//!
//! It is important to keep in mind that a convex shape must
//! always be... convex, otherwise it may not be drawn correctly.
//! Moreover, the points must be defined in order; using a random
//! order would result in an incorrect shape.

use libc::{c_float, c_uint};
use std::ptr;

use traits::Wrappable;
use graphics::{Shape, Drawable, Transformable, Color, Texture, RenderTarget, FloatRect, IntRect, Transform, RenderStates};
use sfml_types::Vector2f;

use sfml_types::sfBool;
use csfml_graphics_sys as ffi;

// pub mod rc;

/// Specialized shape representing a convex polygon
///
/// It is important to keep in mind that a convex shape must
/// always be... convex, otherwise it may not be drawn correctly.
/// Moreover, the points must be defined in order; using a random
/// order would result in an incorrect shape.
pub struct ConvexShape<'s> {
    convex_shape: *mut ffi::sfConvexShape,
    texture: Option<&'s Texture>
}

/// An iterator over the points of a ConvexShape
pub struct ConvexShapePoints {
    convex_shape: *mut ffi::sfConvexShape,
    pos: u32
}

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape
    ///
    /// # Arguments
    /// * points_count - The number of point for the convex shape
    ///
    /// Return Some(ConvexShape) or None
    pub fn new(points_count: u32) -> Option<ConvexShape<'s>> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        if shape.is_null() {
            None
        } else {
            unsafe {
                ffi::sfConvexShape_setPointCount(shape, points_count as c_uint);
            }
            Some(ConvexShape {
                    convex_shape: shape,
                    texture: None
                })
        }
    }

    /// Create a new convex shape with a texture
    ///
    /// # Arguments
    /// * texture - The texture to apply to the convex shape
    /// * points_count - The number of point for the convex shape
    ///
    /// Return Some(ConvexShape) or None
    pub fn new_with_texture(texture: &'s Texture,
                            points_count: u32) -> Option<ConvexShape<'s>> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        if shape.is_null() {
            None
        } else {
            unsafe {
                ffi::sfConvexShape_setTexture(shape, texture.unwrap(), sfBool::SFTRUE);
                ffi::sfConvexShape_setPointCount(shape, points_count as c_uint)
            }
            Some(ConvexShape {
                    convex_shape: shape,
                    texture: Some(texture)
                })
        }
    }

    /// Clone an existing convex shape
    ///
    /// Return Some(ConvexShape) or None
    pub fn clone_opt(&self) -> Option<ConvexShape<'s>> {
        let shape = unsafe { ffi::sfConvexShape_copy(self.convex_shape) };
        if shape.is_null() {
            None
        } else {
            Some(ConvexShape {
                    convex_shape: shape,
                    texture: self.texture
                })
        }
    }

    /// Set the position of a point.
    ///
    /// Don't forget that the polygon must remain convex, and the points need to stay ordered!
    /// set_point_count must be called first in order to set the total number of points.
    /// The result is undefined if index is out of the valid range.
    ///
    /// # Arguments
    /// * index - Index of the point to change, in range [0 .. getPointCount() - 1]
    /// * point - New position of the point
    pub fn set_point(&mut self, index: u32, point: &Vector2f) {
        unsafe {
            ffi::sfConvexShape_setPoint(self.convex_shape,
                                        index as c_uint, *point)
        }
    }

    /// Set the number of points of a convex
    ///
    /// # Arguments
    /// * count - New number of points of the convex
    pub fn set_point_count(&mut self, count: u32) {
        unsafe {
            ffi::sfConvexShape_setPointCount(self.convex_shape, count as c_uint)
        }
    }

    /// Return an immutable iterator over all the points of the ConvexShape
    pub fn points(&self) -> ConvexShapePoints {
        ConvexShapePoints {
            convex_shape: self.convex_shape.clone(),
            pos: 0
        }
    }
}

impl<'s> Drawable for ConvexShape<'s> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, render_states: &mut RenderStates) {
        render_target.draw_convex_shape(self, render_states)
    }
}


impl<'s> Transformable for ConvexShape<'s> {
    /// Set the position of a convex shape
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a convex Shape object is (0, 0).
    ///
    /// # Arguments
    /// * position - New position
    fn set_position(&mut self, position: &Vector2f) {
        unsafe {
            ffi::sfConvexShape_setPosition(self.convex_shape, *position)
        }
    }

    /// Set the position of a convex shape
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a convex Shape object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x coordinate
    /// * y - New y coordinate
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::sfConvexShape_setPosition(self.convex_shape, Vector2f::new(x, y))
        }
    }

    /// Set the orientation of a convex shape
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a convex Shape object is 0.
    ///
    /// # Arguments
    /// * rotation - New rotation
    fn set_rotation(&mut self, angle: f32){
        unsafe {
            ffi::sfConvexShape_setRotation(self.convex_shape, angle as c_float)
        }
    }

    /// Set the scale factors of a convex shape
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a convex Shape object is (1, 1).
    ///
    /// # Arguments
    /// * scale - New scale factors
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe {
            ffi::sfConvexShape_setScale(self.convex_shape, *scale)
        }
    }

    /// Set the scale factors of a convex shape
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a convex Shape object is (1, 1).
    ///
    /// # Arguments
    /// * scale_x - New x scale factor
    /// * scale_y - New y scale factor
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfConvexShape_setScale(self.convex_shape,
                                        Vector2f::new(scale_x, scale_y))
        }
    }

    /// Set the local origin of a convex shape
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a convex Shape object is (0, 0).
    ///
    /// # Arguments
    /// * origin - New origin
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe {
            ffi::sfConvexShape_setOrigin(self.convex_shape, *origin)
        }
    }

    /// Set the local origin of a convex shape
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a convex Shape object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x origin coordinate
    /// * y - New y origin coordinate
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::sfConvexShape_setOrigin(self.convex_shape, Vector2f::new(x, y))
        }
    }

    /// Get the position of a convex shape
    ///
    /// Return the current position
    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getPosition(self.convex_shape)
        }
    }

    /// Get the orientation of a convex shape
    ///
    /// The rotation is always in the range [0, 360].
    ///
    /// Return the current rotation, in degrees
    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfConvexShape_getRotation(self.convex_shape) as f32
        }
    }

    /// Get the current scale of a convex shape
    ///
    /// Return the current scale factors
    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getScale(self.convex_shape)
        }
    }

    /// Get the local origin of a convex shape
    ///
    /// return the current origin
    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getOrigin(self.convex_shape)
        }
    }

    /// Move a convex shape by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sfconvexShape_setPosition which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    fn move_(&mut self, offset: &Vector2f) {
        unsafe {
            ffi::sfConvexShape_move(self.convex_shape, *offset)
        }
    }

    /// Move a convex shape by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sfconvexShape_setPosition which overwrites it.
    ///
    /// # Arguments
    /// * offsetX - Offset x
    /// * offsetY - Offset y
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfConvexShape_move(self.convex_shape,
                                    Vector2f::new(offset_x, offset_y))
        }
    }

    /// Rotate a convex shape
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - Angle of rotation, in degrees
    fn rotate(&mut self, angle: f32) {
        unsafe {
            ffi::sfConvexShape_rotate(self.convex_shape, angle as c_float)
        }
    }

    /// Scale a convex shape
    ///
    /// This function multiplies the current scale of the object,
    /// unlike sfconvexShape_setScale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    fn scale(&mut self, factors: &Vector2f) {
        unsafe {
            ffi::sfConvexShape_scale(self.convex_shape, *factors)
        }
    }

    /// Scale a convex shape
    ///
    /// This function multiplies the current scale of the object,
    /// unlike sfconvexShape_setScale which overwrites it.
    ///
    /// # Arguments
    /// * factor_x - Scale factor x
    /// * factor_y - Scale factor y
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfConvexShape_scale(self.convex_shape,
                                     Vector2f::new(factor_x, factor_y))
        }
    }

    /// Get the combined transform of a convex shape
    ///
    /// Return transform combining the position/rotation/scale/origin of the object
    fn get_transform(&self) -> Transform {
        unsafe {
            Transform(ffi::sfConvexShape_getTransform(self.convex_shape))
        }
    }

    /// Get the inverse of the combined transform of a convex shape
    ///
    /// Return inverse of the combined transformations applied to the object
    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            Transform(ffi::sfConvexShape_getInverseTransform(self.convex_shape))
        }
    }
}

impl<'s> Shape<'s> for ConvexShape<'s> {
    /// Change the source texture of a convex shape
    ///
    /// The texture argument refers to a texture that must
    /// exist as long as the shape uses it. Indeed, the shape
    /// doesn't store its own copy of the texture, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the source texture is destroyed and the shape tries to
    /// use it, the behaviour is undefined.
    /// If reset_rect is true, the TextureRect property of
    /// the shape is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// # Arguments
    /// * texture - New texture
    /// * reset_rect - Should the texture rect be reset to the size of the new texture?
    fn set_texture(&mut self,
                       texture: &'s Texture,
                       reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfConvexShape_setTexture(self.convex_shape, texture.unwrap(), sfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    fn disable_texture(&mut self) {
        self.texture = None;
        unsafe {
            ffi::sfConvexShape_setTexture(self.convex_shape,
                                          ptr::null_mut(),
                                          sfBool::SFTRUE)
        }
    }

    /// Get the sub-rectangle of the texture displayed by a convex shape
    ///
    /// Return the texture rectangle of the shape
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe {
            ffi::sfConvexShape_setTextureRect(self.convex_shape, *rect)
        }
    }

    /// Set the fill color of a convex shape
    ///
    /// This color is modulated (multiplied) with the shape's
    /// texture if any. It can be used to colorize the shape,
    /// or change its global opacity.
    /// You can use sfTransparent to make the inside of
    /// the shape transparent, and have the outline alone.
    /// By default, the shape's fill color is opaque white.
    ///
    /// # Arguments
    /// * color - New color of the shape
    fn set_fill_color(&mut self, color: &Color) {
        unsafe {
            ffi::sfConvexShape_setFillColor(self.convex_shape, color.0)
        }
    }

    /// Set the outline color of a convex shape
    ///
    /// You can use Transparent to disable the outline.
    /// By default, the shape's outline color is opaque white.
    ///
    /// # Arguments
    /// * color - New outline color of the shape
    fn set_outline_color(&mut self, color: &Color) {
        unsafe {
            ffi::sfConvexShape_setOutlineColor(self.convex_shape, color.0)
        }
    }

    /// Set the thickness of a convex shape's outline
    ///
    /// This number cannot be negative. Using zero disables
    /// the outline.
    /// By default, the outline thickness is 0.
    ///
    /// # Arguments
    /// * thickness - New outline thickness
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            ffi::sfConvexShape_setOutlineThickness(self.convex_shape,
                                                   thickness as c_float)
        }
    }

    /// Get the source texture of a convex shape
    ///
    /// You can't modify the texture when you retrieve it with this function.
    ///
    /// Return the shape's texture
    fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Get the sub-rectangle of the texture displayed by a convex shape
    ///
    /// Return the texture rectangle of the shape
    fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfConvexShape_getTextureRect(self.convex_shape)
        }
    }

    /// Get the fill color of a convex shape
    ///
    /// Return the fill color of the shape
    fn get_fill_color(&self) -> Color {
        unsafe {
            Color(ffi::sfConvexShape_getFillColor(self.convex_shape))
        }
    }

    /// Get the outline color of a convex shape
    ///
    /// Return the outline color of the shape
    fn get_outline_color(&self) -> Color {
        unsafe {
            Color(ffi::sfConvexShape_getOutlineColor(self.convex_shape))
        }
    }

    /// Get the outline thickness of a convex shape
    ///
    /// Return the outline thickness of the shape
    fn get_outline_thickness(&self) -> f32 {
        unsafe {
            ffi::sfConvexShape_getOutlineThickness(self.convex_shape) as f32
        }
    }

    /// Get the total number of points of a convex shape
    ///
    /// Return the number of points of the shape
    fn get_point_count(&self) -> u32 {
        unsafe {
            ffi::sfConvexShape_getPointCount(self.convex_shape) as u32
        }
    }

    /// Get a point of a convex shape
    ///
    /// The result is undefined if index is out of the valid range.
    ///
    /// # Arguments
    /// * index- Index of the point to get, in range [0 .. getPointCount() - 1]
    ///
    /// Return the index-th point of the shape
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getPoint(self.convex_shape, index as c_uint)
        }
    }

    /// Get the local bounding rectangle of a convex shape
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    ///
    /// Return the local bounding rectangle of the entity
    fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfConvexShape_getLocalBounds(self.convex_shape)
        }
    }

    /// Get the global bounding rectangle of a convex shape
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    ///
    /// Return the global bounding rectangle of the entity
    fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfConvexShape_getGlobalBounds(self.convex_shape)
        }
    }
}

impl<'s> Clone for ConvexShape<'s> {
    /// Return a new ConvexShape or panic! if there is not enough memory
    fn clone(&self) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_copy(self.convex_shape) };
        if shape.is_null() {
            panic!("Not enough memory to clone ConvexShape")
        } else {
            ConvexShape {
                convex_shape:  shape,
                texture:       self.texture.clone()
            }
        }
    }
}

impl Iterator for ConvexShapePoints {
    type Item = Vector2f;

    fn next(&mut self) -> Option<Vector2f> {
        let point_count =
            unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) as u32 };
        if self.pos == point_count {
            None
        } else {
            self.pos += 1;
            unsafe {
                Some(ffi::sfConvexShape_getPoint(self.convex_shape,
                                                 self.pos as c_uint))
            }
        }
    }
}

#[doc(hidden)]
impl<'s> Wrappable<*mut ffi::sfConvexShape> for ConvexShape<'s> {
    #[doc(hidden)]
    fn wrap(convex_shape: *mut ffi::sfConvexShape) -> ConvexShape<'s> {
        ConvexShape {
            convex_shape: convex_shape,
            texture: None
        }
    }

    #[doc(hidden)]
    fn unwrap(&self) -> *mut ffi::sfConvexShape {
        self.convex_shape
    }
}

impl<'s> Drop for ConvexShape<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfConvexShape_destroy(self.convex_shape)
        }
    }
}
