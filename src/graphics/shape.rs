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

use graphics::{Drawable, Transformable, TextureRef, IntRect, FloatRect, Color};
use system::Vector2f;

/// Trait for textured shapes with outline.
pub trait Shape<'s>: Drawable + Transformable {
    /// Changes the source texture of the shape.
    ///
    /// If `reset_rect` is `true`, the `texture_rect` property of the shape is automatically
    /// adjusted to the size of the new texture.
    /// If it is `false`, the texture rect is left unchanged.
    fn set_texture(&mut self, texture: &'s TextureRef, reset_rect: bool);
    /// Disables texturing for this shape.
    fn disable_texture(&mut self);
    /// Sets the sub-rectangle of the texture that the shape will display.
    ///
    /// The texture rect is useful when you don't want to display the whole texture,
    /// but rather a part of it. By default, the texture rect covers the entire texture.
    fn set_texture_rect(&mut self, rect: &IntRect);
    /// Sets the fill color of the shape.
    ///
    /// This color is modulated (multiplied) with the shape's texture if any.
    /// It can be used to colorize the shape, or change its global opacity.
    /// You can use `Color::transparent()` to make the inside of the shape transparent,
    /// and have the outline alone. By default, the shape's fill color is opaque white.
    fn set_fill_color(&mut self, color: &Color);
    /// Sets the outline color of the shape.
    ///
    /// By default, the shape's outline color is opaque white.
    fn set_outline_color(&mut self, color: &Color);
    /// Sets the thickness of the shape's outline.
    ///
    /// Note that negative values are allowed
    /// (so that the outline expands towards the center of the shape),
    /// and using zero disables the outline. By default, the outline thickness is 0.
    fn set_outline_thickness(&mut self, thickness: f32);
    /// Gets the source texture of the shape.
    ///
    /// If the shape has no source texture, None is returned.
    fn texture(&self) -> Option<&'s TextureRef>;
    /// Gets the sub-rectangle of the texture displayed by the shape.
    fn texture_rect(&self) -> IntRect;
    /// Gets the fill color of this shape.
    fn fill_color(&self) -> Color;
    /// Gets the outline color of this shape.
    fn outline_color(&self) -> Color;
    /// Gets the outline thickness of this shape.
    fn outline_thickness(&self) -> f32;
    /// Gets the total number of points of the shape.
    fn point_count(&self) -> u32;
    /// Gets a point of the shape.
    ///
    /// The returned point is in local coordinates, that is, the shape's transforms
    /// (position, rotation, scale) are not taken into account.
    /// The result is unspecified if index is out of the valid range (`0..point_count()`).
    fn point(&self, index: u32) -> Vector2f;
    /// Gets the local bounding rectangle of the entity.
    ///
    /// The returned rectangle is in local coordinates, which means that it ignores the
    /// transformations (translation, rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the entity in the entity's
    /// coordinate system.
    fn local_bounds(&self) -> FloatRect;
    /// Gets the global (non-minimal) bounding rectangle of the entity.
    ///
    /// The returned rectangle is in global coordinates,
    /// which means that it takes into account the transformations
    /// (translation, rotation, scale, ...) that are applied to the entity.
    /// In other words, this function returns the bounds of the shape in the global 2D world's
    /// coordinate system.
    ///
    /// This function does not necessarily return the minimal bounding rectangle.
    /// It merely ensures that the returned rectangle covers all the vertices (but possibly more).
    /// This allows for a fast approximation of the bounds as a first check;
    /// you may want to use more precise checks on top of that.
    fn global_bounds(&self) -> FloatRect;
}
