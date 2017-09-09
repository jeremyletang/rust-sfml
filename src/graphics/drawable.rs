use graphics::{RenderStates, RenderTarget};

/// The trait drawable is inherited by each object who can be drawn in a `RenderTarget`
pub trait Drawable {
    /// Draw a drawable object into a `RenderTarget`
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut RenderTarget,
        states: RenderStates<'texture, 'shader, 'shader_texture>,
    );
}
