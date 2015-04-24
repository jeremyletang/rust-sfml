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

//! Define a set of one or more 2D primitives

use traits::Drawable;
use graphics::{Vertex, FloatRect, PrimitiveType, RenderTarget, RenderStates};

use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
/// A vector of one or more 2D primitives
pub struct VertexArray {
	primitive_type: PrimitiveType,
	vertices: Vec<Vertex>
}

impl VertexArray {
	/// Create a new vertex array with the given primitive type.
	pub fn new(primitive_type: PrimitiveType) -> VertexArray {
		VertexArray::from_vec(primitive_type, vec![])
	}

	/// Create a new vertex array with the given initial capacity.
	pub fn with_capacity(primitive_type: PrimitiveType, capacity: usize) -> VertexArray {
		VertexArray::from_vec(primitive_type, Vec::with_capacity(capacity))
	}

	/// Create a new vertex array from an existing `Vec<Vertex>`.
	pub fn from_vec(primitive_type: PrimitiveType, vertices: Vec<Vertex>) -> VertexArray {
		VertexArray {
			primitive_type: primitive_type,
			vertices: vertices
		}
	}

    /// Get the type of the primitives drawn by the vertex array.
	pub fn get_primitive_type(&self) -> PrimitiveType {
		self.primitive_type
	}

    /// Set the type of the primitives of the vertex array.
    ///
    /// This function defines how the vertices shall be interpreted
    /// when it's time to draw them.
	pub fn set_primitive_type(&mut self, primitive_type: PrimitiveType) {
		self.primitive_type = primitive_type;
	}

    /// Compute the bounding rectangle of a vertex array
    ///
    /// This function returns the axis-aligned rectangle that
    /// contains all the vertices of the array.
    ///
    /// Return the bounding rectangle of the vertex array
    pub fn get_bounds(&self) -> FloatRect {
		// [SFML sourced]
		if self.len() == 0 {
			FloatRect::new(0., 0., 0., 0.)
		} else {
			let mut left = self[0].position.x;
			let mut top = self[0].position.y;
			let (mut right, mut bottom) = (left, top);

			for &Vertex { position, .. } in &self[1..] {
				if position.x < left {
					left = position.x
				} else if position.x > right {
					right = position.x
				}
				
				if position.y < top {
					top = position.y
				} else if position.y > bottom {
					bottom = position.y
				}
			}
			
			FloatRect::new(left, top, right - left, bottom - top)
		}
    }
}

impl Deref for VertexArray {
	type Target = Vec<Vertex>;
	
	fn deref(&self) -> &Vec<Vertex> {
		&self.vertices
	}
}

impl DerefMut for VertexArray {
	fn deref_mut(&mut self) -> &mut Vec<Vertex> {
		&mut self.vertices
	}
}

impl Drawable for VertexArray {
    fn draw<RT: RenderTarget>(&self,
                                 render_target: &mut RT,
                                 render_states: &RenderStates) -> () {
		render_target.draw_primitives_rs(&self, self.primitive_type, render_states)
    }
}
