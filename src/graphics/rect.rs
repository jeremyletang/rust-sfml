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
* Manipulating 2D rectangles
*
* Utility class for manipulating 2D axis aligned rectangles
*
*/

use std::libc::{c_int};
#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_int};
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