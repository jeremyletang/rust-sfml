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

//! 2D graphics module: sprites, text, shapes..

pub use graphics::render_target::RenderTarget;
pub use graphics::render_states::RenderStates;
pub use graphics::render_window::{RenderWindow, Events};
pub use graphics::rect::{Rect, FloatRect, IntRect};
pub use graphics::texture::{Texture, TextureRef};
pub use graphics::blend_mode::BlendMode;
pub use graphics::transform::Transform;
pub use graphics::text::Text;
pub use graphics::shader::Shader;
pub use graphics::color::Color;
pub use graphics::font::Font;
pub use graphics::view::{View, ViewRef};
pub use graphics::image::Image;
pub use graphics::sprite::Sprite;
pub use graphics::circle_shape::CircleShape;
pub use graphics::rectangle_shape::RectangleShape;
pub use graphics::convex_shape::{ConvexShape, ConvexShapePoints};
pub use graphics::primitive_type::*;
pub use graphics::vertex::Vertex;
pub use graphics::glyph::Glyph;
pub use graphics::render_texture::RenderTexture;
pub use graphics::custom_shape::CustomShape;
pub use graphics::vertex_array::{VertexArray, Vertices};
pub use graphics::text_style::TextStyle;
pub use graphics::drawable::Drawable;
pub use graphics::shape::{Shape, ShapeImpl};
pub use graphics::transformable::Transformable;

mod drawable;
mod shape;
mod transformable;
mod render_target;
mod render_states;
mod render_window;
mod texture;
pub mod blend_mode;
mod transform;
mod text;
pub mod text_style;
mod shader;
mod color;
pub mod font;
mod view;
mod image;
mod sprite;
mod circle_shape;
mod rectangle_shape;
mod convex_shape;
mod primitive_type;
mod vertex;
mod vertex_array;
mod render_texture;
mod custom_shape;
mod rect;
mod glyph;
