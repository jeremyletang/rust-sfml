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

//! Loading, manipulating and saving images.

use libc::{c_uint, size_t};
use std::ffi::CString;

use system::Vector2u;
use graphics::{Color, IntRect};

use ffi::{SfBool, Foreign};
use ffi::graphics as ffi;

/// Loading, manipulating and saving images.
pub struct Image(Foreign<ffi::sfImage>);

impl Image {
    /// Create an image
    ///
    /// This image is filled with black pixels.
    ///
    /// # Arguments
    /// * width - Width of the image
    /// * height - Height of the image
    ///
    /// Return Some(Image) or None
    pub fn new(width: u32, height: u32) -> Option<Image> {
        unsafe {
			Foreign::new(ffi::sfImage_create(width as c_uint, height as c_uint))
		}.map(Image)
    }

    /// Create an image from memory
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    ///
    /// Return Some(Image) or None
    pub fn new_from_memory(mem: &[u8]) -> Option<Image> {
        unsafe {
			Foreign::new(ffi::sfImage_createFromMemory(mem.as_ptr(), mem.len() as size_t))
		}.map(Image)
    }

    /// Create an image and fill it with a unique color
    ///
    /// # Arguments
    /// * width - Width of the image
    /// * height - Height of the image
    /// * color - Fill color
    ///
    /// Return Some(Image) or None
    pub fn new_from_color(width: u32,
                          height: u32,
                          color: &Color) -> Option<Image> {
        unsafe {
			Foreign::new(ffi::sfImage_createFromColor(width as c_uint, height as c_uint, *color))
		}.map(Image)
    }

