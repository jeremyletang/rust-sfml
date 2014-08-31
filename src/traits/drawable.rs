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

//! Drawable trait
//!
//! Implemented by each drawable object to specifiy their drawing operations for
//! RenderTargets.

#![allow(unused_variable)]

use graphics::{RenderStates, RenderTarget, rc};

/// The trait drawable is inherited by each object who can be drawn in a RenderTarget
pub trait Drawable {
    fn draw<RT: RenderTarget>(&self, target: &mut RT);

    fn draw_rs<RT: RenderTarget>(&self,
                                 target: &mut RT,
                                 states: &mut RenderStates){
        println!("Error: Bad Usage: Can't draw a ref-counted drawable with borrow-based RenderStates");
    }

    fn draw_rs_rc<RT: RenderTarget>(&self,
                                    target: &mut RT,
                                    states: &mut rc::RenderStates){
        println!("Error: Bad Usage: Can't draw a borrow-based drawable with refcount based RenderStates");
    }
}
