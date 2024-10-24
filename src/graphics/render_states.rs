use crate::graphics::{BlendMode, Shader, Texture, Transform};

/// Define the states used for drawing to a [`RenderTarget`].
///
/// There are four global states that can be applied to the drawn objects:
///
/// - the blend mode: how pixels of the object are blended with the background
/// - the transform: how the object is positioned/rotated/scaled
/// - the texture: what image is mapped to the object
/// - the shader: what custom effect is applied to the object
///
/// High-level objects such as sprites or text force some of these states when they are drawn.
/// For example, a sprite will set its own texture, so that you don't have to care about it
/// when drawing the sprite.
///
/// The transform is a special case: sprites, texts and shapes
/// (and it's a good idea to do it with your own drawable classes too) combine their transform
/// with the one that is passed in the `RenderStates` structure.
/// So that you can use a "global" transform on top of each object's transform.
///
/// Most objects, especially high-level drawables, can be drawn directly without defining
/// render states explicitly – the default set of states is ok in most cases.
///
/// ```no_run
/// # use sfml::graphics::RenderTarget;
/// # let window: sfml::graphics::RenderWindow = unimplemented!();
/// # let sprite: sfml::graphics::Sprite = unimplemented!();
/// window.draw(&sprite);
/// ```
///
/// To draw with a specific render state, use [`RenderTarget::draw_with_renderstates`].
///
/// ```no_run
/// # use sfml::graphics::*;
/// # let mut window: RenderWindow = unimplemented!();
/// # let shader: Shader = unimplemented!();
/// # let sprite: Sprite = unimplemented!();
/// let mut states = RenderStates::default();
/// states.shader = Some(&shader);
/// window.draw_with_renderstates(&sprite, &states);
/// ```
///
/// When you're inside the `draw` function of a drawable object (implementing [`Drawable`]),
/// you can either pass the render states unmodified, or change some of them.
/// For example, a transformable object will combine the current transform with its own transform.
/// A sprite will set its texture. Etc.
///
/// [`RenderTarget`]: crate::graphics::RenderTarget
/// [`RenderTarget::draw_with_renderstates`]: crate::graphics::RenderTarget::draw_with_renderstates
/// [`Drawable`]: crate::graphics::Drawable
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderStates<'texture, 'shader, 'shader_texture: 'shader> {
    /// The blending mode
    pub blend_mode: BlendMode,
    /// The transform
    pub transform: Transform,
    /// The texture that will be bound
    pub texture: Option<&'texture Texture>,
    /// The shader that will be used
    pub shader: Option<&'shader Shader<'shader_texture>>,
}

impl RenderStates<'_, '_, '_> {
    /// The default render state.
    ///
    /// This can be used in a const context, unlike the [`Default`] implementation.
    pub const DEFAULT: Self = Self {
        blend_mode: BlendMode::ALPHA,
        transform: Transform::IDENTITY,
        texture: None,
        shader: None,
    };
}

impl Default for RenderStates<'_, '_, '_> {
    fn default() -> Self {
        Self::DEFAULT
    }
}
