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
 * OpenGL contextSettings
 *
 * Structure defining the window's creation settings
 *
 */


pub struct ContextSettings {
    depth_bits: 		uint,
    stencil_bits: 		uint,
    antialiasing_level: uint,
    major_version: 		uint,
    minor_version: 		uint
}

impl ContextSettings {
	pub fn default() -> ContextSettings {
		ContextSettings {
			depth_bits : 			0,
			stencil_bits : 			0,
			antialiasing_level : 	0,
			major_version : 		2,
			minor_version : 		0
		}
	}
}