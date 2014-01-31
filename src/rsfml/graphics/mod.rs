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
* 2D graphics module: sprites, text, shapes
*
*
*
*/

pub use self::render_states::RenderStates;
pub use self::render_window::RenderWindow;
pub use self::rect::{FloatRect, IntRect};
pub use self::texture::Texture;
pub use self::blend_mode::{BlendMode, BlendAlpha, BlendAdd, BlendMultiply, BlendNone};
pub use self::transform::Transform;
pub use self::text::Text;
pub use self::shader::Shader;
pub use self::color::Color;
pub use self::font::Font;
pub use self::view::View;
pub use self::image::Image;
pub use self::sprite::Sprite;
pub use self::circle_shape::RcCircleShape;
pub use self::circle_shape::CircleShape;
pub use self::rectangle_shape::RectangleShape;
pub use self::convex_shape::ConvexShape;
pub use self::primitive_type::{PrimitiveType, Points, Lines, LinesStrip, Triangles, TrianglesStrip, TrianglesFan, Quads};
pub use self::vertex::Vertex;
pub use self::transformable::Transformable;
pub use self::glyph::Glyph;
pub use self::render_texture::RenderTexture;
pub use self::shape::Shape;
pub use self::vertex_array::VertexArray;

#[doc(hidden)]
#[cfg(target_os="macos")]
#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod platform {
    #[link(name = "csfml-graphics")]
    extern {}
}

pub mod render_states;
pub mod render_window;
pub mod rect;
pub mod texture;
pub mod blend_mode;
pub mod transform;
pub mod text;
pub mod shader;
pub mod color;
pub mod font;
pub mod view;
pub mod image;
pub mod sprite;
pub mod circle_shape;
pub mod rectangle_shape;
pub mod convex_shape;
pub mod primitive_type;
pub mod vertex;
pub mod vertex_array;
pub mod transformable;
pub mod glyph;
pub mod render_texture;
pub mod shape;
