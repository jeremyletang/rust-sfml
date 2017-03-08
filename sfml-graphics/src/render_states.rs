// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Define the states used for drawing to a `RenderTarget`

use std::ptr;

use sfml::system::raw_conv::{Raw, RawMut};
use {BlendMode, blend_mode, Shader, Texture, Transform};

use csfml_graphics_sys as ffi;

/// Define the states used for drawing to a `RenderTarget`
pub struct RenderStates<'s> {
    sf_render_states: ffi::sfRenderStates,
    /// Blending mode.
    pub blend_mode: BlendMode,
    /// Transform
    pub transform: Transform,
    /// Texture
    pub texture: Option<&'s Texture>,
    /// Shader
    pub shader: Option<&'s Shader<'s>>,
}

impl<'s> RenderStates<'s> {
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
               texture: Option<&'s Texture>,
               shader: Option<&'s Shader<'s>>)
               -> RenderStates<'s> {
        RenderStates {
            sf_render_states: ffi::sfRenderStates {
                blendMode: blend_mode.raw(),
                transform: transform.0,
                texture: ptr::null_mut(),
                shader: ptr::null_mut(),
            },
            blend_mode: blend_mode,
            transform: transform,
            texture: texture,
            shader: shader,
        }
    }
}

impl<'s> RawMut for RenderStates<'s> {
    type Raw = *mut ffi::sfRenderStates;
    fn raw_mut(&mut self) -> Self::Raw {
        self.sf_render_states.blendMode = self.blend_mode.raw();
        self.sf_render_states.transform = self.transform.0;
        self.sf_render_states.texture = match self.texture {
            Some(texture) => texture.raw(),
            None => ptr::null_mut(),
        };
        self.sf_render_states.shader = match self.shader {
            Some(shader) => shader.raw(),
            None => ptr::null_mut(),
        };

        &mut self.sf_render_states as *mut ffi::sfRenderStates
    }
}

/// Create a new `RenderStates` initialized to default.
///
/// # default
/// * `blend_mode` is initialized to `BlendMode::blend_alpha()`
/// * `transform` is initialized to the identity matrix
/// * `texture` is initialized to `None`
/// * `shader` is initialized to `None`
///
/// Return a new default `RenderStates`
impl<'s> Default for RenderStates<'s> {
    fn default() -> RenderStates<'s> {
        RenderStates {
            sf_render_states: ffi::sfRenderStates {
                blendMode: blend_mode::ALPHA.raw(),
                transform: Transform::new_identity().0,
                texture: ptr::null_mut(),
                shader: ptr::null_mut(),
            },
            blend_mode: blend_mode::ALPHA,
            transform: Transform::new_identity(),
            texture: None,
            shader: None,
        }
    }
}
