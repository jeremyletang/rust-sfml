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

//! Specialized shape representing a convex polygon
//!
//! It is important to keep in mind that a convex shape must
//! always be... convex, otherwise it may not be drawn correctly.
//! Moreover, the points must be defined in order; using a random
//! order would result in an incorrect shape.

use std::ops::{Deref, DerefMut};

use graphics::{Texture, RenderTarget, RenderStates, Drawable, ShapeImpl, Shape, BaseShape};
use system::Vector2f;

#[derive(Clone)]
struct ConvexImpl(Vec<Vector2f>);

impl ShapeImpl for ConvexImpl {
    /// Get the total number of points for the shape.
    fn get_point_count(&self) -> u32 {
		self.0.len() as u32
	}

	/// Get the coordinates of a point of the shape by index.
    fn get_point(&self, point: u32) -> Vector2f {
		self.0[point as usize]
	}
}

/// Specialized shape representing a convex polygon
///
/// It is important to keep in mind that a convex shape must
/// always be... convex, otherwise it may not be drawn correctly.
/// Moreover, the points must be defined in order; using a random
/// order would result in an incorrect shape.
pub struct ConvexShape<'s> {
	shape: BaseShape<'s>,
	points: Box<ConvexImpl>
}

impl<'s> ConvexShape<'s> {
    /// Create a new convex shape with no points.
    pub fn new() -> Option<ConvexShape<'s>> {
		ConvexShape::from_vec(Vec::new())
    }

	/// Create a new convex shape from the specified points.
    pub fn from_vec(points: Vec<Vector2f>) -> Option<ConvexShape<'s>> {
		let boxed = Box::new(ConvexImpl(points));
		let ptr = unsafe {
			::std::mem::transmute::<&ConvexImpl, &'s ConvexImpl>(&*boxed)
		};
		BaseShape::new(ptr).map(|shape| ConvexShape {
			shape: shape,
			points: boxed
		})
    }

    /// Create a new convex shape with the specified points and texture.
    pub fn new_with_texture(points: Vec<Vector2f>, texture: &'s Texture) -> Option<ConvexShape<'s>> {
		ConvexShape::from_vec(points).map(|mut shape| {
			shape.set_texture(texture, true);
			shape
		})
    }

    /// Clone an existing convex shape
    ///
    /// Return Some(ConvexShape) or None
    pub fn clone_opt(&self) -> Option<ConvexShape<'s>> {
		ConvexShape::from_vec(self.points.0.clone()).map(|mut shape| {
			match self.get_texture() {
				Some(tex) => shape.set_texture(tex, false),
				None => shape.disable_texture()
			}
			shape.set_texture_rect(&self.get_texture_rect());
			shape.set_fill_color(&self.get_fill_color());
			shape.set_outline_color(&self.get_outline_color());
			shape.set_outline_thickness(self.get_outline_thickness());
			shape
		})
    }

    /// Get the points contained within this convex shape.
    pub fn points(&self) -> &[Vector2f] {
        &self.points.0
    }

    /// Mutably access the points contained within this convex shape.
    pub fn points_mut(&mut self) -> &mut Vec<Vector2f> {
        &mut self.points.0
    }

	/// Set the points contained within this convex shape to completely new
	/// values.
	pub fn set_points(&mut self, points: Vec<Vector2f>) {
		self.points.0 = points;
		self.update();
	}
}

impl<'s> Deref for ConvexShape<'s> {
	type Target = BaseShape<'s>;
	fn deref(&self) -> &BaseShape<'s> {
		&self.shape
	}
}

impl<'s> DerefMut for ConvexShape<'s> {
	fn deref_mut(&mut self) -> &mut BaseShape<'s> {
		&mut self.shape
	}
}

impl<'s> Clone for ConvexShape<'s> {
    fn clone(&self) -> ConvexShape<'s> {
		self.clone_opt().expect("Failed to clone ConvexShape")
    }
}

impl<'s> Drawable for ConvexShape<'s> {
    fn draw<RT: RenderTarget>(&self,
                                 render_target: &mut RT,
                                 render_states: &RenderStates) -> () {
        render_target.draw_shape_rs(self, render_states)
    }
}
