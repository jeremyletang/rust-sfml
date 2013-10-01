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
* Manipulating 2D rectangles
*
* Utility class for manipulating 2D axis aligned rectangles
*
*/

use std::libc::{c_int};
use sfml_types::*;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_int};
    use sfml_types::{SfBool};
    use graphics::rect::IntRect;
    use graphics::rect::FloatRect;

    extern "C" {
        pub fn sfIntRect_contains(rect : *IntRect, x : c_int, y : c_int) -> SfBool;
        pub fn sfIntRect_intersects(rect1 : *IntRect, rect2 : *IntRect, intersectons : *IntRect) -> SfBool;
        pub fn sfFloatRect_intersects(rect1 : *FloatRect, rect2 : *FloatRect, intersectons : *FloatRect) -> SfBool;
        pub fn sfFloatRect_contains(rect : *FloatRect, x : f32, y : f32) -> SfBool;
    }
}

/**
* utility classes for manipulating rectangles of int.
*/
#[deriving(Clone)]
pub struct IntRect {
    left :      i32,
    top :       i32,
    width :     i32,
    height :    i32
}

/**
* utility classes for manipulating rectangles of f32.
*/
#[deriving(Clone)]
pub struct FloatRect {
    left :      f32,
    top :       f32,
    width :     f32,
    height :    f32
}

impl IntRect {
    /**
    * Construct a new IntRect
    */
    pub fn new(left : i32, top : i32, width : i32, height : i32) -> IntRect {
        IntRect {
            left :      left, 
            top :       top, 
            width :     width, 
            height :    height
        }
    }
    
    /**
    *  Check if a point is inside a rectangle's area 
    *
    * # Arguments
    * * x - X coordinate of the point to test
    * * y - Y coordinate of the point to test
    * 
    * Return true if the point is inside
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn contains(self, x : int, y : int) -> bool {
        match unsafe { ffi::sfIntRect_contains(&self, x as c_int, y as c_int) } {
            SFFALSE => false,
            SFTRUE  => true,
            _       => unreachable!()
        }
    }
    
    /**
    * Check intersection between two rectangles
    *
    * # Arguments
    * * rect1 - First rectangle to test
    * * rect2 - Second rectangle to test
    * * intersection - Rectangle to be filled with overlapping rect 
    *
    * Return strue if rectangles overlap
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn intersects(rect1 : &IntRect, rect2 : &IntRect, intersections : &IntRect) -> bool {
        match unsafe { ffi::sfIntRect_intersects(rect1, rect2, intersections) } {
            SFFALSE => false,
            SFTRUE  => true,
            _       => unreachable!()
        
        }
    }
}

impl Eq for IntRect {
    fn eq(&self, other : &IntRect) -> bool {
        self.left == other.left && 
        self.top == other.top &&
        self.width == other.width &&
        self.height == other.height   
    }
    fn ne(&self, other : &IntRect) -> bool {
        self.left != other.left || 
        self.top != other.top ||
        self.width != other.width ||
        self.height != other.height  
    }
}


impl FloatRect {
    /**
    * Construct a new FloatRect
    */
    pub fn new(left : f32, top : f32, width : f32, height : f32) -> FloatRect {
        FloatRect {
            left :      left, 
            top :       top, 
            width :     width, 
            height :    height
        }
    }
    
    /**
    *  Check if a point is inside a rectangle's area 
    *
    * # Arguments
    * * x - X coordinate of the point to test
    * * y - Y coordinate of the point to test
    * 
    * Return true if the point is inside
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn contains(self, x : f32, y : f32) -> bool {
        match unsafe { ffi::sfFloatRect_contains(&self, x, y) } {
            SFFALSE => false,
            SFTRUE  => true,
            _       => unreachable!()
        }
    }

    /**
    * Check intersection between two rectangles
    *
    * # Arguments
    * * rect1 - First rectangle to test
    * * rect2 - Second rectangle to test
    * * intersection - Rectangle to be filled with overlapping rect 
    *
    * Return strue if rectangles overlap
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn intersects(rect1 : &FloatRect, rect2 : &FloatRect, intersections : &FloatRect) -> bool {
        match unsafe { ffi::sfFloatRect_intersects(rect1, rect2, intersections) } {
            SFFALSE => false,
            SFTRUE  => true,
            _       => unreachable!()
        }
    }
}

impl Eq for FloatRect {
    fn eq(&self, other : &FloatRect) -> bool {
        self.left == other.left && 
        self.top == other.top &&
        self.width == other.width &&
        self.height == other.height   
    }
    fn ne(&self, other : &FloatRect) -> bool {
        self.left != other.left || 
        self.top != other.top ||
        self.width != other.width ||
        self.height != other.height  
    }
}
