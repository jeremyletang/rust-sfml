/*!
* 2D camera that defines what region is shown on screen
*
* This is a very powerful concept: you can scroll, rotate or zoom the entire scene without altering the way that your drawable objects are drawn.
*
*/

use core::libc::{c_float};
use system::vector2;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_float, c_void};
    use system::vector2;

    pub struct sfView {
        This : *c_void
    }

    pub extern "C" {
        fn sfView_create() -> *sfView;
        //fn sfView_createFromRect(rectangle : sfFloatRect rectangle) -> *sfView;
        fn sfView_copy(view : *sfView) -> *sfView;
        fn sfView_destroy(view : *sfView) -> ();
        fn sfView_setCenter(view : *sfView, center : vector2::csfml::sfVector2f) -> ();
        fn sfView_setSize(view : *sfView, size : vector2::csfml::sfVector2f) -> ();
        fn sfView_setRotation(view : *sfView, angle : c_float) -> ();
        //fn sfView_setViewport(view : *sfView, viewport : sfFloatRect) -> ();
        //fn sfView_reset(view : *sfView, rectangle : sfFloatRect) -> ();
        fn sfView_getCenter(view : *sfView) -> vector2::csfml::sfVector2f;
        fn sfView_getSize(view : *sfView) -> vector2::csfml::sfVector2f;
        fn sfView_getRotation(view : *sfView) -> c_float;
        //fn sfView_getViewport(view : *sfView) -> sfFloatRect;
        fn sfView_move(view : *sfView, offset : vector2::csfml::sfVector2f) -> ();
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
    * Create a view by copyinh an existant one.
    */
    pub fn new_copy(view : &View) -> View {
        View { view : view.unwrap_view()}
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
            csfml::sfView_setCenter(self.view, vector2::unwrap_vector2f(*center))
        }
    }

    pub fn set_size(&self, size : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_setSize(self.view, vector2::unwrap_vector2f(*size))
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfView_move(self.view, vector2::unwrap_vector2f(*offset))
        }
    }

    pub fn get_center(&self) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe {csfml::sfView_getCenter(self.view)})
    }

    pub fn get_size(&self) -> vector2::Vector2f {
        vector2::wrap_vector2f(unsafe {csfml::sfView_getSize(self.view)})
    }

    #[doc(hidden)]
    pub fn wrap_view(view : *csfml::sfView) -> View {
        View { view : view }
    } 

    #[doc(hidden)]
    pub fn unwrap_view(&self) -> *csfml::sfView {
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