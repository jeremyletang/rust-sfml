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

/// Specialized shape representing a convex polygon
///
/// It is important to keep in mind that a convex shape must
/// always be... convex, otherwise it may not be drawn correctly.
/// Moreover, the points must be defined in order; using a random
/// order would result in an incorrect shape.
#[derive(Debug)]
pub struct ConvexShape<'s> {
    handle: NonNull<ffi::sfConvexShape>,
    texture: PhantomData<&'s Texture>,
}

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape
    ///
    /// # Arguments
    /// * `points_count` - The number of point for the convex shape
    ///
    /// # Panics
    ///
    /// Panics if for some reason a `ConvexShape` can't be created.
    #[must_use]
    pub fn new(points_count: usize) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_new() };
        let mut shape = ConvexShape {
            handle: NonNull::new(shape).expect("Failed to create ConvexShape"),
            texture: PhantomData,
        };
        shape.set_point_count(points_count);
        shape
    }

    /// Create a new convex shape with a texture
    ///
    /// # Arguments
    /// * texture - The texture to apply to the convex shape
    /// * `points_count` - The number of point for the convex shape
    #[must_use]
    pub fn with_texture(points_count: usize, texture: &'s Texture) -> ConvexShape<'s> {
        let mut shape = ConvexShape::new(points_count);
        shape.set_texture(texture, true);
        shape
    }

    /// Set the position of a point.
    ///
    /// Don't forget that the polygon must remain convex, and the points need to stay ordered!
    /// [`set_point_count`] must be called first in order to set the total number of points.
    /// The result is undefined if index is out of the valid range.
    ///
    /// [`set_point_count`]: ConvexShape::set_point_count
    ///
    /// # Arguments
    /// * index - Index of the point to change, in range `[0 .. get_point_count() - 1]`
    /// * point - New position of the point
    ///
    /// # Panics
    ///
    /// Panics on out of bounds access
    pub fn set_point<P: Into<Vector2f>>(&mut self, index: usize, point: P) {
        assert!(
            index < self.point_count(),
            "Index out of bounds. Index: {}, len: {}",
            index,
            self.point_count()
        );
        unsafe { ffi::sfConvexShape_setPoint(self.handle.as_ptr(), index, point.into()) }
    }

    /// Set the number of points of a convex
    ///
    /// # Arguments
    /// * count - New number of points of the convex
    pub fn set_point_count(&mut self, count: usize) {
        unsafe { ffi::sfConvexShape_setPointCount(self.handle.as_ptr(), count) }
    }

    pub(super) fn raw(&self) -> *const ffi::sfConvexShape {
        self.handle.as_ptr()
    }
}

impl Drawable for ConvexShape<'_> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_convex_shape(self, states)
    }
}

impl Transformable for ConvexShape<'_> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfConvexShape_setPosition(self.handle.as_ptr(), position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_setRotation(self.handle.as_ptr(), angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfConvexShape_setScale(self.handle.as_ptr(), scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfConvexShape_setOrigin(self.handle.as_ptr(), origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfConvexShape_getPosition(self.handle.as_ptr()) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getRotation(self.handle.as_ptr()) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfConvexShape_getScale(self.handle.as_ptr()) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfConvexShape_getOrigin(self.handle.as_ptr()) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfConvexShape_move(self.handle.as_ptr(), offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_rotate(self.handle.as_ptr(), angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfConvexShape_scale(self.handle.as_ptr(), factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfConvexShape_getTransform(self.handle.as_ptr()) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfConvexShape_getInverseTransform(self.handle.as_ptr()) }
    }
}

impl<'s> Shape<'s> for ConvexShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfConvexShape_setTexture(self.handle.as_ptr(), texture, reset_rect) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfConvexShape_setTexture(self.handle.as_ptr(), ptr::null_mut(), true) }
    }
    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfConvexShape_setTextureRect(self.handle.as_ptr(), rect) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfConvexShape_setFillColor(self.handle.as_ptr(), color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfConvexShape_setOutlineColor(self.handle.as_ptr(), color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfConvexShape_setOutlineThickness(self.handle.as_ptr(), thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe { ffi::sfConvexShape_getTexture(self.handle.as_ptr()).as_ref() }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfConvexShape_getTextureRect(self.handle.as_ptr()) }
    }
    fn fill_color(&self) -> Color {
        unsafe { ffi::sfConvexShape_getFillColor(self.handle.as_ptr()) }
    }
    fn outline_color(&self) -> Color {
        unsafe { ffi::sfConvexShape_getOutlineColor(self.handle.as_ptr()) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getOutlineThickness(self.handle.as_ptr()) }
    }
    fn point_count(&self) -> usize {
        unsafe { ffi::sfConvexShape_getPointCount(self.handle.as_ptr()) }
    }
    fn point(&self, index: usize) -> Vector2f {
        unsafe {
            // ConvexShape stores items in a vector, and does unchecked indexing.
            // To retain safety, we check for OOB here.
            assert!(
                index < self.point_count(),
                "Index out of bounds. Index: {}, len: {}",
                index,
                self.point_count()
            );
            ffi::sfConvexShape_getPoint(self.handle.as_ptr(), index)
        }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfConvexShape_getLocalBounds(self.handle.as_ptr()) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfConvexShape_getGlobalBounds(self.handle.as_ptr()) }
    }
}

impl<'s> Clone for ConvexShape<'s> {
    /// Return a new `ConvexShape` or panic if there is not enough memory
    fn clone(&self) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_cpy(self.handle.as_ptr()) };
        ConvexShape {
            handle: NonNull::new(shape).expect("Not enough memory to clone ConvexShape"),
            texture: self.texture,
        }
    }
}

impl Drop for ConvexShape<'_> {
    fn drop(&mut self) {
        unsafe { ffi::sfConvexShape_del(self.handle.as_ptr()) }
    }
}
