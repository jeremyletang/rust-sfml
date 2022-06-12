//! 2D graphics module: sprites, text, shapes..

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
    vertex_buffer::{VertexBuffer, VertexBufferUsage},
    view::View,
};
pub use crate::ffi::graphics::ShaderType;

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
mod vertex_buffer;
mod view;

/// Compute the bounding rectangle of the vertex array.
///
/// This function returns the minimal axis-aligned rectangle that contains all the vertices of the array.
#[must_use]
pub fn vertex_array_bounds(vertices: &[Vertex]) -> FloatRect {
    let mut vertices = vertices.iter();
    if let Some(fst) = vertices.next() {
        let mut left = fst.position.x;
        let mut top = fst.position.y;
        let mut right = fst.position.x;
        let mut bottom = fst.position.y;

        for vert in vertices {
            let pos = vert.position;

            // Update left and right
            if pos.x < left {
                left = pos.x;
            } else if pos.x > right {
                right = pos.x;
            }

            // Update top and botttom
            if pos.y < top {
                top = pos.y;
            } else if pos.y > bottom {
                bottom = pos.y
            }
        }
        FloatRect::new(left, top, right - left, bottom - top)
    } else {
        FloatRect::default()
    }
}
