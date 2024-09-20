use {
    crate::{
        ffi::graphics as ffi,
        graphics::{Color, IntRect},
        system::{InputStream, Vector2u},
        SfError, SfResult,
    },
    std::{
        error::Error,
        ffi::CString,
        fmt,
        io::{Read, Seek},
        slice,
    },
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
        let image = unsafe { ffi::sfImage_createFromStream(&mut *input_stream.stream) };
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
        let image = unsafe { ffi::sfImage_createFromColor(width, height, color) };
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
    pub fn from_file(filename: &str) -> SfResult<Self> {
        let c_filename = CString::new(filename).unwrap();
        let image = unsafe { ffi::sfImage_createFromFile(c_filename.as_ptr()) };
        if image.is_null() {
            Err(SfError::CallFailed)
        } else {
            Ok(Self { image })
        }
    }

    /// Create an image from an vector of pixels
    ///
    /// # Arguments
    /// * width - Width of the image
    /// * height - Height of the image
    /// * pixels - Vector of pixels to copy to the image
    ///
    /// Returns `None` if creation fails.
    ///
    /// # Safety
    ///
    /// The pixel vector is assumed to contain 32-bits RGBA pixels,
    /// and have the given width and height. If not, this is
    /// an undefined behaviour.
    #[must_use]
    pub unsafe fn create_from_pixels(width: u32, height: u32, pixels: &[u8]) -> Option<Self> {
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
        let c_str = CString::new(filename).unwrap();
        unsafe { ffi::sfImage_saveToFile(self.image, c_str.as_ptr()) }
    }

    /// Return the size of an image
    ///
    /// Return the size in pixels
    #[must_use]
    pub fn size(&self) -> Vector2u {
        unsafe { ffi::sfImage_getSize(self.image) }
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
        unsafe { ffi::sfImage_createMaskFromColor(self.image, color, alpha) }
    }

    /// Change the color of a pixel in an image
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to change
    /// * y - Y coordinate of pixel to change
    /// * color - New color of the pixel
    ///
    /// # Safety
    ///
    /// This function doesn't check the validity of the pixel
    /// coordinates, using out-of-range values will result in
    /// an undefined behaviour.
    pub unsafe fn set_pixel_unchecked(&mut self, x: u32, y: u32, color: Color) {
        unsafe { ffi::sfImage_setPixel(self.image, x, y, color) }
    }

    /// Change the color of a pixel in an image
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to change
    /// * y - Y coordinate of pixel to change
    /// * color - New color of the pixel
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), PixelAccessError> {
        let image_size = self.size();
        if x >= image_size.x {
            return Err(PixelAccessError::XTooLarge {
                x,
                width: image_size.x - 1,
            });
        }
        if y >= image_size.y {
            return Err(PixelAccessError::YTooLarge {
                y,
                height: image_size.y - 1,
            });
        }

        // Since we check for index validity before setting the pixel, it is safe unless the
        // image has been unloaded, but I doubt you can even do that.
        unsafe { ffi::sfImage_setPixel(self.image, x, y, color) }
        Ok(())
    }

    /// Get the color of a pixel in an image
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to get
    /// * y - Y coordinate of pixel to get
    ///
    /// Return the Color of the pixel at coordinates (x, y)
    ///
    /// # Safety
    ///
    /// This function doesn't check the validity of the pixel
    /// coordinates, using out-of-range values will result in
    /// an undefined behaviour.
    #[must_use]
    pub unsafe fn pixel_at_unchecked(&self, x: u32, y: u32) -> Color {
        unsafe { ffi::sfImage_getPixel(self.image, x, y) }
    }

    /// Get the color of a pixel in an image
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to get
    /// * y - Y coordinate of pixel to get
    ///
    /// Return the Color of the pixel at coordinates (x, y)
    #[must_use]
    pub fn pixel_at(&self, x: u32, y: u32) -> Option<Color> {
        let image_size = self.size();
        if image_size.x <= x || image_size.y <= y {
            return None;
        }

        // Since we check for index validity before getting the pixel, it is safe unless the
        // image has been unloaded, but I doubt you can even do that.
        unsafe { Some(ffi::sfImage_getPixel(self.image, x, y)) }
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
        source_rect: IntRect,
        apply_alpha: bool,
    ) {
        unsafe {
            ffi::sfImage_copyImage(
                self.image,
                source.raw(),
                dest_x,
                dest_y,
                source_rect,
                apply_alpha,
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

/// Error that can happen when trying to access a pixel
#[derive(Debug, Copy, Clone)]
pub enum PixelAccessError {
    /// X coordinate is larger than the width of the image
    XTooLarge {
        /// X coordinate access was attempted at
        x: u32,
        /// Width of the image
        width: u32,
    },
    /// Y coordinate is larger than the height of the image
    YTooLarge {
        /// Y coordinate access was attempted at
        y: u32,
        /// Height of the image
        height: u32,
    },
}

impl Error for PixelAccessError {}

impl fmt::Display for PixelAccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::XTooLarge { x, width } => {
                write!(f, "x index out of bounds. x:{} width:{}", x, width)
            }
            Self::YTooLarge { y, height } => {
                write!(f, "y index out of bounds. y:{} height:{}", y, height)
            }
        }
    }
}
