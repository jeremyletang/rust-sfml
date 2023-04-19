use {
    crate::{
        ffi::graphics as ffi,
        graphics::{
            Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture,
            Transform, Transformable,
        },
        system::Vector2f,
    },
    std::{marker::PhantomData, ptr},
};

/// Specialized shape representing a convex polygon
///
/// It is important to keep in mind that a convex shape must
/// always be... convex, otherwise it may not be drawn correctly.
/// Moreover, the points must be defined in order; using a random
/// order would result in an incorrect shape.
#[derive(Debug)]
pub struct ConvexShape<'s> {
    convex_shape: *mut ffi::sfConvexShape,
    texture: PhantomData<&'s Texture>,
}

/// An iterator over the points of a [`ConvexShape`].
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub struct ConvexShapePoints {
    convex_shape: *mut ffi::sfConvexShape,
    pos: usize,
}

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape
    ///
    /// # Arguments
    /// * `points_count` - The number of point for the convex shape
    #[must_use]
    pub fn new(points_count: usize) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_create() };
        assert!(!shape.is_null(), "Failed to create ConvexShape");
        let mut shape = ConvexShape {
            convex_shape: shape,
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
    pub fn set_point<P: Into<Vector2f>>(&mut self, index: usize, point: P) {
        assert!(
            index < self.point_count(),
            "Index out of bounds. Index: {}, len: {}",
            index,
            self.point_count()
        );
        unsafe { ffi::sfConvexShape_setPoint(self.convex_shape, index, point.into()) }
    }

    /// Set the number of points of a convex
    ///
    /// # Arguments
    /// * count - New number of points of the convex
    pub fn set_point_count(&mut self, count: usize) {
        unsafe { ffi::sfConvexShape_setPointCount(self.convex_shape, count) }
    }

    /// Return an immutable iterator over all the points of the `ConvexShape`
    #[must_use]
    pub fn points(&self) -> ConvexShapePoints {
        ConvexShapePoints {
            convex_shape: self.convex_shape,
            pos: 0,
        }
    }
    pub(super) fn raw(&self) -> *const ffi::sfConvexShape {
        self.convex_shape
    }
}

impl<'s> Drawable for ConvexShape<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_convex_shape(self, states)
    }
}

impl<'s> Transformable for ConvexShape<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfConvexShape_setPosition(self.convex_shape, position.into()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_setRotation(self.convex_shape, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfConvexShape_setScale(self.convex_shape, scale.into()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfConvexShape_setOrigin(self.convex_shape, origin.into()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { ffi::sfConvexShape_getPosition(self.convex_shape) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getRotation(self.convex_shape) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { ffi::sfConvexShape_getScale(self.convex_shape) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { ffi::sfConvexShape_getOrigin(self.convex_shape) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfConvexShape_move(self.convex_shape, offset.into()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_rotate(self.convex_shape, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfConvexShape_scale(self.convex_shape, factors.into()) }
    }
    fn transform(&self) -> &Transform {
        unsafe { &*ffi::sfConvexShape_getTransform(self.convex_shape) }
    }
    fn inverse_transform(&self) -> &Transform {
        unsafe { &*ffi::sfConvexShape_getInverseTransform(self.convex_shape) }
    }
}

impl<'s> Shape<'s> for ConvexShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe { ffi::sfConvexShape_setTexture(self.convex_shape, texture, reset_rect) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfConvexShape_setTexture(self.convex_shape, ptr::null_mut(), true) }
    }
    fn set_texture_rect(&mut self, rect: IntRect) {
        unsafe { ffi::sfConvexShape_setTextureRect(self.convex_shape, rect) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfConvexShape_setFillColor(self.convex_shape, color) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfConvexShape_setOutlineColor(self.convex_shape, color) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfConvexShape_setOutlineThickness(self.convex_shape, thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe { ffi::sfConvexShape_getTexture(self.convex_shape).as_ref() }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { ffi::sfConvexShape_getTextureRect(self.convex_shape) }
    }
    fn fill_color(&self) -> Color {
        unsafe { ffi::sfConvexShape_getFillColor(self.convex_shape) }
    }
    fn outline_color(&self) -> Color {
        unsafe { ffi::sfConvexShape_getOutlineColor(self.convex_shape) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getOutlineThickness(self.convex_shape) }
    }
    fn point_count(&self) -> usize {
        unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) }
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
            ffi::sfConvexShape_getPoint(self.convex_shape, index)
        }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { ffi::sfConvexShape_getLocalBounds(self.convex_shape) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { ffi::sfConvexShape_getGlobalBounds(self.convex_shape) }
    }
}

impl<'s> Clone for ConvexShape<'s> {
    /// Return a new `ConvexShape` or panic if there is not enough memory
    fn clone(&self) -> ConvexShape<'s> {
        let shape = unsafe { ffi::sfConvexShape_copy(self.convex_shape) };
        if shape.is_null() {
            panic!("Not enough memory to clone ConvexShape")
        } else {
            ConvexShape {
                convex_shape: shape,
                texture: self.texture,
            }
        }
    }
}

impl Iterator for ConvexShapePoints {
    type Item = Vector2f;

    fn next(&mut self) -> Option<Vector2f> {
        let point_count = unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) };
        if self.pos == point_count {
            None
        } else {
            let point = unsafe { ffi::sfConvexShape_getPoint(self.convex_shape, self.pos) };
            self.pos += 1;
            Some(point)
        }
    }
}

impl<'s> Drop for ConvexShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfConvexShape_destroy(self.convex_shape) }
    }
}
