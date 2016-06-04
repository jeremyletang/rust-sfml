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

use libc::c_float;
use std::ptr;

use raw_conv::{FromRaw, Raw};
use graphics::{Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture,
               Transform, Transformable};
use system::Vector2f;

use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use csfml_graphics_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

// pub mod rc;

/// Specialized shape representing a circle.
pub struct CircleShape<'s> {
    circle_shape: *mut ffi::sfCircleShape,
    texture: Option<&'s Texture>,
}

impl<'s> CircleShape<'s> {
    /// Create a new circle shape
    ///
    /// Return Some(CircleShape) or None
    pub fn new() -> Option<CircleShape<'s>> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if circle.is_null() {
            None
        } else {
            Some(CircleShape {
                circle_shape: circle,
                texture: None,
            })
        }
    }

    /// Create a new circle shape initialized with a texture
    ///
    /// # Arguments
    /// * texture - The texture to initialize the CircleShape with.
    ///
    /// Return Some(CircleShape) or None
    pub fn new_with_texture(texture: &'s Texture) -> Option<CircleShape<'s>> {
        let circle = unsafe { ffi::sfCircleShape_create() };
        if circle.is_null() {
            None
        } else {
            unsafe {
                ffi::sfCircleShape_setTexture(circle, texture.raw(), sfTrue);
            }
            Some(CircleShape {
                circle_shape: circle,
                texture: Some(texture),
            })
        }
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
        let circle = unsafe { ffi::sfCircleShape_create() };
        if circle.is_null() {
            None
        } else {
            unsafe {
                ffi::sfCircleShape_setRadius(circle, radius as c_float);
                ffi::sfCircleShape_setPointCount(circle, point_count as usize);
            }
            Some(CircleShape {
                circle_shape: circle,
                texture: None,
            })
        }
    }

    /// Copy an existing circle shape
    ///
    /// # Arguments
    /// * shape - Shape to copy
    ///
    /// Return Some(CircleShape) or None
    pub fn clone_opt(&self) -> Option<CircleShape<'s>> {
        let circle = unsafe { ffi::sfCircleShape_copy(self.circle_shape) };
        if circle.is_null() {
            None
        } else {
            Some(CircleShape {
                circle_shape: circle,
                texture: self.texture,
            })
        }
    }

    /// Set the radius of a circle
    ///
    /// # Arguments
    /// * radius - New radius of the circle
    pub fn set_radius(&self, radius: f32) {
        unsafe { ffi::sfCircleShape_setRadius(self.circle_shape, radius as c_float) }
    }

    /// Set the radius of a circle
    ///
    /// Return the radius of the circle
    pub fn get_radius(&self) -> f32 {
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

impl<'s> Drawable for CircleShape<'s> {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT, render_states: &mut RenderStates) {
        render_target.draw_circle_shape(self, render_states)
    }
}

impl<'s> Transformable for CircleShape<'s> {
    /// Set the position of a circle shape
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a circle Shape object is (0, 0).
    ///
    /// # Arguments
    /// * position - New position
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfCircleShape_setPosition(self.circle_shape, position.raw()) }
    }

    /// Set the position of a circle shape
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a circle Shape object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x coordinate
    /// * y - New y coordinate
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfCircleShape_setPosition(self.circle_shape, sfVector2f { x: x, y: y }) }
    }

    /// Set the orientation of a circle shape
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a circle Shape object is 0.
    ///
    /// # Arguments
    /// * rotation - New rotation
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_setRotation(self.circle_shape, angle as c_float) }
    }

    /// Set the scale factors of a circle shape
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a circle Shape object is (1, 1).
    ///
    /// # Arguments
    /// * scale - New scale factors
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfCircleShape_setScale(self.circle_shape, scale.raw()) }
    }

    /// Set the scale factors of a circle shape
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a circle Shape object is (1, 1).
    ///
    /// # Arguments
    /// * scale_x - New x scale factor
    /// * scale_y - New y scale factor
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfCircleShape_setScale(self.circle_shape,
                                        sfVector2f {
                                            x: scale_x,
                                            y: scale_y,
                                        })
        }
    }

    /// Set the local origin of a circle shape
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a circle Shape object is (0, 0).
    ///
    /// # Arguments
    /// * origin - New origin
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfCircleShape_setOrigin(self.circle_shape, origin.raw()) }
    }

    /// Set the local origin of a circle shape
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a circle Shape object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x origin coordinate
    /// * y - New y origin coordinate
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfCircleShape_setOrigin(self.circle_shape, sfVector2f { x: x, y: y }) }
    }

    /// Get the position of a circle shape
    ///
    /// Return the current position
    fn get_position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getPosition(self.circle_shape)) }
    }

    /// Get the orientation of a circle shape
    ///
    /// The rotation is always in the range [0, 360].
    ///
    /// Return the current rotation, in degrees
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRotation(self.circle_shape) as f32 }
    }

    /// Get the current scale of a circle shape
    ///
    /// Return the current scale factors
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getScale(self.circle_shape)) }
    }

    /// Get the local origin of a circle shape
    ///
    /// return the current origin
    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getOrigin(self.circle_shape)) }
    }

    /// Move a circle shape by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sset_position which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfCircleShape_move(self.circle_shape, offset.raw()) }
    }

    /// Move a circle shape by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sset_position which overwrites it.
    ///
    /// # Arguments
    /// * offsetX - Offset x
    /// * offsetY - Offset y
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfCircleShape_move(self.circle_shape,
                                    sfVector2f {
                                        x: offset_x,
                                        y: offset_y,
                                    })
        }
    }

    /// Rotate a circle shape
    ///
    /// This function adds to the current rotation of the object,
    /// unlike set_rotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - Angle of rotation, in degrees
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_rotate(self.circle_shape, angle as c_float) }
    }

    /// Scale a circle shape
    ///
    /// This function multiplies the current scale of the object,
    /// unlike sfCircleShape_setScale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfCircleShape_scale(self.circle_shape, factors.raw()) }
    }

    /// Scale a circle shape
    ///
    /// This function multiplies the current scale of the object,
    /// unlike sfCircleShape_setScale which overwrites it.
    ///
    /// # Arguments
    /// * factor_x - Scale x factor
    /// * factor_y - Scale y factor
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfCircleShape_scale(self.circle_shape,
                                     sfVector2f {
                                         x: factor_x,
                                         y: factor_y,
                                     })
        }
    }

    /// Get the combined transform of a circle shape
    ///
    /// Return transform combining the position/rotation/scale/origin
    /// of the object
    fn get_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfCircleShape_getTransform(self.circle_shape)) }
    }

    /// Get the inverse of the combined transform of a circle shape
    ///
    /// Return inverse of the combined transformations applied to the object
    fn get_inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfCircleShape_getInverseTransform(self.circle_shape)) }
    }
}

