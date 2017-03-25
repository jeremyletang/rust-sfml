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

use std::ptr;

use system::raw_conv::Raw;
use graphics::{BlendMode, blend_mode, Shader, Texture, Transform};

use csfml_graphics_sys as ffi;

/// Define the states used for drawing to a `RenderTarget`
pub struct RenderStates<'te, 'sh, 'shte>
    where 'shte: 'sh
{
    /// Blending mode.
    pub blend_mode: BlendMode,
    /// Transform
    pub transform: Transform,
    /// Texture
    pub texture: Option<&'te Texture>,
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
               texture: Option<&'te Texture>,
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

impl<'te, 'sh, 'shte> Default for RenderStates<'te, 'sh, 'shte> {
    /// Default values:
    ///
    /// ```ignore
    /// blend_mode: blend_mode::ALPHA,
    /// transform: Transform::new_identity(),
    /// texture: None,
    /// shader: None,
    /// ```
    fn default() -> Self {
        Self {
            blend_mode: blend_mode::ALPHA,
            transform: Transform::new_identity(),
            texture: None,
            shader: None,
        }
    }
}
