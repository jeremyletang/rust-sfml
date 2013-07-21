/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* 2D camera that defines what region is shown on screen
*
* This is a very powerful concept: you can scroll, rotate or zoom the entire scene without altering the way that your drawable objects are drawn.
*
*/

use std::libc::{c_float};
use std::ptr;

use traits::wrappable::Wrappable;
use system::vector2::Vector2f;
use graphics::rect::FloatRect;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_float, c_void};

    use system::vector2::Vector2f;
    use graphics::rect::FloatRect;

    pub struct sfView {
        This : *c_void
    }

    extern "C" {
        pub fn sfView_create() -> *sfView;
        pub fn sfView_createFromRect(rectangle : FloatRect) -> *sfView;
        pub fn sfView_copy(view : *sfView) -> *sfView;
        pub fn sfView_destroy(view : *sfView) -> ();
        pub fn sfView_setCenter(view : *sfView, center : Vector2f) -> ();
        pub fn sfView_setSize(view : *sfView, size : Vector2f) -> ();
        pub fn sfView_setRotation(view : *sfView, angle : c_float) -> ();
        pub fn sfView_setViewport(view : *sfView, viewport : FloatRect) -> ();
        pub fn sfView_reset(view : *sfView, rectangle : FloatRect) -> ();
        pub fn sfView_getCenter(view : *sfView) -> Vector2f;
        pub fn sfView_getSize(view : *sfView) -> Vector2f;
        pub fn sfView_getRotation(view : *sfView) -> c_float;
        pub fn sfView_getViewport(view : *sfView) -> FloatRect;
        pub fn sfView_move(view : *sfView, offset : Vector2f) -> ();
        pub fn sfView_rotate(view : *sfView, angle : c_float) -> ();
        pub fn sfView_zoom(view : *sfView, factor : c_float) -> ();
    }
}

#[doc(hidden)]
pub struct View {
    priv dropable : bool,
    priv view : *ffi::sfView
}

impl View {
    /**
    * Create a default view
    *
    * This function creates a default view of (0, 0, 1000, 1000)
    * 
    * Return a new option to View object
    */
    pub fn new() -> Option<View> {
        let view = unsafe { ffi::sfView_create() };
        if ptr::is_null(view) {
            None
        }
        else {
            Some(View {
                dropable: true,
                view : view
            })
        }
    }

    /**
    * Create a default view
    *
    * This function creates a default view with initialized position and size
    * 
    * # Arguments
    * * center - The center of the view
    * * size - The size of the view
    *
    * Return a new option to View object
    */
    pub fn new_init(center : &Vector2f, size : &Vector2f) -> Option<View> {
        let view = unsafe { ffi::sfView_create() };
        if ptr::is_null(view) {
            None
        }
        else {
            unsafe {
                ffi::sfView_setCenter(view, *center);
                ffi::sfView_setSize(view, *size);
            }
            Some(View {
                dropable : true,
                view : view
            })
        }
    }
    
    /**
    * Create a view by copying an existant one.
    *
    * Return a new option to View object
    */
    pub fn new_copy(&self) -> Option<View> {
        let view = unsafe { ffi::sfView_copy(self.view) };
        if ptr::is_null(view) {
            None
        }
        else {
            Some(View {
                dropable: true,
                view : view
            })
        }    
    }

    /**
    * Construct a view from a rectangle
    *
    * # Arguments
    * * rectangle - The rectangle defining the zone to display
    *
    * Return a new View object
    */
    pub fn new_from_rect(rectangle : *FloatRect) -> Option<View> {
        let view = unsafe { ffi::sfView_createFromRect(*rectangle) };
        if ptr::is_null(view) {
            None
        }
        else {
            Some(View {
                dropable: true,
                view : view
            })
        }
    }

