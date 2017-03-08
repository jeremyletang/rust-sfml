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

//! Image used for drawing
//!
//! Texture stores pixels that can be drawn, with a sprite for example.

use std::ptr;
use std::ffi::CString;
use std::io::{Read, Seek};
use std::ops::Deref;

use sfml::system::raw_conv::{Raw, FromRaw};
use {RenderWindow, Image, IntRect};
use sfml::system::Vector2u;
use inputstream::InputStream;
use sfml_window::Window;

use csfml_system_sys::sfBool;
use csfml_graphics_sys as ffi;
use sfml::system::SfBoolExt;

/// Image used for drawing
///
/// Texture stores pixels that can be drawn, with a sprite for example.
pub struct Texture {
    texture: *mut ffi::sfTexture,
}

impl Deref for Texture {
    type Target = TextureRef;

    fn deref(&self) -> &TextureRef {
        unsafe { &*(self.texture as *const TextureRef) }
    }
}

/// A non-owning `Texture`.
pub enum TextureRef {}

impl TextureRef {
    /// Return the size of the texture
    ///
    /// Return the Size in pixels
    pub fn get_size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfTexture_getSize(self as *const _ as _)) }
    }
    /// Tell whether the smooth filter is enabled or not for a texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfTexture_isSmooth(self as *const _ as _) }.to_bool()
    }
    /// Tell whether a texture is repeated or not
    ///
    /// Return frue if repeat mode is enabled, false if it is disabled
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfTexture_isRepeated(self as *const _ as _) }.to_bool()
    }
    /// Copy a texture's pixels to an image
    ///
    /// Return an image containing the texture's pixels
    pub fn copy_to_image(&self) -> Option<Image> {
        let img = unsafe { ffi::sfTexture_copyToImage(self as *const _ as _) };
        if img.is_null() {
            None
        } else {
            Some(unsafe { Image::from_raw(img) })
        }
    }
    /// Tell whether the texture source is converted from sRGB or not.
    pub fn is_srgb(&self) -> bool {
        unsafe { ffi::sfTexture_isSrgb(self as *const _ as _).to_bool() }
    }
    /// Get the underlying OpenGL handle of the texture.
    ///
    /// You shouldn't need to use this function, unless you have very specific stuff to implement
    /// that SFML doesn't support, or implement a temporary workaround until a bug is fixed.
    pub fn native_handle(&self) -> u32 {
        unsafe { ffi::sfTexture_getNativeHandle(self as *const _ as _) }
    }

    /// Bind a texture for rendering
    ///
    /// This function is not part of the graphics API, it mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix `Texture` with OpenGL code.
    pub fn bind(&self) {
        unsafe { ffi::sfTexture_bind(self as *const _ as _) }
    }
}

