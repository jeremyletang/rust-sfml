use {
    crate::{
        cpp::FBox,
        ffi::graphics as ffi,
        graphics::{Color, IntRect},
        system::{InputStream, Vector2u},
        IntoSfResult, SfResult,
    },
    std::{
        error::Error,
        ffi::CString,
        fmt,
        io::{Read, Seek},
        slice,
    },
};

decl_opaque! {
    /// Loading, manipulating and saving images.
    pub Image;
}

/// Creation and loading
impl Image {
    /// Create a new (empty) image.
    pub fn new() -> SfResult<FBox<Self>> {
        FBox::new(unsafe { ffi::sfImage_new() }).into_sf_result()
    }
    /// Create a new `Image` filled with a solid color.
    ///
    /// See [`Self::recreate_solid`].
    pub fn new_solid(width: u32, height: u32, color: Color) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.recreate_solid(width, height, color);
        Ok(new)
    }
    /// Create a new `Image` from the provided RGBA pixel data.
    ///
    /// See [`Self::recreate_from_pixels`].
    ///
    /// # Safety
    ///
    /// Also see [`Self::recreate_from_pixels`].
    pub unsafe fn from_pixels(width: u32, height: u32, data: &[u8]) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        unsafe {
            new.recreate_from_pixels(width, height, data);
        }
        Ok(new)
    }
    /// Create a new `Image` from an image file on the filesystem.
    ///
    /// See [`Self::load_from_file`].
    pub fn from_file(filename: &str) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_file(filename)?;
        Ok(new)
    }
    /// Create a new `Image` from image file data in memory.
    ///
    /// See [`Self::load_from_memory`].
    pub fn from_memory(data: &[u8]) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_memory(data)?;
        Ok(new)
    }
    /// Create a new `Image` from a stream.
    ///
    /// See [`Self::load_from_stream`].
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_stream(stream)?;
        Ok(new)
    }
    /// Recreate with the given size, filled with a solid color.
    pub fn recreate_solid(&mut self, width: u32, height: u32, color: Color) {
        unsafe {
            ffi::sfImage_create_w_h_color(self, width, height, color);
        }
    }
    /// Recreate from the provided RGBA pixel data.
    ///
    /// # Safety
    ///
    /// `data` is assumed to contain 32-bit RGBA pixels, and match the given size.
    pub unsafe fn recreate_from_pixels(&mut self, width: u32, height: u32, data: &[u8]) {
        unsafe {
            ffi::sfImage_create_w_h_pixels(self, width, height, data.as_ptr());
        }
    }
    /// Load from image file data on the filesystem.
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif,
    /// psd, hdr and pic. Some format options are not supported,
    /// like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    pub fn load_from_file(&mut self, path: &str) -> SfResult<()> {
        let c_path = CString::new(path)?;
        unsafe { ffi::sfImage_loadFromFile(self, c_path.as_ptr()) }.into_sf_result()
    }
    /// Load from image file data in memory.
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif, psd, hdr and pic.
    /// Some format options are not supported, like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    pub fn load_from_memory(&mut self, data: &[u8]) -> SfResult<()> {
        unsafe { ffi::sfImage_loadFromMemory(self, data.as_ptr(), data.len()) }.into_sf_result()
    }
    /// Load from image file data coming from a custom stream.
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif, psd, hdr and pic.
    /// Some format options are not supported, like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    pub fn load_from_stream<T: Read + Seek>(&mut self, stream: &mut T) -> SfResult<()> {
        let mut input_stream = InputStream::new(stream);
        unsafe { ffi::sfImage_loadFromStream(self, &mut *input_stream.stream) }.into_sf_result()?;
        Ok(())
    }
}

/// Get/set pixels
impl Image {
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
        unsafe { ffi::sfImage_setPixel(self, x, y, color) }
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
        unsafe { ffi::sfImage_setPixel(self, x, y, color) }
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
        unsafe { ffi::sfImage_getPixel(self, x, y) }
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
        unsafe { Some(ffi::sfImage_getPixel(self, x, y)) }
    }

    /// Return the memory buffer of this image.
    #[must_use]
    pub fn pixel_data(&self) -> &[u8] {
        unsafe {
            let size = self.size();
            let pixels = ffi::sfImage_getPixelsPtr(self);

            slice::from_raw_parts(pixels, (size.x * size.y * 4) as usize)
        }
    }
}

/// Image data manipulation
impl Image {
    /// Create a transparency mask from a specified color-key
    ///
    /// This function sets the alpha value of every pixel matching
    /// the given color to alpha (0 by default), so that they
    /// become transparent.
    ///
    /// # Arguments
    /// * color - Color to make transparent
    /// * alpha - Alpha value to assign to transparent pixels
    pub fn create_mask_from_color(&mut self, color: Color, alpha: u8) {
        unsafe { ffi::sfImage_createMaskFromColor(self, color, alpha) }
    }

    /// Flip an image horizontally (left <-> right)
    pub fn flip_horizontally(&mut self) {
        unsafe { ffi::sfImage_flipHorizontally(self) }
    }

    /// Flip an image vertically (top <-> bottom)
    pub fn flip_vertically(&mut self) {
        unsafe { ffi::sfImage_flipVertically(self) }
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
        unsafe { ffi::sfImage_copy(self, source, dest_x, dest_y, source_rect, apply_alpha) }
    }
}

/// Etc.
impl Image {
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
    pub fn save_to_file(&self, filename: &str) -> SfResult<()> {
        let c_str = CString::new(filename)?;
        unsafe { ffi::sfImage_saveToFile(self, c_str.as_ptr()) }.into_sf_result()
    }

    /// Return the size of an image
    ///
    /// Return the size in pixels
    #[must_use]
    pub fn size(&self) -> Vector2u {
        unsafe { ffi::sfImage_getSize(self) }
    }
}

impl ToOwned for Image {
    type Owned = FBox<Self>;

    fn to_owned(&self) -> Self::Owned {
        let ptr = unsafe { ffi::sfImage_cpy(self) };
        match FBox::new(ptr) {
            Some(new) => new,
            None => panic!("Failed to copy image"),
        }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            ffi::sfImage_del(self);
        }
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
