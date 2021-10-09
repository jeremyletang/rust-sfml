use crate::{
    ffi::graphics as ffi,
    graphics::{BlendMode, Shader, Texture, Transform},
};
use std::{marker::PhantomData, ptr};

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
/// states.set_shader(Some(&shader));
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
#[repr(transparent)]
pub struct RenderStates<'texture, 'shader, 'shader_texture: 'shader> {
    repr: ffi::graphics::sfRenderStates,
    _texture: PhantomData<&'texture Texture>,
    _shader: PhantomData<&'shader Shader<'shader_texture>>,
}

impl<'texture, 'shader, 'shader_texture> RenderStates<'texture, 'shader, 'shader_texture> {
    /// Create a new `RenderStates`.
    ///
    /// # Arguments
    /// * `blend_mode` - The `BlendMode`
    /// * transform - The transform
    /// * texture - Some(texture) if there is a texture, None otherwise
    /// * shader - Some(shader) if there is a shader, None otherwise
    ///
    /// Return a new default `RenderStates`
    #[must_use]
    pub fn new(
        blend_mode: BlendMode,
        transform: Transform,
        texture: Option<&'texture Texture>,
        shader: Option<&'shader Shader<'shader_texture>>,
    ) -> Self {
        Self {
            repr: ffi::sfRenderStates {
                blendMode: blend_mode,
                transform: transform.raw(),
                texture: match texture {
                    Some(tex) => tex.raw(),
                    None => ptr::null(),
                },
                shader: match shader {
                    Some(shader) => shader.raw(),
                    None => ptr::null(),
                },
            },
            _texture: PhantomData,
            _shader: PhantomData,
        }
    }
    pub(super) fn raw_ref(&self) -> *const ffi::sfRenderStates {
        let ptr: *const Self = self;
        ptr as *const ffi::sfRenderStates
    }
    /// Sets the blending mode
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.repr.blendMode = blend_mode;
    }
    /// Sets the transform
    pub fn set_transform(&mut self, transform: Transform) {
        self.repr.transform = transform.raw();
    }
    /// Sets the texture
    pub fn set_texture(&mut self, texture: Option<&'texture Texture>) {
        self.repr.texture = match texture {
            None => ptr::null(),
            Some(tex) => tex.raw(),
        };
    }
    /// Sets the shader
    pub fn set_shader(&mut self, shader: Option<&'shader Shader<'shader_texture>>) {
        self.repr.shader = match shader {
            None => ptr::null(),
            Some(shader) => shader.raw(),
        };
    }
}

impl RenderStates<'static, 'static, 'static> {
    /// The default render state.
    ///
    /// This can be used in a const context, unlike the [`Default`] implementation.
    pub const DEFAULT: Self = Self {
        repr: ffi::sfRenderStates {
            blendMode: BlendMode::ALPHA,
            transform: Transform::IDENTITY.raw(),
            texture: ptr::null(),
            shader: ptr::null(),
        },
        _texture: PhantomData,
        _shader: PhantomData,
    };
}

impl Default for RenderStates<'static, 'static, 'static> {
    fn default() -> Self {
        Self::DEFAULT
    }
}
