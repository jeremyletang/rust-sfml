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

//! Specialized shape representing a circle.

use libc::{c_float, c_uint};
use std::ptr;

use graphics::{IntRect, FloatRect, Color, Texture, Transformable,
               RenderTarget, Transform, RenderStates, Drawable};
use system::vector2::Vector2f;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Specialized shape representing a circle.
pub struct CircleShape<'s> {
    ptr: Foreign<ffi::sfCircleShape>,
    texture: Option<&'s Texture>
}

impl<'s> CircleShape<'s> {
    /// Create a new circle shape
    ///
    /// Return Some(CircleShape) or None
    pub fn new() -> Option<CircleShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfCircleShape_create())
		}.map(|ptr| CircleShape {
			ptr: ptr,
			texture: None
		})
    }

    /// Create a new circle shape initialized with a texture
    ///
    /// # Arguments
    /// * texture - The texture to initialize the CircleShape with.
    ///
    /// Return Some(CircleShape) or None
    pub fn new_with_texture(texture: &'s Texture) -> Option<CircleShape<'s>> {
        CircleShape::new().map(|mut shape| {
			shape.set_texture(texture, true);
			shape
		})
    }

    /// Create a new CircleShape and initialize it.
    ///
    /// # Arguments:
    /// * radius - The radius of the CircleShape
    /// * point_count - The number of points to define the CircleShape
    ///
    /// Default value on SFML are radius = 0 / pointCount = 30
    ///
    /// Return Some(CircleShape) or None
    pub fn new_init(radius: f32, point_count: u32) -> Option<CircleShape<'s>> {
        CircleShape::new().map(|mut shape| {
			shape.set_radius(radius);
			shape.set_point_count(point_count);
			shape
		})
    }

	fn raw(&self) -> &ffi::sfCircleShape { self.ptr.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfCircleShape { self.ptr.as_mut() }
	#[doc(hidden)]
	pub unsafe fn unwrap(&self) -> &ffi::sfCircleShape { self.raw() }

    /// Copy an existing circle shape
    ///
    /// # Arguments
    /// * shape - Shape to copy
    ///
    /// Return Some(CircleShape) or None
    pub fn clone_opt(&self) -> Option<CircleShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfCircleShape_copy(self.raw()))
		}.map(|ptr| CircleShape {
			ptr: ptr,
			texture: None
		})
    }

    /// Change the source texture of a circle shape
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
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfCircleShape_setTexture(self.raw_mut(), texture.unwrap(), SfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) {
        self.texture = None;
        unsafe {
            ffi::sfCircleShape_setTexture(self.raw_mut(), ptr::null_mut(), SfBool::SFTRUE)
        }
    }

    /// Set the sub-rectangle of the texture that a circle shape will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rec - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: &IntRect) -> () {
        unsafe {
            ffi::sfCircleShape_setTextureRect(self.raw_mut(), *rect)
        }
    }

    /// Set the fill color of a circle shape
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
            ffi::sfCircleShape_setFillColor(self.raw_mut(), *color)
        }
    }

    /// Set the outline color of a circle shape
    ///
    /// You can use Transparent to disable the outline.
    /// By default, the shape's outline color is opaque white.
    ///
    /// # Arguments
    /// * color - New outline color of the shape
    pub fn set_outline_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfCircleShape_setOutlineColor(self.raw_mut(), *color)
        }
    }

    /// Set the thickness of a circle shape's outline
    ///
    /// This number cannot be negative. Using zero disables
    /// the outline.
    /// By default, the outline thickness is 0.
    ///
    /// # Arguments
    /// * thickness - New outline thickness
    pub fn set_outline_thickness(&mut self, thickness: f32) -> () {
        unsafe {
            ffi::sfCircleShape_setOutlineThickness(self.raw_mut(),
                                                   thickness as c_float)
        }
    }

    /// Get the source texture of a circle shape
    ///
    /// You can't modify the texture when you retrieve it with this function.
    ///
    /// Return the shape's texture
    pub fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Get the sub-rectangle of the texture displayed by a circle shape
    ///
    /// Return the texture rectangle of the shape
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfCircleShape_getTextureRect(self.raw())
        }
    }

    /// Get the fill color of a circle shape
    ///
    /// Return the fill color of the shape
    pub fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfCircleShape_getFillColor(self.raw())
        }
    }

    /// Get the outline color of a circle shape
    ///
    /// Return the outline color of the shape
    pub fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfCircleShape_getOutlineColor(self.raw())
        }
    }

    /// Get the outline thickness of a circle shape
    ///
    /// Return the outline thickness of the shape
    pub fn get_outline_thickness(&self) -> f32 {
        unsafe {
            ffi::sfCircleShape_getOutlineThickness(self.raw()) as f32
        }
    }

    /// Get the total number of points of a circle shape
    ///
    /// Return the number of points of the shape
    pub fn get_point_count(&self) -> u32 {
        unsafe {
            ffi::sfCircleShape_getPointCount(self.raw()) as u32
        }
    }

    /// Get a point of a circle shape
    ///
    /// The result is undefined if index is out of the valid range.
    ///
    /// # Arguments
    /// * index - Index of the point to get, in range [0 .. getPointCount() - 1]
    ///
    /// Return the index-th point of the shape
    pub fn get_point(&self, index: u32) -> () {
        unsafe {
            ffi::sfCircleShape_getPoint(self.raw(), index as c_uint)
        }
    }

    /// Set the radius of a circle
    ///
    /// # Arguments
    /// * radius - New radius of the circle
    pub fn set_radius(&mut self, radius: f32) -> () {
        unsafe {
            ffi::sfCircleShape_setRadius(self.raw_mut(), radius as c_float)
        }
    }

    /// Set the radius of a circle
    ///
    /// Return the radius of the circle
    pub fn get_radius(&self) -> f32 {
        unsafe {
            ffi::sfCircleShape_getRadius(self.raw()) as f32
        }
    }

    /// Set the number of points of a circle
    ///
    /// # Arguments
    /// * count - New number of points of the circle
    pub fn set_point_count(&mut self, count: u32) -> () {
        unsafe {
            ffi::sfCircleShape_setPointCount(self.raw_mut(), count as c_uint)
        }
    }

    /// Get the local bounding rectangle of a circle shape
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
            ffi::sfCircleShape_getLocalBounds(self.raw())
        }
    }

    /// Get the global bounding rectangle of a circle shape
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
            ffi::sfCircleShape_getGlobalBounds(self.raw())
        }
    }
}

