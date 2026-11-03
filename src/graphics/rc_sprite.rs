use {
    super::Rect,
    crate::{
        cpp::FBox,
        ffi::graphics as ffi,
        graphics::{
            Color, Drawable, FloatRect, IntRect, RcTexture, RenderStates, RenderTarget, Texture,
            Transform, Transformable,
        },
        system::{Angle, Vector2f},
    },
    std::{cell::RefCell, ptr::NonNull, rc::Weak},
};

/// Drawable representation of a texture (reference counted)
///
/// `RcSprite` is a drawable type that allows to easily
/// display a [`RcTexture`] (or a part of it) on a render target.
///
/// This is an alternative to [`Sprite`], allowing for complete separation from the texture's lifetime.
///
/// Underneath, it uses reference counting to ensure that the `RcTexture` is alive.
/// An `RcSprite` remains fully usable except for drawing and the texture
/// accessor; drawing does nothing and `texture` returns `None` when the texture is gone.
///
/// [`Sprite`]: crate::graphics::Sprite
#[derive(Debug)]
pub struct RcSprite {
    handle: NonNull<ffi::sfSprite>,
    texture: Weak<RefCell<FBox<Texture>>>,
}

impl RcSprite {
    fn texture_exists(&self) -> bool {
        self.texture.strong_count() != 0
    }

    fn set_rc_texture(&mut self, texture: &RcTexture) {
        self.texture = texture.downgrade();
    }

    /// Create a new sprite with a texture
    #[must_use]
    pub fn with_texture(texture: &RcTexture) -> RcSprite {
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
    ///
    /// Panics if the sprite allocation fails.
    #[must_use]
    pub fn with_texture_and_rect(texture: &RcTexture, rect: IntRect) -> RcSprite {
        let sp = unsafe { ffi::sfSprite_new(texture.raw_texture(), rect) };
        RcSprite {
            handle: NonNull::new(sp).expect("Failed to create Sprite"),
            texture: texture.downgrade(),
        }
    }

    /// Change the source texture of a sprite
    ///
    /// The texture argument refers to a texture that must
    /// exist as long as the sprite uses it. Indeed, the sprite
    /// doesn't store its own copy of the texture, but rather keeps
    /// a pointer to the one that you passed to this function.
    /// If the source texture is destroyed and the sprite tries to
    /// use it, the behavior is defined within `RcSprite`. `RcSprite` will either do nothing,
    /// panic, or return a default implementation for specific functions.
    /// If `reset_rect` is true, the [`texture_rect`] property of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// [`texture_rect`]: RcSprite::texture_rect
    ///
    /// # Arguments
    /// * `texture` - New texture
    /// * `reset_rect` - Should the texture rect be reset to the size
    ///   of the new texture?
    pub fn set_texture(&mut self, texture: &RcTexture, reset_rect: bool) {
        self.set_rc_texture(texture);
        unsafe {
            ffi::sfSprite_setTexture(self.handle.as_ptr(), texture.raw_texture(), reset_rect)
        };
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
    pub fn texture(&self) -> Option<&Texture> {
        if self.texture_exists() {
            Some(unsafe { &*(*(self.texture.as_ptr())).as_ptr() })
        } else {
            None
        }
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
    /// Return the local bounding rectangle of the entity.
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

impl Clone for RcSprite {
    /// Return a new Sprite or panic! if there is not enough memory
    fn clone(&self) -> Self {
        let sp = unsafe { ffi::sfSprite_cpy(self.handle.as_ptr()) };
        Self {
            handle: NonNull::new(sp).expect("Failed to copy Sprite"),
            texture: self.texture.clone(),
        }
    }
}

impl Drawable for RcSprite {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        if self.texture_exists() {
            target.draw_rc_sprite(self, states);
        }
    }
}

impl Transformable for RcSprite {
    /// Reference [`Transformable::set_position`] for additional information
    ///
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfSprite_setPosition(self.handle.as_ptr(), position.into()) }
    }
    /// Reference [`Transformable::set_rotation`] for additional information
    ///
    fn set_rotation(&mut self, angle: Angle) {
        unsafe { ffi::sfSprite_setRotation(self.handle.as_ptr(), angle.as_degrees()) }
    }
    /// Reference [`Transformable::set_scale`] for additional information
    ///
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfSprite_setScale(self.handle.as_ptr(), scale.into()) }
    }
    /// Reference [`Transformable::set_origin`] for additional information
    ///
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfSprite_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    /// Reference [`Transformable::position`] for additional information
    ///
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getPosition(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::rotation`] for additional information
    ///
    fn rotation(&self) -> Angle {
        Angle::degrees(unsafe { ffi::sfSprite_getRotation(self.handle.as_ptr()) })
    }
    /// Reference [`Transformable::scale`] for additional information
    ///
    fn scale(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getScale(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::origin`] for additional information
    ///
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getOrigin(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::move_`] for additional information
    ///
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfSprite_move(self.handle.as_ptr(), offset.into()) }
    }
    /// Reference [`Transformable::rotate`] for additional information
    ///
    fn rotate(&mut self, angle: Angle) {
        unsafe { ffi::sfSprite_rotate(self.handle.as_ptr(), angle.as_degrees()) }
    }
    /// Reference [`Transformable::scale_by`] for additional information
    ///
    fn scale_by<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfSprite_scale(self.handle.as_ptr(), factors.into()) }
    }
    /// Reference [`Transformable::transform`] for additional information
    ///
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getTransform(self.handle.as_ptr()) }
    }
    /// Reference [`Transformable::inverse_transform`] for additional information
    ///
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl Drop for RcSprite {
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_del(self.handle.as_ptr()) }
    }
}
