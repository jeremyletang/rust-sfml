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

use graphics::Color;
use system::Vector2f;
use csfml_graphics_sys as ffi;
use system::raw_conv::Raw;

/// Define a point with color and texture coordinates.
///
/// A vertex is an improved point.
///
/// It has a position and other extra attributes that will be used for drawing:
/// in SFML, vertices also have a color and a pair of texture coordinates.
///
/// The vertex is the building block of drawing.
/// Everything which is visible on screen is made of vertices.
/// They are grouped as 2D primitives (triangles, quads, ...), and these primitives are grouped
/// to create even more complex 2D entities such as sprites, texts, etc.
///
/// If you use the graphical entities of SFML (sprite, text, shape) you won't have to
/// deal with vertices directly.
/// But if you want to define your own 2D entities, such as tiled maps or particle systems,
/// using vertices will allow you to get maximum performances.
///
/// Example:
///
/// ```no_run
/// # use sfml::system::*;
/// # use sfml::graphics::*;
/// # let mut window: RenderWindow = unimplemented!();
/// // define a 100x100 square, red, with a 10x10 texture mapped on it
/// let vertices = [
///     Vertex::new(Vector2f::new(  0.,   0.), Color::red(), Vector2f::new( 0.,  0.)),
///     Vertex::new(Vector2f::new(  0., 100.), Color::red(), Vector2f::new( 0., 10.)),
///     Vertex::new(Vector2f::new(100., 100.), Color::red(), Vector2f::new(10., 10.)),
///     Vertex::new(Vector2f::new(100.,   0.), Color::red(), Vector2f::new(10.,  0.)),
/// ];
/// // draw it
/// window.draw_primitives(&vertices, PrimitiveType::sfQuads, RenderStates::default());
/// ```
///
/// Note: although texture coordinates are supposed to be an integer amount of pixels,
/// their type is float because of some buggy graphics drivers that are not able to
/// process integer coordinates correctly.
pub struct Vertex {
    /// 2D position of the vertex
    pub position: Vector2f,
    /// Color of the vertex.
    pub color: Color,
    /// Coordinates of the texture's pixel to map to the vertex.
    pub tex_coords: Vector2f,
}

impl Vertex {
    /// Create a new Vertex
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * color - Color of the vertex
    /// * tex_coords - Texture coordinate of the vertex
    ///
    /// Return a Vertex
    pub fn new(position: Vector2f, color: Color, tex_coords: Vector2f) -> Self {
        Self {
            position: position,
            color: color,
            tex_coords: tex_coords,
        }
    }

    /// Create a new Vertex whit a position
    ///
    /// # Arguments
    /// * position - Position of the vertex
    ///
    /// # Default
    /// * color - white
    /// * tex_coords - (0., 0.)
    ///
    /// Return a Vertex
    pub fn with_pos(position: Vector2f) -> Self {
        Self::new(position, Color::white(), Vector2f::new(0., 0.))
    }

    /// Create a new Vertex with the position and the color
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * color - Color of the vertex
    ///
    /// # Default
    /// * tex_coords - (0., 0)
    ///
    /// Return a Vertex
    pub fn with_pos_color(position: Vector2f, color: Color) -> Vertex {
        Self::new(position, color, Vector2f::new(0., 0.))
    }

    /// Create a new Vertex whit the position and the texture coordinates
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * tex_coords - Texture coordinate of the vertex
    ///
    /// # Default
    /// * color - white
    ///
    /// Return a Vertex
    pub fn with_pos_coords(position: Vector2f, tex_coords: Vector2f) -> Vertex {
        Self::new(position, Color::white(), tex_coords)
    }
}

/// Create a new default `Vertex`
///
/// # Default
/// * `position` - (0., 0.)
/// * `color` - white
/// * `tex_coords` - (0., 0.)
///
/// Return a `Vertex`
impl Default for Vertex {
    fn default() -> Vertex {
        Self::new(Vector2f::new(0., 0.), Color::white(), Vector2f::new(0., 0.))
    }
}

impl Raw for Vertex {
    type Raw = ffi::sfVertex;

    fn raw(&self) -> Self::Raw {
        ffi::sfVertex {
            position: self.position.raw(),
            color: self.color.raw(),
            texCoords: self.tex_coords.raw(),
        }
    }
}
