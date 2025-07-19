use {
    crate::{
        IntoSfResult, SfResult,
        cpp::{FBox, RawDefault},
        ffi::graphics as ffi,
        graphics::FloatRect,
        system::{Angle, Vector2f},
    },
    std::ptr::NonNull,
};

decl_opaque! {
/// 2D camera that defines what region is shown on screen
///
/// This is a very powerful concept: you can scroll,
/// rotate or zoom the entire scene without altering
/// the way that your drawable objects are drawn.
pub View;
}

/// Creation
impl View {
    /// Creates a default `View` of (0, 0, 1000, 1000)
    pub fn new() -> SfResult<FBox<Self>> {
        FBox::new(unsafe { ffi::sfView_new() }).into_sf_result()
    }
    /// Creates a view with position and size
    ///
    /// # Arguments
    /// * center - The center of the view
    /// * size - The size of the view
    #[must_use]
    pub fn with_center_and_size(center: Vector2f, size: Vector2f) -> FBox<View> {
        let mut view: FBox<View> = Default::default();
        view.set_center(center);
        view.set_size(size);
        view
    }

    /// Construct a view from a rectangle
    ///
    /// # Arguments
    /// * rectangle - The rectangle defining the zone to display
    #[must_use]
    pub fn from_rect(rectangle: FloatRect) -> FBox<View> {
        let mut view: FBox<View> = Default::default();
        view.set_center(rectangle.center());
        view.set_size(rectangle.size);

        view
    }
}

/// Query properties
impl View {
    /// Get the current orientation of a view
    ///
    /// Return the rotation angle of the view, in degrees
    #[must_use]
    pub fn rotation(&self) -> f32 {
        unsafe { ffi::sfView_getRotation(self) }
    }
    /// Get the center of a view
    ///
    /// Return the center of the view
    #[must_use]
    pub fn center(&self) -> Vector2f {
        unsafe { ffi::sfView_getCenter(self) }
    }

    /// Get the size of a view
    ///
    /// Return the size of the view
    #[must_use]
    pub fn size(&self) -> Vector2f {
        unsafe { ffi::sfView_getSize(self) }
    }

    /// Get the target viewport rectangle of a view
    ///
    /// Return the viewport rectangle, expressed as a factor of the target size
    #[must_use]
    pub fn viewport(&self) -> FloatRect {
        unsafe { ffi::sfView_getViewport(self) }
    }

    /// Get the scissor rectangle of the view
    ///
    /// Return the scissor rectangle, expressed as a factor of the target size
    #[must_use]
    pub fn scissor(&self) -> FloatRect {
        unsafe { ffi::sfView_getScissor(self) }
    }
}

/// Set properties
impl View {
    /// Set the orientation of a view
    ///
    /// The default rotation of a view is 0 degree.
    ///
    /// # Arguments
    /// * angle - New angle
    pub fn set_rotation(&mut self, angle: Angle) {
        unsafe { ffi::sfView_setRotation(self, angle.as_degrees()) }
    }

    /// Rotate a view relatively to its current orientation
    ///
    /// # Arguments
    /// * angle - Angle to rotate
    pub fn rotate(&mut self, angle: Angle) {
        unsafe { ffi::sfView_rotate(self, angle.as_degrees()) }
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
        unsafe { ffi::sfView_zoom(self, factor) }
    }

    /// Set the center of a view
    ///
    /// # Arguments
    /// * center - New center
    pub fn set_center<C: Into<Vector2f>>(&mut self, center: C) {
        unsafe { ffi::sfView_setCenter(self, center.into()) }
    }

    /// Set the size of a view
    ///
    /// # Arguments
    /// * size - New size of the view
    pub fn set_size<S: Into<Vector2f>>(&mut self, size: S) {
        unsafe { ffi::sfView_setSize(self, size.into()) }
    }

    /// Move a view relatively to its current position
    ///
    /// # Arguments
    /// * offset - Offset
    pub fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        unsafe { ffi::sfView_move(self, offset.into()) }
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
        unsafe { ffi::sfView_setViewport(self, viewport) }
    }

    /// Set the target scissor rectangle
    ///
    /// The scissor rectangle, expressed as a factor (between 0 and 1) of
    /// the `RenderTarget`, specifies the region of the `RenderTarget` whose
    /// pixels are able to be modified by draw or clear operations.
    /// Any pixels which lie outside of the scissor rectangle will
    /// not be modified by draw or clear operations.
    /// For example, a scissor rectangle which only allows modifications
    /// to the right side of the target would be defined
    /// with `sfView_setScissor(view, (sfFloatRect){{0.5f, 0.f}, {0.5f, 1.f}})`.
    /// By default, a view has a scissor rectangle which allows
    /// modifications to the entire target. This is equivalent to
    /// disabling the scissor test entirely. Passing the default
    /// scissor rectangle to this function will also disable
    /// scissor testing.
    ///
    /// # Arguments
    /// * view - View object
    /// * scissor - New scissor rectangle
    pub fn set_scissor(&mut self, scissor: FloatRect) {
        unsafe { ffi::sfView_setScissor(self, scissor) }
    }
}

impl ToOwned for View {
    type Owned = FBox<Self>;
    fn to_owned(&self) -> Self::Owned {
        let view = unsafe { ffi::sfView_cpy(self) };
        FBox::new(view).expect("Failed to copy View")
    }
}

impl RawDefault for View {
    fn raw_default() -> NonNull<Self> {
        NonNull::new(unsafe { ffi::sfView_new() }).expect("Failed to create view")
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe { ffi::sfView_del(self) }
    }
}
