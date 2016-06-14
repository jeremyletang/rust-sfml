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

//! Available blending modes for drawing
use csfml_graphics_sys as ffi;

use csfml_graphics_sys::sfBlendEquation::*;
use csfml_graphics_sys::sfBlendFactor::*;

/// Available Blending modes for drawing.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct BlendMode(pub ffi::sfBlendMode);

impl BlendMode {
    /// Create a new BlendMode
    pub fn new(col_src: ffi::sfBlendFactor,
               col_dst: ffi::sfBlendFactor,
               col_equ: ffi::sfBlendEquation,
               alpha_src: ffi::sfBlendFactor,
               alpha_dst: ffi::sfBlendFactor,
               alpha_equ: ffi::sfBlendEquation)
               -> BlendMode {
        BlendMode(ffi::sfBlendMode {
            colorSrcFactor: col_src,
            colorDstFactor: col_dst,
            colorEquation: col_equ,
            alphaSrcFactor: alpha_src,
            alphaDstFactor: alpha_dst,
            alphaEquation: alpha_equ,
        })
    }

    /// BlendMode ALPHA
    pub fn blend_alpha() -> BlendMode {
        BlendMode::new(sfBlendFactorSrcAlpha,
                       sfBlendFactorOneMinusSrcAlpha,
                       sfBlendEquationAdd,
                       sfBlendFactorOne,
                       sfBlendFactorOneMinusSrcAlpha,
                       sfBlendEquationAdd)
    }

    /// BlendMode ADD
    pub fn blend_add() -> BlendMode {
        BlendMode::new(sfBlendFactorSrcAlpha,
                       sfBlendFactorOne,
                       sfBlendEquationAdd,
                       sfBlendFactorOne,
                       sfBlendFactorOne,
                       sfBlendEquationAdd)
    }

    /// BlendMode MULTIPLY
    pub fn blend_multiply() -> BlendMode {
        BlendMode::new(sfBlendFactorDstColor,
                       sfBlendFactorZero,
                       sfBlendEquationAdd,
                       sfBlendFactorDstColor,
                       sfBlendFactorZero,
                       sfBlendEquationAdd)
    }
    /// BlendMode NONE
    pub fn blend_none() -> BlendMode {
        BlendMode::new(sfBlendFactorOne,
                       sfBlendFactorZero,
                       sfBlendEquationAdd,
                       sfBlendFactorOne,
                       sfBlendFactorZero,
                       sfBlendEquationAdd)
    }
}
