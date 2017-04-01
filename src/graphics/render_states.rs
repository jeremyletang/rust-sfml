use csfml_graphics_sys as ffi;
use graphics::{BlendMode, Shader, TextureRef, Transform};
use std::ptr;
use system::raw_conv::Raw;

/// Define the states used for drawing to a `RenderTarget`.
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
/// ```ignore
/// window.draw(sprite);
/// ```
///
/// To draw with a specific render state, use `RenderTarget::draw_with_renderstates`.
///
/// ```no_run
/// # use sfml::graphics::*;
/// # let mut window: RenderWindow = unimplemented!();
/// # let shader: Shader = unimplemented!();
/// # let sprite: Sprite = unimplemented!();
/// let mut states = RenderStates::default();
/// states.shader = Some(&shader);
/// window.draw_with_renderstates(&sprite, states);
/// ```
///
/// When you're inside the `draw` function of a drawable object (implementing `Drawable`),
/// you can either pass the render states unmodified, or change some of them.
/// For example, a transformable object will combine the current transform with its own transform.
/// A sprite will set its texture. Etc.
#[derive(Default)]
pub struct RenderStates<'te, 'sh, 'shte>
    where 'shte: 'sh
{
    /// Blending mode.
    pub blend_mode: BlendMode,
    /// Transform
    pub transform: Transform,
    /// Texture
    pub texture: Option<&'te TextureRef>,
    /// Shader
    pub shader: Option<&'sh Shader<'shte>>,
}

impl<'te, 'sh, 'shte> RenderStates<'te, 'sh, 'shte> {
    /// Create a new RenderStates.
    ///
    /// # Arguments
    /// * blend_mode - The BlendMode
    /// * transform - The transform
    /// * texture - Some(texture) if there is a texture, None otherwise
    /// * shader - Some(shader) if there is a shader, None otherwise
    ///
    /// Return a new default RenderStates
    pub fn new(blend_mode: BlendMode,
               transform: Transform,
               texture: Option<&'te TextureRef>,
               shader: Option<&'sh Shader<'shte>>)
               -> RenderStates<'te, 'sh, 'shte> {
        RenderStates {
            blend_mode: blend_mode,
            transform: transform,
            texture: texture,
            shader: shader,
        }
    }
}

impl<'te, 'sh, 'shte> Raw for RenderStates<'te, 'sh, 'shte> {
    type Raw = ffi::sfRenderStates;
    fn raw(&self) -> Self::Raw {
        ffi::sfRenderStates {
            blendMode: self.blend_mode.raw(),
            transform: self.transform.0,
            texture: match self.texture {
                Some(texture) => texture.raw(),
                None => ptr::null_mut(),
            },
            shader: match self.shader {
                Some(shader) => shader.raw(),
                None => ptr::null_mut(),
            },
        }
    }
}
