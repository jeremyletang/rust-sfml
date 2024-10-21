use {
    crate::{
        ffi::graphics as ffi,
        graphics::{
            Color, Drawable, FloatRect, IntRect, RcTexture, RenderStates, RenderTarget, Texture,
            Transform, Transformable,
        },
        sf_box::SfBox,
        system::Vector2f,
    },
    std::{cell::RefCell, ptr::NonNull, rc::Weak},
};

const ERROR_MSG: &str = "Sprite does not hold a texture. Ignoring transformation!";
const RETURN_ERROR_MSG: &str = "Sprite does not hold a texture. Returning default value!";
const PANIC_ERROR_MSG: &str = "Sprite does not hold a texture! Return value cannot be discerned!";

/// Drawable representation of a texture (reference counted)
///
/// `RcSprite` is a drawable type that allows to easily
/// display a [`RcTexture`] (or a part of it) on a render target.
///
/// This is an alternative to [`Sprite`], allowing for complete seperation from the texture's lifetime.
///
/// Underneath, it uses reference counting to ensure that the `RcTexture` is alive,
/// and disallows performing certain actions if the `RcTexture` is no longer alive.
/// It will print an error message in these cases.
///
/// [`Sprite`]: crate::graphics::Sprite
#[derive(Debug)]
pub struct RcSprite {
    sprite: NonNull<ffi::sfSprite>,
    texture: Weak<RefCell<SfBox<Texture>>>,
}

impl RcSprite {
    /// Create a new sprite
    #[must_use]
    pub fn new() -> Self {
        let sp = unsafe { ffi::sfSprite_new() };
        Self {
            sprite: NonNull::new(sp).expect("Failed to create Sprite"),
            texture: Weak::new(),
        }
    }

    fn texture_exists(&self) -> bool {
        self.texture.strong_count() != 0
    }

    fn set_rc_texture(&mut self, texture: &RcTexture) {
        self.texture = texture.downgrade()
    }

    /// Create a new sprite with a texture
    #[must_use]
    pub fn with_texture(texture: &RcTexture) -> RcSprite {
        let mut sprite = Self::new();
        sprite.set_texture(texture, true);
        sprite
    }

    /// Create a new sprite with a texture and a source rectangle
    #[must_use]
    pub fn with_texture_and_rect(texture: &RcTexture, rect: IntRect) -> RcSprite {
        let mut sprite = Self::with_texture(texture);
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
        #[expect(clippy::unwrap_used)]
        let raw_texture = unsafe {
            (*self.texture.upgrade().unwrap().as_ptr())
                .0
                .as_ptr()
                .cast_const()
        };
        unsafe { ffi::sfSprite_setTexture(self.sprite.as_ptr(), raw_texture, reset_rect) };
    }

    /// Set the global color of a sprite
    ///
    /// This color is modulated (multiplied) with the sprite's
    /// texture. It can be used to colorize the sprite, or change
    /// its global opacity.
    /// By default, the sprite's color is opaque white.
    /// Function fails and prints an error message if `RcSprite`'s texture is dead.
    ///
    /// # Arguments
    /// * color - New color of the sprite
    pub fn set_color(&mut self, color: Color) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
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
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    #[must_use]
    pub fn color(&self) -> Color {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
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
    /// Return the local bounding rectangle of the entity.
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getLocalBounds(self.sprite.as_ptr()) }
    }

    /// Get the global bounding rectangle of a sprite
    ///
    /// The returned rectangle is in global coordinates, which means
    /// that it takes in account the transformations (translation,
    /// rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the
    /// sprite in the global 2D world's coordinate system.
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    ///
    /// Return the global bounding rectangle of the entity
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getGlobalBounds(self.sprite.as_ptr()) }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    ///
    /// Return the texture rectangle of the sprite
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    #[must_use]
    pub fn texture_rect(&self) -> IntRect {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getTextureRect(self.sprite.as_ptr()) }
    }

    /// Set the sub-rectangle of the texture that a sprite will display
    ///
    /// The texture rect is useful when you don't want to display
    /// the whole texture, but rather a part of it.
    /// By default, the texture rect covers the entire texture.
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the region of the texture to display
    pub fn set_texture_rect(&mut self, rect: IntRect) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_setTextureRect(self.sprite.as_ptr(), rect) }
    }

    pub(super) fn raw(&self) -> *const ffi::sfSprite {
        self.sprite.as_ptr()
    }
}

