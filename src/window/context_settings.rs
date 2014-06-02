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

/**
 * OpenGL context settings
 *
 * Structure defining the window's creation settings
 */

#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord, Show)]
pub struct ContextSettings {
    /// Bits of the depth buffer.
    pub depth_bits:         u32,
    /// Bits of the stencil buffer.
    pub stencil_bits:       u32,
    /// Level of antialiasing.
    pub antialiasing_level: u32,
    /// Major number of the context version
    pub major_version:      u32,
    /// Minor number of the context version
    pub minor_version:      u32
}

impl ContextSettings {
    /**
     * Create a default ContextSettings
     *
     * # Default values:
     * * `depth_bits`: 0
     * * `stencil_bits`: 0
     * * `antialiasing_level`: 0
     * * `major_version`: 2
     * * `minor_version`: 0
     */
    pub fn default() -> ContextSettings {
        ContextSettings {
            depth_bits :            0,
            stencil_bits :          0,
            antialiasing_level :    0,
            major_version :         2,
            minor_version :         0
        }
    }
}
