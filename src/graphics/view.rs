use crate::{
    ffi::graphics as ffi,
    graphics::FloatRect,
    sf_box::{Dispose, RawDefault},
    system::Vector2f,
    SfBox,
};
use std::borrow::ToOwned;

decl_opaque! {
/// 2D camera that defines what region is shown on screen
///
/// This is a very powerful concept: you can scroll,
/// rotate or zoom the entire scene without altering
/// the way that your drawable objects are drawn.
View;
}

impl View {
    /// Get the current orientation of a view
    ///
    /// Return the rotation angle of the view, in degrees
    #[must_use]
    pub fn rotation(&self) -> f32 {
        unsafe { ffi::sfView_getRotation(self.raw()) }
    }
    /// Get the center of a view
    ///
    /// Return the center of the view
    #[must_use]
    pub fn center(&self) -> Vector2f {
        unsafe { ffi::sfView_getCenter(self.raw()) }
    }

    /// Get the size of a view
    ///
    /// Return the size of the view
    #[must_use]
    pub fn size(&self) -> Vector2f {
        unsafe { ffi::sfView_getSize(self.raw()) }
    }

    /// Get the target viewport rectangle of a view
    ///
    /// Return the viewport rectangle, expressed as a factor of the target size
    #[must_use]
    pub fn viewport(&self) -> FloatRect {
        unsafe { ffi::sfView_getViewport(self.raw()) }
    }
    /// Creates a view with position and size
    ///
    /// # Arguments
    /// * center - The center of the view
    /// * size - The size of the view
    #[must_use]
    pub fn new(center: Vector2f, size: Vector2f) -> SfBox<View> {
        let mut view: SfBox<View> = Default::default();
        view.set_center(center);
        view.set_size(size);
        view
    }

    /// Construct a view from a rectangle
    ///
    /// # Arguments
    /// * rectangle - The rectangle defining the zone to display
    #[must_use]
    pub fn from_rect(rectangle: FloatRect) -> SfBox<View> {
        let view = unsafe { ffi::sfView_createFromRect(rectangle) };
        SfBox::new(view as *mut Self).expect("Failed to create View from Rect")
    }

    /// Set the orientation of a view
    ///
    /// The default rotation of a view is 0 degree.
    ///
    /// # Arguments
    /// * angle - New angle, in degrees
    pub fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfView_setRotation(self.raw_mut(), angle) }
    }

    /// Rotate a view relatively to its current orientation
    ///
    /// # Arguments
    /// * angle - Angle to rotate, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfView_rotate(self.raw_mut(), angle) }
    }

    /// Resize a view rectangle relatively to its current size
    ///
    /// Resizing the view simulates a zoom, as the zone displayed
    /// on screen grows or shrinks.
    ///
    /// # factor is a multiplier:
    /// * 1 keeps the size unchanged
    /// * bigger than 1 makes the view bigger (objects appear smaller)
    /// * smaller than 1 makes the view smaller (objects appear bigger)
    ///
    /// # Arguments
    /// * factor - Zoom factor to apply
    pub fn zoom(&mut self, factor: f32) {
        unsafe { ffi::sfView_zoom(self.raw_mut(), factor) }
    }

    /// Set the center of a view
    ///
    /// # Arguments
    /// * center - New center
    pub fn set_center<C: Into<Vector2f>>(&mut self, center: C) {
        unsafe { ffi::sfView_setCenter(self.raw_mut(), center.into()) }
    }

    /// Set the size of a view
    ///
    /// # Arguments
    /// * size - New size of the view
    pub fn set_size<S: Into<Vector2f>>(&mut self, size: S) {
        unsafe { ffi::sfView_setSize(self.raw_mut(), size.into()) }
    }

    /// Move a view relatively to its current position
    ///
    /// # Arguments
    /// * offset - Offset
    pub fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfView_move(self.raw_mut(), offset.into()) }
    }

    /// Set the target viewport of a view
    ///
    /// The viewport is the rectangle into which the contents of the
    /// view are displayed, expressed as a factor (between 0 and 1)
    /// of the size of the render target to which the view is applied.
    /// For example, a view which takes the left side of the target would
    /// be defined by a rect of (0, 0, 0.5, 1).
    /// By default, a view has a viewport which covers the entire target.
    ///
    /// # Arguments
    /// * viewport - New viewport rectangle
    pub fn set_viewport(&mut self, viewport: FloatRect) {
        unsafe { ffi::sfView_setViewport(self.raw_mut(), viewport) }
    }

    /// Reset a view to the given rectangle
    ///
    /// Note that this function resets the rotation angle to 0.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the zone to display
    pub fn reset(&mut self, rectangle: FloatRect) {
        unsafe { ffi::sfView_reset(self.raw_mut(), rectangle) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfView {
        let ptr: *const Self = self;
        ptr as _
    }
    fn raw_mut(&mut self) -> *mut ffi::sfView {
        let ptr: *mut Self = self;
        ptr as _
    }
}

impl ToOwned for View {
    type Owned = SfBox<Self>;
    fn to_owned(&self) -> Self::Owned {
        let view = unsafe { ffi::sfView_copy(self.raw()) };
        SfBox::new(view as *mut Self).expect("Failed to copy View")
    }
}

impl RawDefault for View {
    fn raw_default() -> *mut Self {
        let view = unsafe { ffi::sfView_create() };
        view as _
    }
}

impl Dispose for View {
    unsafe fn dispose(&mut self) {
        let ptr: *mut Self = self;
        ffi::sfView_destroy(ptr as _)
    }
}
