use crate::graphics::{RenderStates, RenderTarget};

/// Something that is drawable by a [`RenderTarget`].
pub trait Drawable {
    /// Draws into `target` with [`RenderStates`] `states`.
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    );
}
