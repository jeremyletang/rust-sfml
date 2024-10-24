use {
    crate::{
        ffi::graphics as ffi,
        graphics::{
            Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture,
            Transform, Transformable,
        },
        system::Vector2f,
    },
    std::{
        marker::PhantomData,
        ptr::{self, NonNull},
    },
};

/// Specialized shape representing a rectangle
#[derive(Debug)]
pub struct RectangleShape<'s> {
    handle: NonNull<ffi::sfRectangleShape>,
    texture: PhantomData<&'s Texture>,
}

impl<'s> RectangleShape<'s> {
    /// Returns a new `RectangleShape`.
    ///
    /// # Panics
    ///
    /// Panics if a `RectangleShape` can't be created for some reason.
    #[must_use]
    pub fn new() -> RectangleShape<'s> {
        let rectangle = unsafe { ffi::sfRectangleShape_new() };
        RectangleShape {
            handle: NonNull::new(rectangle).expect("Failed to create RectangleShape"),
            texture: PhantomData,
        }
    }

    /// Returns a new `RectangleShape` with the provided texture.
    #[must_use]
    pub fn with_texture(texture: &'s Texture) -> RectangleShape<'s> {
        let mut shape = Self::new();
        shape.set_texture(texture, true);
        shape
    }

    /// Returns a new `RectangleShape` with the provided size.
    #[must_use]
    pub fn with_size(size: Vector2f) -> RectangleShape<'s> {
        let mut shape = Self::new();
        shape.set_size(size);
        shape
    }

    /// Returns a new `RectangleShape` created from a [`FloatRect`].
    #[must_use]
    pub fn from_rect(rect: FloatRect) -> Self {
        let mut shape = Self::new();
        shape.set_size((rect.width, rect.height));
        shape.set_position((rect.left, rect.top));
        shape
    }

    /// Get the size of a rectangle shape
    ///
    /// Return the height Size of the rectangle
    #[must_use]
    pub fn size(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getSize(self.handle.as_ptr()) }
    }

    /// Set the size of a rectangle shape
    ///
    /// # Arguments
    /// * size - The new size of the rectangle
    pub fn set_size<S: Into<Vector2f>>(&mut self, size: S) {
        unsafe { ffi::sfRectangleShape_setSize(self.handle.as_ptr(), size.into()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfRectangleShape {
        self.handle.as_ptr()
    }
}

impl Default for RectangleShape<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for RectangleShape<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_rectangle_shape(self, states);
    }
}

impl Transformable for RectangleShape<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfRectangleShape_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfRectangleShape_setRotation(self.handle.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfRectangleShape_setScale(self.handle.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfRectangleShape_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getPosition(self.handle.as_ptr()) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getRotation(self.handle.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getScale(self.handle.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getOrigin(self.handle.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfRectangleShape_move(self.handle.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfRectangleShape_rotate(self.handle.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfRectangleShape_scale(self.handle.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfRectangleShape_getTransform(self.handle.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfRectangleShape_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl<'s> Shape<'s> for RectangleShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfRectangleShape_setTexture(self.handle.as_ptr(), texture, reset_rect) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfRectangleShape_setTexture(self.handle.as_ptr(), ptr::null_mut(), true) }
    }
    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfRectangleShape_setTextureRect(self.handle.as_ptr(), rect) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfRectangleShape_setFillColor(self.handle.as_ptr(), color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfRectangleShape_setOutlineColor(self.handle.as_ptr(), color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfRectangleShape_setOutlineThickness(self.handle.as_ptr(), thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe { ffi::sfRectangleShape_getTexture(self.handle.as_ptr()).as_ref() }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfRectangleShape_getTextureRect(self.handle.as_ptr()) }
    }
    fn fill_color(&self) -> Color {
        unsafe { ffi::sfRectangleShape_getFillColor(self.handle.as_ptr()) }
    }
    fn outline_color(&self) -> Color {
        unsafe { ffi::sfRectangleShape_getOutlineColor(self.handle.as_ptr()) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfRectangleShape_getOutlineThickness(self.handle.as_ptr()) }
    }
    fn point_count(&self) -> usize {
        unsafe { ffi::sfRectangleShape_getPointCount(self.handle.as_ptr()) }
    }
    fn point(&self, index: usize) -> Vector2f {
        unsafe { ffi::sfRectangleShape_getPoint(self.handle.as_ptr(), index) }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfRectangleShape_getLocalBounds(self.handle.as_ptr()) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfRectangleShape_getGlobalBounds(self.handle.as_ptr()) }
    }
}

impl<'s> Clone for RectangleShape<'s> {
    /// Return a new `RectangleShape` or panic! if there is not enough memory
    fn clone(&self) -> RectangleShape<'s> {
        let copy = unsafe { ffi::sfRectangleShape_cpy(self.handle.as_ptr()) };
        RectangleShape {
            handle: NonNull::new(copy).expect("Not enough memory to clone RectangleShape"),
            texture: self.texture,
        }
    }
}

impl Drop for RectangleShape<'_> {
    fn drop(&mut self) {
        unsafe { ffi::sfRectangleShape_del(self.handle.as_ptr()) }
    }
}
