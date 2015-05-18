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

//! Specialized shape representing a rectangle

use libc::{c_float, c_uint};
use std::ptr;

use system::Vector2f;
use graphics::{FloatRect, IntRect, Color, Texture, Transformable,
               RenderTarget, Transform, RenderStates, Drawable, Shape};

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Specialized shape representing a rectangle
pub struct RectangleShape<'s> {
    rectangle_shape: Foreign<ffi::sfRectangleShape>,
    texture: Option<&'s Texture>
}

impl<'s> RectangleShape<'s> {
    /// Create a new rectangle shape
    ///
    /// Return Some(RectangleShape) or None
    pub fn new() -> Option<RectangleShape<'s>> {
		unsafe {
			Foreign::new(ffi::sfRectangleShape_create())
		}.map(|rect| RectangleShape {
			rectangle_shape: rect,
			texture: None
		})
    }

    /// Create a new rectangle shape with a texture
    ///
    /// Return Some(RectangleShape) or None
    pub fn new_with_texture(texture: &'s Texture) -> Option<RectangleShape<'s>> {
		RectangleShape::new().map(|mut rect| {
			rect.set_texture(texture, true);
			rect
		})
    }

    /// Create a new rectangle shape initialized
    ///
    /// Default value on SFML is size = Vector2f(0, 0)
    ///
    /// Return Some(RectangleShape) or None
    pub fn new_init(size: &Vector2f) -> Option<RectangleShape<'s>> {
		RectangleShape::new().map(|mut rect| {
			rect.set_size(size);
			rect
		})
    }

	fn raw(&self) -> &ffi::sfRectangleShape { self.rectangle_shape.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfRectangleShape { self.rectangle_shape.as_mut() }
	#[doc(hidden)]
    pub unsafe fn unwrap(&self) -> &ffi::sfRectangleShape { self.raw() }

    /// Clone an existing rectangle shape
    ///
    /// Return Some(RectangleShape) or None
    pub fn clone_opt(&self) -> Option<RectangleShape<'s>> {
        unsafe {
			Foreign::new(ffi::sfRectangleShape_copy(self.raw()))
		}.map(|rect| RectangleShape {
			rectangle_shape: rect,
			texture: self.texture
		})
    }

    /// Get the size of a rectangle shape
    ///
    /// Return the height Size of the rectangle
    pub fn get_size(&self) -> Vector2f {
        unsafe {
            ffi::sfRectangleShape_getSize(self.raw())
        }
    }

    /// Set the size of a rectangle shape
    ///
    /// # Arguments
    /// * size - The new size of the rectangle
    pub fn set_size(&mut self, size: &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setSize(self.raw_mut(), *size)
        }
    }

    /// Set the size of a rectangle shape
    ///
    /// # Arguments
    /// * size_x - The new size x of the rectangle
    /// * size_y - The new size y of the rectangle
    pub fn set_size2f(&mut self, size_x: f32, size_y: f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setSize(self.raw_mut(), Vector2f::new(size_x, size_y))
        }
    }

    /// Change the source texture of a rectangle shape
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
    /// * reset_rect - Should the texture rect be reset to
    /// the size of the new texture?
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) -> () {
        self.texture = Some(texture);
        unsafe {
            ffi::sfRectangleShape_setTexture(self.raw_mut(),
                                             texture.unwrap(),
                                             SfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) -> () {
        self.texture = None;
        unsafe {
            ffi::sfRectangleShape_setTexture(self.raw_mut(),
                                             ptr::null_mut(),
                                             SfBool::SFTRUE)
        }
    }

    /// Get the source texture of a rectangle shape
    ///
    /// You can't modify the texture when you retrieve it with this function.
    ///
    /// Return the shape's texture
    pub fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }

    /// Get the sub-rectangle of the texture displayed by a rectangle shape
    ///
    /// Return the texture rectangle of the shape
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfRectangleShape_getTextureRect(self.raw())
        }
    }

    /// Set the sub-rectangle of the texture that a rectangle shape will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rec - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: &IntRect) -> () {
        unsafe {
            ffi::sfRectangleShape_setTextureRect(self.raw_mut(), *rect)
        }
    }

    /// Get the global bounding rectangle of a rectangle shape
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
            ffi::sfRectangleShape_getGlobalBounds(self.raw())
        }
    }

    /// Get the local bounding rectangle of a rectangle shape
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
            ffi::sfRectangleShape_getLocalBounds(self.raw())
        }
    }
}

impl<'s> Shape for RectangleShape<'s> {
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            ffi::sfRectangleShape_getPoint(self.raw(), index as c_uint)
        }
    }

    fn set_fill_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfRectangleShape_setFillColor(self.raw_mut(), *color)
        }
    }

    fn set_outline_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfRectangleShape_setOutlineColor(self.raw_mut(), *color)
        }
    }

    fn set_outline_thickness(&mut self, thickness: f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setOutlineThickness(self.raw_mut(), thickness as c_float)
        }
    }

    fn get_fill_color(&self) -> Color {
        unsafe {
            ffi::sfRectangleShape_getFillColor(self.raw())
        }
    }

    fn get_outline_color(&self) -> Color {
        unsafe {
            ffi::sfRectangleShape_getOutlineColor(self.raw())
        }
    }

    fn get_outline_thickness(&self) -> f32 {
        unsafe {
            ffi::sfRectangleShape_getOutlineThickness(self.raw())
        }
    }

    fn get_point_count(&self) -> u32 {
        unsafe {
            ffi::sfRectangleShape_getPointCount(self.raw()) as u32
        }
    }
}

impl<'s> Transformable for RectangleShape<'s> {
    fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setPosition(self.raw_mut(), *position)
        }
    }

    fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfRectangleShape_setRotation(self.raw_mut(), angle as c_float)
        }
    }

    fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setScale(self.raw_mut(), *scale)
        }
    }

    fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_setOrigin(self.raw_mut(), *origin)
        }
    }

    fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_scale(self.raw_mut(), *factors)
        }
    }

    fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfRectangleShape_move(self.raw_mut(), *offset)
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfRectangleShape_getRotation(self.raw()) as f32
        }
    }

    fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfRectangleShape_rotate(self.raw_mut(), angle as c_float)
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfRectangleShape_getPosition(self.raw())
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfRectangleShape_getScale(self.raw())
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfRectangleShape_getOrigin(self.raw())
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfRectangleShape_getTransform(self.raw())
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfRectangleShape_getInverseTransform(self.raw())
        }
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
