//! 2D graphics module: sprites, text, shapes..

extern crate csfml_graphics_sys;

#[doc(inline)]
pub use self::blend_mode::BlendMode;
pub use self::{
    circle_shape::CircleShape,
    color::Color,
    convex_shape::{ConvexShape, ConvexShapePoints},
    custom_shape::{CustomShape, CustomShapePoints},
    drawable::Drawable,
    font::{Font, Info as FontInfo},
    glyph::Glyph,
    image::Image,
    primitive_type::PrimitiveType,
    rect::{FloatRect, IntRect, Rect},
    rectangle_shape::RectangleShape,
    render_states::RenderStates,
    render_target::RenderTarget,
    render_texture::RenderTexture,
    render_window::RenderWindow,
    shader::Shader,
    shape::Shape,
    sprite::Sprite,
    text::Text,
    text_style::TextStyle,
    texture::Texture,
    transform::Transform,
    transformable::Transformable,
    vertex::Vertex,
    vertex_array::{VertexArray, Vertices},
    vertex_buffer::{VertexBuffer, VertexBufferUsage},
    view::View,
};

pub mod blend_mode;
mod circle_shape;
mod color;
mod convex_shape;
mod custom_shape;
mod drawable;
mod font;
pub mod glsl;
mod glyph;
mod image;
mod primitive_type;
mod rect;
mod rectangle_shape;
mod render_states;
mod render_target;
mod render_texture;
mod render_window;
mod shader;
mod shape;
mod sprite;
mod text;
mod text_style;
mod texture;
mod transform;
mod transformable;
mod vertex;
mod vertex_array;
mod vertex_buffer;
mod view;
