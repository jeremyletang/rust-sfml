/*!
* Define a 3x3 transform matrix.
*
* A Transform specifies how to translate, rotate, scale, shear, project, whatever things.
*
*/

extern mod std;
pub use std::c_vec::{CVec, len, get};
use core::libc::{c_float};
use system::vector2;
use graphics::rect::FloatRect;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_float};
    use system::vector2;
    use graphics::rect::FloatRect;
    use graphics::transform::Transform;

    pub extern "C" {
        fn sfTransform_fromMatrix(a01 : f32, a02 : f32, a03 : f32, b01 : f32, b02 : f32, b03 : f32, c01 : f32, c02 : f32, c03 : f32) -> Transform;
        //fn sfTransform_getMatrix(tranform : *sfTransform, matrix : *mut f32) -> ();
        fn sfTransform_getInverse(transform : *Transform) -> Transform;
        fn sfTransform_transformPoint(transform : *Transform, point : vector2::Vector2f) -> vector2::Vector2f;
        fn sfTransform_transformRect(transform : *Transform, rectangle : FloatRect) -> FloatRect;
        fn sfTransform_combine(transform : *Transform, other : *Transform) -> ();
        fn sfTransform_translate(transform : *Transform, x : c_float, y : c_float) -> ();
        fn sfTransform_rotate(transform : *Transform, angle : c_float) -> ();
        fn sfTransform_rotateWithCenter(transform : *Transform, angle : c_float, centerX : c_float, centerY : c_float) -> ();
        fn sfTransform_scale(transform : *Transform, scaleX : c_float, scaleY : c_float) -> ();
        fn sfTransform_scaleWithCenter(transform: *Transform, scaleX : c_float, scaleY : c_float, centerX : c_float, centerY : c_float) -> ();
    }
}


/*
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
    

*/

#[doc(hidden)]
pub struct Transform {
        a1 : f32,
        a2 : f32,
        a3 : f32,
        a4 : f32,
        a5 : f32,
        a6 : f32,
        a7 : f32,
        a8 : f32,
        a9 : f32
//    transform : csfml::sfTransform
}

impl Transform {
    /**
    * Create a new transform from a matrix
    */
    pub fn new(a01 : f32, a02 : f32, a03 : f32, b01 : f32, b02 : f32, b03 : f32, c01 : f32, c02 : f32, c03 : f32) -> Transform {
        unsafe {
            csfml::sfTransform_fromMatrix(a01, a02, a03, b01, b02, b03, c01, c02, c03)
        }
    }

/*    pub fn get_matrix(&self) -> ~[f32] {
        unsafe {
            let matrix : *mut f32 = ptr::null();
            let mut return_matrix : ~[f32] = ~[];
            unsafe {
                csfml::sfTransform_getMatrix(&self.transform, matrix);
                let cvec = CVec(matrix, 16);
                let mut d : uint = 0;
                return_matrix.push(get(cvec, d));
                d += 1;
                while d != 16 {
                    return_matrix.push(get(cvec, d));
                    d += 1;
                }
            }
        return_matrix
        }
    }*/
    
    /**
    * Create a new identity transform
    */
    pub fn new_identity() -> Transform {
        unsafe {
            csfml::sfTransform_fromMatrix(1., 0., 0., 0., 1., 0., 0., 0., 1.)
        }
    }

    /**
    * Return the inverse of a transform
    */
    pub fn get_inverse(&self) -> Transform {
        unsafe {csfml::sfTransform_getInverse(&*self)}
    }
    
    /**
    * Combine two transforms
    */
    pub fn combine(&self, other : &Transform) -> () {
        unsafe {
            csfml::sfTransform_combine(&*self, &*other)
        }
    }
    
    /**
    * Combine a transform with a translation
    */
    pub fn translate(&self, x : f32, y : f32) -> () {
        unsafe {
            csfml::sfTransform_translate(&*self, x as c_float, y as c_float)
        }
    }

    /**
    * Combine the current transform with a rotation
    */
    pub fn rotate(&self, angle : f32) -> () {
        unsafe {
            csfml::sfTransform_rotate(&*self, angle as c_float)
        }
    }
    
    /**
    * Combine the current transform with a rotation
    */
    pub fn rotate_with_center(&self, angle : f32, centerX : f32, centerY : f32) -> () {
        unsafe {
            csfml::sfTransform_rotateWithCenter(&*self, angle as c_float, centerX as c_float, centerY as c_float)
        }
    }

    /**
    * Combine the current transform with a scaling
    */
    pub fn scale(&self, scaleX : f32, scaleY : f32) -> () {
        unsafe {
            csfml::sfTransform_scale(&*self, scaleX as c_float, scaleY as c_float)
        }
    }
    
    /**
    * Combine the current transform with a scaling
    */
    pub fn scale_with_center(&self, scaleX : f32, scaleY : f32, centerX : f32, centerY : f32) -> () {
        unsafe {
            csfml::sfTransform_scaleWithCenter(&*self, scaleX, scaleY, centerX, centerY)
        }
    }

    pub fn transform_point(&self, point : &vector2::Vector2f) -> vector2::Vector2f {
        unsafe {
            csfml::sfTransform_transformPoint(&*self, *point)
        }
    }

    pub fn transform_rect(&self, rectangle : &FloatRect) -> FloatRect {
        unsafe {
            csfml::sfTransform_transformRect(&*self, *rectangle)
        }
    }
    
}