use crate::{
    ffi::{self as ffi, sfBool, sfTrue},
    graphics::{
        Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture, Transform,
        Transformable,
    },
    sf_bool_ext::SfBoolExt,
    system::Vector2f,
};
use std::{
    marker::PhantomData,
    ptr::{self, NonNull},
};

/// Specialized shape representing a circle.
#[derive(Debug)]
pub struct CircleShape<'s> {
    circle_shape: NonNull<ffi::sfCircleShape>,
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
    pub fn new(radius: f32, point_count: u32) -> CircleShape<'s> {
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
        unsafe { ffi::sfCircleShape_setRadius(self.circle_shape.as_ptr(), radius) }
    }

    /// Set the radius of a circle
    ///
    /// Return the radius of the circle
    #[must_use]
    pub fn radius(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRadius(self.circle_shape.as_ptr()) }
    }

    /// Set the number of points of a circle
    ///
    /// # Arguments
    /// * count - New number of points of the circle
    pub fn set_point_count(&mut self, count: u32) {
        unsafe { ffi::sfCircleShape_setPointCount(self.circle_shape.as_ptr(), count as usize) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfCircleShape {
        self.circle_shape.as_ptr()
    }
}

impl<'s> Default for CircleShape<'s> {
    fn default() -> Self {
        let circle = unsafe { ffi::sfCircleShape_create() };
        CircleShape {
            circle_shape: NonNull::new(circle).expect("Failed to create CircleShape"),
            texture: PhantomData,
        }
    }
}

impl<'s> Drawable for CircleShape<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_circle_shape(self, states)
    }
}

impl<'s> Transformable for CircleShape<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfCircleShape_setPosition(self.circle_shape.as_ptr(), position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_setRotation(self.circle_shape.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfCircleShape_setScale(self.circle_shape.as_ptr(), scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfCircleShape_setOrigin(self.circle_shape.as_ptr(), origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getPosition(self.circle_shape.as_ptr())) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getRotation(self.circle_shape.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getScale(self.circle_shape.as_ptr())) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfCircleShape_getOrigin(self.circle_shape.as_ptr())) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfCircleShape_move(self.circle_shape.as_ptr(), offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfCircleShape_rotate(self.circle_shape.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfCircleShape_scale(self.circle_shape.as_ptr(), factors.into().raw()) }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfCircleShape_getTransform(self.circle_shape.as_ptr())) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe {
            Transform(ffi::sfCircleShape_getInverseTransform(
                self.circle_shape.as_ptr(),
            ))
        }
    }
}

impl<'s> Shape<'s> for CircleShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe {
            ffi::sfCircleShape_setTexture(
                self.circle_shape.as_ptr(),
                texture.raw(),
                sfBool::from_bool(reset_rect),
            )
        }
    }
    fn disable_texture(&mut self) {
        unsafe {
            ffi::sfCircleShape_setTexture(self.circle_shape.as_ptr(), ptr::null_mut(), sfTrue)
        }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfCircleShape_setTextureRect(self.circle_shape.as_ptr(), rect.raw()) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfCircleShape_setFillColor(self.circle_shape.as_ptr(), color.0) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfCircleShape_setOutlineColor(self.circle_shape.as_ptr(), color.0) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfCircleShape_setOutlineThickness(self.circle_shape.as_ptr(), thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe {
            let raw = ffi::sfCircleShape_getTexture(self.circle_shape.as_ptr());

            if raw.is_null() {
                None
            } else {
                Some(&*(raw as *const Texture))
            }
        }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe {
            IntRect::from_raw(ffi::sfCircleShape_getTextureRect(
                self.circle_shape.as_ptr(),
            ))
        }
    }
    fn fill_color(&self) -> Color {
        unsafe { Color(ffi::sfCircleShape_getFillColor(self.circle_shape.as_ptr())) }
    }
    fn outline_color(&self) -> Color {
        unsafe {
            Color(ffi::sfCircleShape_getOutlineColor(
                self.circle_shape.as_ptr(),
            ))
        }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfCircleShape_getOutlineThickness(self.circle_shape.as_ptr()) }
    }
    fn point_count(&self) -> u32 {
        use std::convert::TryInto;
        unsafe {
            ffi::sfCircleShape_getPointCount(self.circle_shape.as_ptr())
                .try_into()
                .unwrap()
        }
    }
    fn point(&self, index: u32) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfCircleShape_getPoint(
                self.circle_shape.as_ptr(),
                index as usize,
            ))
        }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe {
            FloatRect::from_raw(ffi::sfCircleShape_getLocalBounds(
                self.circle_shape.as_ptr(),
            ))
        }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe {
            FloatRect::from_raw(ffi::sfCircleShape_getGlobalBounds(
                self.circle_shape.as_ptr(),
            ))
        }
    }
}

impl<'s> Clone for CircleShape<'s> {
    /// Return a new `CircleShape` or panic if there is not enough memory
    fn clone(&self) -> CircleShape<'s> {
        let circle = unsafe { ffi::sfCircleShape_copy(self.circle_shape.as_ptr()) };
        CircleShape {
            circle_shape: NonNull::new(circle).expect("Not enough memory to clone CircleShape"),
            texture: self.texture,
        }
    }
}

impl<'s> Drop for CircleShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfCircleShape_destroy(self.circle_shape.as_ptr()) }
    }
}
