use crate::{
    graphics::{csfml_graphics_sys as ffi, Color, IntRect},
    inputstream::InputStream,
    sf_bool_ext::SfBoolExt,
    system::Vector2u,
};
use csfml_system_sys::sfBool;
use std::{
    ffi::CString,
    io::{Read, Seek},
    slice,
};

/// Loading, manipulating and saving images.
#[derive(Debug)]
pub struct Image {
    image: *mut ffi::sfImage,
}

impl Image {
    /// Create an image
    ///
    /// This image is filled with black pixels.
    ///
    /// # Arguments
    /// * width - Width of the image
    /// * height - Height of the image
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        let image = unsafe { ffi::sfImage_create(width, height) };
        assert!(!image.is_null(), "Failed to create Image");
        Self { image }
    }

    /// Create an image from a custom stream.
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif, psd, hdr and pic.
    /// Some format options are not supported, like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    ///
    /// Returns `None` if loading fails
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<Self> {
        let mut input_stream = InputStream::new(stream);
        let image = unsafe { ffi::sfImage_createFromStream(&mut input_stream.0) };
        if image.is_null() {
            None
        } else {
            Some(Self { image })
        }
    }

    /// Create an image from a file in memory
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif, psd, hdr and pic.
    /// Some format options are not supported, like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    ///
    /// Returns `None` if loading fails.
    #[must_use]
    pub fn from_memory(mem: &[u8]) -> Option<Self> {
        let image = unsafe { ffi::sfImage_createFromMemory(mem.as_ptr() as *const _, mem.len()) };
        if image.is_null() {
            None
        } else {
            Some(Self { image })
        }
    }

    /// Create an image and fill it with a unique color
    ///
    /// # Arguments
    /// * width - Width of the image
    /// * height - Height of the image
    /// * color - Fill color
    ///
    /// Returns `None` if creation fails.
    #[must_use]
    pub fn from_color(width: u32, height: u32, color: Color) -> Option<Self> {
        let image = unsafe { ffi::sfImage_createFromColor(width, height, color.0) };
        if image.is_null() {
            None
        } else {
            Some(Self { image })
        }
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
    /// Returns `None` if loading fails
    #[must_use]
    pub fn from_file(filename: &str) -> Option<Self> {
        let c_filename = CString::new(filename.as_bytes()).unwrap();
        let image = unsafe { ffi::sfImage_createFromFile(c_filename.as_ptr()) };
        if image.is_null() {
            None
        } else {
            Some(Self { image })
        }
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
    /// Returns `None` if creation fails.
    #[must_use]
    pub fn create_from_pixels(width: u32, height: u32, pixels: &[u8]) -> Option<Self> {
        let image = unsafe { ffi::sfImage_createFromPixels(width, height, pixels.as_ptr()) };
        if image.is_null() {
            None
        } else {
            Some(Self { image })
        }
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
    #[must_use]
    pub fn save_to_file(&self, filename: &str) -> bool {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        unsafe { ffi::sfImage_saveToFile(self.image, c_str.as_ptr()) }.to_bool()
    }

    /// Return the size of an image
    ///
    /// Return the size in pixels
    #[must_use]
    pub fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfImage_getSize(self.image)) }
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
    pub fn create_mask_from_color(&self, color: Color, alpha: u8) {
        unsafe { ffi::sfImage_createMaskFromColor(self.image, color.0, alpha) }
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
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        unsafe { ffi::sfImage_setPixel(self.image, x, y, color.0) }
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
    #[must_use]
    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        unsafe { Color(ffi::sfImage_getPixel(self.image, x, y)) }
    }

    /// Return the memory buffer of this image.
    #[must_use]
    pub fn pixel_data(&self) -> &[u8] {
        unsafe {
            let size = self.size();
            let pixels = ffi::sfImage_getPixelsPtr(self.image);

            slice::from_raw_parts(pixels, (size.x * size.y * 4) as usize)
        }
    }

    /// Flip an image horizontally (left <-> right)
    pub fn flip_horizontally(&mut self) {
        unsafe { ffi::sfImage_flipHorizontally(self.image) }
    }

    /// Flip an image vertically (top <-> bottom)
    pub fn flip_vertically(&mut self) {
        unsafe { ffi::sfImage_flipVertically(self.image) }
    }

    /// Copy pixels from an image onto another
    ///
    /// This function does a slow pixel copy and should not be
    /// used intensively. It can be used to prepare a complex
    /// static image from several others, but if you need this
    /// kind of feature in real-time you'd better use [`RenderTexture`].
    ///
    /// [`RenderTexture`]: crate::graphics::RenderTexture
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
    pub fn copy_image(
        &mut self,
        source: &Image,
        dest_x: u32,
        dest_y: u32,
        source_rect: &IntRect,
        apply_alpha: bool,
    ) {
        unsafe {
            ffi::sfImage_copyImage(
                self.image,
                source.raw(),
                dest_x,
                dest_y,
                source_rect.raw(),
                sfBool::from_bool(apply_alpha),
            )
        }
    }
    pub(super) fn raw(&self) -> *const ffi::sfImage {
        self.image
    }
    pub(super) unsafe fn from_raw(raw: *mut ffi::sfImage) -> Self {
        Image { image: raw }
    }
}

impl Clone for Image {
    /// Return a new `Image` or panic! if there is not enough memory
    fn clone(&self) -> Self {
        let image = unsafe { ffi::sfImage_copy(self.image) };
        if image.is_null() {
            panic!("Not enough memory to clone Image")
        } else {
            Self { image }
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { ffi::sfImage_destroy(self.image) }
    }
}
