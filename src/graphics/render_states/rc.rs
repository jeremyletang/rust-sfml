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

#![allow(non_snake_case)]

use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

use traits::Wrappable;
use graphics::{BlendMode, BlendAlpha, Texture, rc, Transform};

use ffi::graphics::render_states as ffi;

/// Define the states used for drawing to a RenderTarget
pub struct RenderStates {
    #[doc(hidden)]
    sfRenderStates: ffi::sfRenderStates,
    /// Blending mode.
    pub blendMode: BlendMode,
    /// Transform
    pub transform: Transform,
    /// Texture
    pub texture: Option<Rc<RefCell<Texture>>>,
    /// Shader
    pub shader: Option<Rc<RefCell<rc::Shader>>>
}

impl RenderStates {

    /// Create a new RenderStates.
    ///
    /// # Arguments
    /// * blend_mode - The BlendMode
    /// * transform - The transform
    /// * texture - Some(texture) if there is a texture, None otherwise
    /// * shader - Some(shader) if there is a shader, None otherwise
    ///
    /// Return a new initialized RenderState
    pub fn new(blend_mode: BlendMode,
        transform: Transform,
        texture: Option<Rc<RefCell<Texture>>>,
        shader: Option<Rc<RefCell<rc::Shader>>>) -> RenderStates {

        RenderStates {
            sfRenderStates: ffi::sfRenderStates {
                blendMode: blend_mode as i32,
                transform: transform,
                texture: ptr::mut_null(),
                shader: ptr::mut_null()
            },
            blendMode: blend_mode,
            transform: transform,
            texture: texture,
            shader: shader
        }
    }

    /// Create a new RenderStates initialized to default.
    ///
    /// # default
    /// * blendMode is initialized to BlendAlpha
    /// * transform is initialized to the identity matrix
    /// * texture is initialized to None
    /// * shader is initialized to None
    ///
    /// Return a new default RenderStates
    pub fn default() -> RenderStates {
        RenderStates {
            sfRenderStates: ffi::sfRenderStates {
                blendMode: BlendAlpha as i32,
                transform: Transform::new_identity(),
                texture: ptr::mut_null(),
                shader: ptr::mut_null()
            },
            blendMode: BlendAlpha,
            transform: Transform::new_identity(),
            texture: None,
            shader: None
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&mut self) -> *mut ffi::sfRenderStates {
        self.sfRenderStates.blendMode = self.blendMode as i32;
        self.sfRenderStates.transform = self.transform;
        self.sfRenderStates.texture = if !self.texture.is_none() {
            self.texture.as_ref().unwrap().borrow().unwrap()
        } else {
            ptr::mut_null()
        };
        self.sfRenderStates.shader = if !self.shader.is_none() {
            self.shader.as_ref().unwrap().borrow().unwrap()
        } else {
            ptr::mut_null()
        };

        &mut self.sfRenderStates as *mut ffi::sfRenderStates
    }
}