impl<'a> Transformable for CircleShape<'a> {
    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfCircleShape_getPosition(self.raw())
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfCircleShape_getScale(self.raw())
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfCircleShape_getOrigin(self.raw())
        }
    }

    fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_move(self.raw_mut(), *offset)
        }
    }

    fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_scale(self.raw_mut(), *factors)
        }
    }

    fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_setPosition(self.raw_mut(), *position)
        }
    }

    fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_setScale(self.raw_mut(), *scale)
        }
    }

    fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfCircleShape_setOrigin(self.raw_mut(), *origin)
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfCircleShape_getTransform(self.raw())
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfCircleShape_getInverseTransform(self.raw())
        }
    }

    fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfCircleShape_setRotation(self.raw_mut(), angle as c_float)
        }
    }
	
    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfCircleShape_getRotation(self.raw()) as f32
        }
    }

    fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfCircleShape_rotate(self.raw_mut(), angle as c_float)
        }
    }
}

impl<'s> Clone for CircleShape<'s> {
    fn clone(&self) -> CircleShape<'s> {
		self.clone_opt().expect("Failed to clone CircleShape")
    }
}

impl<'s> Drawable for CircleShape<'s> {
    fn draw<RT:RenderTarget>(&self,
                                render_target: &mut RT,
                                render_states: &RenderStates) -> () {
        render_target.draw_circle_shape_rs(self, render_states)
    }
}
