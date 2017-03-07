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

//! Drawable representation of a texture
//!
//! Sprite is a drawable class that allows to easily
//! display a texture (or a part of it) on a render target.

use std::ptr;

use raw_conv::{Raw, FromRaw};
use graphics::{Drawable, Transformable, FloatRect, IntRect, Color, Texture, RenderTarget, Transform,
               RenderStates};
use system::Vector2f;

use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use csfml_graphics_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Drawable representation of a texture
///
/// Sprite is a drawable class that allows to easily
/// display a texture (or a part of it) on a render target.
pub struct Sprite<'s> {
    sprite: *mut ffi::sfSprite,
    texture: Option<&'s Texture>,
}

impl<'s> Sprite<'s> {
    /// Create a new sprite
    ///
    /// Return Some(Sprite) or None
    pub fn new() -> Sprite<'s> {
        let sp = unsafe { ffi::sfSprite_create() };
        if sp.is_null() {
            panic!("sfSprite_create returned null.")
        } else {
            Sprite {
                sprite: sp,
                texture: None,
            }
        }
    }

    /// Create a new sprite with a texture
    ///
    /// Return Some(Sprite) or None
    pub fn with_texture(texture: &'s Texture) -> Sprite<'s> {
        let sp = unsafe { ffi::sfSprite_create() };
        if sp.is_null() {
            panic!("sfSprite_create returned null.")
        } else {
            unsafe {
                ffi::sfSprite_setTexture(sp, texture.raw(), sfTrue);
            }
            Sprite {
                sprite: sp,
                texture: Some(texture),
            }
        }
    }

    /// Change the source texture of a sprite
    ///
    /// The texture argument refers to a texture that must
    /// exist as long as the sprite uses it. Indeed, the sprite
    /// doesn't store its own copy of the texture, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the source texture is destroyed and the sprite tries to
    /// use it, the behaviour is undefined.
    /// If reset_rect is true, the TextureRect property of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// # Arguments
    /// * texture - New texture
    /// * reset_rect - Should the texture rect be reset to the size
    /// of the new texture?
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfSprite_setTexture(self.sprite, texture.raw(), sfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) {
        self.texture = None;
        unsafe { ffi::sfSprite_setTexture(self.sprite, ptr::null_mut(), sfTrue) }
    }

    /// Set the global color of a sprite
    ///
    /// This color is modulated (multiplied) with the sprite's
    /// texture. It can be used to colorize the sprite, or change
    /// its global opacity.
    /// By default, the sprite's color is opaque white.
    ///
    /// # Arguments
    /// * color - New color of the sprite
    pub fn set_color(&mut self, color: &Color) {
        unsafe { ffi::sfSprite_setColor(self.sprite, color.raw()) }
    }

    /// Get the source texture of a sprite
    ///
    /// If the sprite has no source texture, None is returned.
    /// You can't
    /// modify the texture when you retrieve it with this function.
    ///
    /// Return an Option to the sprite's texture
    pub fn get_texture(&self) -> Option<&'s Texture> {
        if self.texture.is_none() {
            None
        } else {
            self.texture
        }
    }

    /// Get the global color of a sprite
    ///
    /// Return the global color of the sprite
    pub fn get_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfSprite_getColor(self.sprite)) }
    }

    /// Get the local bounding rectangle of a sprite
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    ///
    /// Return the local bounding rectangle of the entity
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfSprite_getLocalBounds(self.sprite)) }
    }

    /// Get the global bounding rectangle of a sprite
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    ///
    /// Return the global bounding rectangle of the entity
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfSprite_getGlobalBounds(self.sprite)) }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    ///
    /// Return the texture rectangle of the sprite
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfSprite_getTextureRect(self.sprite)) }
    }

    /// Set the sub-rectangle of the texture that a sprite will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfSprite_setTextureRect(self.sprite, rect.raw()) }
    }
}

impl<'s> Default for Sprite<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Clone for Sprite<'s> {
    /// Return a new Sprite or panic! if there is not enough memory
    fn clone(&self) -> Sprite<'s> {
        let sp = unsafe { ffi::sfSprite_copy(self.sprite) };
        if sp.is_null() {
            panic!("sfSprite_copy returned null.")
        } else {
            Sprite {
                sprite: sp,
                texture: self.texture,
            }
        }
    }
}

impl<'s> Drawable for Sprite<'s> {
    fn draw(&self, render_target: &mut RenderTarget, render_states: &mut RenderStates) {
        render_target.draw_sprite(self, render_states)
    }
}

impl<'s> Transformable for Sprite<'s> {
    /// Set the position of a sprite
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * position - New position
    fn set_position(&mut self, position: &Vector2f) {
        unsafe { ffi::sfSprite_setPosition(self.sprite, position.raw()) }
    }

