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

use system::vector2;
use graphics::rect::FloatRect;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_float, c_void};

    use system::vector2;
    use graphics::rect::FloatRect;

    pub struct sfView {
        This : *c_void
    }

    pub extern "C" {
        fn sfView_create() -> *sfView;
        fn sfView_createFromRect(rectangle : FloatRect) -> *sfView;
        fn sfView_copy(view : *sfView) -> *sfView;
        fn sfView_destroy(view : *sfView) -> ();
        fn sfView_setCenter(view : *sfView, center : vector2::Vector2f) -> ();
        fn sfView_setSize(view : *sfView, size : vector2::Vector2f) -> ();
        fn sfView_setRotation(view : *sfView, angle : c_float) -> ();
        fn sfView_setViewport(view : *sfView, viewport : FloatRect) -> ();
        fn sfView_reset(view : *sfView, rectangle : FloatRect) -> ();
        fn sfView_getCenter(view : *sfView) -> vector2::Vector2f;
        fn sfView_getSize(view : *sfView) -> vector2::Vector2f;
        fn sfView_getRotation(view : *sfView) -> c_float;
        fn sfView_getViewport(view : *sfView) -> FloatRect;
        fn sfView_move(view : *sfView, offset : vector2::Vector2f) -> ();
        fn sfView_rotate(view : *sfView, angle : c_float) -> ();
        fn sfView_zoom(view : *sfView, factor : c_float) -> ();
    }
}

#[doc(hidden)]
pub struct View {
    priv dropable : bool,
    priv view : *csfml::sfView
}

impl View {
    /**
    * Create a default view
    *
    * This function creates a default view of (0, 0, 1000, 1000)
    * 
    * Return a new View object
    */
    pub fn new() -> View {
        View { dropable: true, view : unsafe {csfml::sfView_create()} }
    }

    /**
    * Construct a view from a rectangle
    *
    * # Arguments
    * * rectangle - The rectangle defining the zone to display
    *
    * Return a new View object
    */
    pub fn create_from_rect(rectangle : *FloatRect) -> View {
        View { dropable: true, view : unsafe {csfml::sfView_createFromRect(*rectangle)}}
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
            csfml::sfView_setRotation(self.view, angle as c_float)
        }
    }
    
    /**
    * Get the current orientation of a view
    *
    * Return the rotation angle of the view, in degrees
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfView_getRotation(self.view) as float
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
            csfml::sfView_rotate(self.view, angle as c_float)
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
            csfml::sfView_zoom(self.view, factor as c_float)
        }
    }

    /**
    * Set the center of a view
    *
    * # Arguments
    * * center - New center
    */
    pub fn set_center(&mut self, center : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_setCenter(self.view, *center)
        }
    }

    /**
    * Set the size of a view
    *
    * # Arguments
    * * size - New size of the view
    */
    pub fn set_size(&mut self, size : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_setSize(self.view, *size)
        }
    }

    /**
    * Move a view relatively to its current position
    *
    * # Arguments
    * * offset - Offset
    */
    pub fn move(&mut self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_move(self.view, *offset)
        }
    }

    /**
    * Get the center of a view
    *
    * Return the center of the view
    */
    pub fn get_center(&self) -> vector2::Vector2f {
        unsafe {csfml::sfView_getCenter(self.view)}
    }

    /**
    * Get the size of a view
    *
    * Return the size of the view
    */
    pub fn get_size(&self) -> vector2::Vector2f {
        unsafe {csfml::sfView_getSize(self.view)}
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
            csfml::sfView_setViewport(self.view, *viewport)
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
    pub fn reset(&self, rectangle : &FloatRect) -> () {
        unsafe {
            csfml::sfView_reset(self.view, *rectangle)
        }
    }

    /**
    * Get the target viewport rectangle of a view
    * 
    * Return the viewport rectangle, expressed as a factor of the target size
    */
    pub fn get_viewport(&self) -> FloatRect {
        unsafe {
            csfml::sfView_getViewport(self.view)
        }
    }

    #[doc(hidden)]
    pub fn wrap(view : *csfml::sfView) -> View {
        View { dropable: false, view : view }
    } 

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfView {
        self.view
    }
        
}

impl Clone for View {
    fn clone(&self) -> View {
        unsafe {
            View {
                dropable: self.dropable,
                view: csfml::sfView_copy(self.view)
            }
        }
    }
}


impl Drop for View {
    /// Destructor for class View
    fn finalize(&self) -> () {
      if self.dropable
      {
        unsafe {
          csfml::sfView_destroy(self.view)
        }
      }
    }
}
