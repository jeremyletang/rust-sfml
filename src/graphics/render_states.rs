/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

use std::ptr;

use graphics::{BlendMode, Shader, Texture, Transform};

use ffi::graphics as ffi;

/// Defines the states used for drawing to a `RenderTarget`.
///
/// High-level objects such as sprites or text force some of these states when
/// they are drawn. For example, a sprite will set its own texture, so that you
/// don't have to care about it when drawing the sprite.
///
/// The transform is a special case: sprites, texts and shapes (and it's a good
/// idea to do it with your own drawable classes too) combine their transform
/// with the one that is passed in the `RenderStates` structure, so that you can
/// use a "global" transform on top of each object's transform.
///
/// Most objects, especially high-level drawables, can be drawn directly without
/// defining render states explicitly – the default set of states is ok in most
/// cases.
///
/// When you're inside the `draw()` function of an implementation of `Drawable`,
/// you can either pass the render states unmodified, or change some of them.
/// For example, a transformable object will combine the current transform with
/// its own transform, a sprite will set its texture, and so on.
///
/// To easily override just part of an existing RenderStates, code like the
/// following may be used:
///
/// ```ignore
/// // Applying a texture:
/// fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
///     self.vertices.draw(target, &RenderStates { texture: self.texture, .. *states });
/// }
/// // Combining transforms:
/// fn draw(&self, target: &mut RenderTarget, states: &RenderStates) {
///     let transform = states.transform.combine(self.transform);
///     self.child.draw(target, &RenderStates { transform: transform, .. *states });
/// }
/// ```
#[derive(Clone, Default)]
pub struct RenderStates<'s> {
    /// How pixels of objects are blended with the background.
    pub blend_mode: BlendMode,
    /// How objects are positioned/rotated/scaled.
    pub transform: Transform,
    /// What texture, if any, is applied to objects.
    pub texture: Option<&'s Texture>,
    /// What custom effect, if any, is applied to objects.
    pub shader: Option<&'s Shader<'s>>
}

impl<'s> RenderStates<'s> {
    /// Create a new RenderStates with the given fields.
    ///
	/// Specifying a texture and shader is optional; use None if no texture or
	/// shader is desired.
    pub fn new(blend_mode: BlendMode,
               transform: Transform,
               texture: Option<&'s Texture>,
               shader: Option<&'s Shader<'s>>) -> RenderStates<'s> {
        RenderStates {
            blend_mode: blend_mode,
            transform: transform,
            texture: texture,
            shader: shader
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> ffi::sfRenderStates {
		ffi::sfRenderStates {
			blendMode: self.blend_mode,
			transform: self.transform,
			texture: self.texture.map_or(ptr::null(), |x| x.unwrap()),
			shader: self.shader.map_or(ptr::null(), |x| x.unwrap())
		}
    }
}
