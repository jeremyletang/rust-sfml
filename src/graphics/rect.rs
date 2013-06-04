/*!
* Manipulating 2D rectangles
*
* Utility class for manipulating 2D axis aligned rectangles
*
*/

use core::libc::{c_int};
#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_int};
    use rsfml::sfTypes::{sfBool};
    use graphics::rect::IntRect;
    use graphics::rect::FloatRect;

    pub extern "C" {
        fn sfIntRect_contains(rect : *IntRect, x : c_int, y : c_int) -> sfBool;
        fn sfIntRect_intersects(rect1 : *IntRect, rect2 : *IntRect, intersectons : *IntRect) -> sfBool;
        fn sfFloatRect_intersects(rect1 : *FloatRect, rect2 : *FloatRect, intersectons : *FloatRect) -> sfBool;
        fn sfFloatRect_contains(rect : *FloatRect, x : f32, y : f32) -> sfBool;
    }
}

pub struct IntRect {
    left : i32,
    top : i32,
    width : i32,
    height : i32
}

pub struct FloatRect {
    left : f32,
    top : f32,
    width : f32,
    height : f32
}

impl IntRect {
    /**
    * Construct a new IntRect
    */
    pub fn new(left : i32, top : i32, width : i32, height : i32) -> IntRect {
        IntRect {left : left, top : top, width : width, height : height}
    }
    
    /**
    *  Check if a point is inside a rectangle's area 
    */
    pub fn contains(self, x : int, y : int) -> bool {
        match unsafe {csfml::sfIntRect_contains(&self, x as c_int, y as c_int) } {
            0 => false,
            _ => true
        }
    }
    
    pub fn intersects(rect1 : &IntRect, rect2 : &IntRect, intersections : &IntRect) -> bool {
        match unsafe {csfml::sfIntRect_intersects(rect1, rect2, intersections)} {
            0 => false,
            _ => true
        }
    }
}

impl FloatRect {
    /**
    * Construct a new FloatRect
    */
    pub fn new(left : f32, top : f32, width : f32, height : f32) -> FloatRect {
        FloatRect {left : left, top : top, width : width, height : height}
    }
    
    /**
    *  Check if a point is inside a rectangle's area 
    */
    pub fn contains(self, x : f32, y : f32) -> bool {
        match unsafe {csfml::sfFloatRect_contains(&self, x, y) } {
            0 => false,
            _ => true
        }
    }

    pub fn intersects(rect1 : &FloatRect, rect2 : &FloatRect, intersections : &FloatRect) -> bool {
        match unsafe {csfml::sfFloatRect_intersects(rect1, rect2, intersections)} {
            0 => false,
            _ => true
        }
    }

}