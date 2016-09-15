// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! base shape trait

use graphics::{Drawable, Transformable, Texture, IntRect, FloatRect, Color};
use system::Vector2f;

/// Implement this shape to create a new `Shape`
pub trait ShapeImpl {
    /// Get the total count of the point for the Shape who implement this trait.
    ///
    /// Return the points count
    fn get_point_count(&self) -> u32;

    /// Get a given point of a `Shape`.
    ///
    /// # Argument
    /// * point - The index of the point to return
    ///
    /// Return a `Vector2f` who contains the point coordinates.
    fn get_point(&self, point: u32) -> Vector2f;
}

/// The trait drawable is inherited by each object who can be drawn in a `RenderTarget`.
#[allow(missing_docs)]
pub trait Shape<'s>: Drawable + Transformable {
    fn set_texture(&mut self, texture: &'s Texture, reset_rect: bool);
    fn disable_texture(&mut self);

    fn set_texture_rect(&mut self, rect: &IntRect);
    fn set_fill_color(&mut self, color: &Color);
    fn set_outline_color(&mut self, color: &Color);
    fn set_outline_thickness(&mut self, thickness: f32);
    fn get_texture(&self) -> Option<&'s Texture>;
    fn get_texture_rect(&self) -> IntRect;
    fn get_fill_color(&self) -> Color;
    fn get_outline_color(&self) -> Color;
    fn get_outline_thickness(&self) -> f32;
    fn get_point_count(&self) -> u32;
    fn get_point(&self, index: u32) -> Vector2f;
    fn get_local_bounds(&self) -> FloatRect;
    fn get_global_bounds(&self) -> FloatRect;
}
