use crate::{
    ffi::graphics as ffi,
    graphics::{
        Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Texture, Transform,
        Transformable,
    },
    system::Vector2f,
};
use std::{
    marker::PhantomData,
    ptr::{self, NonNull},
};

/// Drawable representation of a texture
///
/// Sprite is a drawable type that allows to easily
/// display a [`Texture`] (or a part of it) on a render target.
///
/// __Note:__
/// Currently, it is not feasible to store sprites long term.
/// A common pattern with rust-sfml is to create a `Sprite` right before you start drawing,
/// and draw all the sprites you want with it. You can change its properties using
/// `set_texture`, `set_position`, etc., before drawing it, as many times as you need to.
#[derive(Debug)]
pub struct Sprite<'s> {
    sprite: NonNull<ffi::sfSprite>,
    texture: PhantomData<&'s Texture>,
}

impl<'s> Sprite<'s> {
    /// Create a new sprite
    #[must_use]
    pub fn new() -> Sprite<'s> {
        let sp = unsafe { ffi::sfSprite_create() };
        Sprite {
            sprite: NonNull::new(sp).expect("Failed to create Sprite"),
            texture: PhantomData,
        }
    }

    /// Create a new sprite with a texture
    #[must_use]
    pub fn with_texture(texture: &'s Texture) -> Sprite<'s> {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, true);
        sprite
    }

    /// Create a new sprite with a texture and a source rectangle
    #[must_use]
    pub fn with_texture_and_rect(texture: &'s Texture, rect: IntRect) -> Self {
        let mut sprite = Sprite::with_texture(texture);
        sprite.set_texture_rect(rect);
        sprite
    }

    /// Change the source texture of a sprite
    ///
    /// The texture argument refers to a texture that must
    /// exist as long as the sprite uses it. Indeed, the sprite
    /// doesn't store its own copy of the texture, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the source texture is destroyed and the sprite tries to
    /// use it, the behaviour is undefined.
    /// If `reset_rect` is true, the [`texture_rect`] property of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// [`texture_rect`]: Sprite::texture_rect
    ///
    /// # Arguments
    /// * `texture` - New texture
    /// * `reset_rect` - Should the texture rect be reset to the size
    /// of the new texture?
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfSprite_setTexture(self.sprite.as_ptr(), texture.raw(), reset_rect) }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) {
        unsafe { ffi::sfSprite_setTexture(self.sprite.as_ptr(), ptr::null_mut(), true) }
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
    pub fn set_color(&mut self, color: Color) {
        unsafe { ffi::sfSprite_setColor(self.sprite.as_ptr(), color) }
    }

    /// Get the source texture of a sprite
    ///
    /// If the sprite has no source texture, None is returned.
    /// You can't
    /// modify the texture when you retrieve it with this function.
    ///
    /// Return an Option to the sprite's texture
    #[must_use]
    pub fn texture(&self) -> Option<&'s Texture> {
        unsafe {
            let ptr = ffi::sfSprite_getTexture(self.sprite.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *const Texture))
            }
        }
    }

    /// Get the global color of a sprite
    ///
    /// Return the global color of the sprite
    #[must_use]
    pub fn color(&self) -> Color {
        unsafe { ffi::sfSprite_getColor(self.sprite.as_ptr()) }
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
    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfSprite_getLocalBounds(self.sprite.as_ptr()) }
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
    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfSprite_getGlobalBounds(self.sprite.as_ptr()) }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    ///
    /// Return the texture rectangle of the sprite
    #[must_use]
    pub fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfSprite_getTextureRect(self.sprite.as_ptr()) }
    }

    /// Set the sub-rectangle of the texture that a sprite will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfSprite_setTextureRect(self.sprite.as_ptr(), rect) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfSprite {
        self.sprite.as_ptr()
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
        let sp = unsafe { ffi::sfSprite_copy(self.sprite.as_ptr()) };
        Sprite {
            sprite: NonNull::new(sp).expect("Failed to copy Sprite"),
            texture: PhantomData,
        }
    }
}

impl<'s> Drawable for Sprite<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_sprite(self, states)
    }
}

impl<'s> Transformable for Sprite<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfSprite_setPosition(self.sprite.as_ptr(), position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_setRotation(self.sprite.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfSprite_setScale(self.sprite.as_ptr(), scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfSprite_setOrigin(self.sprite.as_ptr(), origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getPosition(self.sprite.as_ptr())) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfSprite_getRotation(self.sprite.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getScale(self.sprite.as_ptr())) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getOrigin(self.sprite.as_ptr())) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfSprite_move(self.sprite.as_ptr(), offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_rotate(self.sprite.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfSprite_scale(self.sprite.as_ptr(), factors.into().raw()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getTransform(self.sprite.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getInverseTransform(self.sprite.as_ptr()) }
    }
}

impl<'s> Drop for Sprite<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_destroy(self.sprite.as_ptr()) }
    }
}
