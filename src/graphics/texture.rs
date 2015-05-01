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

//! Image used for drawing
//!
//! Texture stores pixels that can be drawn, with a sprite for example.

use libc::{c_uint, size_t};
use std::ptr;
use std::ffi::CString;

use graphics::{RenderWindow, Image, IntRect};
use system::Vector2u;
use window::Window;

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Image used for drawing
///
/// Texture stores pixels that can be drawn, with a sprite for example.
pub struct Texture(Foreign<ffi::sfTexture>);

impl Texture {
    /// Create a new texture
    ///
    /// # Arguments
    /// * width - Texture width
    /// * height - Texture height
    ///
    /// Return Some(Texture) or None
    pub fn new(width: u32, height: u32) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_create(width as c_uint, height as c_uint))
		}.map(Texture)
    }

    /// Create a new texture from memory
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    /// * area - Area of the image to load
    ///
    /// Return Some(Texture) or None
    pub fn new_from_memory(mem: &[u8], area: &IntRect) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_createFromMemory(mem.as_ptr(), mem.len() as size_t, area))
		}.map(Texture)
    }

    /// Create a new texture from a file
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    ///
    /// Return Some(Texture) or None
    pub fn new_from_file(filename: &str) -> Option<Texture> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfTexture_createFromFile(c_str.as_ptr(), ptr::null()))
        }.map(Texture)
    }

    /// Create a new texture from a file with a given area
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    /// * area - Area of the source image to load
    ///
    /// Return Some(Texture) or None
    pub fn new_from_file_with_rect(filename: &str, area: &IntRect) -> Option<Texture> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfTexture_createFromFile(c_str.as_ptr(), area))
        }.map(Texture)
    }

    /// Create a new texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    ///
    /// Return Some(Texture) or None
    pub fn new_from_image(image: &Image) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_createFromImage(image.unwrap(), ptr::null()))
		}.map(Texture)
    }

    /// Create a new texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    /// * area - Area of the source image to load
    ///
    /// Return Some(Texture) or None
    pub fn new_from_image_with_rect(image: &Image, area: &IntRect) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_createFromImage(image.unwrap(), area))
		}.map(Texture)
    }
	
	#[doc(hidden)]
	pub unsafe fn clone_of(ptr: *const ffi::sfTexture) -> Option<Texture> {
		Foreign::new(ffi::sfTexture_copy(ptr)).map(Texture)
	}

	fn raw(&self) -> &ffi::sfTexture { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfTexture { self.0.as_mut() }
	#[doc(hidden)]
    pub fn unwrap(&self) -> &ffi::sfTexture { self.raw() }

    /// Create a new texture by copying a exitant one
    ///
    /// # Arguments
    /// * texture - Texture to copy
    ///
    /// Return Some(Texture) or None
    pub fn clone_opt(&self) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_copy(self.raw()))
		}.map(Texture)
	}

    /// Return the size of the texture
    ///
    /// Return the Size in pixels
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfTexture_getSize(self.raw())
        }
    }

    /// Update a texture from the contents of a window
    ///
    /// # Arguments
    /// * window - Window to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_window(&mut self,
                              window: &Window,
                              x: u32,
                              y: u32) -> () {
        unsafe {
            ffi::sfTexture_updateFromWindow(self.raw_mut(),
                                            window.unwrap(),
                                            x as c_uint,
                                            y as c_uint)
        }
    }

    /// Update a texture from the contents of a render window
    ///
    /// # Arguments
    /// * renderWindow - Render-window to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_render_window(&mut self,
                                     render_window: &RenderWindow,
                                     x: u32,
                                     y: u32) -> () {
        unsafe {
            ffi::sfTexture_updateFromRenderWindow(self.raw_mut(),
                                                  render_window.unwrap(),
                                                  x as c_uint,
                                                  y as c_uint)
        }
    }

    /// Update a texture from the contents of an image
    ///
    /// # Arguments
    /// * image - Image to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_image(&mut self,
                             image: &Image,
                             x: u32,
                             y: u32) -> () {
        unsafe {
            ffi::sfTexture_updateFromImage(self.raw_mut(),
                                           image.unwrap(),
                                           x as c_uint,
                                           y as c_uint)
        }
    }

    /// Update a texture from the contents of a Vector of pixels
    ///
    /// # Arguments
    /// * pixels - Pixels to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_pixels(&mut self,
                              pixels: &[u8],
                              width: u32,
                              height: u32,
                              x: u32,
                              y: u32) -> () {
        unsafe {
            ffi::sfTexture_updateFromPixels(self.raw_mut(),
                                            pixels.as_ptr(),
                                            width as c_uint,
                                            height as c_uint,
                                            x as c_uint,
                                            y as c_uint)
        }
    }

    /// Enable or disable the smooth filter on a texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) -> () {
        unsafe {
            ffi::sfTexture_setSmooth(self.raw_mut(), SfBool::from_bool(smooth))
        }
    }

    /// Tell whether the smooth filter is enabled or not for a texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfTexture_isSmooth(self.raw()) }.to_bool()
    }

    /// Enable or disable repeating for a texture
    ///
    /// epeating is involved when using texture coordinates
    /// outside the texture rectangle [0, 0, width, height].
    /// In this case, if repeat mode is enabled, the whole texture
    /// will be repeated as many times as needed to reach the
    /// coordinate (for example, if the X texture coordinate is
    /// 3 * width, the texture will be repeated 3 times).
    /// If repeat mode is disabled, the "extra space" will instead
    /// be filled with border pixels.
    /// Warning: on very old graphics cards, white pixels may appear
    /// when the texture is repeated. With such cards, repeat mode
    /// can be used reliably only if the texture has power-of-two
    /// dimensions (such as 256x128).
    /// Repeating is disabled by default.
    ///
    /// # Arguments
    /// * repeated  - true to repeat the texture, false to disable repeating
    pub fn set_repeated(&mut self, repeated: bool) -> () {
        unsafe {
            ffi::sfTexture_setRepeated(self.raw_mut(), SfBool::from_bool(repeated))
        }
    }

    /// Tell whether a texture is repeated or not
    ///
    /// Return frue if repeat mode is enabled, false if it is disabled
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfTexture_isRepeated(self.raw()) }.to_bool()
    }

    /// Bind a texture for rendering
    ///
    /// This function is not part of the graphics API, it mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix sfTexture with OpenGL code.
    pub fn bind(&mut self) -> () {
        unsafe {
            ffi::sfTexture_bind(self.raw_mut())
        }
    }

    /// Copy a texture's pixels to an image
    ///
    /// Return an image containing the texture's pixels
    pub fn copy_to_image(&self) -> Option<Image> {
		unsafe {
			Image::wrap(ffi::sfTexture_copyToImage(self.raw()))
		}
    }

    /// Get the maximum texture size allowed
    ///
    /// Return the maximum size allowed for textures, in pixels
    pub fn get_maximum_size() -> u32 {
        unsafe {
            ffi::sfTexture_getMaximumSize() as u32
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Texture {
		self.clone_opt().expect("Failed to clone Texture")
    }
}
