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

//! Specialized shape representing a circle.

use std::ptr;
use std::marker::PhantomData;

use system::raw_conv::{Raw, FromRaw};
use graphics::{Drawable, Transformable, Shape, IntRect, FloatRect, Color, Texture, TextureRef,
               RenderTarget, Transform, RenderStates};
use system::Vector2f;

use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use csfml_graphics_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Specialized shape representing a circle.
pub struct CircleShape<'s> {
    circle_shape: *mut ffi::sfCircleShape,
    texture: PhantomData<&'s Texture>,
}

impl<'s> CircleShape<'s> {
    /// Create a new circle shape
    ///
    /// Return Some(CircleShape) or None
    pub fn new() -> CircleShape<'s> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if circle.is_null() {
            panic!("sfCircleShape_create returned null.")
        } else {
            CircleShape {
                circle_shape: circle,
                texture: PhantomData,
            }
        }
    }

    /// Create a new circle shape initialized with a texture
    ///
    /// # Arguments
    /// * texture - The texture to initialize the CircleShape with.
    ///
    /// Return Some(CircleShape) or None
    pub fn with_texture(texture: &'s Texture) -> CircleShape<'s> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if circle.is_null() {
            panic!("sfCircleShape_create returned null.")
        } else {
            unsafe {
                ffi::sfCircleShape_setTexture(circle, texture.raw(), sfTrue);
            }
            CircleShape {
                circle_shape: circle,
                texture: PhantomData,
            }
        }
    }

    /// Create a new CircleShape and initialize it.
    ///
    /// # Arguments:
    /// * radius - The radius of the CircleShape
    /// * point_count - The number of points to define the CircleShape
    ///
    /// Default value on SFML are radius = 0 / pointCount = 30
    pub fn new_init(radius: f32, point_count: u32) -> CircleShape<'s> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if circle.is_null() {
            panic!("sfCircleShape_create returned null.")
        } else {
            unsafe {
                ffi::sfCircleShape_setRadius(circle, radius);
                ffi::sfCircleShape_setPointCount(circle, point_count as usize);
            }
            CircleShape {
                circle_shape: circle,
                texture: PhantomData,
            }
        }
    }

    /// Set the radius of a circle
    ///
    /// # Arguments
    /// * radius - New radius of the circle
    pub fn set_radius(&self, radius: f32) {
        unsafe { ffi::sfCircleShape_setRadius(self.circle_shape, radius) }
    }

    /// Set the radius of a circle
    ///
    /// Return the radius of the circle
    pub fn radius(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRadius(self.circle_shape) as f32 }
    }

    /// Set the number of points of a circle
    ///
    /// # Arguments
    /// * count - New number of points of the circle
    pub fn set_point_count(&mut self, count: u32) {
        unsafe { ffi::sfCircleShape_setPointCount(self.circle_shape, count as usize) }
    }
}

impl<'s> Default for CircleShape<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Drawable for CircleShape<'s> {
    fn draw(&self, render_target: &mut RenderTarget, render_states: &mut RenderStates) {
        render_target.draw_circle_shape(self, render_states)
    }
}

impl<'s> Transformable for CircleShape<'s> {
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfCircleShape_setPosition(self.circle_shape, position.raw()) }
    }
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfCircleShape_setPosition(self.circle_shape, sfVector2f { x: x, y: y }) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_setRotation(self.circle_shape, angle) }
    }
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfCircleShape_setScale(self.circle_shape, scale.raw()) }
    }
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfCircleShape_setScale(self.circle_shape,
                                        sfVector2f {
                                            x: scale_x,
                                            y: scale_y,
                                        })
        }
    }
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfCircleShape_setOrigin(self.circle_shape, origin.raw()) }
    }
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfCircleShape_setOrigin(self.circle_shape, sfVector2f { x: x, y: y }) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getPosition(self.circle_shape)) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRotation(self.circle_shape) as f32 }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getScale(self.circle_shape)) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getOrigin(self.circle_shape)) }
    }
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfCircleShape_move(self.circle_shape, offset.raw()) }
    }
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfCircleShape_move(self.circle_shape,
                                    sfVector2f {
                                        x: offset_x,
                                        y: offset_y,
                                    })
        }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_rotate(self.circle_shape, angle) }
    }
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfCircleShape_scale(self.circle_shape, factors.raw()) }
    }
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfCircleShape_scale(self.circle_shape,
                                     sfVector2f {
                                         x: factor_x,
                                         y: factor_y,
                                     })
        }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfCircleShape_getTransform(self.circle_shape)) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfCircleShape_getInverseTransform(self.circle_shape)) }
    }
}

impl<'s> Shape<'s> for CircleShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe {
            ffi::sfCircleShape_setTexture(self.circle_shape,
                                          texture.raw(),
                                          sfBool::from_bool(reset_rect))
        }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfCircleShape_setTexture(self.circle_shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfCircleShape_setTextureRect(self.circle_shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfCircleShape_setFillColor(self.circle_shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfCircleShape_setOutlineColor(self.circle_shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfCircleShape_setOutlineThickness(self.circle_shape, thickness) }
    }
    fn texture(&self) -> Option<&'s TextureRef> {
        unsafe {
            let raw = ffi::sfCircleShape_getTexture(self.circle_shape);

            if raw.is_null() {
                None
            } else {
                Some(&*(raw as *const TextureRef))
            }
        }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfCircleShape_getTextureRect(self.circle_shape)) }
    }
    fn fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfCircleShape_getFillColor(self.circle_shape)) }
    }
    fn outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfCircleShape_getOutlineColor(self.circle_shape)) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getOutlineThickness(self.circle_shape) as f32 }
    }
    fn point_count(&self) -> u32 {
        unsafe { ffi::sfCircleShape_getPointCount(self.circle_shape) as u32 }
    }
    fn point(&self, index: u32) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfCircleShape_getPoint(self.circle_shape, index as usize))
        }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfCircleShape_getLocalBounds(self.circle_shape)) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfCircleShape_getGlobalBounds(self.circle_shape)) }
    }
}

impl<'s> Clone for CircleShape<'s> {
    /// Return a new CircleShape or panic! if there is not enough memory
    fn clone(&self) -> CircleShape<'s> {
        let circle = unsafe { ffi::sfCircleShape_copy(self.circle_shape) };
        if circle.is_null() {
            panic!("Not enough memory to clone CircleShape")
        } else {
            CircleShape {
                circle_shape: circle,
                texture: self.texture,
            }
        }
    }
}


impl<'s> Drop for CircleShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfCircleShape_destroy(self.circle_shape) }
    }
}

impl<'s> Raw for CircleShape<'s> {
    type Raw = *mut ffi::sfCircleShape;
    fn raw(&self) -> Self::Raw {
        self.circle_shape
    }
}
