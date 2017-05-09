use csfml_system_sys::{sfBool, sfTrue, sfVector2f};
use graphics::{Color, Drawable, FloatRect, IntRect, RenderStates, RenderTarget, Shape, TextureRef,
               Transform, Transformable};
use graphics::csfml_graphics_sys as ffi;
use sf_bool_ext::SfBoolExt;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::ptr;
use system::Vector2f;

/// The points of a custom shape.
pub trait CustomShapePoints {
    /// Gets the total count of points.
    ///
    /// Return the points count
    fn point_count(&self) -> u32;

    /// Gets a given point.
    ///
    /// # Argument
    /// * point - The index of the point to return
    ///
    /// Returns a `Vector2f` containing the coordinates.
    fn point(&self, point: u32) -> Vector2f;
}

/// A custom textured shape with outline.
#[derive(Debug)]
pub struct CustomShape<'s> {
    shape: *mut ffi::sfShape,
    texture: PhantomData<&'s TextureRef>,
    points: *mut Box<CustomShapePoints + Send>,
}

unsafe extern "C" fn get_point_count_callback(obj: *mut c_void) -> usize {
    let shape = obj as *mut Box<CustomShapePoints + Send>;
    let ret = (*shape).point_count();
    ret as usize
}

unsafe extern "C" fn get_point_callback(point: usize, obj: *mut c_void) -> sfVector2f {
    let shape = obj as *mut Box<CustomShapePoints + Send>;
    let ret = (*shape).point(point as u32);
    ret.raw()
}


impl<'s> CustomShape<'s> {
    /// Create a new CustomShape
    ///
    /// # Arguments
    /// * points - Implementation of CustomShapePoints
    pub fn new(points: Box<CustomShapePoints + Send>) -> CustomShape<'s> {
        let raw_impl = Box::into_raw(Box::new(points));
        let sp = unsafe {
            ffi::sfShape_create(Some(get_point_count_callback),
                                Some(get_point_callback),
                                raw_impl as *mut _)
        };
        assert!(!sp.is_null(), "Failed to create CustomShape");
        CustomShape {
            shape: sp,
            texture: PhantomData,
            points: raw_impl,
        }
    }

    /// Create a new CustomShape with a texture
    ///
    /// # Arguments
    /// * points - Implementation of CustomShapePoints trait
    /// * texture - The texture to bind to the CustomShape
    pub fn with_texture(points: Box<CustomShapePoints + Send>,
                        texture: &'s TextureRef)
                        -> CustomShape<'s> {
        let mut shape = Self::new(points);
        shape.set_texture(texture, true);
        shape
    }

    /// Recompute the internal geometry of a shape
    ///
    /// This function must be called by specialized shape objects
    /// everytime their points change (ie. the result of either
    /// the getPointCount or getPoint callbacks is different).
    pub fn update(&mut self) {
        unsafe { ffi::sfShape_update(self.shape) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfShape {
        self.shape
    }
}

impl<'s> Shape<'s> for CustomShape<'s> {
    fn set_texture(&mut self, texture: &'s TextureRef, reset_rect: bool) {
        unsafe { ffi::sfShape_setTexture(self.shape, texture.raw(), sfBool::from_bool(reset_rect)) }
    }
    fn disable_texture(&mut self) {
        unsafe { ffi::sfShape_setTexture(self.shape, ptr::null_mut(), sfTrue) }
    }
    fn set_texture_rect(&mut self, rect: &IntRect) {
        unsafe { ffi::sfShape_setTextureRect(self.shape, rect.raw()) }
    }
    fn set_fill_color(&mut self, color: &Color) {
        unsafe { ffi::sfShape_setFillColor(self.shape, color.raw()) }
    }
    fn set_outline_color(&mut self, color: &Color) {
        unsafe { ffi::sfShape_setOutlineColor(self.shape, color.raw()) }
    }
    fn set_outline_thickness(&mut self, thickness: f32) {
        unsafe { ffi::sfShape_setOutlineThickness(self.shape, thickness) }
    }
    fn texture(&self) -> Option<&'s TextureRef> {
        unsafe {
            let raw = ffi::sfShape_getTexture(self.shape);

            if raw.is_null() {
                None
            } else {
                Some(&*(raw as *const TextureRef))
            }
        }
    }
    fn texture_rect(&self) -> IntRect {
        unsafe { IntRect::from_raw(ffi::sfShape_getTextureRect(self.shape)) }
    }
    fn fill_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfShape_getFillColor(self.shape)) }
    }
    fn outline_color(&self) -> Color {
        unsafe { Color::from_raw(ffi::sfShape_getOutlineColor(self.shape)) }
    }
    fn outline_thickness(&self) -> f32 {
        unsafe { ffi::sfShape_getOutlineThickness(self.shape) }
    }
    fn point_count(&self) -> u32 {
        unsafe { ffi::sfShape_getPointCount(self.shape) as u32 }
    }
    fn point(&self, index: u32) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getPoint(self.shape, index as usize)) }
    }
    fn local_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfShape_getLocalBounds(self.shape)) }
    }
    fn global_bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfShape_getGlobalBounds(self.shape)) }
    }
}

impl<'s> Drawable for CustomShape<'s> {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_shape(self, states)
    }
}

impl<'s> Transformable for CustomShape<'s> {
    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        unsafe { ffi::sfShape_setPosition(self.shape, position.into().raw()) }
    }
    fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfShape_setRotation(self.shape, angle) }
    }
    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        unsafe { ffi::sfShape_setScale(self.shape, scale.into().raw()) }
    }
    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        unsafe { ffi::sfShape_setOrigin(self.shape, origin.into().raw()) }
    }
    fn position(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getPosition(self.shape)) }
    }
    fn rotation(&self) -> f32 {
        unsafe { ffi::sfShape_getRotation(self.shape) }
    }
    fn get_scale(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getScale(self.shape)) }
    }
    fn origin(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfShape_getOrigin(self.shape)) }
    }
    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfShape_move(self.shape, offset.into().raw()) }
    }
    fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfShape_rotate(self.shape, angle) }
    }
    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        unsafe { ffi::sfShape_scale(self.shape, factors.into().raw()) }
    }
    fn transform(&self) -> Transform {
        unsafe { Transform(ffi::sfShape_getTransform(self.shape)) }
    }
    fn inverse_transform(&self) -> Transform {
        unsafe { Transform(ffi::sfShape_getInverseTransform(self.shape)) }
    }
}

impl<'s> Drop for CustomShape<'s> {
    fn drop(&mut self) {
        unsafe {
            ffi::sfShape_destroy(self.shape);
            let _ = Box::from_raw(self.points);
        }
    }
}
