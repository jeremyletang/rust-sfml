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

//! Define the states used for drawing to a RenderTarget

use std::ptr;

use traits::Wrappable;
use graphics::{BlendMode, Shader, Texture, Transform};

use ffi::graphics as ffi;

/// Define the states used for drawing to a RenderTarget
#[derive(Clone)]
pub struct RenderStates<'s> {
    /// Blending mode.
    pub blend_mode: BlendMode,
    /// Transform
    pub transform: Transform,
    /// Texture
    pub texture: Option<&'s Texture>,
    /// Shader
    pub shader: Option<&'s Shader<'s>>
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
               shader: Option<&'s Shader<'s>>) -> RenderStates<'s> {
        RenderStates {
            blend_mode: blend_mode,
            transform: transform,
            texture: texture,
            shader: shader
        }
    }

    /// Create a new RenderStates initialized to default.
    ///
    /// # default
    /// * blend_mode is initialized to BlendAlpha
    /// * transform is initialized to the identity matrix
    /// * texture is initialized to None
    /// * shader is initialized to None
    ///
    /// Return a new default RenderStates
    pub fn default() -> RenderStates<'s> {
        RenderStates {
            blend_mode: BlendMode::alpha(),
            transform: Transform::new_identity(),
            texture: None,
            shader: None
        }
    }

    // Internal rust-sfml use only
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
