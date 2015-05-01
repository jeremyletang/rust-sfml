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

//! Available styles applicable to windows.

use std::default::Default;

// Manual #[doc] tags are to work around apparent bitflags! bug where the three
// slashes are included in the resulting docs.

bitflags! {
	#[doc="Available styles applicable to windows."]
	#[derive(Debug)]
	#[repr(C)]
	flags WindowStyle: u32 {
		#[doc="No decorations (cannot be combined with other flags)."]
		const NO_STYLE = 0,
		#[doc="Title bar and fixed border."]
		const TITLEBAR = 1,
		#[doc="Title bar, resizable border, and maximize button."]
		const RESIZE = 2,
		#[doc="Title bar and close button."]
		const CLOSE = 4,
		#[doc="Fullscreen mode (ignores other flags)."]
		const FULLSCREEN = 8,
		#[doc="Default window style: title bar, resizable border, and close button."]
		const DEFAULT_STYLE = 1 | 2 | 4
	}
}

impl Default for WindowStyle {
	fn default() -> WindowStyle {
		DEFAULT_STYLE
	}
}
