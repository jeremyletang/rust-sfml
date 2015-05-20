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

use libc::{c_float};

use graphics::{FloatRect, IntRect, Color, Texture, Transformable,
               RenderTarget, Transform, RenderStates, Drawable};
use system::Vector2f;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Drawable representation of a texture, with its own transformations, color,
/// etc.
///
/// `Sprite` is a drawable type that allows to easily display all or part of a
/// texture on a render target.
///
/// It implements all the functions from `Transformable`: position, rotation,
/// scale, origin. It also adds sprite-specific properties such as the texture
/// to use, the part of it to display, and some convenience functions to change
/// the overall color of the sprite, or to get its bounding rectangle.
///
/// `Sprite` works in combination with the `Texture` class, which loads and
/// provides the pixel data of a given texture. The separation of `Sprite` and
/// `Texture` allows more flexibility and better performances: indeed a
/// `Texture` is a heavy resource, and any operation on it is slow (often too
/// slow for real-time applications). On the other side, a `Sprite` is a
/// lightweight object which can use the pixel data of a `Texture` and draw it
/// with its own transformation/color/blending attributes.
///
/// See also the note on coordinates and undistorted rendering in
/// `Transformable`.
pub struct Sprite<'s> {
    sprite: Foreign<ffi::sfSprite>,
    texture: Option<&'s Texture>
}

impl<'s> Sprite<'s> {
    /// Create a new sprite with no source texture.
    ///
    /// Returns Some(Sprite) or None on failure.
    pub fn new() -> Option<Sprite<'s>> {
		unsafe {
			Foreign::new(ffi::sfSprite_create())
		}.map(|sp| Sprite {
			sprite: sp,
			texture: None
		})
    }

    /// Create a new sprite from a source texture.
    ///
    /// Returns Some(Sprite) or None on failure.
    pub fn new_with_texture(texture: &'s Texture) -> Option<Sprite<'s>> {
		Sprite::new().map(|mut sprite| {
			sprite.set_texture(texture, true);
			sprite
		})
    }

    /// Copy an existing sprite.
    ///
    /// Returns Some(Sprite) or None on failure.
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

    /// Change the source texture of the sprite.
    ///
    /// If `reset_rect` is true, the texture rect of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        self.texture = Some(texture);
        unsafe {
            ffi::sfSprite_setTexture(self.raw_mut(),
                                     texture.unwrap(),
                                     SfBool::from_bool(reset_rect))
        }
    }

	// Note: Sprite does not support disable_texture(). SFML requires a valid
	// const Texture&, and CSFML does nothing if null is passed.

    /// Get the source texture of the sprite.
    ///
    /// If the sprite has no source texture, None is returned. You can't modify
    /// the texture when you retrieve it with this function.
    pub fn get_texture(&self) -> Option<&'s Texture> {
		self.texture
    }

    /// Set the global color of the sprite.
    ///
    /// This color is modulated (multiplied) with the sprite's
    /// texture. It can be used to colorize the sprite, or change
    /// its global opacity.
    /// By default, the sprite's color is opaque white.
    pub fn set_color(&mut self, color: Color) {
        unsafe { ffi::sfSprite_setColor(self.raw_mut(), color) }
    }

    /// Get the global color of the sprite.
    pub fn get_color(&self) -> Color {
        unsafe { ffi::sfSprite_getColor(self.raw()) }
    }

    /// Set the sub-rectangle of the texture that the sprite will display.
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    pub fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfSprite_setTextureRect(self.raw_mut(), rect) }
    }

    /// Get the sub-rectangle of the texture displayed by the sprite.
    pub fn get_texture_rect(&self) -> IntRect {
        unsafe { ffi::sfSprite_getTextureRect(self.raw()) }
    }

    /// Get the local bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in local coordinates, which means
    /// that it ignores the transformations (translation, rotation,
    /// scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// entity in the entity's coordinate system.
    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfSprite_getLocalBounds(self.raw()) }
    }

    /// Get the global bounding rectangle of the sprite.
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfSprite_getGlobalBounds(self.raw()) }
    }
}

impl<'s> Transformable for Sprite<'s> {
    fn set_position(&mut self, position: Vector2f) {
        unsafe { ffi::sfSprite_setPosition(self.raw_mut(), position) }
    }
    fn scale(&mut self, factors: Vector2f) {
        unsafe { ffi::sfSprite_scale(self.raw_mut(), factors) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getScale(self.raw()) }
    }
    fn get_origin(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getOrigin(self.raw()) }
    }
    fn move_(&mut self, offset: Vector2f) {
        unsafe { ffi::sfSprite_move(self.raw_mut(), offset) }
    }
    fn set_scale(&mut self, scale: Vector2f) {
        unsafe { ffi::sfSprite_setScale(self.raw_mut(), scale) }
    }
    fn set_origin(&mut self, origin: Vector2f) {
        unsafe { ffi::sfSprite_setOrigin(self.raw_mut(), origin) }
    }
    fn get_position(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getPosition(self.raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_setRotation(self.raw_mut(), angle as c_float) }
    }
    fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfSprite_getRotation(self.raw()) as f32 }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_rotate(self.raw_mut(), angle as c_float) }
    }
    fn get_transform(&self) -> Transform {
        unsafe { ffi::sfSprite_getTransform(self.raw()) }
    }
    fn get_inverse_transform(&self) -> Transform {
        unsafe { ffi::sfSprite_getInverseTransform(self.raw()) }
    }
}

impl<'s> Clone for Sprite<'s> {
    fn clone(&self) -> Sprite<'s> {
		self.clone_opt().expect("Failed to clone Sprite")
    }
}

impl<'s> Drawable for Sprite<'s> {
    fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
        target.draw_sprite_rs(self, states)
    }
}
