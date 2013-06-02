/*!
*
*
*
*
*/

use graphics::color::Color;
use system::vector2::Vector2f;
use system::vector2;

#[doc(hidden)]
pub mod csfml {
    
    use graphics::color;
    use system::vector2;

    pub struct sfVertex {
        position : vector2::csfml::sfVector2f,
        color : color::csfml::sfColor,
        texCoords : vector2::csfml::sfVector2f 
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

    pub fn wrap_vertex(vertex : csfml::sfVertex) -> Vertex {
        Vertex {position : vector2::wrap_vector2f(vertex.position), color : Color::wrap_color(vertex.color), texCoords : vector2::wrap_vector2f(vertex.texCoords)}
    }

    pub fn unwrap_vertex(vertex : &Vertex) -> csfml::sfVertex {
        csfml::sfVertex {position : vector2::unwrap_vector2f(&vertex.position), color : vertex.color.unwrap_color(), texCoords : vector2::unwrap_vector2f(&vertex.texCoords)}
    }
}