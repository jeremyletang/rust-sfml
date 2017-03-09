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

use std::ptr;

use system::raw_conv::{Raw, FromRaw};
use system::Vector2f;
use graphics::{Drawable, Shape, Transformable, FloatRect, IntRect, Color, Texture, RenderTarget,
               Transform, RenderStates};

use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use csfml_graphics_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Specialized shape representing a rectangle
pub struct RectangleShape<'s> {
    rectangle_shape: *mut ffi::sfRectangleShape,
    texture: Option<&'s Texture>,
}

impl<'s> RectangleShape<'s> {
    /// Returns a new `RectangleShape`.
    pub fn new() -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        if rectangle.is_null() {
            panic!("sfRectangleShape_create returned null.")
        } else {
            RectangleShape {
                rectangle_shape: rectangle,
                texture: None,
            }
        }
    }

    /// Returns a new `RectangleShape` with the provided texture.
    pub fn with_texture(texture: &'s Texture) -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        if rectangle.is_null() {
            panic!("sfRectangleShape_create returned null.")
        } else {
            unsafe {
                ffi::sfRectangleShape_setTexture(rectangle, texture.raw(), sfTrue);
            }
            RectangleShape {
                rectangle_shape: rectangle,
                texture: Some(texture),
            }
        }
    }

    /// Returns a new `RectangleShape` with the provided size.
    pub fn with_size(size: &Vector2f) -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        if rectangle.is_null() {
            panic!("sfRectangleShape_create returned null.")
        } else {
            unsafe {
                ffi::sfRectangleShape_setSize(rectangle, size.raw());
            }
            RectangleShape {
                rectangle_shape: rectangle,
                texture: None,
            }
        }
    }

    /// Get the size of a rectangle shape
    ///
    /// Return the height Size of the rectangle
    pub fn size(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getSize(self.rectangle_shape)) }
    }

    /// Set the size of a rectangle shape
    ///
    /// # Arguments
    /// * size - The new size of the rectangle
    pub fn set_size(&mut self, size: &Vector2f) {
        unsafe { ffi::sfRectangleShape_setSize(self.rectangle_shape, size.raw()) }
    }

    /// Set the size of a rectangle shape
    ///
    /// # Arguments
    /// * size_x - The new size x of the rectangle
    /// * size_y - The new size y of the rectangle
    pub fn set_size2f(&mut self, size_x: f32, size_y: f32) {
        unsafe {
            ffi::sfRectangleShape_setSize(self.rectangle_shape,
                                          sfVector2f {
                                              x: size_x,
                                              y: size_y,
                                          })
        }
    }
}

impl<'s> Default for RectangleShape<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Drawable for RectangleShape<'s> {
    fn draw(&self, render_target: &mut RenderTarget, render_states: &mut RenderStates) {
        render_target.draw_rectangle_shape(self, render_states);
    }
}

impl<'s> Transformable for RectangleShape<'s> {
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfRectangleShape_setPosition(self.rectangle_shape, position.raw()) }
    }
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe {
            ffi::sfRectangleShape_setPosition(self.rectangle_shape, sfVector2f { x: x, y: y })
        }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfRectangleShape_setRotation(self.rectangle_shape, angle) }
    }
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfRectangleShape_setScale(self.rectangle_shape, scale.raw()) }
    }
    fn set_scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfRectangleShape_setScale(self.rectangle_shape,
                                           sfVector2f {
                                               x: factor_x,
                                               y: factor_y,
                                           })
        }
    }
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfRectangleShape_setOrigin(self.rectangle_shape, origin.raw()) }
    }
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfRectangleShape_setOrigin(self.rectangle_shape, sfVector2f { x: x, y: y }) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getPosition(self.rectangle_shape)) }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getRotation(self.rectangle_shape) as f32 }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getScale(self.rectangle_shape)) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getOrigin(self.rectangle_shape)) }
    }
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfRectangleShape_move(self.rectangle_shape, offset.raw()) }
    }
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfRectangleShape_move(self.rectangle_shape,
                                       sfVector2f {
                                           x: offset_x,
                                           y: offset_y,
                                       })
        }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfRectangleShape_rotate(self.rectangle_shape, angle) }
    }
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfRectangleShape_scale(self.rectangle_shape, factors.raw()) }
    }
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfRectangleShape_scale(self.rectangle_shape,
                                        sfVector2f {
                                            x: factor_x,
                                            y: factor_y,
                                        })
        }
    }
    fn get_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfRectangleShape_getTransform(self.rectangle_shape)) }
    }
    fn get_inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfRectangleShape_getInverseTransform(self.rectangle_shape)) }
    }
}

impl<'s> Shape<'s> for RectangleShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfRectangleShape_setTexture(self.rectangle_shape,
                                             texture.raw(),
                                             sfBool::from_bool(reset_rect))
        }
    }
    fn disable_texture(&mut self) {
        self.texture = None;
        unsafe { ffi::sfRectangleShape_setTexture(self.rectangle_shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfRectangleShape_setTextureRect(self.rectangle_shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfRectangleShape_setFillColor(self.rectangle_shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfRectangleShape_setOutlineColor(self.rectangle_shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfRectangleShape_setOutlineThickness(self.rectangle_shape, thickness) }
    }
    fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }
    fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfRectangleShape_getTextureRect(self.rectangle_shape)) }
    }
    fn get_fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfRectangleShape_getFillColor(self.rectangle_shape)) }
    }
    fn get_outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfRectangleShape_getOutlineColor(self.rectangle_shape)) }
    }
    fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getOutlineThickness(self.rectangle_shape) }
    }
    fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfRectangleShape_getPointCount(self.rectangle_shape) as u32 }
    }
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRectangleShape_getPoint(self.rectangle_shape, index as usize))
        }
    }
    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfRectangleShape_getLocalBounds(self.rectangle_shape)) }
    }
    fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfRectangleShape_getGlobalBounds(self.rectangle_shape)) }
    }
}

impl<'s> Clone for RectangleShape<'s> {
    /// Return a new RectangleShape or panic! if there is not enough memory
    fn clone(&self) -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_copy(self.rectangle_shape) };
        if rectangle.is_null() {
            panic!("Not enough memory to clone RectangleShape")
        } else {
            RectangleShape {
                rectangle_shape: rectangle,
                texture: self.texture,
            }
        }
    }
}

impl<'s> Raw for RectangleShape<'s> {
    type Raw = *mut ffi::sfRectangleShape;
    fn raw(&self) -> Self::Raw {
        self.rectangle_shape
    }
}

impl<'s> Drop for RectangleShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfRectangleShape_destroy(self.rectangle_shape) }
    }
}