    /// Create an image from a file on disk
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif,
    /// psd, hdr and pic. Some format options are not supported,
    /// like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    ///
    /// Return Some(Image) or None
    pub fn new_from_file(filename: &str) -> Option<Image> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfImage_createFromFile(c_str.as_ptr()))
        }.map(Image)
    }

    /// Copy an existing image
    ///
    /// Return Some(Image) or None
    pub fn clone_opt(&self) -> Option<Image> {
        unsafe {
			Foreign::new(ffi::sfImage_copy(self.raw()))
		}.map(Image)
    }

    /// Create an image from an vector of pixels
    ///
    /// The pixel vector is assumed to contain 32-bits RGBA pixels,
    /// and have the given width and height. If not, this is
    /// an undefined behaviour.
    ///
    /// # Arguments
    /// * width - Width of the image
    /// * height - Height of the image
    /// * pixels - Vector of pixels to copy to the image
    ///
    /// Return Some(Image) or None
    pub fn create_from_pixels(width: u32,
                              height: u32,
                              pixels: &[u8]) -> Option<Image> {
        unsafe {
			Foreign::new(ffi::sfImage_createFromPixels(width as c_uint,
				height as c_uint,
				pixels.as_ptr()))
		}.map(Image)
    }

	fn raw(&self) -> &ffi::sfImage { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfImage { self.0.as_mut() }
	#[doc(hidden)]
	pub fn unwrap(&self) -> &ffi::sfImage { self.raw() }
	#[doc(hidden)]
	pub unsafe fn wrap(ptr: *mut ffi::sfImage) -> Option<Image> {
		Foreign::new(ptr).map(Image)
	}

    /// Save an image to a file on disk
    ///
    /// The format of the image is automatically deduced from
    /// the extension. The supported image formats are bmp, png,
    /// tga and jpg. The destination file is overwritten
    /// if it already exists. This function fails if the image is empty.
    ///
    /// # Arguments
    /// * filename - Path of the file to save
    ///
    /// Return true if saving was successful
    pub fn save_to_file(&self, filename: &str) -> bool {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return false
		};
        unsafe { ffi::sfImage_saveToFile(self.raw(), c_str.as_ptr()) }.to_bool()
    }

    /// Return the size of an image
    ///
    /// Return the size in pixels
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfImage_getSize(self.raw())
        }
    }

    /// Create a transparency mask from a specified color-key
    ///
    /// This function sets the alpha value of every pixel matching
    /// the given color to alpha (0 by default), so that they
    /// become transparent.
    ///
    /// # Arguments
    /// * color - Color to make transparent
    /// * alpha - Alpha value to assign to transparent pixels
    pub fn create_mask_from_color(&mut self, color: &Color, alpha: u8) -> () {
        unsafe {
            ffi::sfImage_createMaskFromColor(self.raw_mut(), *color, alpha)
        }
    }

    /// Change the color of a pixel in an image
    ///
    /// This function doesn't check the validity of the pixel
    /// coordinates, using out-of-range values will result in
    /// an undefined behaviour.
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to change
    /// * y - Y coordinate of pixel to change
    /// * color - New color of the pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, color: &Color) -> () {
        unsafe {
            ffi::sfImage_setPixel(self.raw_mut(), x as c_uint, y as c_uint, *color)
        }
    }

    /// Get the color of a pixel in an image
    ///
    /// This function doesn't check the validity of the pixel
    /// coordinates, using out-of-range values will result in
    /// an undefined behaviour.
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to get
    /// * y - Y coordinate of pixel to get
    ///
    /// Return the Color of the pixel at coordinates (x, y)
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        unsafe {
            ffi::sfImage_getPixel(self.raw(), x as c_uint, y as c_uint)
        }
    }

	/// Get a read-only pointer to the array of pixels.
	///
	/// The returned value points to an array of RGBA pixels made of 8-bit
	/// integer components. The size of the array is `width * height * 4`.
	pub fn get_pixels(&self) -> &[u8] {
		unsafe {
			let pixels = ffi::sfImage_getPixelsPtr(self.raw());
			if pixels.is_null() {
				// SFML returns null if the image is empty, i.e. if its size
				// is (0, 0). In order to present a simpler interface, just
				// return an empty pixel array.
				static EMPTY: &'static [u8] = &[];
				EMPTY
			} else {
				let size = self.get_size();
				let len = 4 * size.x as usize * size.y as usize;
				::std::slice::from_raw_parts(pixels, len)
			}
		}
	}

	/// Get a read-only pointer to the array of pixels, as a color array.
	///
	/// The returned array has the size `width * height`.
	pub fn get_pixels_colors(&self) -> &[Color] {
		let pixels = self.get_pixels();
		unsafe {
			::std::slice::from_raw_parts(pixels.as_ptr() as *const Color, pixels.len() / 4)
		}
	}

    /// Flip an image horizontally (left <-> right)
    pub fn flip_horizontally(&mut self) -> () {
        unsafe {
            ffi::sfImage_flipHorizontally(self.raw_mut())
        }
    }

    /// Flip an image vertically (top <-> bottom)
    pub fn flip_vertically(&mut self) -> () {
        unsafe {
            ffi::sfImage_flipVertically(self.raw_mut())
        }
    }

    /// Copy pixels from an image onto another
    ///
    /// This function does a slow pixel copy and should not be
    /// used intensively. It can be used to prepare a complex
    /// static image from several others, but if you need this
    /// kind of feature in real-time you'd better use sfRenderTexture.
    ///
    /// If sourceRect is empty, the whole image is copied.
    /// If applyAlpha is set to true, the transparency of
    /// source pixels is applied. If it is false, the pixels are
    /// copied unchanged with their alpha value.
    ///
    /// # Arguments
    /// * source - Source image to copy
    /// * destX - X coordinate of the destination position
    /// * destY - Y coordinate of the destination position
    /// * sourceRect - Sub-rectangle of the source image to copy
    /// * applyAlpha - Should the copy take in account the source transparency?
    pub fn copy_image(&mut self,
                      source: &Image,
                      dest_x: u32,
                      dest_y: u32,
                      source_rect: &IntRect,
                      apply_alpha: bool) -> () {
        unsafe {
            ffi::sfImage_copyImage(self.raw_mut(),
                                   source.unwrap(),
                                   dest_x as c_uint,
                                   dest_y as c_uint,
                                   *source_rect,
                                   SfBool::from_bool(apply_alpha))
        }
    }
}

impl Clone for Image {
    fn clone(&self) -> Image {
		self.clone_opt().expect("Failed to clone Image")
    }
}
