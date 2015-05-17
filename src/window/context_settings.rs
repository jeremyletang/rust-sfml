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

/// Settings for an OpenGL context attached to a window.
///
/// Except for the antialiasing level, these settings have no impact on regular
/// SFML rendering (the graphics module), so usually the defaults will suffice.
///
/// Only OpenGL versions greater or equal to 3.0 are relevant; versions lesser
/// than 3.0 are all handled the same way (i.e. you can use any version < 3.0 if
/// you don't want an OpenGL 3 context).
///
/// Please note that these values are only a hint. No failure will be reported
/// if one or more of these values are not supported by the system; instead,
/// SFML will try to find the closest valid match. You can then retrieve the
/// settings that the window actually used to create its context with
/// the `get_settings()` method.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct ContextSettings {
    /// Bits per pixel of the depth buffer.
    pub depth_bits: u32,
    /// Bits per pixel of the stencil buffer.
    pub stencil_bits: u32,
    /// The number of multisampling levels for antialiasing.
    pub antialiasing_level: u32,
    /// Major number of the OpenGL context version.
    pub major_version: u32,
    /// Minor number of the OpenGL context version.
    pub minor_version: u32
}

impl Default for ContextSettings {
	/// A default ContextSettings has the following values:
	///
    /// * `depth_bits`: 0
    /// * `stencil_bits`: 0
    /// * `antialiasing_level`: 0
    /// * `major_version`: 2
    /// * `minor_version`: 0
    fn default() -> ContextSettings {
        ContextSettings {
            depth_bits: 0,
            stencil_bits: 0,
            antialiasing_level: 0,
            major_version: 2,
            minor_version: 0
        }
    }
}
