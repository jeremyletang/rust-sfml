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

use libc::{c_uint, size_t};
use std::ptr;
use std::ffi::CString;
use std::io::{Read, Seek};

use graphics::{RenderWindow, Image, IntRect};
use system::{Vector2u, InputStream};
use window::Window;

use ffi::{SfBool, Foreign, ForeignHolder};
use ffi::graphics as ffi;

/// Texture living on the graphics card that can be used for drawing.
///
/// A texture lives in the graphics card memory, therefore it is very fast to
/// draw a texture to a render target, or copy a render target to a texture (the
/// graphics card can access both directly).
///
/// Being stored in the graphics card memory has some drawbacks. A texture
/// cannot be manipulated as freely as an `Image`: you need to prepare the
/// pixels first and then upload them to the texture in a single operation
/// (see the `update()` method).
///
/// `Texture` makes it easy to convert from/to `Image`, but keep in mind that
/// these calls require transfers between the graphics card and the central
/// memory, and are therefore slow operations.
///
/// A texture can be loaded from an image, and also directly from a
/// file/memory/stream. The necessary shortcuts are defined so that you don't
/// need an image first for the most common cases. However, if you want to
/// perform some modifications on the pixels before creating the final texture,
/// you can load your file to an `Image`, do whatever you need with the pixels,
/// and then call `new_from_image()`.
///
/// Since they live in the graphics card memory, the pixels of a texture cannot
/// be accessed without a slow copy first, and cannot be accessed individually.
/// Therefore, if you need to read the texture's pixels (such as for
/// pixel-perfect collisions), it is recommended to store the collision
/// information separately, for example in an array of booleans.
///
/// Like `Image`, `Texture` can handle a unique internal representation of
/// pixels, which is 32-bit RGBA. This means that a pixel must be composed of
/// 8-bit red, green, blue, and alpha channels – just like a `Color`.
///
/// `Texture` can be used directly for custom OpenGL geometry with `bind()`.
pub struct Texture(Foreign<ffi::sfTexture>);

