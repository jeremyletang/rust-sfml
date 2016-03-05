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

use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

use graphics::{BlendMode, Texture, rc, Transform};

use ffi::graphics::render_states as ffi;

/// Define the states used for drawing to a RenderTarget
pub struct RenderStates {
    sf_render_states: ffi::sfRenderStates,
    /// Blending mode.
    pub blend_mode: BlendMode,
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
                sf_render_states: ffi::sfRenderStates {
                blendMode: blend_mode as i32,
                transform: transform,
                texture: ptr::null_mut(),
                shader: ptr::null_mut()
            },
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
    pub fn default() -> RenderStates {
        RenderStates {
                sf_render_states: ffi::sfRenderStates {
                blendMode: BlendMode::Alpha as i32,
                transform: Transform::new_identity(),
                texture: ptr::null_mut(),
                shader: ptr::null_mut()
            },
            blend_mode: BlendMode::Alpha,
            transform: Transform::new_identity(),
            texture: None,
            shader: None
        }
    }

    #[doc(hidden)]
    pub fn unwrap(&mut self) -> *mut ffi::sfRenderStates {
        self.sf_render_states.blendMode = self.blend_mode as i32;
        self.sf_render_states.transform = self.transform;
        self.sf_render_states.texture = if !self.texture.is_none() {
            self.texture.as_ref().unwrap().borrow().unwrap()
        } else {
            ptr::null_mut()
        };
        self.sf_render_states.shader = if !self.shader.is_none() {
            self.shader.as_ref().unwrap().borrow().unwrap()
        } else {
            ptr::null_mut()
        };

        &mut self.sf_render_states as *mut ffi::sfRenderStates
    }
}
