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

#![allow(missing_copy_implementations)]

//! Specialized shape representing a convex polygon
//!
//! It is important to keep in mind that a convex shape must
//! always be... convex, otherwise it may not be drawn correctly.
//! Moreover, the points must be defined in order; using a random
//! order would result in an incorrect shape.

use libc::{c_float, c_uint};
use std::ptr;

use graphics::{Color, Texture, RenderTarget, FloatRect, IntRect, Transform, RenderStates, Drawable, Transformable};
use system::vector2::Vector2f;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Specialized shape representing a convex polygon
///
/// It is important to keep in mind that a convex shape must
/// always be... convex, otherwise it may not be drawn correctly.
/// Moreover, the points must be defined in order; using a random
/// order would result in an incorrect shape.
pub struct ConvexShape<'s> {
    convex_shape: Foreign<ffi::sfConvexShape>,
    texture: Option<&'s Texture>
}

// TODO: iterators over the points of a ConvexShape

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape
    ///
    /// # Arguments
    /// * points_count - The number of point for the convex shape
    ///
    /// Return Some(ConvexShape) or None
    pub fn new(points_count: u32) -> Option<ConvexShape<'s>> {
		unsafe {
			Foreign::new(ffi::sfConvexShape_create())
		}.map(|shape| ConvexShape {
			convex_shape: shape,
			texture: None
		}).map(|mut shape| {
			shape.set_point_count(points_count);
			shape
		})
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
		ConvexShape::new(points_count).map(|mut shape| {
			shape.set_texture(texture, true);
			shape
		})
    }

	fn raw(&self) -> &ffi::sfConvexShape { self.convex_shape.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfConvexShape { self.convex_shape.as_mut() }
    #[doc(hidden)]
    pub unsafe fn unwrap(&self) -> &ffi::sfConvexShape { self.raw() }

    /// Clone an existing convex shape
    ///
    /// Return Some(ConvexShape) or None
    pub fn clone_opt(&self) -> Option<ConvexShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfConvexShape_copy(self.raw()))
		}.map(|shape| ConvexShape {
			convex_shape: shape,
			texture: self.texture
		})
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
    pub fn set_point(&mut self, index: u32, point: &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setPoint(self.raw_mut(),
                                        index as c_uint, *point)
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
    pub fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getPoint(self.raw(), index as c_uint)
        }
    }

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
    pub fn set_texture(&mut self,
                       texture: &'s Texture,
                       reset_rect: bool) -> () {
        self.texture = Some(texture);
        unsafe {
            ffi::sfConvexShape_setTexture(self.raw_mut(), texture.unwrap(), SfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) -> () {
        self.texture = None;
        unsafe {
            ffi::sfConvexShape_setTexture(self.raw_mut(),
                                          ptr::null_mut(),
                                          SfBool::SFTRUE)
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
    pub fn set_fill_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfConvexShape_setFillColor(self.raw_mut(), *color)
        }
    }

    /// Set the outline color of a convex shape
    ///
    /// You can use Transparent to disable the outline.
    /// By default, the shape's outline color is opaque white.
    ///
    /// # Arguments
    /// * color - New outline color of the shape
    pub fn set_outline_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfConvexShape_setOutlineColor(self.raw_mut(), *color)
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
    pub fn set_outline_thickness(&mut self, thickness: f32) -> () {
        unsafe {
            ffi::sfConvexShape_setOutlineThickness(self.raw_mut(),
                                                   thickness as c_float)
        }
    }

    /// Get the source texture of a convex shape
    ///
    /// You can't modify the texture when you retrieve it with this function.
    ///
    /// Return the shape's texture
    pub fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Get the fill color of a convex shape
    ///
    /// Return the fill color of the shape
    pub fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfConvexShape_getFillColor(self.raw())
        }
    }

    /// Get the outline color of a convex shape
    ///
    /// Return the outline color of the shape
    pub fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfConvexShape_getOutlineColor(self.raw())
        }
    }

    /// Get the outline thickness of a convex shape
    ///
    /// Return the outline thickness of the shape
    pub fn get_outline_thickness(&self) -> f32 {
        unsafe {
            ffi::sfConvexShape_getOutlineThickness(self.raw()) as f32
        }
    }

    /// Get the total number of points of a convex shape
    ///
    /// Return the number of points of the shape
    pub fn get_point_count(&self) -> u32 {
        unsafe {
            ffi::sfConvexShape_getPointCount(self.raw()) as u32
        }
    }

    /// Set the number of points of a convex
    ///
    /// # Arguments
    /// * count - New number of points of the convex
    pub fn set_point_count(&mut self, count: u32) -> () {
        unsafe {
            ffi::sfConvexShape_setPointCount(self.raw_mut(), count as c_uint)
        }
    }

    /// Get the sub-rectangle of the texture displayed by a convex shape
    ///
    /// Return the texture rectangle of the shape
    pub fn set_texture_rect(&mut self, rect: &IntRect) -> () {
        unsafe {
            ffi::sfConvexShape_setTextureRect(self.raw_mut(), *rect)
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
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfConvexShape_getLocalBounds(self.raw())
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
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfConvexShape_getGlobalBounds(self.raw())
        }
    }

    /// Get the sub-rectangle of the texture displayed by a convex shape
    ///
    /// Return the texture rectangle of the shape
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfConvexShape_getTextureRect(self.raw())
        }
    }

	/*
    /// Return an immutable iterator over all the points of the ConvexShape
    pub fn points(&self) -> ConvexShapePoints {
        ConvexShapePoints {
            convex_shape: self.convex_shape.clone(),
            pos: 0
        }
    }
	*/
}

impl<'s> Transformable for ConvexShape<'s> {
    fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setPosition(self.raw_mut(), *position)
        }
    }

    fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setScale(self.raw_mut(), *scale)
        }
    }

    fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_setOrigin(self.raw_mut(), *origin)
        }
    }

    fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_move(self.raw_mut(), *offset)
        }
    }

    fn move2f(&mut self, offset_x: f32, offset_y: f32) -> () {
        unsafe {
            ffi::sfConvexShape_move(self.raw_mut(),
                                    Vector2f::new(offset_x, offset_y))
        }
    }

    fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfConvexShape_scale(self.raw_mut(), *factors)
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getPosition(self.raw())
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getScale(self.raw())
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfConvexShape_getOrigin(self.raw())
        }
    }

    fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfConvexShape_setRotation(self.raw_mut(), angle as c_float)
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfConvexShape_getRotation(self.raw()) as f32
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfConvexShape_getTransform(self.raw())
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfConvexShape_getInverseTransform(self.raw())
        }
    }

    fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfConvexShape_rotate(self.raw_mut(), angle as c_float)
        }
    }
}

impl<'s> Clone for ConvexShape<'s> {
    fn clone(&self) -> ConvexShape<'s> {
		self.clone_opt().expect("Failed to clone ConvexShape")
    }
}

impl<'s> Drawable for ConvexShape<'s> {
    fn draw<RT: RenderTarget>(&self,
                                 render_target: &mut RT,
                                 render_states: &RenderStates) -> () {
        render_target.draw_convex_shape_rs(self, render_states)
    }
}
