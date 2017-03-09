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
use graphics::{Shape, Drawable, Transformable, Color, Texture, RenderTarget, FloatRect, IntRect,
               Transform, RenderStates};
use system::Vector2f;

use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use csfml_graphics_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Specialized shape representing a convex polygon
///
/// It is important to keep in mind that a convex shape must
/// always be... convex, otherwise it may not be drawn correctly.
/// Moreover, the points must be defined in order; using a random
/// order would result in an incorrect shape.
pub struct ConvexShape<'s> {
    convex_shape: *mut ffi::sfConvexShape,
    texture: Option<&'s Texture>,
}

/// An iterator over the points of a `ConvexShape`
pub struct ConvexShapePoints {
    convex_shape: *mut ffi::sfConvexShape,
    pos: u32,
}

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape
    ///
    /// # Arguments
    /// * points_count - The number of point for the convex shape
    pub fn new(points_count: u32) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        if shape.is_null() {
            panic!("sfConvexShape_create returned null.")
        } else {
            unsafe {
                ffi::sfConvexShape_setPointCount(shape, points_count as usize);
            }
            ConvexShape {
                convex_shape: shape,
                texture: None,
            }
        }
    }

    /// Create a new convex shape with a texture
    ///
    /// # Arguments
    /// * texture - The texture to apply to the convex shape
    /// * points_count - The number of point for the convex shape
    pub fn with_texture(texture: &'s Texture, points_count: u32) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        if shape.is_null() {
            panic!("sfConvexShape_create returned null.")
        } else {
            unsafe {
                ffi::sfConvexShape_setTexture(shape, texture.raw(), sfTrue);
                ffi::sfConvexShape_setPointCount(shape, points_count as usize)
            }
            ConvexShape {
                convex_shape: shape,
                texture: Some(texture),
            }
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
        unsafe { ffi::sfConvexShape_setPoint(self.convex_shape, index as usize, point.raw()) }
    }

    /// Set the number of points of a convex
    ///
    /// # Arguments
    /// * count - New number of points of the convex
    pub fn set_point_count(&mut self, count: u32) {
        unsafe { ffi::sfConvexShape_setPointCount(self.convex_shape, count as usize) }
    }

    /// Return an immutable iterator over all the points of the ConvexShape
    pub fn points(&self) -> ConvexShapePoints {
        ConvexShapePoints {
            convex_shape: self.convex_shape,
            pos: 0,
        }
    }
}

impl<'s> Drawable for ConvexShape<'s> {
    fn draw(&self, render_target: &mut RenderTarget, render_states: &mut RenderStates) {
        render_target.draw_convex_shape(self, render_states)
    }
}


impl<'s> Transformable for ConvexShape<'s> {
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfConvexShape_setPosition(self.convex_shape, position.raw()) }
    }
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfConvexShape_setPosition(self.convex_shape, sfVector2f { x: x, y: y }) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_setRotation(self.convex_shape, angle) }
    }
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfConvexShape_setScale(self.convex_shape, scale.raw()) }
    }
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfConvexShape_setScale(self.convex_shape,
                                        sfVector2f {
                                            x: scale_x,
                                            y: scale_y,
                                        })
        }
    }
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfConvexShape_setOrigin(self.convex_shape, origin.raw()) }
    }
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfConvexShape_setOrigin(self.convex_shape, sfVector2f { x: x, y: y }) }
    }
    fn get_position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfConvexShape_getPosition(self.convex_shape)) }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getRotation(self.convex_shape) as f32 }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfConvexShape_getScale(self.convex_shape)) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfConvexShape_getOrigin(self.convex_shape)) }
    }
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfConvexShape_move(self.convex_shape, offset.raw()) }
    }
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfConvexShape_move(self.convex_shape,
                                    sfVector2f {
                                        x: offset_x,
                                        y: offset_y,
                                    })
        }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_rotate(self.convex_shape, angle) }
    }
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfConvexShape_scale(self.convex_shape, factors.raw()) }
    }
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfConvexShape_scale(self.convex_shape,
                                     sfVector2f {
                                         x: factor_x,
                                         y: factor_y,
                                     })
        }
    }
    fn get_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfConvexShape_getTransform(self.convex_shape)) }
    }
    fn get_inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfConvexShape_getInverseTransform(self.convex_shape)) }
    }
}

impl<'s> Shape<'s> for ConvexShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfConvexShape_setTexture(self.convex_shape,
                                          texture.raw(),
                                          sfBool::from_bool(reset_rect))
        }
    }
    fn disable_texture(&mut self) {
        self.texture = None;
        unsafe { ffi::sfConvexShape_setTexture(self.convex_shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfConvexShape_setTextureRect(self.convex_shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfConvexShape_setFillColor(self.convex_shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfConvexShape_setOutlineColor(self.convex_shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfConvexShape_setOutlineThickness(self.convex_shape, thickness) }
    }
    fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }
    fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfConvexShape_getTextureRect(self.convex_shape)) }
    }
    fn get_fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfConvexShape_getFillColor(self.convex_shape)) }
    }
    fn get_outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfConvexShape_getOutlineColor(self.convex_shape)) }
    }
    fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getOutlineThickness(self.convex_shape) as f32 }
    }
    fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) as u32 }
    }
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfConvexShape_getPoint(self.convex_shape, index as usize))
        }
    }
    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfConvexShape_getLocalBounds(self.convex_shape)) }
    }
    fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfConvexShape_getGlobalBounds(self.convex_shape)) }
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
                convex_shape: shape,
                texture: self.texture,
            }
        }
    }
}

impl Iterator for ConvexShapePoints {
    type Item = Vector2f;

    fn next(&mut self) -> Option<Vector2f> {
        let point_count = unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) as u32 };
        if self.pos == point_count {
            None
        } else {
            self.pos += 1;
            unsafe {
                Some(Vector2f::from_raw(ffi::sfConvexShape_getPoint(self.convex_shape,
                                                                    self.pos as usize)))
            }
        }
    }
}

impl<'s> Raw for ConvexShape<'s> {
    type Raw = *mut ffi::sfConvexShape;
    fn raw(&self) -> Self::Raw {
        self.convex_shape
    }
}

impl<'s> Drop for ConvexShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfConvexShape_destroy(self.convex_shape) }
    }
}
