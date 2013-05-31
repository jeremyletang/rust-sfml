/*!
* Define a 3x3 transform matrix.
*
* A Transform specifies how to translate, rotate, scale, shear, project, whatever things.
*
*/

use core::libc::{c_float};

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_float};

    pub struct sfTransform {
        a1 : i32,
        a2 : i32,
        a3 : i32,
        a4 : i32,
        a5 : i32,
        a6 : i32,
        a7 : i32,
        a8 : i32,
        a9 : i32
    }
    
    pub extern "C" {
        fn sfTransform_fromMatrix(a01 : f32, a02 : f32, a03 : f32, b01 : f32, b02 : f32, b03 : f32, c01 : f32, c02 : f32, c03 : f32) -> sfTransform;
        fn sfTransform_getMatrix(tranform : *sfTransform, matrix : *c_float) -> ();
        fn sfTransform_getInverse(transform : *sfTransform) -> sfTransform;
        //fn sfTransform_transformPoint(transform : *sfTransform, point : sfVector2f) -> vector2::csfml::sfVector2f;
        //fn sfTransform_transformRect(transform : *sfTransform, rectangle : sfFloatRect) -> sfFloatRect;
        fn sfTransform_combine(transform : *sfTransform, other : *sfTransform) -> ();
        fn sfTransform_translate(transform : *sfTransform, x : c_float, y : c_float) -> ();
        fn sfTransform_rotate(transform : *sfTransform, angle : c_float) -> ();
        fn sfTransform_rotateWithCenter(transform : *sfTransform, angle : c_float, centerX : c_float, centerY : c_float) -> ();
        fn sfTransform_scale(transform : *sfTransform, scaleX : c_float, scaleY : c_float) -> ();
        fn sfTransform_scaleWithCenter(transform: *sfTransform, scaleX : c_float, scaleY : c_float, centerX : c_float, centerY : c_float) -> ();
    }
}

#[doc(hidden)]
pub struct Transform {
    transform : csfml::sfTransform
}

impl Transform {
    /**
    * Create a new transform from a matrix
    */
    pub fn new(a01 : f32, a02 : f32, a03 : f32, b01 : f32, b02 : f32, b03 : f32, c01 : f32, c02 : f32, c03 : f32) -> Transform {
        unsafe {
            Transform { transform : csfml::sfTransform_fromMatrix(a01, a02, a03, b01, b02, b03, c01, c02, c03)}
        }
    }
    
    /**
    * Create a new identity transform
    */
    pub fn new_identity() -> Transform {
        unsafe {
            Transform { transform : csfml::sfTransform_fromMatrix(1., 0., 0., 0., 1., 0., 0., 0., 1.)}
        }
    }
    /*
    pub fn get_matrix(&self) -> ~[f32] {

    
            let mut i : size_t = 0;
        let mut ret_tab : ~[f32, ..16] = ~[];
        unsafe {
            let mut tab : *f32 = csfml::sfVideoMode_getFullscreenModes(&i) as *mut csfml::sfVideoMode;
            let mut cvec = CVec(tab, 16);
            let mut d : uint = 0;
            ret_tab.push(VideoMode::wrap_videoMode(get(cvec, d)));
            d += 1;
            while d != 16 {
                ret_tab.push(get(cvec, d));
                d += 1;
            }
        }
        ret_tab
    }
    */

    /**
    * Return the inverse of a transform
    */
    pub fn get_inverse(&self) -> Transform {
        Transform {transform : unsafe {csfml::sfTransform_getInverse(&self.transform)}}
    }
    
    /**
    * Combine two transforms
    */
    pub fn combine(&self, other : &Transform) -> () {
        unsafe {
            csfml::sfTransform_combine(&self.transform, &other.transform)
        }
    }
    
    /**
    * Combine a transform with a translation
    */
    pub fn translate(&self, x : f32, y : f32) -> () {
        unsafe {
            csfml::sfTransform_translate(&self.transform, x as c_float, y as c_float)
        }
    }

    /**
    * Combine the current transform with a rotation
    */
    pub fn rotate(&self, angle : f32) -> () {
        unsafe {
            csfml::sfTransform_rotate(&self.transform, angle as c_float)
        }
    }
    
    /**
    * Combine the current transform with a rotation
    */
    pub fn rotate_with_center(&self, angle : f32, centerX : f32, centerY : f32) -> () {
        unsafe {
            csfml::sfTransform_rotateWithCenter(&self.transform, angle as c_float, centerX as c_float, centerY as c_float)
        }
    }

    /**
    * Combine the current transform with a scaling
    */
    pub fn scale(&self, scaleX : f32, scaleY : f32) -> () {
        unsafe {
            csfml::sfTransform_scale(&self.transform, scaleX as c_float, scaleY as c_float)
        }
    }
    
    /**
    * Combine the current transform with a scaling
    */
    pub fn scale_with_center(&self, scaleX : f32, scaleY : f32, centerX : f32, centerY : f32) -> () {
        unsafe {

            csfml::sfTransform_scaleWithCenter(&self.transform, scaleX, scaleY, centerX, centerY)
        }
    }
}