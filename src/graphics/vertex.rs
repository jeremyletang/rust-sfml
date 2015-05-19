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

use graphics::Color;
use system::Vector2f;

/// A point with color and texture coordinates.
///
/// The vertex is the building block of drawing. Everything which is visible on
/// screen is made of vertices. They are grouped as 2D primitives (triangles,
/// quads, ...), and these primitives are grouped to create even more complex 2D
/// entities such as sprites, texts, etc.
///
/// If you use the graphical entities of SFML (sprite, text, shape) you won't
/// have to deal with vertices directly. But if you want to define your own 2D
/// entities, such as tiled maps or particle systems, using vertices will allow
/// you to get maximum performance.
///
/// Note: although texture coordinates are supposed to be an integer amount of
/// pixels, their type is float because of some buggy graphics drivers that are
/// not able to process integer coordinates correctly.
#[repr(C)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Copy)]
pub struct Vertex {
    /// 2D position of the vertex.
    pub position: Vector2f,
    /// Color of the vertex.
    pub color: Color,
    /// Coordinates of the texture's pixel to map to the vertex.
    pub tex_coords: Vector2f
}

impl Vertex {
    /// Create a new Vertex from its position, color, and texture coordinates.
    pub fn new(position: &Vector2f, color: &Color, tex_coords: &Vector2f) -> Vertex {
        Vertex {
            position: *position,
            color: *color,
            tex_coords: *tex_coords
        }
    }

	/// Create a new Vertex from its position.
	///
	/// The color will be white and the texture coordinates will be (0, 0).
    pub fn new_with_pos(position: &Vector2f) -> Vertex {
        Vertex {
            position: *position,
            color: Color::white(),
            tex_coords: Vector2f { x: 0., y: 0. }
        }
    }

	/// Create a new vertex from its position and color.
	///
	/// The texture coordinates will be (0, 0).
    pub fn new_with_pos_color(position: &Vector2f, color: &Color) -> Vertex {
        Vertex {
            position: *position,
            color: *color,
            tex_coords: Vector2f { x: 0., y: 0. }
        }
    }

	/// Create a new Vertex from its position and texture coordinates.
	///
	/// The color will be white.
    pub fn new_with_pos_coords(position: &Vector2f, tex_coords: &Vector2f) -> Vertex {
        Vertex {
            position: *position,
            color: Color::white(),
            tex_coords: *tex_coords
        }
    }
}

impl Default for Vertex {
	/// Create a Vertex with position (0, 0), color white, and texture
	/// coordinates (0, 0).
	fn default() -> Vertex {
		Vertex {
			position: Vector2f::default(),
			color: Color::white(),
			tex_coords: Vector2f::default()
		}
	}
}
