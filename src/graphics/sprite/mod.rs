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

//! Drawable representation of a texture
//!
//! Sprite is a drawable class that allows to easily
//! display a texture (or a part of it) on a render target.

use libc::{c_float};
use std::ptr;

use graphics::{FloatRect, IntRect, Color, Texture, Transformable,
               RenderTarget, Transform, RenderStates, Drawable};
use system::Vector2f;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Drawable representation of a texture
///
/// Sprite is a drawable class that allows to easily
/// display a texture (or a part of it) on a render target.
pub struct Sprite<'s> {
    sprite: Foreign<ffi::sfSprite>,
    texture: Option<&'s Texture>
}

impl<'s> Sprite<'s> {
    /// Create a new sprite
    ///
    /// Return Some(Sprite) or None
    pub fn new() -> Option<Sprite<'s>> {
		unsafe {
			Foreign::new(ffi::sfSprite_create())
		}.map(|sp| Sprite {
			sprite: sp,
			texture: None
		})
    }

    /// Create a new sprite with a texture
    ///
    /// Return Some(Sprite) or None
    pub fn new_with_texture(texture: &'s Texture) -> Option<Sprite<'s>> {
		Sprite::new().map(|mut sprite| {
			sprite.set_texture(texture, true);
			sprite
		})
    }

    /// Copy an existing sprite
    ///
    /// Return Some(Sprite) or None
    pub fn clone_opt(&self) -> Option<Sprite<'s>> {
        unsafe {
			Foreign::new(ffi::sfSprite_copy(self.raw()))
		}.map(|sp| Sprite {
			sprite: sp,
			texture: self.texture
		})
    }

	fn raw(&self) -> &ffi::sfSprite { self.sprite.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfSprite { self.sprite.as_mut() }
	#[doc(hidden)]
    pub unsafe fn unwrap(&self) -> &ffi::sfSprite { self.raw() }

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
            ffi::sfSprite_setTexture(self.raw_mut(),
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
            ffi::sfSprite_setTexture(self.raw_mut(), ptr::null_mut(), SfBool::SFTRUE)
        }
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
    pub fn set_color(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfSprite_setColor(self.raw_mut(), *color)
        }
    }

    /// Get the source texture of a sprite
    ///
    /// If the sprite has no source texture, None is returned.
    /// You can't
    /// modify the texture when you retrieve it with this function.
    ///
    /// Return an Option to the sprite's texture
    pub fn get_texture(&self) -> Option<&'s Texture> {
		self.texture
    }

    /// Get the global color of a sprite
    ///
    /// Return the global color of the sprite
    pub fn get_color(&self) -> Color {
        unsafe {
            ffi::sfSprite_getColor(self.raw())
        }
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
        unsafe {
            ffi::sfSprite_getLocalBounds(self.raw())
        }
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
        unsafe {
            ffi::sfSprite_getGlobalBounds(self.raw())
        }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    ///
    /// Return the texture rectangle of the sprite
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            ffi::sfSprite_getTextureRect(self.raw())
        }
    }

    /// Set the sub-rectangle of the texture that a sprite will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: &IntRect) -> () {
        unsafe {
            ffi::sfSprite_setTextureRect(self.raw_mut(), *rect)
        }
    }
}

impl<'s> Transformable for Sprite<'s> {
    fn set_position(&mut self, position: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_setPosition(self.raw_mut(), *position)
        }
    }

    fn scale(&mut self, factors: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_scale(self.raw_mut(), *factors)
        }
    }

    fn get_scale(&self) -> Vector2f {
        unsafe {
            ffi::sfSprite_getScale(self.raw())
        }
    }

    fn get_origin(&self) -> Vector2f {
        unsafe {
            ffi::sfSprite_getOrigin(self.raw())
        }
    }

    fn move_(&mut self, offset: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_move(self.raw_mut(), *offset)
        }
    }

    fn set_scale(&mut self, scale: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_setScale(self.raw_mut(), *scale)
        }
    }

    fn set_origin(&mut self, origin: &Vector2f) -> () {
        unsafe {
            ffi::sfSprite_setOrigin(self.raw_mut(), *origin)
        }
    }

    fn set_origin2f(&mut self, x: f32, y: f32) -> () {
        unsafe {
            ffi::sfSprite_setOrigin(self.raw_mut(), Vector2f::new(x, y))
        }
    }

    fn get_position(&self) -> Vector2f {
        unsafe {
            ffi::sfSprite_getPosition(self.raw())
        }
    }

    fn set_rotation(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfSprite_setRotation(self.raw_mut(), angle as c_float)
        }
    }

    fn get_rotation(&self) -> f32 {
        unsafe {
            ffi::sfSprite_getRotation(self.raw()) as f32
        }
    }

    fn rotate(&mut self, angle: f32) -> () {
        unsafe {
            ffi::sfSprite_rotate(self.raw_mut(), angle as c_float)
        }
    }

    fn get_transform(&self) -> Transform {
        unsafe {
            ffi::sfSprite_getTransform(self.raw())
        }
    }

    fn get_inverse_transform(&self) -> Transform {
        unsafe {
            ffi::sfSprite_getInverseTransform(self.raw())
        }
    }
}

impl<'s> Clone for Sprite<'s> {
    fn clone(&self) -> Sprite<'s> {
		self.clone_opt().expect("Failed to clone Sprite")
    }
}

impl<'s> Drawable for Sprite<'s> {
    fn draw<RT:RenderTarget>(&self,
                                render_target: &mut RT,
                                render_states: &RenderStates) -> () {
        render_target.draw_sprite_rs(self, render_states)
    }
}
