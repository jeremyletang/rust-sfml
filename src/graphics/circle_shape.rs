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

/// Specialized shape representing a circle.
#[derive(Debug)]
pub struct CircleShape<'s> {
    handle: NonNull<ffi::sfCircleShape>,
    texture: PhantomData<&'s Texture>,
}

impl<'s> CircleShape<'s> {
    /// Create a new circle shape initialized with a texture
    ///
    /// # Arguments
    /// * texture - The texture to initialize the `CircleShape` with.
    #[must_use]
    pub fn with_texture(texture: &'s Texture) -> CircleShape<'s> {
        let mut shape = CircleShape::default();
        shape.set_texture(texture, true);
        shape
    }

    /// Create a new `CircleShape` and initialize it.
    ///
    /// # Arguments:
    /// * radius - The radius of the `CircleShape`
    /// * `point_count` - The number of points to define the `CircleShape`
    ///
    /// Default value on SFML are radius = 0 / pointCount = 30
    #[must_use]
    pub fn new(radius: f32, point_count: usize) -> CircleShape<'s> {
        let mut shape = CircleShape::default();
        shape.set_radius(radius);
        shape.set_point_count(point_count);
        shape
    }

    /// Set the radius of a circle
    ///
    /// # Arguments
    /// * radius - New radius of the circle
    pub fn set_radius(&mut self, radius: f32) {
        unsafe { ffi::sfCircleShape_setRadius(self.handle.as_ptr(), radius) }
    }

    /// Set the radius of a circle
    ///
    /// Return the radius of the circle
    #[must_use]
    pub fn radius(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRadius(self.handle.as_ptr()) }
    }

    /// Set the number of points of a circle
    ///
    /// # Arguments
    /// * count - New number of points of the circle
    pub fn set_point_count(&mut self, count: usize) {
        unsafe { ffi::sfCircleShape_setPointCount(self.handle.as_ptr(), count) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfCircleShape {
        self.handle.as_ptr()
    }
}

impl Default for CircleShape<'_> {
    fn default() -> Self {
        let circle = unsafe { ffi::sfCircleShape_new() };
        CircleShape {
            handle: NonNull::new(circle).expect("Failed to create CircleShape"),
            texture: PhantomData,
        }
    }
}

impl Drawable for CircleShape<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_circle_shape(self, states)
    }
}

impl Transformable for CircleShape<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfCircleShape_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_setRotation(self.handle.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfCircleShape_setScale(self.handle.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfCircleShape_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfCircleShape_getPosition(self.handle.as_ptr()) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRotation(self.handle.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfCircleShape_getScale(self.handle.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfCircleShape_getOrigin(self.handle.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfCircleShape_move(self.handle.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_rotate(self.handle.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfCircleShape_scale(self.handle.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfCircleShape_getTransform(self.handle.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfCircleShape_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl<'s> Shape<'s> for CircleShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfCircleShape_setTexture(self.handle.as_ptr(), texture, reset_rect) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfCircleShape_setTexture(self.handle.as_ptr(), ptr::null_mut(), true) }
    }
    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfCircleShape_setTextureRect(self.handle.as_ptr(), rect) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfCircleShape_setFillColor(self.handle.as_ptr(), color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfCircleShape_setOutlineColor(self.handle.as_ptr(), color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfCircleShape_setOutlineThickness(self.handle.as_ptr(), thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe {
            let raw = ffi::sfCircleShape_getTexture(self.handle.as_ptr());
            raw.as_ref()
        }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfCircleShape_getTextureRect(self.handle.as_ptr()) }
    }
    fn fill_color(&self) -> Color {
        unsafe { ffi::sfCircleShape_getFillColor(self.handle.as_ptr()) }
    }
    fn outline_color(&self) -> Color {
        unsafe { ffi::sfCircleShape_getOutlineColor(self.handle.as_ptr()) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getOutlineThickness(self.handle.as_ptr()) }
    }
    fn point_count(&self) -> usize {
        unsafe { ffi::sfCircleShape_getPointCount(self.handle.as_ptr()) }
    }
    fn point(&self, index: usize) -> Vector2f {
        unsafe { ffi::sfCircleShape_getPoint(self.handle.as_ptr(), index) }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfCircleShape_getLocalBounds(self.handle.as_ptr()) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfCircleShape_getGlobalBounds(self.handle.as_ptr()) }
    }
}

impl<'s> Clone for CircleShape<'s> {
    /// Return a new `CircleShape` or panic if there is not enough memory
    fn clone(&self) -> CircleShape<'s> {
        let circle = unsafe { ffi::sfCircleShape_cpy(self.handle.as_ptr()) };
        CircleShape {
            handle: NonNull::new(circle).expect("Not enough memory to clone CircleShape"),
            texture: self.texture,
        }
    }
}

impl Drop for CircleShape<'_> {
    fn drop(&mut self) {
        unsafe { ffi::sfCircleShape_del(self.handle.as_ptr()) }
    }
}
