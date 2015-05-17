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

use libc::size_t;
use ffi::window as ffi;

/// A video mode composed of width, height, and bits per pixel.
///
/// Video modes are provided to windows at creation time to determine their
/// size. Video modes are most relevant for fullscreen mode: one of the valid
/// video modes allowed by the OS (defined by what the monitor and graphics card
/// support) must be used, otherwise window creation will fail.
#[repr(C)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct VideoMode {
    /// Video mode width, in pixels.
    pub width: u32,
    /// Video mode height, in pixels.
    pub height: u32,
    /// Video mode pixel depth, in bits per pixels.
    pub bits_per_pixel: u32
}

impl VideoMode {
	/// Construct a VideoMode with the given width and height, and 32 bits per
	/// pixel.
	#[inline]
    pub fn new(width: u32, height: u32) -> VideoMode {
		VideoMode::new_bpp(width, height, 32)
    }

	/// Construct a VideoMode with the given width, height, and bits per pixel.
	#[inline]
    pub fn new_bpp(width: u32, height: u32, bits_per_pixel: u32) -> VideoMode {
        VideoMode {
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel
        }
    }

    /// Tell whether or not a video mode is valid.
    ///
    /// The validity of video modes is only relevant when using
    /// fullscreen windows; otherwise any video mode can be used
    /// with no restriction.
    pub fn is_valid(&self) -> bool {
        unsafe { ffi::sfVideoMode_isValid(*self) }.to_bool()
    }

	/// Get the current desktop video mode.
    pub fn get_desktop_mode() -> VideoMode {
		unsafe { ffi::sfVideoMode_getDesktopMode() }
    }

    /// Retrieve all the video modes supported in fullscreen mode.
    ///
    /// When creating a fullscreen window, the video mode is restricted
    /// to be compatible with what the graphics driver and monitor
    /// support. This function returns the complete list of all video
    /// modes that can be used in fullscreen mode.
    /// The returned array is sorted from best to worst, so that
    /// the first element will always give the best mode (higher
    /// width, height and bits-per-pixel).
    pub fn get_fullscreen_modes() -> Vec<VideoMode> {
        let mut size: size_t = 0;
        let table = unsafe {
            ffi::sfVideoMode_getFullscreenModes(&mut size)
        };
		if size == 0 {
			Vec::new()
		} else {
			unsafe {
				::std::slice::from_raw_parts(table, size as usize)
			}.to_vec()
		}
	}
}
