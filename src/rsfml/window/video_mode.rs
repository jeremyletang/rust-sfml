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
* Defines VideoModes
*
* A video mode is defined by a width and a height (in pixels) and a depth (in bits per pixel).
* Video modes are used to setup windows at creation time.
*
*/

use extra::c_vec::CVec;
use std::libc::{c_uint, size_t};

use traits::Wrappable;

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi = ffi::window::video_mode;

/**
* VideoMode defines a video mode (width, height, bpp, frequency) 
*
* Provides functions for getting modes supported by the display device
*
*/
#[deriving(Clone, Eq, Ord, Show, ToStr)]
pub struct VideoMode {
    /// Video mode width, in pixels.
    width:          uint,
    /// Video mode height, in pixels.
    height:         uint,
    /// Video mode pixel depth, in bits per pixels. 
    bits_per_pixel: uint
}

impl VideoMode {
    /**
    * Default constructor for class VideoMode.
    *
    * Return a new VideoMode
    */
    pub fn new() -> VideoMode {
        VideoMode{
            width :             0, 
            height :            0,
            bits_per_pixel :    0
        }
    }

    /**
    * Constructor with parameters for class VideoMode.
    *
    * Return a new VideoMode initialized
    */
    pub fn new_init(width : uint, height : uint, bits_per_pixel : uint) -> VideoMode {
        VideoMode{
            width :             width,
            height :            height,
            bits_per_pixel :    bits_per_pixel
        }
    }

    /**
    * Tell whether or not a video mode is valid
    *
    * The validity of video modes is only relevant when using
    * fullscreen windows; otherwise any video mode can be used
    * with no restriction.
    *
    * return true if the video mode is valid for fullscreen mode
    */
    pub fn is_valid(&self) -> bool {
        let i =  unsafe { ffi::sfVideoMode_isValid(ffi::sfVideoMode{
            width : self.width as c_uint, 
            height : self.height as c_uint, 
            bits_per_pixel : self.bits_per_pixel as c_uint
        }) };
         match i {
             SFFALSE => false,
             SFTRUE  => true
        }
     }

    /**
    * Static Method, get the current desktop video mode
    *
    * return the urrent desktop video mode
    */
    pub fn get_desktop_mode() -> VideoMode {
        let mode = unsafe { ffi::sfVideoMode_getDesktopMode() };
        VideoMode{
            width :             mode.width as uint, 
            height :            mode.height as uint, 
            bits_per_pixel :    mode.bits_per_pixel as uint
        }
    }

    /**
    * Static Method, retrieve all the video modes supported in fullscreen mode
    *
    * When creating a fullscreen window, the video mode is restricted
    * to be compatible with what the graphics driver and monitor
    * support. This function returns the complete list of all video
    * modes that can be used in fullscreen mode.
    * The returned array is sorted from best to worst, so that
    * the first element will always give the best mode (higher
    * width, height and bits_per_pixel).
    *
    * Return a vector containing all the supported VideoMode
    */
    pub fn get_fullscreen_modes() -> Option<~[VideoMode]> {
        let i : size_t = 0;
        let mut ret_tab : ~[VideoMode] = ~[];
        unsafe {
            let tab : *mut ffi::sfVideoMode = ffi::sfVideoMode_getFullscreenModes(&i) as *mut ffi::sfVideoMode;
                if i == 0 {
                    return None;
                }
                let cvec = CVec::new(tab, i as uint);
                let mut d : uint = 0;
                ret_tab.push(Wrappable::wrap(*cvec.get(d)));
                d += 1;
                while d != i as uint {
                    ret_tab.push(Wrappable::wrap(*cvec.get(d)));
                    d += 1;
                }
            }
        Some(ret_tab)
    }
}

#[doc(hidden)]
impl Wrappable<ffi::sfVideoMode> for VideoMode {
    fn wrap(mode: ffi::sfVideoMode) -> VideoMode {
        VideoMode{
            width :             mode.width as uint,
            height :            mode.height as uint,
            bits_per_pixel :    mode.bits_per_pixel as uint
        }
    }

    fn unwrap(&self) -> ffi::sfVideoMode {
        ffi::sfVideoMode{
            width :             self.width as c_uint,
            height :            self.height as c_uint,
            bits_per_pixel :    self.bits_per_pixel as c_uint
        }
    }
}
