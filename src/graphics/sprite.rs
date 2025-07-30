use {
    super::Rect,
    crate::{
        ffi::graphics as ffi,
        graphics::{
            Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Texture, Transform,
            Transformable,
        },
        system::{Angle, Vector2f},
    },
    std::{marker::PhantomData, ptr::NonNull},
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
    handle: NonNull<ffi::sfSprite>,
    texture: PhantomData<&'s Texture>,
}

impl<'s> Sprite<'s> {
    /// Create a new sprite with a texture
    #[must_use]
    pub fn with_texture(texture: &'s Texture) -> Sprite<'s> {
        Self::with_texture_and_rect(
            texture,
            Rect {
                position: Default::default(),
                size: texture.size().as_other(),
            },
        )
    }

    /// Create a new sprite with a texture and a source rectangle
    ///
    /// # Panics
    /// Panics during const evaluation.
    /// This method will panic during const evaluation if the pointer cannot be
    /// determined to be null or not.
    #[must_use]
    pub fn with_texture_and_rect(texture: &'s Texture, rect: IntRect) -> Self {
        let sp = unsafe { ffi::sfSprite_new(texture, rect) };
        Sprite {
            handle: NonNull::new(sp).expect("Failed to create Sprite"),
            texture: PhantomData,
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
    /// If `reset_rect` is true, the [`texture_rect`] property of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// [`texture_rect`]: Sprite::texture_rect
    ///
    /// # Arguments
    /// * `texture` - New texture
    /// * `reset_rect` - Should the texture rect be reset to the size of the new texture?
    pub fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfSprite_setTexture(self.handle.as_ptr(), texture, reset_rect) }
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
        unsafe { ffi::sfSprite_setColor(self.handle.as_ptr(), color) }
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
        unsafe { ffi::sfSprite_getTexture(self.handle.as_ptr()).as_ref() }
    }

    /// Get the global color of a sprite
    ///
    /// Return the global color of the sprite
    #[must_use]
    pub fn color(&self) -> Color {
        unsafe { ffi::sfSprite_getColor(self.handle.as_ptr()) }
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
        unsafe { ffi::sfSprite_getLocalBounds(self.handle.as_ptr()) }
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
        unsafe { ffi::sfSprite_getGlobalBounds(self.handle.as_ptr()) }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    ///
    /// Return the texture rectangle of the sprite
    #[must_use]
    pub fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfSprite_getTextureRect(self.handle.as_ptr()) }
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
        unsafe { ffi::sfSprite_setTextureRect(self.handle.as_ptr(), rect) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfSprite {
        self.handle.as_ptr()
    }
}

impl<'s> Clone for Sprite<'s> {
    /// Return a new Sprite or panic! if there is not enough memory
    fn clone(&self) -> Sprite<'s> {
        let sp = unsafe { ffi::sfSprite_cpy(self.handle.as_ptr()) };
        Sprite {
            handle: NonNull::new(sp).expect("Failed to copy Sprite"),
            texture: PhantomData,
        }
    }
}

impl Drawable for Sprite<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_sprite(self, states);
    }
}

impl Transformable for Sprite<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfSprite_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: Angle) {
        unsafe { ffi::sfSprite_setRotation(self.handle.as_ptr(), angle.as_degrees()) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfSprite_setScale(self.handle.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfSprite_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getPosition(self.handle.as_ptr()) }
    }
    fn rotation(&self) -> Angle {
        Angle::degrees(unsafe { ffi::sfSprite_getRotation(self.handle.as_ptr()) })
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getScale(self.handle.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getOrigin(self.handle.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfSprite_move(self.handle.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: Angle) {
        unsafe { ffi::sfSprite_rotate(self.handle.as_ptr(), angle.as_degrees()) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfSprite_scale(self.handle.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getTransform(self.handle.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl Drop for Sprite<'_> {
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_del(self.handle.as_ptr()) }
    }
}
