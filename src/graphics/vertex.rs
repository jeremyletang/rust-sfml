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


//! Define a point with color and texture coordinates
//!
//! A vertex is an improved point.
//!
//! It has a position and other extra attributes that will be used for drawing:
//! in SFML, vertices also have a color and a pair of texture coordinates.

use graphics::Color;
use system::Vector2f;
use csfml_graphics_sys as ffi;
use csfml_system_sys::sfVector2f;
use raw_conv::Raw;

/// Define a point with color and texture coordinates
///
/// A vertex is an improved point.
///
/// It has a position and other extra attributes that will be used for drawing:
/// in SFML, vertices also have a color and a pair of texture coordinates.
#[repr(C)]
pub struct Vertex(pub ffi::sfVertex);

impl Vertex {
    /// Create a new Vertex
    ///
    /// # Arguments
    /// * position - Position of the vertex
    /// * color - Color of the vertex
    /// * tex_coords - Texture coordinate of the vertex
    ///
    /// Return a Vertex
    pub fn new(position: &Vector2f, color: &Color, tex_coords: &Vector2f) -> Vertex {
        Vertex(ffi::sfVertex {
            position: position.raw(),
            color: color.raw(),
            texCoords: tex_coords.raw(),
        })
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
    pub fn with_pos(position: &Vector2f) -> Vertex {
        Vertex(ffi::sfVertex {
            position: position.raw(),
            color: Color::white().raw(),
            texCoords: sfVector2f { x: 0., y: 0. },
        })
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
    pub fn with_pos_color(position: &Vector2f, color: &Color) -> Vertex {
        Vertex(ffi::sfVertex {
            position: position.raw(),
            color: color.raw(),
            texCoords: sfVector2f { x: 0., y: 0. },
        })
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
    pub fn with_pos_coords(position: &Vector2f, tex_coords: &Vector2f) -> Vertex {
        Vertex(ffi::sfVertex {
            position: position.raw(),
            color: Color::white().raw(),
            texCoords: tex_coords.raw(),
        })
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
        Vertex(ffi::sfVertex {
            position: sfVector2f { x: 0., y: 0. },
            color: Color::white().raw(),
            texCoords: sfVector2f { x: 0., y: 0. },
        })
    }
}