    /// Set the position of a sprite
    ///
    /// This function completely overwrites the previous position.
    /// See move to apply an offset based on the previous position instead.
    /// The default position of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x coordinate
    /// * y - New y coordinate
    fn set_position2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfSprite_setPosition(self.sprite, sfVector2f { x: x, y: y }) }
    }

    /// Set the orientation of a sprite
    ///
    /// This function completely overwrites the previous rotation.
    /// See rotate to add an angle based on the previous rotation instead.
    /// The default rotation of a sprite Sprite object is 0.
    ///
    /// # Arguments
    /// * angle - New rotation, in degrees
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_setRotation(self.sprite, angle) }
    }

    /// Set the scale factors of a sprite
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a sprite Sprite object is (1, 1).
    ///
    /// # Arguments
    /// * scale - New scale factors
    fn set_scale(&mut self, scale: &Vector2f) {
        unsafe { ffi::sfSprite_setScale(self.sprite, scale.raw()) }
    }

    /// Set the scale factors of a sprite
    ///
    /// This function completely overwrites the previous scale.
    /// See scale to add a factor based on the previous scale instead.
    /// The default scale of a sprite Sprite object is (1, 1).
    ///
    /// # Arguments
    /// * scale_x - New x scale factor
    /// * scale_y - New y scale factor
    fn set_scale2f(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::sfSprite_setScale(self.sprite,
                                   sfVector2f {
                                       x: scale_x,
                                       y: scale_y,
                                   })
        }
    }

    /// Set the local origin of a sprite
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * origin - New origin
    fn set_origin(&mut self, origin: &Vector2f) {
        unsafe { ffi::sfSprite_setOrigin(self.sprite, origin.raw()) }
    }

    /// Set the local origin of a sprite
    ///
    /// The origin of an object defines the center point for
    /// all transformations (position, scale, rotation).
    /// The coordinates of this point must be relative to the
    /// top-left corner of the object, and ignore all
    /// transformations (position, scale, rotation).
    /// The default origin of a sprite Sprite object is (0, 0).
    ///
    /// # Arguments
    /// * x - New x origin coordinate
    /// * y - New y origin coordinate
    fn set_origin2f(&mut self, x: f32, y: f32) {
        unsafe { ffi::sfSprite_setOrigin(self.sprite, sfVector2f { x: x, y: y }) }
    }

    /// Get the position of a sprite
    ///
    /// Return the current position
    fn get_position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getPosition(self.sprite)) }
    }

    /// Get the orientation of a sprite
    ///
    /// The rotation is always in the range [0, 360].
    ///
    /// Return the current rotation, in degrees
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfSprite_getRotation(self.sprite) as f32 }
    }

    /// Get the current scale of a sprite
    ///
    /// Return the current scale factors
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getScale(self.sprite)) }
    }

    /// Get the local origin of a sprite
    ///
    /// Return the current origin
    fn get_origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getOrigin(self.sprite)) }
    }

    /// Move a sprite by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sfSprite_setPosition which overwrites it.
    ///
    /// # Arguments
    /// * offset - Offset
    fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfSprite_move(self.sprite, offset.raw()) }
    }

    /// Move a sprite by a given offset
    ///
    /// This function adds to the current position of the object,
    /// unlike sfSprite_setPosition which overwrites it.
    ///
    /// # Arguments
    /// * offsetX - Offset x
    /// * offsetY - Offset y
    fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfSprite_move(self.sprite,
                               sfVector2f {
                                   x: offset_x,
                                   y: offset_y,
                               })
        }
    }

    /// Rotate a sprite
    ///
    /// This function adds to the current rotation of the object,
    /// unlike sfSprite_setRotation which overwrites it.
    ///
    /// # Arguments
    /// * angle - Angle of rotation, in degrees
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_rotate(self.sprite, angle) }
    }

    /// Scale a sprite
    ///
    /// This function multiplies the current scale of the object,
    /// unlike setScale which overwrites it.
    ///
    /// # Arguments
    /// * factors - Scale factors
    fn scale(&mut self, factors: &Vector2f) {
        unsafe { ffi::sfSprite_scale(self.sprite, factors.raw()) }
    }

    /// Scale a sprite
    ///
    /// This function multiplies the current scale of the object,
    /// unlike setScale which overwrites it.
    ///
    /// # Arguments
    /// * factor_x - Scale x factor
    /// * factor_y - Scale y factor
    fn scale2f(&mut self, factor_x: f32, factor_y: f32) {
        unsafe {
            ffi::sfSprite_scale(self.sprite,
                                sfVector2f {
                                    x: factor_x,
                                    y: factor_y,
                                })
        }
    }

    /// Get the combined transform of a sprite
    ///
    /// Return the transform combining the position/rotation/scale/origin
    /// of the object
    fn get_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfSprite_getTransform(self.sprite)) }
    }

    /// Get the inverse of the combined transform of a sprite
    ///
    /// Return the inverse of the combined transformations applied to the object
    fn get_inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfSprite_getInverseTransform(self.sprite)) }
    }
}

impl<'s> Raw for Sprite<'s> {
    type Raw = *mut ffi::sfSprite;
    fn raw(&self) -> Self::Raw {
        self.sprite
    }
}

impl<'s> Drop for Sprite<'s> {
    /// Destroy an existing sprite
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_destroy(self.sprite) }
    }
}