    /**
    * Set the orientation of a view
    *
    * The default rotation of a view is 0 degree.
    *
    * # Arguments
    * * angle - New angle, in degrees
    */
    pub fn set_rotation(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfView_setRotation(self.view, angle as c_float)
        }
    }
    
    /**
    * Get the current orientation of a view
    *
    * Return the rotation angle of the view, in degrees
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            ffi::sfView_getRotation(self.view) as float
        }
    }

    /**
    * Rotate a view relatively to its current orientation
    *
    * # Arguments
    * * angle - Angle to rotate, in degrees
    */
    pub fn rotate(&mut self, angle : float) -> () {
        unsafe {
            ffi::sfView_rotate(self.view, angle as c_float)
        }
    }

    /**
    * Resize a view rectangle relatively to its current size
    *
    * Resizing the view simulates a zoom, as the zone displayed on screen grows or shrinks.
    * # factor is a multiplier:
    * * 1 keeps the size unchanged
    * * bigger than 1 makes the view bigger (objects appear smaller)
    * * smaller than 1 makes the view smaller (objects appear bigger)
    *
    * # Arguments
    * * factor - Zoom factor to apply
    */
    pub fn zoom(&mut self, factor : float) -> () {
        unsafe {
            ffi::sfView_zoom(self.view, factor as c_float)
        }
    }

    /**
    * Set the center of a view
    *
    * # Arguments
    * * center - New center
    */
    pub fn set_center(&mut self, center : &Vector2f) -> () {
        unsafe {
            ffi::sfView_setCenter(self.view, *center)
        }
    }

    /**
    * Set the center of a view
    *
    * # Arguments
    * * centerX - New x center coordinate
    * * centerY - New y center coordinate
    */
    pub fn set_center2f(&mut self, centerX : f32, centerY : f32) -> () {
        unsafe {
            ffi::sfView_setCenter(self.view, Vector2f::new(centerX, centerY))
        }
    }

    /**
    * Set the size of a view
    *
    * # Arguments
    * * size - New size of the view
    */
    pub fn set_size(&mut self, size : &Vector2f) -> () {
        unsafe {
            ffi::sfView_setSize(self.view, *size)
        }
    }

    /**
    * Set the size of a view
    *
    * # Arguments
    * * sizeX - New size x of the view
    * * sizeY - New size y of the view
    */
    pub fn set_size2f(&mut self, sizeX : f32, sizeY : f32) -> () {
        unsafe {
            ffi::sfView_setSize(self.view, Vector2f::new(sizeX, sizeY))
        }
    }

    /**
    * Move a view relatively to its current position
    *
    * # Arguments
    * * offset - Offset
    */
    pub fn move(&mut self, offset : &Vector2f) -> () {
        unsafe {
            ffi::sfView_move(self.view, *offset)
        }
    }
    /**
    * Move a view relatively to its current position
    *
    * # Arguments
    * * offsetX - Offset x
    * * offsetY - Offset y
    */
    pub fn move2f(&mut self, offsetX : f32, offsetY : f32) -> () {
        unsafe {
            ffi::sfView_move(self.view, Vector2f::new(offsetX, offsetY))
        }
    }

    /**
    * Get the center of a view
    *
    * Return the center of the view
    */
    pub fn get_center(&self) -> Vector2f {
        unsafe {ffi::sfView_getCenter(self.view)}
    }

    /**
    * Get the size of a view
    *
    * Return the size of the view
    */
    pub fn get_size(&self) -> Vector2f {
        unsafe {
            ffi::sfView_getSize(self.view)
        }
    }

    /**
    * Set the target viewport of a view
    *
    * The viewport is the rectangle into which the contents of the
    * view are displayed, expressed as a factor (between 0 and 1)
    * of the size of the render target to which the view is applied.
    * For example, a view which takes the left side of the target would
    * be defined by a rect of (0, 0, 0.5, 1).
    * By default, a view has a viewport which covers the entire target.
    *
    * # Arguments
    * * viewport - New viewport rectangle
    */
    pub fn set_viewport(&mut self, viewport : &FloatRect) -> () {
        unsafe {
            ffi::sfView_setViewport(self.view, *viewport)
        }
    }

    /**
    * Reset a view to the given rectangle
    *
    * Note that this function resets the rotation angle to 0.
    *
    * # Arguments
    * * rectangle - Rectangle defining the zone to display
    */
    pub fn reset(&mut self, rectangle : &FloatRect) -> () {
        unsafe {
            ffi::sfView_reset(self.view, *rectangle)
        }
    }

    /**
    * Get the target viewport rectangle of a view
    * 
    * Return the viewport rectangle, expressed as a factor of the target size
    */
    pub fn get_viewport(&self) -> FloatRect {
        unsafe {
            ffi::sfView_getViewport(self.view)
        }
    }
}

#[doc(hidden)]
impl Wrappable<*ffi::sfView> for View {
    pub fn wrap(view : *ffi::sfView) -> View {
        View { 
            dropable: false, 
            view : view
        }
    } 

    pub fn unwrap(&self) -> *ffi::sfView {
        self.view
    }
}

impl Drop for View {
    /// Destructor for class View
    fn drop(&self) -> () {
      if self.dropable
      {
        unsafe {
          ffi::sfView_destroy(self.view)
        }
      }
    }
}
