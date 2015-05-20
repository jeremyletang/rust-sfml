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

use system::Vector2f;
use graphics::{FloatRect, IntRect, Color, Texture, Transformable,
               RenderTarget, Transform, RenderStates, Drawable, Shape};

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Specialized shape representing a rectangle.
///
/// This type implements all the functionality of `Transformable` and `Shape`.
pub struct RectangleShape<'s> {
    rectangle_shape: Foreign<ffi::sfRectangleShape>,
    texture: Option<&'s Texture>
}

impl<'s> RectangleShape<'s> {
    /// Create a new rectangle shape with the default size of (0, 0).
    ///
    /// Returns Some(RectangleShape) or None on failure.
    pub fn new() -> Option<RectangleShape<'s>> {
		unsafe {
			Foreign::new(ffi::sfRectangleShape_create())
		}.map(|rect| RectangleShape {
			rectangle_shape: rect,
			texture: None
		})
    }

    /// Create a new rectangle shape initialized with a texture.
    ///
    /// Returns Some(RectangleShape) or None on failure.
    pub fn new_with_texture(texture: &'s Texture) -> Option<RectangleShape<'s>> {
		RectangleShape::new().map(|mut rect| {
			rect.set_texture(texture, true);
			rect
		})
    }

    /// Create a new rectangle shape initialized with a size.
    ///
    /// Returns Some(RectangleShape) or None on failure.
    pub fn new_init(size: Vector2f) -> Option<RectangleShape<'s>> {
		RectangleShape::new().map(|mut rect| {
			rect.set_size(size);
			rect
		})
    }

	fn raw(&self) -> &ffi::sfRectangleShape { self.rectangle_shape.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfRectangleShape { self.rectangle_shape.as_mut() }
	#[doc(hidden)]
    pub unsafe fn unwrap(&self) -> &ffi::sfRectangleShape { self.raw() }

    /// Clone an existing rectangle shape.
    ///
    /// Returns Some(RectangleShape) or None on failure.
    pub fn clone_opt(&self) -> Option<RectangleShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfRectangleShape_copy(self.raw()))
		}.map(|rect| RectangleShape {
			rectangle_shape: rect,
			texture: self.texture
		})
    }

    /// Get the size of the rectangle.
    pub fn get_size(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getSize(self.raw()) }
    }

	/// Set the size of the rectangle.
    pub fn set_size(&mut self, size: Vector2f) {
        unsafe { ffi::sfRectangleShape_setSize(self.raw_mut(), size) }
    }

	/// Set the size of the rectangle.
	#[inline]
    pub fn set_size2f(&mut self, size_x: f32, size_y: f32) {
		self.set_size(Vector2f::new(size_x, size_y))
    }

    /// Change the source texture of the shape.
    ///
    /// If reset_rect is true, the TextureRect property of
    /// the shape is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfRectangleShape_setTexture(self.raw_mut(),
                                             texture.unwrap(),
                                             SfBool::from_bool(reset_rect))
        }
    }

    /// Disable the source texture of this shape and reset the texture rect.
    pub fn disable_texture(&mut self) {
        self.texture = None;
        unsafe {
            ffi::sfRectangleShape_setTexture(self.raw_mut(),
                                             ptr::null_mut(),
                                             SfBool::SFTRUE)
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
        unsafe { ffi::sfRectangleShape_setTextureRect(self.raw_mut(), rect) }
    }

    /// Get the sub-rectangle of the texture displayed by the shape.
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe { ffi::sfRectangleShape_getTextureRect(self.raw()) }
    }

    /// Get the local bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfRectangleShape_getLocalBounds(self.raw())
        }
    }

    /// Get the global bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfRectangleShape_getGlobalBounds(self.raw()) }
    }
}

impl<'s> Shape for RectangleShape<'s> {
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getPoint(self.raw(), index as c_uint) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfRectangleShape_setFillColor(self.raw_mut(), color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfRectangleShape_setOutlineColor(self.raw_mut(), color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe {
            ffi::sfRectangleShape_setOutlineThickness(self.raw_mut(), thickness as c_float)
        }
    }
    fn get_fill_color(&self) -> Color {
        unsafe { ffi::sfRectangleShape_getFillColor(self.raw()) }
    }
    fn get_outline_color(&self) -> Color {
        unsafe { ffi::sfRectangleShape_getOutlineColor(self.raw()) }
    }
    fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getOutlineThickness(self.raw()) }
    }
    fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfRectangleShape_getPointCount(self.raw()) as u32 }
    }
}

impl<'s> Transformable for RectangleShape<'s> {
    fn set_position(&mut self, position: Vector2f) {
        unsafe { ffi::sfRectangleShape_setPosition(self.raw_mut(), position) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe {
            ffi::sfRectangleShape_setRotation(self.raw_mut(), angle as c_float)
        }
    }
    fn set_scale(&mut self, scale: Vector2f) {
        unsafe { ffi::sfRectangleShape_setScale(self.raw_mut(), scale) }
    }
    fn set_origin(&mut self, origin: Vector2f) {
        unsafe { ffi::sfRectangleShape_setOrigin(self.raw_mut(), origin) }
    }
    fn scale(&mut self, factors: Vector2f) {
        unsafe { ffi::sfRectangleShape_scale(self.raw_mut(), factors) }
    }
    fn move_(&mut self, offset: Vector2f) {
        unsafe { ffi::sfRectangleShape_move(self.raw_mut(), offset) }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getRotation(self.raw()) as f32 }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe {
            ffi::sfRectangleShape_rotate(self.raw_mut(), angle as c_float)
        }
    }
    fn get_position(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getPosition(self.raw()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getScale(self.raw()) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getOrigin(self.raw()) }
    }
    fn get_transform(&self) -> Transform {
        unsafe { ffi::sfRectangleShape_getTransform(self.raw()) }
    }
    fn get_inverse_transform(&self) -> Transform {
        unsafe { ffi::sfRectangleShape_getInverseTransform(self.raw()) }
    }
}

impl<'s> Clone for RectangleShape<'s> {
    fn clone(&self) -> RectangleShape<'s> {
		self.clone_opt().expect("Failed to clone RectangleShape")
    }
}

impl<'s> Drawable for RectangleShape<'s> {
    fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
        target.draw_rectangle_shape_rs(self, states);
    }
}
