/*!
*
*
*
*
*/

use graphics::color::Color;
use system::vector2::Vector2f;

#[doc(hidden)]
pub mod csfml {
    
    use graphics::color;
    use system::vector2;

    pub struct sfVertex {
        position : vector2::Vector2f,
        color : color::Color,
        texCoords : vector2::Vector2f 
    }
}

pub struct Vertex {
    position : Vector2f,
    color : Color,
    texCoords : Vector2f
}

impl Vertex {
    pub fn new(position : &Vector2f, color : &Color, texCoords : &Vector2f) -> Vertex {
        Vertex {position : *position, color : *color, texCoords : *texCoords }
    }
}