impl Texture {
	/// Create a new texture with the specified size in pixels.
	///
	/// The maximum size for a texture depends on the graphics driver and can be
	/// retrieved with the `get_maximum_size()` function.
    ///
    /// Returns Some(Texture) or None on failure.
    pub fn new(width: u32, height: u32) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_create(width as c_uint, height as c_uint))
		}.map(Texture)
    }

    /// Create a new texture from memory.
	///
	/// Acts as a shortcut for creating an `Image` from memory and creating a
	/// `Texture` from that image.
	///
	/// The area argument can be used to load only a sub-rectangle of the whole
	/// image. If you want the entire image, provide None. If the area rectangle
	/// crosses the bounds of the image, it is adjusted to fit the image size.
    ///
    /// Returns Some(Texture) or None on failure.
    pub fn new_from_memory(mem: &[u8], area: Option<IntRect>) -> Option<Texture> {
		let area_ptr = match area {
			Some(ref rect) => rect as *const IntRect,
			None => ptr::null()
		};
        unsafe {
			Foreign::new(ffi::sfTexture_createFromMemory(mem.as_ptr(), mem.len() as size_t, area_ptr))
		}.map(Texture)
    }

	/// Create a new texture from an input stream.
	///
	/// Acts as a shortcut for creating an `Image` from the stream and creating
	/// a `Texture` from that image.
	///
	/// Returns Some(Texture) or None on failure.
	pub fn new_from_stream<T: Read + Seek>(stream: &mut T) -> Option<Texture> {
		unsafe {
			Foreign::new(ffi::sfTexture_createFromStream(&mut InputStream::new(stream)))
		}.map(Texture)
	}

    /// Create a new texture from a file on disk.
	///
	/// Acts as a shortcut for creating an `Image` from the file and creating a
	/// `Texture` from that image.
    ///
    /// Returns Some(Texture) or None on failure.
    pub fn new_from_file(filename: &str) -> Option<Texture> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfTexture_createFromFile(c_str.as_ptr(), ptr::null()))
        }.map(Texture)
    }

    /// Create a new texture from a file with the given texture rect.
	///
	/// Acts as a shortcut for creating an `Image` from the file and creating a
	/// `Texture` from that image.
    ///
    /// Returns Some(Texture) or None on failure.
    pub fn new_from_file_with_rect(filename: &str, area: &IntRect) -> Option<Texture> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfTexture_createFromFile(c_str.as_ptr(), area))
        }.map(Texture)
    }

    /// Create a new texture from an image.
	///
	/// The area argument can be used to load only a sub-rectangle of the whole
	/// image. If you want the entire image, provide None. If the area rectangle
	/// crosses the bounds of the image, it is adjusted to fit the image size.
    ///
    /// Returns Some(Texture) or None on failure.
    pub fn new_from_image(image: &Image, area: Option<IntRect>) -> Option<Texture> {
		let area_ptr = match area {
			Some(ref rect) => rect as *const IntRect,
			None => ptr::null()
		};
        unsafe {
			Foreign::new(ffi::sfTexture_createFromImage(image.unwrap(), area_ptr))
		}.map(Texture)
    }

	fn raw(&self) -> &ffi::sfTexture { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfTexture { self.0.as_mut() }
	#[doc(hidden)]
    pub fn unwrap(&self) -> &ffi::sfTexture { self.raw() }

    /// Create a new texture by copying an existing one.
    ///
    /// Returns Some(Texture) or None on failure.
    pub fn clone_opt(&self) -> Option<Texture> {
        unsafe {
			Foreign::new(ffi::sfTexture_copy(self.raw()))
		}.map(Texture)
	}

    /// Return the size of the texture in pixels.
    pub fn get_size(&self) -> Vector2u {
        unsafe { ffi::sfTexture_getSize(self.raw()) }
    }

    /// Update the texture from the contents of a window.
	///
	/// The position specified affects where in the texture the contents of the
	/// window are pasted.
	///
	/// No additional check is performed on the size of the window: passing an
	/// invalid combination of window size and offset is undefined.
    pub fn update_from_window(&mut self, window: &Window, x: u32, y: u32) {
        unsafe {
            ffi::sfTexture_updateFromWindow(self.raw_mut(), window.unwrap(), x as c_uint, y as c_uint)
        }
    }

    /// Update the texture from the contents of a render window.
	///
	/// The position specified affects where in the texture the contents of the
	/// window are pasted.
	///
	/// No additional check is performed on the size of the window: passing an
	/// invalid combination of window size and offset is undefined.
    pub fn update_from_render_window(&mut self, window: &RenderWindow, x: u32, y: u32) {
        unsafe {
            ffi::sfTexture_updateFromRenderWindow(self.raw_mut(), window.unwrap(), x as c_uint, y as c_uint)
        }
    }

    /// Update the texture from the contents of an image.
	///
	/// The position specified affects where in the texture the contents of the
	/// window are pasted.
	///
	/// No additional check is performed on the size of the window: passing an
	/// invalid combination of window size and offset is undefined.
    pub fn update_from_image(&mut self, image: &Image, x: u32, y: u32) {
        unsafe {
            ffi::sfTexture_updateFromImage(self.raw_mut(), image.unwrap(), x as c_uint, y as c_uint)
        }
    }

    /// Update a texture from a slice of pixels.
	///
	/// The size of the `pixels` slice must be `4 * width * height` and contain
	/// 32-bit RGBA pixels.
	///
	/// The position specified affects where in the texture the contents of the
	/// window are pasted.
	///
	/// No additional check is performed on the size of the window: passing an
	/// invalid combination of window size and offset is undefined.
    pub fn update_from_pixels(&mut self,
                              pixels: &[u8],
                              width: u32,
                              height: u32,
                              x: u32,
                              y: u32) {
		if width as usize * height as usize * 4 != pixels.len() {
			// TODO: report error somehow
			return
		}
        unsafe {
            ffi::sfTexture_updateFromPixels(self.raw_mut(),
                                            pixels.as_ptr(),
                                            width as c_uint,
                                            height as c_uint,
                                            x as c_uint,
                                            y as c_uint)
        }
    }

    /// Enable or disable the smooth filter.
	///
	/// When the filter is activated, the texture appears smoother so that
	/// pixels are less noticeable. However, if you want the texture to look
	/// exactly the same as its source file, you should leave it disabled. The
	/// smooth filter is disabled by default.
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe {
            ffi::sfTexture_setSmooth(self.raw_mut(), SfBool::from_bool(smooth))
        }
    }

    /// Tell whether the smooth filter is enabled or not.
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfTexture_isSmooth(self.raw()) }.to_bool()
    }

    /// Enable or disable repeating.
	///
	/// Repeating is involved when using texture coordinates outside the texture
	/// rectangle [0, 0, width, height]. In this case, if repeat mode is
	/// enabled, the whole texture will be repeated as many times as needed to
	/// reach the coordinate (for example, if the X texture coordinate is 3 *
	/// width, the texture will be repeated 3 times). If repeat mode is
	/// disabled, the "extra space" will instead be filled with border pixels.
	/// Warning: on very old graphics cards, white pixels may appear when the
	/// texture is repeated. With such cards, repeat mode can be used reliably
	/// only if the texture has power-of-two dimensions (such as 256x128).
	///
	/// Repeating is disabled by default.
    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe {
            ffi::sfTexture_setRepeated(self.raw_mut(), SfBool::from_bool(repeated))
        }
    }

    /// Tell whether the texture is repeated or not.
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfTexture_isRepeated(self.raw()) }.to_bool()
    }

    /// Copy the texture's pixels to an image.
    ///
	/// This function performs a slow operation that downloads the texture's
	/// pixels from the graphics card and copies them to a new image,
	/// potentially applying transformations to pixels if necessary (texture may
	/// be padded or flipped).
	///
    /// Returns Some(Image), or None on failure.
    pub fn copy_to_image(&self) -> Option<Image> {
		unsafe {
			Image::wrap(ffi::sfTexture_copyToImage(self.raw()))
		}
    }

    /// Bind a texture for rendering.
    ///
    /// This function is not part of the graphics API, and mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix `Texture` with OpenGL code.
    pub fn bind(&mut self) {
        unsafe { ffi::sfTexture_bind(self.raw_mut()) }
    }

    /// Get the maximum texture size allowed, in pixels.
	///
	/// This maximum size is defined by the graphics driver. You can expect a
	/// value of 512 pixels for low-end graphics card, and up to 8192 pixels or
	/// more for newer hardware.
	///
	/// Note: The first call to this function, whether by user code or SFML,
	/// will result in a context switch.
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

#[doc(hidden)]
unsafe impl ForeignHolder for Texture {
	type Inner = ffi::sfTexture;
}
