use csfml_system_sys::{sfBool, sfTrue};
use graphics::{Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, TextureRef,
               Transform, Transformable};
use graphics::csfml_graphics_sys as ffi;
use sf_bool_ext::SfBoolExt;
use std::marker::PhantomData;
use std::ptr;
use system::Vector2f;

/// Specialized shape representing a rectangle
#[derive(Debug)]
pub struct RectangleShape<'s> {
    rectangle_shape: *mut ffi::sfRectangleShape,
    texture: PhantomData<&'s TextureRef>,
}

impl<'s> RectangleShape<'s> {
    /// Returns a new `RectangleShape`.
    pub fn new() -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_create() };
        assert!(!rectangle.is_null(), "Failed to create RectangleShape");
        RectangleShape {
            rectangle_shape: rectangle,
            texture: PhantomData,
        }
    }

    /// Returns a new `RectangleShape` with the provided texture.
    pub fn with_texture(texture: &'s TextureRef) -> RectangleShape<'s> {
        let mut shape = Self::new();
        shape.set_texture(texture, true);
        shape
    }

    /// Returns a new `RectangleShape` with the provided size.
    pub fn with_size(size: Vector2f) -> RectangleShape<'s> {
        let mut shape = Self::new();
        shape.set_size(size);
        shape
    }

    /// Get the size of a rectangle shape
    ///
    /// Return the height Size of the rectangle
    pub fn size(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getSize(self.rectangle_shape)) }
    }

    /// Set the size of a rectangle shape
    ///
    /// # Arguments
    /// * size - The new size of the rectangle
    pub fn set_size<S: Into<Vector2f>>(&mut self, size: S) {
        unsafe { ffi::sfRectangleShape_setSize(self.rectangle_shape, size.into().raw()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfRectangleShape {
        self.rectangle_shape
    }
}

impl<'s> Default for RectangleShape<'s> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'s> Drawable for RectangleShape<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(
        &'se self,
        target: &mut RenderTarget,
        states: RenderStates<'tex, 'sh, 'shte>,
    ) where
        'se: 'sh,
    {
        target.draw_rectangle_shape(self, states);
    }
}

impl<'s> Transformable for RectangleShape<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfRectangleShape_setPosition(self.rectangle_shape, position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfRectangleShape_setRotation(self.rectangle_shape, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfRectangleShape_setScale(self.rectangle_shape, scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfRectangleShape_setOrigin(self.rectangle_shape, origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getPosition(self.rectangle_shape)) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getRotation(self.rectangle_shape) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getScale(self.rectangle_shape)) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfRectangleShape_getOrigin(self.rectangle_shape)) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfRectangleShape_move(self.rectangle_shape, offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfRectangleShape_rotate(self.rectangle_shape, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfRectangleShape_scale(self.rectangle_shape, factors.into().raw()) }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfRectangleShape_getTransform(self.rectangle_shape)) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe {
            Transform(ffi::sfRectangleShape_getInverseTransform(
                self.rectangle_shape,
            ))
        }
    }
}

impl<'s> Shape<'s> for RectangleShape<'s> {
    fn set_texture(&mut self, texture: &'s TextureRef, reset_rect: bool) {
        unsafe {
            ffi::sfRectangleShape_setTexture(
                self.rectangle_shape,
                texture.raw(),
                sfBool::from_bool(reset_rect),
            )
        }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfRectangleShape_setTexture(self.rectangle_shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfRectangleShape_setTextureRect(self.rectangle_shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfRectangleShape_setFillColor(self.rectangle_shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfRectangleShape_setOutlineColor(self.rectangle_shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfRectangleShape_setOutlineThickness(self.rectangle_shape, thickness) }
    }
    fn texture(&self) -> Option<&'s TextureRef> {
        unsafe {
            let raw = ffi::sfRectangleShape_getTexture(self.rectangle_shape);

            if raw.is_null() {
                None
            } else {
                Some(&*(raw as *const TextureRef))
            }
        }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfRectangleShape_getTextureRect(self.rectangle_shape)) }
    }
    fn fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfRectangleShape_getFillColor(self.rectangle_shape)) }
    }
    fn outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfRectangleShape_getOutlineColor(self.rectangle_shape)) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getOutlineThickness(self.rectangle_shape) }
    }
    fn point_count(&self) -> u32 {
        unsafe { ffi::sfRectangleShape_getPointCount(self.rectangle_shape) as u32 }
    }
    fn point(&self, index: u32) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRectangleShape_getPoint(
                self.rectangle_shape,
                index as usize,
            ))
        }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfRectangleShape_getLocalBounds(self.rectangle_shape)) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfRectangleShape_getGlobalBounds(self.rectangle_shape)) }
    }
}

impl<'s> Clone for RectangleShape<'s> {
    /// Return a new RectangleShape or panic! if there is not enough memory
    fn clone(&self) -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_copy(self.rectangle_shape) };
        if rectangle.is_null() {
            panic!("Not enough memory to clone RectangleShape")
        } else {
            RectangleShape {
                rectangle_shape: rectangle,
                texture: self.texture,
            }
        }
    }
}

impl<'s> Drop for RectangleShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfRectangleShape_destroy(self.rectangle_shape) }
    }
}
