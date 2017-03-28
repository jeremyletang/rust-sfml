use graphics::{RenderStates, RenderTarget};

/// The trait drawable is inherited by each object who can be drawn in a `RenderTarget`
pub trait Drawable {
    /// Draw a drawable object into a `RenderTarget`
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh;
}
