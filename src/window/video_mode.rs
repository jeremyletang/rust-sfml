/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
extern mod std;
pub use extra::c_vec::{CVec, len, get};
use std::libc::{c_uint, c_int, size_t};

/**
* VideoMode defines a video mode (width, height, bpp, frequency) 
*
* Provides functions for getting modes supported by the display device
*
*/

pub struct VideoMode {
    Width: uint,
    Height: uint,
    BitsPerPixel: uint
}

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_uint, size_t};
    use rsfml::sfTypes::{sfBool};

    pub struct sfVideoMode {
        Width: c_uint,
        Height: c_uint,
        BitsPerPixel: c_uint 
    }
    
    pub extern "C" {
        fn sfVideoMode_getDesktopMode() -> sfVideoMode;
        fn sfVideoMode_getFullscreenModes(Count : *size_t) -> *sfVideoMode;
        fn sfVideoMode_isValid(mode : sfVideoMode) -> sfBool;
    }
}

impl VideoMode {

    /**
    * Default constructor for class VideoMode.
    */
    pub fn new() -> VideoMode {
        VideoMode{Width : 0, Height : 0, BitsPerPixel : 0}
    }
    
    /**
    * Constructor with parameters for class VideoMode.
    */
    pub fn new_init(width : uint, height : uint, bitsPerPixel : uint) -> VideoMode {
        VideoMode{Width : width, Height : height, BitsPerPixel : bitsPerPixel}
    } 
    
    /**
    * Method who verify if the VideoMode is a valide SFML video mode.
    */
    pub fn is_valid(&self) -> bool {
        let i : c_int;
        unsafe {
            i = csfml::sfVideoMode_isValid(csfml::sfVideoMode{Width : self.Width as c_uint, Height : self.Height as c_uint, BitsPerPixel : self.BitsPerPixel as c_uint});
        }
        let tmp : bool;
        match i {
            0 => tmp =  false,
            1 => tmp = true,
            _ => tmp = true
        }
        return tmp;
    }

    /**
    * Static Method who get the Desktop default video mode.
    */
    pub fn get_desktop_mode() -> VideoMode {
        let mode: csfml::sfVideoMode;
        unsafe {
           mode = csfml::sfVideoMode_getDesktopMode();
        }
        return VideoMode{Width : mode.Width as uint, Height : mode.Height as uint, BitsPerPixel : mode.BitsPerPixel as uint};
    }

    /**
    * Static Method to convert a VideoMode to C struct sfVideoMode.
    */
    #[doc(hidden)]
    pub fn unwrap(mode: VideoMode) -> csfml::sfVideoMode {
        csfml::sfVideoMode{Width : mode.Width as c_uint, Height : mode.Height as c_uint, BitsPerPixel : mode.BitsPerPixel as c_uint}
    }

    /**
    * Static Method to convert a C struct sfVideoMode VideoMode.
    */
    #[doc(hidden)]
    pub fn wrap(mode: csfml::sfVideoMode) -> VideoMode {
        VideoMode{Width : mode.Width as uint, Height : mode.Height as uint, BitsPerPixel : mode.BitsPerPixel as uint}
    }

    /**
    * Static Method who retrieve all the supported video fullscreen mode.
    */
    pub fn get_fullscreen_modes() -> Option<~[VideoMode]> {
        let i : size_t = 0;
        let mut ret_tab : ~[VideoMode] = ~[];
        unsafe {
            let tab : *mut csfml::sfVideoMode = csfml::sfVideoMode_getFullscreenModes(&i) as *mut csfml::sfVideoMode;
                if i == 0 {
                    return None;
                }
                let cvec = CVec(tab, i as uint);
                let mut d : uint = 0;
                ret_tab.push(VideoMode::wrap(get(cvec, d)));
                d += 1;
                while d != i as uint {
                    ret_tab.push(VideoMode::wrap(get(cvec, d)));
                    d += 1;
                }
            }
        return Some(ret_tab);
    }
} 