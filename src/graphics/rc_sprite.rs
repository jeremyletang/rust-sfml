use crate::{
    ffi::graphics as ffi,
    graphics::{
        Color, Drawable, FloatRect, IntRect, RcTexture, RenderStates, RenderTarget, Texture,
        Transform, Transformable,
    },
    sf_box::SfBox,
    system::Vector2f,
};
use std::{
    cell::RefCell,
    ptr::{self, NonNull},
    rc::Weak,
};

#[derive(Debug)]
pub struct RcSprite {
    sprite: NonNull<ffi::sfSprite>,
    texture: Weak<RefCell<SfBox<Texture>>>,
}

impl RcSprite {
    #[must_use]
    pub fn new() -> Self {
        let sp = unsafe { ffi::sfSprite_create() };
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

    #[must_use]
    pub fn with_texture(texture: &RcTexture) -> RcSprite {
        let mut sprite = Self::new();
        sprite.set_texture(&texture, true);
        sprite
    }

    #[must_use]
    pub fn with_texture_and_rect(texture: &RcTexture, rect: IntRect) -> RcSprite {
        let mut sprite = Self::with_texture(texture);
        sprite.set_texture_rect(rect);
        sprite
    }

    #[must_use]
    pub fn set_texture(&mut self, texture: &RcTexture, reset_rect: bool) {
        self.set_rc_texture(&texture);
        let raw_texture = unsafe {
            (*self.texture.upgrade().unwrap().as_ptr())
                .0
                .as_ptr()
                .cast_const()
        };
        unsafe { ffi::sfSprite_setTexture(self.sprite.as_ptr(), raw_texture, reset_rect) };
    }

    pub fn disable_texture(&mut self) {
        unsafe { ffi::sfSprite_setTexture(self.sprite.as_ptr(), ptr::null_mut(), true) };
        self.texture = Weak::new();
    }

    pub fn set_color(&mut self, color: Color) {
        unsafe { ffi::sfSprite_setColor(self.sprite.as_ptr(), color) }
    }

    #[must_use]
    pub fn texture<'s>(&'s self) -> Option<&'s Texture> {
        if self.texture_exists() {
            Some(unsafe { &*(*(self.texture.as_ptr())).as_ptr() })
        } else {
            None
        }
    }

    #[must_use]
    pub fn color(&self) -> Color {
        unsafe { ffi::sfSprite_getColor(self.sprite.as_ptr()) }
    }

    #[must_use]
    pub fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfSprite_getLocalBounds(self.sprite.as_ptr()) }
    }

    #[must_use]
    pub fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfSprite_getGlobalBounds(self.sprite.as_ptr()) }
    }

    #[must_use]
    pub fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfSprite_getTextureRect(self.sprite.as_ptr()) }
    }

    pub fn set_texture_rect(&mut self, rect: IntRect) {
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
    fn clone(&self) -> RcSprite {
        let sp = unsafe { ffi::sfSprite_copy(self.sprite.as_ptr()) };
        RcSprite {
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
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfSprite_setPosition(self.sprite.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_setRotation(self.sprite.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfSprite_setScale(self.sprite.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfSprite_setOrigin(self.sprite.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getPosition(self.sprite.as_ptr()) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfSprite_getRotation(self.sprite.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getScale(self.sprite.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfSprite_getOrigin(self.sprite.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfSprite_move(self.sprite.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfSprite_rotate(self.sprite.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfSprite_scale(self.sprite.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getTransform(self.sprite.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfSprite_getInverseTransform(self.sprite.as_ptr()) }
    }
}

impl Drop for RcSprite {
    fn drop(&mut self) {
        unsafe { ffi::sfSprite_destroy(self.sprite.as_ptr()) }
    }
}
