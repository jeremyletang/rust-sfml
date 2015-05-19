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

use graphics::{RenderStates, RenderTarget, Transformable, Color};
use system::Vector2f;

/// Type which can be drawn onto a RenderTarget.
pub trait Drawable {
	/// Draw this drawable onto a RenderTarget, with the provided states.
    fn draw(&self, target: &mut RenderTarget, states: &RenderStates);
}

/// Shape implementation which determines where and how many points exist, used
/// as an argument to `BaseShape`.
pub trait ShapeImpl: Send {
    /// Get the total number of points for the shape.
    fn get_point_count(&self) -> u32;

	/// Get the coordinates of a point of the shape by index.
    fn get_point(&self, point: u32) -> Vector2f;
}

/// A textured shape with an outline.
///
/// `Shape`s are drawable types that allow defining and displaying convex shapes
/// to render targets. In addition to the attributes provided by the specialized
/// shape types, a shape always has the following attributes:
///
/// * a texture
/// * a texture rectangle
/// * a fill color
/// * an outline color
/// * an outline thickness
///
/// Each feature is optional, and can be disabled easily:
///
/// * the texture can be null
/// * the fill/outline colors can be sf::Color::Transparent
/// * the outline thickness can be zero
///
/// Custom shapes can be implemented by using `BaseShape` and `ShapeImpl`.
pub trait Shape: Transformable + Drawable {
    /// Set the fill color of the shape.
    ///
    /// This color is modulated (multiplied) with the shape's
    /// texture if any. It can be used to colorize the shape,
    /// or change its global opacity.
    /// You can use `Color::transparent()` to make the inside of
    /// the shape transparent, and have the outline alone.
    /// By default, the shape's fill color is opaque white.
    fn set_fill_color(&mut self, color: &Color);

    /// Set the outline color of the shape.
    ///
    /// You can use `Color::transparent()` to disable the outline.
    /// By default, the Shape's outline color is opaque white.
    fn set_outline_color(&mut self, color: &Color);

    /// Set the thickness of the shape's outline.
	///
	/// Note that negative values are allowed (so that the outline expands
	/// towards the center of the shape), and using zero disables the outline.
	/// By default, the outline thickness is 0.
    fn set_outline_thickness(&mut self, thickness: f32);

    /// Get the fill color of the shape.
    fn get_fill_color(&self) -> Color;

    /// Get the outline color of the shape.
    fn get_outline_color(&self) -> Color;

    /// Get the outline thickness of the shape.
    fn get_outline_thickness(&self) -> f32;

    /// Get the total number of points of the shape.
    fn get_point_count(&self) -> u32;

	/// Get the coordinates of a point of the shape by index.
	///
	/// The returned point is in local coordinates, that is, the shape's
	/// transforms (position, rotation, scale) are not taken into account. The
	/// result is undefined if index is out of the valid range.
    fn get_point(&self, point: u32) -> Vector2f;
}