impl<'s> Shape<'s> for CircleShape<'s> {
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
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfCircleShape_setTexture(self.circle_shape,
                                          texture.raw(),
                                          sfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    fn disable_texture(&mut self) {
        self.texture = None;
        unsafe { ffi::sfCircleShape_setTexture(self.circle_shape, ptr::null_mut(), sfTrue) }
    }

    /// Set the sub-rectangle of the texture that a circle shape will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rec - Rectangle defining the region of the texture to display
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfCircleShape_setTextureRect(self.circle_shape, rect.raw()) }
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
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfCircleShape_setFillColor(self.circle_shape, color.0) }
    }

    /// Set the outline color of a circle shape
    ///
    /// You can use Transparent to disable the outline.
    /// By default, the shape's outline color is opaque white.
    ///
    /// # Arguments
    /// * color - New outline color of the shape
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfCircleShape_setOutlineColor(self.circle_shape, color.0) }
    }

    /// Set the thickness of a circle shape's outline
    ///
    /// This number cannot be negative. Using zero disables
    /// the outline.
    /// By default, the outline thickness is 0.
    ///
    /// # Arguments
    /// * thickness - New outline thickness
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfCircleShape_setOutlineThickness(self.circle_shape, thickness as c_float) }
    }

    /// Get the source texture of a circle shape
    ///
    /// You can't modify the texture when you retrieve it with this function.
    ///
    /// Return the shape's texture
    fn get_texture(&self) -> Option<&'s Texture> {
        self.texture
    }


    /// Get the sub-rectangle of the texture displayed by a circle shape
    ///
    /// Return the texture rectangle of the shape
    fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfCircleShape_getTextureRect(self.circle_shape)) }
    }

    /// Get the fill color of a circle shape
    ///
    /// Return the fill color of the shape
    fn get_fill_color(&self) -> Color {
        unsafe { Color(ffi::sfCircleShape_getFillColor(self.circle_shape)) }
    }

    /// Get the outline color of a circle shape
    ///
    /// Return the outline color of the shape
    fn get_outline_color(&self) -> Color {
        unsafe { Color(ffi::sfCircleShape_getOutlineColor(self.circle_shape)) }
    }

    /// Get the outline thickness of a circle shape
    ///
    /// Return the outline thickness of the shape
    fn get_outline_thickness(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getOutlineThickness(self.circle_shape) as f32 }
    }

    /// Get the total number of points of a circle shape
    ///
    /// Return the number of points of the shape
    fn get_point_count(&self) -> u32 {
        unsafe { ffi::sfCircleShape_getPointCount(self.circle_shape) as u32 }
    }

    /// Get a point of a circle shape
    ///
    /// The result is undefined if index is out of the valid range.
    ///
    /// # Arguments
    /// * index - Index of the point to get, in range [0 .. getPointCount() - 1]
    ///
    /// Return the index-th point of the shape
    fn get_point(&self, index: u32) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfCircleShape_getPoint(self.circle_shape, index as usize))
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
    fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfCircleShape_getLocalBounds(self.circle_shape)) }
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
    fn get_global_bounds(&self) -> FloatRect {
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
    /// Destroy an existing CircleShape
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
