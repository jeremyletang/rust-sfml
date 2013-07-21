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

/*!
* Define the states used for drawing to a RenderTarget
*
*
*/

use std::ptr;

use traits::wrappable::Wrappable;
use graphics::blend_mode::*; 
use graphics::shader::*; 
use graphics::texture::*; 
use graphics::transform::*; 

#[doc(hidden)]
pub mod ffi {
    
    use graphics::shader; 
    use graphics::texture; 
    use graphics::transform; 

    pub struct sfRenderStates {
        blendMode : i32,
        transform : transform::Transform,
        texture : *texture::ffi::sfTexture,
        shader : *shader::ffi::sfShader
    }
}

/**
* brief Define the states used for drawing to a RenderTarget
*/
pub struct RenderStates {
    priv sfRenderStates : ffi::sfRenderStates,
    blendMode : BlendMode,
    transform : Transform,
    texture : Option<@Texture>,
    shader : Option<@Shader>
}

impl RenderStates {

    /**
    * Create a new RenderStates initialized to default.
    *
    * # default
    * * blendMode is initialized to BlendAlpha
    * * transform is initialized to the identity matrix
    * * texture is initialized to None
    * * shader is initialized to None
    *
    * Return a new default RenderStates
    */
    pub fn default() -> RenderStates {
        RenderStates {
            sfRenderStates : ffi::sfRenderStates {
                blendMode : BlendAlpha as i32,
                transform : Transform::new_identity(),
                texture : ptr::null(),
                shader : ptr::null()
            },
            blendMode : BlendAlpha,
            transform : Transform::new_identity(),
            texture : None,
            shader : None
        }
    }

    /**
    * Internal rsfml use only
    */
    pub fn unwrap(&mut self) -> *ffi::sfRenderStates {
        self.sfRenderStates.blendMode = self.blendMode as i32;
        self.sfRenderStates.transform = self.transform;
        self.sfRenderStates.texture = if !self.texture.is_none() { self.texture.unwrap().unwrap() } else { ptr::null() };
        self.sfRenderStates.shader = if !self.shader.is_none() { self.shader.unwrap().unwrap() } else { ptr::null() };
        
        ptr::to_unsafe_ptr(&self.sfRenderStates)
    }
} 