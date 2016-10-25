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

use csfml_window_sys as ffi;

/// Non-debug, compatibility context (this and the core attribute are mutually exclusive).
pub const CONTEXT_DEFAULT: u32 = 0;
/// Core attribute.
pub const CONTEXT_CORE: u32 = 1;
/// Debug attribute.
pub const CONTEXT_DEBUG: u32 = 1 << 2;

/// OpenGL context settings
///
/// Structure defining the window's creation settings
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct ContextSettings(pub ffi::sfContextSettings);

impl ContextSettings {
    /// Creates a new, default ContextSettings.
    pub fn new() -> ContextSettings {
        Default::default()
    }

    /// Sets the antialiasing level.
    pub fn antialiasing(&mut self, level: u32) -> &mut Self {
        self.0.antialiasingLevel = level;
        self
    }
}

impl Default for ContextSettings {
    /// Creates a `ContextSettings` with the following values:
    ///
    /// ```ignore
    /// depthBits: 0,
    /// stencilBits: 0,
    /// antialiasingLevel: 0,
    /// majorVersion: 2,
    /// minorVersion: 0,
    /// attributeFlags: CONTEXT_DEFAULT,
    /// ```
    fn default() -> ContextSettings {
        ContextSettings(ffi::sfContextSettings {
            depthBits: 0,
            stencilBits: 0,
            antialiasingLevel: 0,
            majorVersion: 2,
            minorVersion: 0,
            attributeFlags: CONTEXT_DEFAULT,
        })
    }
}