impl Texture {
    /// Create a new texture
    ///
    /// # Arguments
    /// * width - Texture width
    /// * height - Texture height
    ///
    /// Return Some(Texture) or None
    pub fn new(width: u32, height: u32) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_create(width, height) };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Create a new texture from memory
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    /// * area - Area of the image to load
    ///
    /// Return Some(Texture) or None
    pub fn from_memory(mem: &[u8], area: &IntRect) -> Option<Texture> {
        let tex = unsafe {
            ffi::sfTexture_createFromMemory(mem.as_ptr() as *const _, mem.len(), &area.raw())
        };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Create a new texture from a stream (a struct implementing Read + Seek)
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    ///
    /// Return Some(Texture) or None
    pub fn from_stream<T: Read + Seek>(stream: &mut T, area: &mut IntRect) -> Option<Texture> {
        let mut input_stream = InputStream::new(stream);
        let tex = unsafe { ffi::sfTexture_createFromStream(&mut input_stream.0, &area.raw()) };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Create a new texture from a file
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    ///
    /// Return Some(Texture) or None
    pub fn from_file(filename: &str) -> Option<Texture> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let tex = unsafe { ffi::sfTexture_createFromFile(c_str.as_ptr() as *mut i8, ptr::null()) };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Create a new texture from a file with a given area
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    /// * area - Area of the source image to load
    ///
    /// Return Some(Texture) or None
    pub fn from_file_with_rect(filename: &str, area: &IntRect) -> Option<Texture> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let tex = unsafe { ffi::sfTexture_createFromFile(c_str.as_ptr() as *mut i8, &area.raw()) };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Create a new texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    /// * area - Area of the source image to load
    ///
    /// Return Some(Texture) or None
    pub fn from_image_with_rect(image: &Image, area: &IntRect) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_createFromImage(image.raw(), &area.raw()) };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Create a new texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    ///
    /// Return Some(Texture) or None
    pub fn from_image(image: &Image) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_createFromImage(image.raw(), ptr::null()) };
        if tex.is_null() {
            None
        } else {
            Some(Texture { texture: tex })
        }
    }

    /// Update a texture from the contents of a window
    ///
    /// # Arguments
    /// * window - Window to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_window(&mut self, window: &Window, x: u32, y: u32) {
        unsafe { ffi::sfTexture_updateFromWindow(self.texture, window.raw(), x, y) }
    }

    /// Update a texture from the contents of a render window
    ///
    /// # Arguments
    /// * renderWindow - Render-window to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_render_window(&mut self, render_window: &RenderWindow, x: u32, y: u32) {
        unsafe { ffi::sfTexture_updateFromRenderWindow(self.texture, render_window.raw(), x, y) }
    }

    /// Update a texture from the contents of an image
    ///
    /// # Arguments
    /// * image - Image to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_image(&mut self, image: &Image, x: u32, y: u32) {
        unsafe { ffi::sfTexture_updateFromImage(self.texture, image.raw(), x, y) }
    }

    /// Update a texture from the contents of a Vector of pixels
    ///
    /// # Arguments
    /// * pixels - Pixels to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_pixels(&mut self, pixels: &[u8], width: u32, height: u32, x: u32, y: u32) {
        unsafe {
            ffi::sfTexture_updateFromPixels(self.texture, pixels.as_ptr(), width, height, x, y)
        }
    }

    /// Enable or disable the smooth filter on a texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfTexture_setSmooth(self.texture, sfBool::from_bool(smooth)) }
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
    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe { ffi::sfTexture_setRepeated(self.texture, sfBool::from_bool(repeated)) }
    }

    /// Get the maximum texture size allowed
    ///
    /// Return the maximum size allowed for textures, in pixels
    pub fn get_maximum_size() -> u32 {
        unsafe { ffi::sfTexture_getMaximumSize() as u32 }
    }

    /// Enable or disable conversion from sRGB.
    ///
    /// When providing texture data from an image file or memory, it can either be stored in a
    /// linear color space or an sRGB color space. Most digital images account for gamma correction
    /// already, so they would need to be "uncorrected" back to linear color space before being
    /// processed by the hardware. The hardware can automatically convert it from the sRGB
    /// color space to a linear color space when it gets sampled. When the rendered image gets
    /// output to the final framebuffer, it gets converted back to sRGB.
    ///
    /// After enabling or disabling sRGB conversion, make sure to reload the texture data in
    /// order for the setting to take effect.
    ///
    /// This option is only useful in conjunction with an sRGB capable framebuffer.
    /// This can be requested during window creation.
    pub fn set_srgb(&mut self, srgb: bool) {
        unsafe { ffi::sfTexture_setSrgb(self.texture, SfBoolExt::from_bool(srgb)) }
    }

    /// Generate a mipmap using the current texture data.
    ///
    /// Mipmaps are pre-computed chains of optimized textures. Each level of texture in a mipmap
    /// is generated by halving each of the previous level's dimensions. This is done until the
    /// final level has the size of 1x1. The textures generated in this process may make use of
    /// more advanced filters which might improve the visual quality of textures when they are
    /// applied to objects much smaller than they are. This is known as minification. Because
    /// fewer texels (texture elements) have to be sampled from when heavily minified, usage of
    /// mipmaps can also improve rendering performance in certain scenarios.
    ///
    /// Mipmap generation relies on the necessary OpenGL extension being available.
    /// If it is unavailable or generation fails due to another reason, this function will return
    /// false. Mipmap data is only valid from the time it is generated until the next time the base
    /// level image is modified, at which point this function will have to be called again to
    /// regenerate it.
    ///
    /// Returns true if mipmap generation was successful, false if unsuccessful.
    pub fn generate_mipmap(&mut self) -> bool {
        unsafe { ffi::sfTexture_generateMipmap(self.texture).to_bool() }
    }
}

impl Clone for Texture {
    /// Return a new Texture or panic! if there is not enough memory
    fn clone(&self) -> Texture {
        let tex = unsafe { ffi::sfTexture_copy(self.texture) };
        if tex.is_null() {
            panic!("Not enough memory to clone Texture")
        } else {
            Texture { texture: tex }
        }
    }
}

impl Raw for Texture {
    type Raw = *mut ffi::sfTexture;
    fn raw(&self) -> Self::Raw {
        self.texture
    }
}

impl FromRaw for Texture {
    unsafe fn from_raw(raw: Self::Raw) -> Self {
        Texture { texture: raw }
    }
}

impl Drop for Texture {
    /// Destroy an existing texture
    fn drop(&mut self) {
        unsafe { ffi::sfTexture_destroy(self.texture) }
    }
}