impl Default for RcSprite {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RcSprite {
    /// Return a new Sprite or panic! if there is not enough memory
    fn clone(&self) -> Self {
        let sp = unsafe { ffi::sfSprite_cpy(self.sprite.as_ptr()) };
        Self {
            sprite: NonNull::new(sp).expect("Failed to copy Sprite"),
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
            target.draw_rc_sprite(self, states)
        }
    }
}

impl Transformable for RcSprite {
    /// Reference [`Transformable::set_position`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_setPosition(self.sprite.as_ptr(), position.into()) }
    }
    /// Reference [`Transformable::set_rotation`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn set_rotation(&mut self, angle: f32) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_setRotation(self.sprite.as_ptr(), angle) }
    }
    /// Reference [`Transformable::set_scale`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_setScale(self.sprite.as_ptr(), scale.into()) }
    }
    /// Reference [`Transformable::set_origin`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_setOrigin(self.sprite.as_ptr(), origin.into()) }
    }
    /// Reference [`Transformable::position`] for additional information
    ///
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    fn position(&self) -> Vector2f {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getPosition(self.sprite.as_ptr()) }
    }
    /// Reference [`Transformable::rotation`] for additional information
    ///
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    fn rotation(&self) -> f32 {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getRotation(self.sprite.as_ptr()) }
    }
    /// Reference [`Transformable::get_scale`] for additional information
    ///
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    fn get_scale(&self) -> Vector2f {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getScale(self.sprite.as_ptr()) }
    }
    /// Reference [`Transformable::origin`] for additional information
    ///
    /// Function fails, returns default, and prints an error message if `RcSprite`'s texture is dead.
    fn origin(&self) -> Vector2f {
        if !self.texture_exists() {
            eprintln!("{RETURN_ERROR_MSG}");
            return Default::default();
        }
        unsafe { ffi::sfSprite_getOrigin(self.sprite.as_ptr()) }
    }
    /// Reference [`Transformable::move_`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_move(self.sprite.as_ptr(), offset.into()) }
    }
    /// Reference [`Transformable::rotate`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn rotate(&mut self, angle: f32) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_rotate(self.sprite.as_ptr(), angle) }
    }
    /// Reference [`Transformable::scale`] for additional information
    ///
    /// Function fails, and prints an error message if `RcSprite`'s texture is dead.
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        if !self.texture_exists() {
            eprintln!("{ERROR_MSG}");
            return;
        }
        unsafe { ffi::sfSprite_scale(self.sprite.as_ptr(), factors.into()) }
    }
    /// Reference [`Transformable::transform`] for additional information
    ///
    /// Panics if texture doesn't exist
    fn transform(&self) -> &Transform {
        if !self.texture_exists() {
            panic!("{}", PANIC_ERROR_MSG);
        }
        unsafe { &*ffi::sfSprite_getTransform(self.sprite.as_ptr()) }
    }
    /// Reference [`Transformable::inverse_transform`] for additional information
    ///
    /// Panics if texture doesn't exist
    fn inverse_transform(&self) -> &Transform {
        if !self.texture_exists() {
            panic!("{}", PANIC_ERROR_MSG);
        }
        unsafe { &*ffi::sfSprite_getInverseTransform(self.sprite.as_ptr()) }
    }
}

impl Drop for RcSprite {
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_del(self.sprite.as_ptr()) }
    }
}
