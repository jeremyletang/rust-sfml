use csfml_system_sys::{sfBool, sfTrue};
use ext::sf_bool_ext::SfBoolExt;
use graphics::{Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, TextureRef,
               Transform, Transformable};
use graphics::csfml_graphics_sys as ffi;
use std::marker::PhantomData;
use std::ptr;
use system::Vector2f;
use system::raw_conv::{FromRaw, Raw};

/// Drawable representation of a texture
///
/// Sprite is a drawable type that allows to easily
/// display a texture (or a part of it) on a render target.
#[derive(Debug)]
pub struct Sprite<'s> {
    sprite: *mut ffi::sfSprite,
    texture: PhantomData<&'s TextureRef>,
}

impl<'s> Sprite<'s> {
    /// Create a new sprite
    ///
    /// Return Some(Sprite) or None
    pub fn new() -> Sprite<'s> {
        let sp = unsafe { ffi::sfSprite_create() };
        assert!(!sp.is_null(), "Failed to create Sprite");
        Sprite {
            sprite: sp,
            texture: PhantomData,
        }
    }

    /// Create a new sprite with a texture
    ///
    /// Return Some(Sprite) or None
    pub fn with_texture(texture: &'s TextureRef) -> Sprite<'s> {
        let mut sprite = Sprite::new();
        sprite.set_texture(texture, true);
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
    /// If reset_rect is true, the TextureRect property of
    /// the sprite is automatically adjusted to the size of the new
    /// texture. If it is false, the texture rect is left unchanged.
    ///
    /// # Arguments
    /// * texture - New texture
    /// * reset_rect - Should the texture rect be reset to the size
    /// of the new texture?
    pub fn set_texture(&mut self, texture: &'s TextureRef, reset_rect: bool) {
        unsafe {
            ffi::sfSprite_setTexture(self.sprite, texture.raw(), sfBool::from_bool(reset_rect))
        }
    }

    /// Disable Texturing
    ///
    /// Disable the current texture and reset the texture rect
    pub fn disable_texture(&mut self) {
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
    pub fn texture(&self) -> Option<&'s TextureRef> {
        unsafe {
            let ptr = ffi::sfSprite_getTexture(self.sprite);
            if ptr.is_null() {
                None
            } else {
                Some(&*(ptr as *const TextureRef))
            }
        }
    }

    /// Get the global color of a sprite
    ///
    /// Return the global color of the sprite
    pub fn color(&self) -> Color {
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
    pub fn local_bounds(&self) -> FloatRect {
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
    pub fn global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfSprite_getGlobalBounds(self.sprite)) }
    }

    /// Get the sub-rectangle of the texture displayed by a sprite
    ///
    /// Return the texture rectangle of the sprite
    pub fn texture_rect(&self) -> IntRect {
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
        assert!(!sp.is_null(), "Failed to copy Sprite");
        Sprite {
            sprite: sp,
            texture: PhantomData,
        }
    }
}

impl<'s> Drawable for Sprite<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_sprite(self, states)
    }
}

impl<'s> Transformable for Sprite<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfSprite_setPosition(self.sprite, position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_setRotation(self.sprite, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfSprite_setScale(self.sprite, scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfSprite_setOrigin(self.sprite, origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getPosition(self.sprite)) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfSprite_getRotation(self.sprite) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getScale(self.sprite)) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfSprite_getOrigin(self.sprite)) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfSprite_move(self.sprite, offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_rotate(self.sprite, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfSprite_scale(self.sprite, factors.into().raw()) }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfSprite_getTransform(self.sprite)) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfSprite_getInverseTransform(self.sprite)) }
    }
}

impl<'s> Raw for Sprite<'s> {
    type Raw = *const ffi::sfSprite;
    fn raw(&self) -> Self::Raw {
        self.sprite
    }
}

impl<'s> Drop for Sprite<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_destroy(self.sprite) }
    }
}
