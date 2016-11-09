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

use libc::{c_uint, size_t};
use std::vec::Vec;

use raw_conv::{Raw, FromRaw};

use csfml_window_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// `VideoMode` defines a video mode (width, height, bpp)
///
/// A video mode is defined by a width and a height (in pixels) and a depth (in bits per pixel).
///
/// Video modes are used to setup windows at creation time.
///
/// The main usage of video modes is for fullscreen mode: indeed you must use one of the valid
/// video modes allowed by the OS (which are defined by what the monitor and
/// the graphics card support), otherwise your window creation will just fail.
///
/// `VideoMode` provides an associated function for retrieving the list of all the video modes
/// supported by the system: `get_fullscreen_modes()`.
///
/// A custom video mode can also be checked directly for fullscreen compatibility
/// with its `is_valid()` function.
///
/// Additionally, `VideoMode` provides a static function to get the mode currently used by
/// the desktop: `get_desktop_mode`. This allows to build windows with the same size or
/// pixel depth as the current resolution.
///
/// # Usage example
///
/// ```
/// use sfml::window::{VideoMode, Window, style};
///
/// // Display the list of all the video modes available for fullscreen
/// let modes = VideoMode::get_fullscreen_modes();
///
/// for mode in modes {
///     println!("{:?}", mode);
/// }
///
/// // Create a window with the same pixel depth as the desktop
/// let desktop = VideoMode::get_desktop_mode();
/// let _window = Window::new(VideoMode::new(1024, 768, desktop.bits_per_pixel),
///                           "SFML window",
///                           style::CLOSE,
///                           &Default::default()).unwrap();
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
pub struct VideoMode {
    /// Video mode width, in pixels.
    pub width: u32,
    /// Video mode height, in pixels.
    pub height: u32,
    /// Video mode pixel depth, in bits per pixels.
    pub bits_per_pixel: u32,
}

impl VideoMode {
    /// Constructor with parameters for class VideoMode.
    ///
    /// Return a new VideoMode initialized
    pub fn new(width: u32, height: u32, bits_per_pixel: u32) -> VideoMode {
        VideoMode {
            width: width,
            height: height,
            bits_per_pixel: bits_per_pixel,
        }
    }

    /// Tell whether or not a video mode is valid
    ///
    /// The validity of video modes is only relevant when using
    /// fullscreen windows; otherwise any video mode can be used
    /// with no restriction.
    ///
    /// return true if the video mode is valid for fullscreen mode
    pub fn is_valid(&self) -> bool {
        unsafe {
                ffi::sfVideoMode_isValid(ffi::sfVideoMode {
                    width: self.width as c_uint,
                    height: self.height as c_uint,
                    bitsPerPixel: self.bits_per_pixel as c_uint,
                })
            }
            .to_bool()
    }

    /// Static Method, get the current desktop video mode
    ///
    /// return the urrent desktop video mode
    pub fn get_desktop_mode() -> VideoMode {
        let mode = unsafe { ffi::sfVideoMode_getDesktopMode() };
        VideoMode {
            width: mode.width as u32,
            height: mode.height as u32,
            bits_per_pixel: mode.bitsPerPixel as u32,
        }
    }

    /// Static Method, retrieve all the video modes supported in fullscreen mode
    ///
    /// When creating a fullscreen window, the video mode is restricted
    /// to be compatible with what the graphics driver and monitor
    /// support. This function returns the complete list of all video
    /// modes that can be used in fullscreen mode.
    /// The returned array is sorted from best to worst, so that
    /// the first element will always give the best mode (higher
    /// width, height and bits_per_pixel).
    ///
    /// Return a vector containing all the supported VideoMode
    pub fn get_fullscreen_modes() -> Vec<VideoMode> {
        let mut size: size_t = 0;
        let tab = unsafe { ffi::sfVideoMode_getFullscreenModes(&mut size) };
        if size == 0 {
            return Vec::new();
        }

        let size = size as u32;

        let tab_slice: &[ffi::sfVideoMode] =
            unsafe { ::std::slice::from_raw_parts(tab, size as usize) };

        let mut ret_tab = Vec::with_capacity(size as usize);

        for sf_video_mode in tab_slice.iter() {
            ret_tab.push(VideoMode::from_raw(*sf_video_mode));
        }

        ret_tab
    }
}

impl Raw for VideoMode {
    type Raw = ffi::sfVideoMode;
    fn raw(&self) -> ffi::sfVideoMode {
        ffi::sfVideoMode {
            width: self.width as c_uint,
            height: self.height as c_uint,
            bitsPerPixel: self.bits_per_pixel as c_uint,
        }
    }
}

impl FromRaw for VideoMode {
    fn from_raw(raw: Self::Raw) -> Self {
        VideoMode {
            width: raw.width as u32,
            height: raw.height as u32,
            bits_per_pixel: raw.bitsPerPixel as u32,
        }
    }
}

impl Default for VideoMode {
    fn default() -> Self {
        VideoMode {
            width: 0,
            height: 0,
            bits_per_pixel: 0,
        }
    }
}
