/*!
* 2D graphics module: sprites, text, shapes
*
*
*
*/

#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod others {
    #[link_args="-lcsfml-graphics"]
    extern {}
}

pub mod render_states;
pub mod render_window;
pub mod rect;
pub mod texture;
pub mod blend_mode;
pub mod transform;
pub mod drawable;
pub mod text;
pub mod shader;
pub mod color;
pub mod font;
pub mod view;
pub mod image;
pub mod sprite;
pub mod circle_shape;
pub mod rectangle_shape;