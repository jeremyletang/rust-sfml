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

/*!
* Define a point with color and texture coordinates
*
* A vertex is an improved point.
*
* It has a position and other extra attributes that will be used for drawing: 
* in SFML, vertices also have a color and a pair of texture coordinates.
*
*/

use graphics::color::Color;
use system::vector2::Vector2f;

#[doc(hidden)]
pub mod ffi {
    
    use graphics::color;
    use system::vector2;

    pub struct sfVertex {
        position : vector2::Vector2f,
        color : color::Color,
        texCoords : vector2::Vector2f 
    }
}

/// Define a point with color and texture coordinates
pub struct Vertex {
    position : Vector2f,
    color : Color,
    texCoords : Vector2f
}

impl Vertex {
    /**
    * Create a new Vertex
    *
    * # Arguments
    * * position - Position of the vertex
    * * color - Color of the vertex
    * * texCoords - Texture coordinate of the vertex
    *
    * Return a Vertex
    */
    pub fn new(position : &Vector2f, color : &Color, texCoords : &Vector2f) -> Vertex {
        Vertex {
            position : *position,
            color : *color,
            texCoords : *texCoords
        }
    }
}