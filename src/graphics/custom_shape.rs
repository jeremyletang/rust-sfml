use {
    crate::{
        ffi::{graphics as ffi, sfVector2f},
        graphics::{
            Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture,
            Transform, Transformable,
        },
        system::Vector2f,
    },
    std::{
        marker::PhantomData,
        os::raw::c_void,
        ptr::{self, NonNull},
    },
};

/// The points of a custom shape.
pub trait CustomShapePoints {
    /// Gets the total count of points.
    ///
    /// Return the points count
    fn point_count(&self) -> usize;

    /// Gets a given point.
    ///
    /// # Argument
    /// * point - The index of the point to return
    ///
    /// Returns a [`Vector2f`] containing the coordinates.
    ///
    /// [`Vector2f`]: crate::system::Vector2f
    fn point(&self, point: usize) -> Vector2f;
}

/// A custom textured shape with outline.
#[derive(Debug)]
pub struct CustomShape<'s> {
    shape: NonNull<ffi::sfCustomShape>,
    texture: PhantomData<&'s Texture>,
    points: NonNull<Box<dyn CustomShapePoints + Send>>,
}

unsafe extern "C" fn get_point_count_callback(obj: *mut c_void) -> usize {
    let shape = obj as *const Box<dyn CustomShapePoints + Send>;
    unsafe { (*shape).point_count() }
}

unsafe extern "C" fn get_point_callback(point: usize, obj: *mut c_void) -> sfVector2f {
    let shape = obj as *const Box<dyn CustomShapePoints + Send>;
    unsafe { (*shape).point(point) }
}

impl<'s> CustomShape<'s> {
    /// Create a new `CustomShape`
    ///
    /// # Arguments
    /// * points - Implementation of [`CustomShapePoints`]
    #[must_use]
    pub fn new(points: Box<dyn CustomShapePoints + Send>) -> CustomShape<'s> {
        // SAFETY: Box::into_raw produces non-null pointer
        let raw_impl = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(points))) };
        let sp = unsafe {
            ffi::sfCustomShape_create(
                Some(get_point_count_callback),
                Some(get_point_callback),
                raw_impl.as_ptr().cast(),
            )
        };
        CustomShape {
            shape: NonNull::new(sp).expect("Failed to create CustomShape"),
            texture: PhantomData,
            points: raw_impl,
        }
    }

    /// Create a new `CustomShape` with a texture
    ///
    /// # Arguments
    /// * points - Implementation of [`CustomShapePoints`] trait
    /// * texture - The texture to bind to the `CustomShape`
    #[must_use]
    pub fn with_texture(
        points: Box<dyn CustomShapePoints + Send>,
        texture: &'s Texture,
    ) -> CustomShape<'s> {
        let mut shape = Self::new(points);
        shape.set_texture(texture, true);
        shape
    }

    /// Recompute the internal geometry of a shape
    ///
    /// This function must be called by specialized shape objects
    /// everytime their points change (ie. the result of either
    /// the [`point_count`] or [`point`] callbacks is different).
    ///
    /// [`point_count`]: CustomShapePoints::point_count
    /// [`point`]: CustomShapePoints::point
    pub fn update(&mut self) {
        unsafe { ffi::sfCustomShape_update(self.shape.as_ptr()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfCustomShape {
        self.shape.as_ptr()
    }
}

impl<'s> Shape<'s> for CustomShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfCustomShape_setTexture(self.shape.as_ptr(), texture, reset_rect) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfCustomShape_setTexture(self.shape.as_ptr(), ptr::null_mut(), true) }
    }
    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfCustomShape_setTextureRect(self.shape.as_ptr(), rect) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfCustomShape_setFillColor(self.shape.as_ptr(), color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfCustomShape_setOutlineColor(self.shape.as_ptr(), color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfCustomShape_setOutlineThickness(self.shape.as_ptr(), thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe { ffi::sfCustomShape_getTexture(self.shape.as_ptr()).as_ref() }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfCustomShape_getTextureRect(self.shape.as_ptr()) }
    }
    fn fill_color(&self) -> Color {
        unsafe { ffi::sfCustomShape_getFillColor(self.shape.as_ptr()) }
    }
    fn outline_color(&self) -> Color {
        unsafe { ffi::sfCustomShape_getOutlineColor(self.shape.as_ptr()) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfCustomShape_getOutlineThickness(self.shape.as_ptr()) }
    }
    fn point_count(&self) -> usize {
        unsafe { ffi::sfCustomShape_getPointCount(self.shape.as_ptr()) }
    }
    fn point(&self, index: usize) -> Vector2f {
        unsafe { ffi::sfCustomShape_getPoint(self.shape.as_ptr(), index) }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfCustomShape_getLocalBounds(self.shape.as_ptr()) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfCustomShape_getGlobalBounds(self.shape.as_ptr()) }
    }
}

impl Drawable for CustomShape<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_shape(self, states)
    }
}

impl Transformable for CustomShape<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfCustomShape_setPosition(self.shape.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfCustomShape_setRotation(self.shape.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfCustomShape_setScale(self.shape.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfCustomShape_setOrigin(self.shape.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfCustomShape_getPosition(self.shape.as_ptr()) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfCustomShape_getRotation(self.shape.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfCustomShape_getScale(self.shape.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfCustomShape_getOrigin(self.shape.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfCustomShape_move(self.shape.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfCustomShape_rotate(self.shape.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfCustomShape_scale(self.shape.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfCustomShape_getTransform(self.shape.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfCustomShape_getInverseTransform(self.shape.as_ptr()) }
    }
}

impl Drop for CustomShape<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfCustomShape_destroy(self.shape.as_ptr());
            let _ = Box::from_raw(self.points.as_ptr());
        }
    }
}
