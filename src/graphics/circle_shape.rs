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

use libc::{c_float, c_uint};
use std::ptr;

use graphics::{IntRect, FloatRect, Color, Texture, Transformable,
               RenderTarget, Transform, RenderStates, Drawable, Shape};
use system::Vector2f;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Specialized shape representing a circle.
///
/// This type implements all the functionality of `Transformable` and `Shape`.
///
/// Since the graphics card can't draw perfect circles, we have to fake them
/// with multiple triangles connected to each other. The "point count" property
/// of `CircleShape` defines how many of these triangles to use, and therefore
/// defines the quality of the circle.
///
/// The number of points can also be used for another purpose; with small
/// numbers you can create any regular polygon shape: equilateral triangle,
/// square, pentagon, hexagon, etc.
pub struct CircleShape<'s> {
    ptr: Foreign<ffi::sfCircleShape>,
    texture: Option<&'s Texture>
}

impl<'s> CircleShape<'s> {
    /// Create a new circle shape.
    ///
    /// Returns Some(CircleShape) or None on failure.
    pub fn new() -> Option<CircleShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfCircleShape_create())
		}.map(|ptr| CircleShape {
			ptr: ptr,
			texture: None
		})
    }

    /// Create a new circle shape initialized with a texture.
    ///
    /// Returns Some(CircleShape) or None on failure.
    pub fn new_with_texture(texture: &'s Texture) -> Option<CircleShape<'s>> {
        CircleShape::new().map(|mut shape| {
			shape.set_texture(texture, true);
			shape
		})
    }

    /// Create a new circle shape initialized with a radius and point count.
	///
	/// The default radius is 0, and the default point count is 30.
    ///
    /// Returns Some(CircleShape) or None on failure.
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

    /// Copy an existing circle shape.
    ///
    /// Returns Some(CircleShape) or None on failure.
    pub fn clone_opt(&self) -> Option<CircleShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfCircleShape_copy(self.raw()))
		}.map(|ptr| CircleShape {
			ptr: ptr,
			texture: None
		})
    }

    /// Change the source texture of the shape.
    ///
    /// If reset_rect is true, the TextureRect property of
    /// the shape is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfCircleShape_setTexture(self.raw_mut(), texture.unwrap(), SfBool::from_bool(reset_rect))
        }
    }

    /// Disable the source texture of this shape and reset the texture rect.
    pub fn disable_texture(&mut self) {
        self.texture = None;
        unsafe {
            ffi::sfCircleShape_setTexture(self.raw_mut(), ptr::null_mut(), SfBool::SFTRUE)
        }
    }

    /// Get the source texture of the shape.
    ///
    /// You can't modify the texture when you retrieve it with this function.
    pub fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Set the sub-rectangle of the texture that the shape will display.
	///
	/// The texture rect is useful when you don't want to display the whole
	/// texture, but rather a part of it. By default, the texture rect covers
	/// the entire texture.
    pub fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfCircleShape_setTextureRect(self.raw_mut(), rect) }
    }

    /// Get the sub-rectangle of the texture displayed by the shape.
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe { ffi::sfCircleShape_getTextureRect(self.raw()) }
    }

    /// Set the radius of the circle.
    pub fn set_radius(&mut self, radius: f32) {
        unsafe {
			ffi::sfCircleShape_setRadius(self.raw_mut(), radius as c_float)
		}
    }

    /// Get the radius of the circle.
    pub fn get_radius(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRadius(self.raw()) as f32 }
    }

    /// Set the number of points of the circle.
    pub fn set_point_count(&mut self, count: u32) {
        unsafe {
            ffi::sfCircleShape_setPointCount(self.raw_mut(), count as c_uint)
        }
    }

    /// Get the local bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfCircleShape_getLocalBounds(self.raw()) }
    }

    /// Get the global bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfCircleShape_getGlobalBounds(self.raw()) }
    }
}

impl<'a> Shape for CircleShape<'a> {
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfCircleShape_setFillColor(self.raw_mut(), color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfCircleShape_setOutlineColor(self.raw_mut(), color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            ffi::sfCircleShape_setOutlineThickness(self.raw_mut(), thickness as c_float)
        }
    }
    fn get_fill_color(&self) -> Color {
        unsafe { ffi::sfCircleShape_getFillColor(self.raw()) }
    }
    fn get_outline_color(&self) -> Color {
        unsafe { ffi::sfCircleShape_getOutlineColor(self.raw()) }
    }
    fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getOutlineThickness(self.raw()) as f32 }
    }
    fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfCircleShape_getPointCount(self.raw()) as u32 }
    }
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe { ffi::sfCircleShape_getPoint(self.raw(), index as c_uint) }
    }
}

impl<'a> Transformable for CircleShape<'a> {
    fn get_position(&self) -> Vector2f {
        unsafe { ffi::sfCircleShape_getPosition(self.raw()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfCircleShape_getScale(self.raw()) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { ffi::sfCircleShape_getOrigin(self.raw()) }
    }
    fn move_(&mut self, offset: Vector2f) {
        unsafe { ffi::sfCircleShape_move(self.raw_mut(), offset) }
    }
    fn scale(&mut self, factors: Vector2f) {
        unsafe { ffi::sfCircleShape_scale(self.raw_mut(), factors) }
    }
    fn set_position(&mut self, position: Vector2f) {
        unsafe { ffi::sfCircleShape_setPosition(self.raw_mut(), position) }
    }
    fn set_scale(&mut self, scale: Vector2f) {
        unsafe { ffi::sfCircleShape_setScale(self.raw_mut(), scale) }
    }
    fn set_origin(&mut self, origin: Vector2f) {
        unsafe { ffi::sfCircleShape_setOrigin(self.raw_mut(), origin) }
    }
    fn get_transform(&self) -> Transform {
        unsafe { ffi::sfCircleShape_getTransform(self.raw()) }
    }
    fn get_inverse_transform(&self) -> Transform {
        unsafe { ffi::sfCircleShape_getInverseTransform(self.raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe {
            ffi::sfCircleShape_setRotation(self.raw_mut(), angle as c_float)
        }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRotation(self.raw()) as f32 }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_rotate(self.raw_mut(), angle as c_float) }
    }
}

impl<'s> Clone for CircleShape<'s> {
    fn clone(&self) -> CircleShape<'s> {
		self.clone_opt().expect("Failed to clone CircleShape")
    }
}

impl<'s> Drawable for CircleShape<'s> {
    fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
        target.draw_circle_shape_rs(self, states)
    }
}
