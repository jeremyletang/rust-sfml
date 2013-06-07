/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
    priv view : *csfml::sfView
}

impl View {
    /**
    * Create a default view
    */
    pub fn new() -> View {
        View { view : unsafe {csfml::sfView_create()} }
    }
    
    /**
    * Create a view by copying an existant one.
    */
    pub fn new_copy(view : &View) -> View {
        View { view : unsafe {csfml::sfView_copy(view.unwrap())}}
    }

    pub fn create_from_rect(rectangle : *FloatRect) -> View {
        View { view : unsafe {csfml::sfView_createFromRect(*rectangle)}}
    }

    /**
    * Set the orientation of a view
    */
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfView_setRotation(self.view, angle as c_float)
        }
    }
    
    /**
    * Get the current orientation of a view
    */
    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfView_getRotation(self.view) as float
        }
    }

    /**
    * Rotate a view relatively to its current orientation
    */
    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfView_rotate(self.view, angle as c_float)
        }
    }

    /**
    * Resize a view rectangle relatively to its current size
    */
    pub fn zoom(&self, factor : float) -> () {
        unsafe {
            csfml::sfView_zoom(self.view, factor as c_float)
        }
    }

    pub fn set_center(&self, center : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_setCenter(self.view, *center)
        }
    }

    pub fn set_size(&self, size : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_setSize(self.view, *size)
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_move(self.view, *offset)
        }
    }

    pub fn get_center(&self) -> vector2::Vector2f {
        unsafe {csfml::sfView_getCenter(self.view)}
    }

    pub fn get_size(&self) -> vector2::Vector2f {
        unsafe {csfml::sfView_getSize(self.view)}
    }

    pub fn set_viewport(&self, viewport : &FloatRect) -> () {
        unsafe {
            csfml::sfView_setViewport(self.view, *viewport)
        }
    }

    pub fn reset(&self, rectangle : &FloatRect) -> () {
        unsafe {
            csfml::sfView_reset(self.view, *rectangle)
        }
    }

    pub fn get_viewport(&self) -> FloatRect {
        unsafe {
            csfml::sfView_getViewport(self.view)
        }
    }

    #[doc(hidden)]
    pub fn wrap(view : *csfml::sfView) -> View {
        View { view : view }
    } 

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfView {
        self.view
    }
        
}

impl Drop for View {
    
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfView_destroy(self.view)
        }
    }
}