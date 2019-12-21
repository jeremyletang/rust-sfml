use crate::graphics::csfml_graphics_sys as ffi;
use crate::graphics::{
    Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, Texture, Transform,
    Transformable,
};
use crate::sf_bool_ext::SfBoolExt;
use crate::system::Vector2f;
use csfml_system_sys::{sfBool, sfTrue};
use std::marker::PhantomData;
use std::ptr;

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
    pos: u32,
}

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape
    ///
    /// # Arguments
    /// * points_count - The number of point for the convex shape
    #[must_use]
    pub fn new(points_count: u32) -> ConvexShape<'s> {
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
    /// * points_count - The number of point for the convex shape
    #[must_use]
    pub fn with_texture(points_count: u32, texture: &'s Texture) -> ConvexShape<'s> {
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
    pub fn set_point<P: Into<Vector2f>>(&mut self, index: u32, point: P) {
        if index >= self.point_count() {
            panic!(
                "Index out of bounds. Index: {}, len: {}",
                index,
                self.point_count()
            );
        }
        unsafe {
            ffi::sfConvexShape_setPoint(self.convex_shape, index as usize, point.into().raw())
        }
    }

    /// Set the number of points of a convex
    ///
    /// # Arguments
    /// * count - New number of points of the convex
    pub fn set_point_count(&mut self, count: u32) {
        unsafe { ffi::sfConvexShape_setPointCount(self.convex_shape, count as usize) }
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
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_convex_shape(self, states)
    }
}

impl<'s> Transformable for ConvexShape<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfConvexShape_setPosition(self.convex_shape, position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_setRotation(self.convex_shape, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfConvexShape_setScale(self.convex_shape, scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfConvexShape_setOrigin(self.convex_shape, origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfConvexShape_getPosition(self.convex_shape)) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getRotation(self.convex_shape) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfConvexShape_getScale(self.convex_shape)) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfConvexShape_getOrigin(self.convex_shape)) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfConvexShape_move(self.convex_shape, offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfConvexShape_rotate(self.convex_shape, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfConvexShape_scale(self.convex_shape, factors.into().raw()) }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfConvexShape_getTransform(self.convex_shape)) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfConvexShape_getInverseTransform(self.convex_shape)) }
    }
}

impl<'s> Shape<'s> for ConvexShape<'s> {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool) {
        unsafe {
            ffi::sfConvexShape_setTexture(
                self.convex_shape,
                texture.raw(),
                sfBool::from_bool(reset_rect),
            )
        }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfConvexShape_setTexture(self.convex_shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfConvexShape_setTextureRect(self.convex_shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: Color) {
        unsafe { ffi::sfConvexShape_setFillColor(self.convex_shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: Color) {
        unsafe { ffi::sfConvexShape_setOutlineColor(self.convex_shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfConvexShape_setOutlineThickness(self.convex_shape, thickness) }
    }
    fn texture(&self) -> Option<&'s Texture> {
        unsafe {
            let raw = ffi::sfConvexShape_getTexture(self.convex_shape);

            if raw.is_null() {
                None
            } else {
                Some(&*(raw as *const Texture))
            }
        }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfConvexShape_getTextureRect(self.convex_shape)) }
    }
    fn fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfConvexShape_getFillColor(self.convex_shape)) }
    }
    fn outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfConvexShape_getOutlineColor(self.convex_shape)) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfConvexShape_getOutlineThickness(self.convex_shape) }
    }
    fn point_count(&self) -> u32 {
        unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) as u32 }
    }
    fn point(&self, index: u32) -> Vector2f {
        unsafe {
            // ConvexShape stores items in a vector, and does unchecked indexing.
            // To retain safety, we check for OOB here.
            if index >= self.point_count() {
                panic!(
                    "Index out of bounds. Index: {}, len: {}",
                    index,
                    self.point_count()
                );
            }
            Vector2f::from_raw(ffi::sfConvexShape_getPoint(
                self.convex_shape,
                index as usize,
            ))
        }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfConvexShape_getLocalBounds(self.convex_shape)) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfConvexShape_getGlobalBounds(self.convex_shape)) }
    }
}

impl<'s> Clone for ConvexShape<'s> {
    /// Return a new ConvexShape or panic! if there is not enough memory
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
        let point_count = unsafe { ffi::sfConvexShape_getPointCount(self.convex_shape) as u32 };
        if self.pos == point_count {
            None
        } else {
            self.pos += 1;
            unsafe {
                Some(Vector2f::from_raw(ffi::sfConvexShape_getPoint(
                    self.convex_shape,
                    self.pos as usize,
                )))
            }
        }
    }
}

impl<'s> Drop for ConvexShape<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfConvexShape_destroy(self.convex_shape) }
    }
}
