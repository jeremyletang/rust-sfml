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

use graphics::{RenderStates, RenderTarget};
use system::Vector2f;

/// The trait drawable is inherited by each object who can be drawn in a RenderTarget
pub trait Drawable {
    /// Draw a drawable object with a RenderState into a RenderTarget
    fn draw<RT: RenderTarget>(&self, target: &mut RT, states: &RenderStates);
}

/// ShapeImpl trait
///
/// Implement this shape to create a new Shape
pub trait ShapeImpl: Send {
    /// Get the total count of the point for the Shape who implement this trait.
    ///
    /// Return the points count
    fn get_point_count(&self) -> u32;

    /// Get a given point of a Shape.
    ///
    /// # Argument
    /// * point - The index of the point to return
    ///
    /// Return a Vector2f who contains the point coordinates.
    fn get_point(&self, point: u32) -> Vector2f;
}
