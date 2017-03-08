// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! 2D camera that defines what region is shown on screen
//!
//! This is a very powerful concept: you can scroll,
//! rotate or zoom the entire scene without altering
//! the way that your drawable objects are drawn.

use raw_conv::{Raw, FromRaw};
use graphics::FloatRect;
use system::Vector2f;
use std::ops::Deref;

use csfml_graphics_sys as ffi;
use csfml_system_sys::sfVector2f;

/// 2D camera that defines what region is shown on screen
///
/// This is a very powerful concept: you can scroll,
/// rotate or zoom the entire scene without altering
/// the way that your drawable objects are drawn.
pub struct View {
    view: *mut ffi::sfView,
}

impl Deref for View {
    type Target = ViewRef;

    fn deref(&self) -> &ViewRef {
        unsafe { &*(self.view as *const ViewRef) }
    }
}

/// A non-owning `View`.
pub enum ViewRef {}

impl ViewRef {
    /// Get the current orientation of a view
    ///
    /// Return the rotation angle of the view, in degrees
    pub fn get_rotation(&self) -> f32 {
        unsafe { ffi::sfView_getRotation(self as *const _ as _) as f32 }
    }
    /// Get the center of a view
    ///
    /// Return the center of the view
    pub fn get_center(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfView_getCenter(self as *const _ as _)) }
    }

    /// Get the size of a view
    ///
    /// Return the size of the view
    pub fn get_size(&self) -> Vector2f {
        unsafe { Vector2f::from_raw(ffi::sfView_getSize(self as *const _ as _)) }
    }

    /// Get the target viewport rectangle of a view
    ///
    /// Return the viewport rectangle, expressed as a factor of the target size
    pub fn get_viewport(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(ffi::sfView_getViewport(self as *const _ as _)) }
    }
}

impl View {
    /// Create a default view
    ///
    /// This function creates a default view of (0, 0, 1000, 1000)
    pub fn new() -> View {
        let view = unsafe { ffi::sfView_create() };
        if view.is_null() {
            panic!("sfView_create returned null.")
        } else {
            View { view: view }
        }
    }

    /// Create a default view
    ///
    /// This function creates a default view with initialized position and size
    ///
    /// # Arguments
    /// * center - The center of the view
    /// * size - The size of the view
    pub fn new_init(center: &Vector2f, size: &Vector2f) -> View {
        let view = unsafe { ffi::sfView_create() };
        if view.is_null() {
            panic!("sfView_create returned null.")
        } else {
            unsafe {
                ffi::sfView_setCenter(view, center.raw());
                ffi::sfView_setSize(view, size.raw());
            }
            View { view: view }
        }
    }

    /// Construct a view from a rectangle
    ///
    /// # Arguments
    /// * rectangle - The rectangle defining the zone to display
    pub fn from_rect(rectangle: &FloatRect) -> View {
        let view = unsafe { ffi::sfView_createFromRect(rectangle.raw()) };
        if view.is_null() {
            panic!("sfView_createFromRect returned null.")
        } else {
            View { view: view }
        }
    }

    /// Set the orientation of a view
    ///
    /// The default rotation of a view is 0 degree.
    ///
    /// # Arguments
    /// * angle - New angle, in degrees
    pub fn set_rotation(&mut self, angle: f32) {
        unsafe { ffi::sfView_setRotation(self.view, angle) }
    }

    /// Rotate a view relatively to its current orientation
    ///
    /// # Arguments
    /// * angle - Angle to rotate, in degrees
    pub fn rotate(&mut self, angle: f32) {
        unsafe { ffi::sfView_rotate(self.view, angle) }
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
        unsafe { ffi::sfView_zoom(self.view, factor) }
    }

    /// Set the center of a view
    ///
    /// # Arguments
    /// * center - New center
    pub fn set_center(&mut self, center: &Vector2f) {
        unsafe { ffi::sfView_setCenter(self.view, center.raw()) }
    }

    /// Set the center of a view
    ///
    /// # Arguments
    /// * center_x - New x center coordinate
    /// * center_y - New y center coordinate
    ///
    pub fn set_center2f(&mut self, center_x: f32, center_y: f32) {
        unsafe {
            ffi::sfView_setCenter(self.view,
                                  sfVector2f {
                                      x: center_x,
                                      y: center_y,
                                  })
        }
    }

    /// Set the size of a view
    ///
    /// # Arguments
    /// * size - New size of the view
    pub fn set_size(&mut self, size: &Vector2f) {
        unsafe { ffi::sfView_setSize(self.view, size.raw()) }
    }

    /// Set the size of a view
    ///
    /// # Arguments
    /// * size_x - New size x of the view
    /// * size_y - New size y of the view
    pub fn set_size2f(&mut self, size_x: f32, size_y: f32) {
        unsafe {
            ffi::sfView_setSize(self.view,
                                sfVector2f {
                                    x: size_x,
                                    y: size_y,
                                })
        }
    }

    /// Move a view relatively to its current position
    ///
    /// # Arguments
    /// * offset - Offset
    pub fn move_(&mut self, offset: &Vector2f) {
        unsafe { ffi::sfView_move(self.view, offset.raw()) }
    }
    /// Move a view relatively to its current position
    ///
    /// # Arguments
    /// * offsetX - Offset x
    /// * offsetY - Offset y
    pub fn move2f(&mut self, offset_x: f32, offset_y: f32) {
        unsafe {
            ffi::sfView_move(self.view,
                             sfVector2f {
                                 x: offset_x,
                                 y: offset_y,
                             })
        }
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
    pub fn set_viewport(&mut self, viewport: &FloatRect) {
        unsafe { ffi::sfView_setViewport(self.view, viewport.raw()) }
    }

    /// Reset a view to the given rectangle
    ///
    /// Note that this function resets the rotation angle to 0.
    ///
    /// # Arguments
    /// * rectangle - Rectangle defining the zone to display
    pub fn reset(&mut self, rectangle: &FloatRect) {
        unsafe { ffi::sfView_reset(self.view, rectangle.raw()) }
    }
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for View {
    /// Return a new View or panic! if there is not enough memory
    fn clone(&self) -> View {
        let view = unsafe { ffi::sfView_copy(self.view) };
        if view.is_null() {
            panic!("Not enough memory to clone View")
        } else {
            View { view: view }
        }
    }
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe { ffi::sfView_destroy(self.view) }
    }
}

impl Raw for View {
    type Raw = *mut ffi::sfView;
    fn raw(&self) -> Self::Raw {
        self.view
    }
}

impl FromRaw for View {
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        View { view: raw }
    }
}
