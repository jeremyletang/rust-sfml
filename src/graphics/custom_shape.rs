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

use std::os::raw::c_void;
use std::ptr;

use system::raw_conv::{Raw, FromRaw};
use graphics::{Drawable, Transformable, RenderTarget, RenderStates, Texture, Color, Transform,
               IntRect, FloatRect, Shape};
use system::Vector2f;
use csfml_graphics_sys as ffi;
use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use ext::sf_bool_ext::SfBoolExt;

/// Implement this shape to create a new `Shape`
pub trait ShapeImpl {
    /// Get the total count of the point for the Shape who implement this trait.
    ///
    /// Return the points count
    fn get_point_count(&self) -> u32;

    /// Get a given point of a `Shape`.
    ///
    /// # Argument
    /// * point - The index of the point to return
    ///
    /// Return a `Vector2f` who contains the point coordinates.
    fn get_point(&self, point: u32) -> Vector2f;
}

/// A custom textured shape with outline.
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

    /// Recompute the internal geometry of a shape
    ///
    /// This function must be called by specialized shape objects
    /// everytime their points change (ie. the result of either
    /// the getPointCount or getPoint callbacks is different).
    pub fn update(&mut self) {
        unsafe { ffi::sfShape_update(self.shape) }
    }
}

impl<'s> Shape<'s> for CustomShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe { ffi::sfShape_setTexture(self.shape, texture.raw(), sfBool::from_bool(reset_rect)) }
    }
    fn disable_texture(&mut self) {
        self.texture = None;
        unsafe { ffi::sfShape_setTexture(self.shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfShape_setTextureRect(self.shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfShape_setFillColor(self.shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfShape_setOutlineColor(self.shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfShape_setOutlineThickness(self.shape, thickness) }
    }
    fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }
    fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfShape_getTextureRect(self.shape)) }
    }
    fn get_fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfShape_getFillColor(self.shape)) }
    }
    fn get_outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfShape_getOutlineColor(self.shape)) }
    }
    fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfShape_getOutlineThickness(self.shape) as f32 }
    }
    fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfShape_getPointCount(self.shape) as u32 }
    }
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getPoint(self.shape, index as usize)) }
    }
    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfShape_getLocalBounds(self.shape)) }
    }
    fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfShape_getGlobalBounds(self.shape)) }
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
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfShape_setPosition(self.shape, position.raw()) }
    }
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfShape_setPosition(self.shape, sfVector2f { x: x, y: y }) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfShape_setRotation(self.shape, angle) }
    }
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfShape_setScale(self.shape, scale.raw()) }
    }
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfShape_setScale(self.shape,
                                  sfVector2f {
                                      x: scale_x,
                                      y: scale_y,
                                  })
        }
    }
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfShape_setOrigin(self.shape, origin.raw()) }
    }
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfShape_setOrigin(self.shape, sfVector2f { x: x, y: y }) }
    }
    fn get_position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getPosition(self.shape)) }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfShape_getRotation(self.shape) as f32 }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getScale(self.shape)) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getOrigin(self.shape)) }
    }
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfShape_move(self.shape, offset.raw()) }
    }
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfShape_move(self.shape,
                              sfVector2f {
                                  x: offset_x,
                                  y: offset_y,
                              })
        }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfShape_rotate(self.shape, angle) }
    }
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfShape_scale(self.shape, factors.raw()) }
    }
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfShape_scale(self.shape,
                               sfVector2f {
                                   x: factor_x,
                                   y: factor_y,
                               })
        }
    }
    fn get_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfShape_getTransform(self.shape)) }
    }
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
