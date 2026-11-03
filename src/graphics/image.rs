use {
    crate::{
        IntoSfResult, SfResult,
        cpp::FBox,
        ffi::graphics::{self as ffi, sfImage_saveToMemory},
        graphics::{Color, IntRect},
        system::{InputStream, Vector2u, buffer::Buffer},
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
    ///
    /// ```
    /// # use sfml::graphics::Image;
    /// # use sfml::system::Vector2u;
    /// let image = Image::new();
    /// assert_eq!(image.size(), Vector2u::new(0, 0));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the image allocation fails.
    #[must_use]
    pub fn new() -> FBox<Self> {
        FBox::new(unsafe { ffi::sfImage_new() }).expect("Failed to create Image")
    }
    /// Create a new `Image` filled with a solid color.
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let image = Image::new_solid(Vector2u::new(2, 3), Color::RED);
    /// assert_eq!(image.size(), Vector2u::new(2, 3));
    /// assert_eq!(image.pixel_at(Vector2u::new(0, 0)), Some(Color::RED));
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the image allocation fails.
    #[must_use]
    pub fn new_solid(size: Vector2u, color: Color) -> FBox<Self> {
        let mut img = Self::new();
        img.resize_with_color(size, color);
        img
    }
    /// Create a new `Image` from the provided RGBA pixel data.
    ///
    /// `data` is expected to contain `size.x * size.y * 4` bytes in RGBA
    /// order. A zero-sized image is allowed and produces an empty image.
    ///
    /// See [`Self::resize_with_pixels`].
    ///
    /// # Panics
    ///
    /// Panics if the image allocation fails.
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let image = Image::from_pixels(Vector2u::new(1, 1), &[255, 0, 0, 255]);
    /// assert_eq!(image.pixel_at(Vector2u::new(0, 0)), Some(Color::RED));
    /// ```
    #[must_use]
    pub fn from_pixels(size: Vector2u, data: &[u8]) -> FBox<Self> {
        let mut img = Self::new();
        img.resize_with_pixels(size, data);
        img
    }
    /// Create a new `Image` from an image file on the filesystem.
    ///
    /// See [`Self::load_from_file`].
    ///
    /// ```
    /// # use sfml::graphics::Image;
    /// let image = Image::from_file("examples/resources/logo.png").expect("failed to load image");
    /// assert!(image.size().x > 0);
    /// assert!(image.size().y > 0);
    /// ```
    pub fn from_file(filename: &str) -> SfResult<FBox<Self>> {
        let mut new = Self::new();
        new.load_from_file(filename)?;
        Ok(new)
    }
    /// Create a new `Image` from image file data in memory.
    ///
    /// See [`Self::load_from_memory`].
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let source = Image::new_solid(Vector2u::new(2, 2), Color::RED);
    /// let data = source.save_to_memory("png").expect("failed to encode image");
    /// let loaded = Image::from_memory(&data).expect("failed to decode image");
    /// assert_eq!(loaded.size(), Vector2u::new(2, 2));
    /// assert_eq!(loaded.pixel_at(Vector2u::new(0, 0)), Some(Color::RED));
    /// ```
    pub fn from_memory(data: &[u8]) -> SfResult<FBox<Self>> {
        let mut new = Self::new();
        new.load_from_memory(data)?;
        Ok(new)
    }
    /// Create a new `Image` from a stream.
    ///
    /// See [`Self::load_from_stream`].
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> SfResult<FBox<Self>> {
        let mut new = Self::new();
        new.load_from_stream(stream)?;
        Ok(new)
    }
    /// Load from image file data on the filesystem.
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif,
    /// psd, hdr, pic, pnm and qoi. Some format options are not supported,
    /// like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    ///
    /// ```
    /// # use sfml::graphics::Image;
    /// let mut image = Image::new();
    /// image.load_from_file("examples/resources/logo.png").expect("failed to load image");
    /// assert!(image.size().x > 0);
    /// ```
    pub fn load_from_file(&mut self, path: &str) -> SfResult<()> {
        let c_path = CString::new(path)?;
        unsafe { ffi::sfImage_loadFromFile(self, c_path.as_ptr()) }.into_sf_result()
    }
    /// Load from image file data in memory.
    ///
    /// The supported image formats are bmp, png, tga, jpg, gif, psd, hdr, pic, pnm and qoi.
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
    /// The supported image formats are bmp, png, tga, jpg, gif, psd, hdr, pic, pnm and qoi.
    /// Some format options are not supported, like progressive jpeg.
    /// If this function fails, the image is left unchanged.
    pub fn load_from_stream<T: Read + Seek>(&mut self, stream: &mut T) -> SfResult<()> {
        let mut input_stream = InputStream::new(stream);
        unsafe { ffi::sfImage_loadFromStream(self, &raw mut *input_stream.stream) }
            .into_sf_result()?;
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
    pub unsafe fn set_pixel_unchecked(&mut self, coords: Vector2u, color: Color) {
        unsafe { ffi::sfImage_setPixel(self, coords, color) }
    }

    /// Change the color of a pixel in an image
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to change
    /// * y - Y coordinate of pixel to change
    /// * color - New color of the pixel
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let mut image = Image::new_solid(Vector2u::new(2, 2), Color::BLACK);
    /// image.set_pixel(Vector2u::new(1, 0), Color::GREEN).expect("in-bounds");
    /// assert_eq!(image.pixel_at(Vector2u::new(1, 0)), Some(Color::GREEN));
    /// ```
    pub fn set_pixel(&mut self, coords: Vector2u, color: Color) -> Result<(), PixelAccessError> {
        let image_size = self.size();
        if coords.x >= image_size.x {
            return Err(PixelAccessError::XTooLarge {
                x: coords.x,
                width: image_size.x.saturating_sub(1),
            });
        }
        if coords.y >= image_size.y {
            return Err(PixelAccessError::YTooLarge {
                y: coords.y,
                height: image_size.y.saturating_sub(1),
            });
        }

        // Since we check for index validity before setting the pixel, it is safe unless the
        // image has been unloaded, but I doubt you can even do that.
        unsafe { ffi::sfImage_setPixel(self, coords, color) }
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
    pub unsafe fn pixel_at_unchecked(&self, coords: Vector2u) -> Color {
        unsafe { ffi::sfImage_getPixel(self, coords) }
    }

    /// Get the color of a pixel in an image
    ///
    /// # Arguments
    /// * x - X coordinate of pixel to get
    /// * y - Y coordinate of pixel to get
    ///
    /// Return the Color of the pixel at coordinates (x, y)
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let image = Image::new_solid(Vector2u::new(2, 2), Color::CYAN);
    /// assert_eq!(image.pixel_at(Vector2u::new(1, 1)), Some(Color::CYAN));
    /// assert_eq!(image.pixel_at(Vector2u::new(2, 2)), None);
    /// ```
    #[must_use]
    pub fn pixel_at(&self, coords: Vector2u) -> Option<Color> {
        let image_size = self.size();
        if image_size.x <= coords.x || image_size.y <= coords.y {
            return None;
        }

        // Since we check for index validity before getting the pixel, it is safe unless the
        // image has been unloaded, but I doubt you can even do that.
        unsafe { Some(ffi::sfImage_getPixel(self, coords)) }
    }

    /// Return the memory buffer of this image.
    ///
    /// Returns an empty slice when the image has no pixel data.
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let color = Color::rgba(1, 2, 3, 4);
    /// let image = Image::new_solid(Vector2u::new(1, 1), color);
    /// assert_eq!(image.pixel_data(), &[1, 2, 3, 4]);
    /// assert_eq!(Image::new().pixel_data(), &[]);
    /// ```
    #[must_use]
    pub fn pixel_data(&self) -> &[u8] {
        let size = self.size();
        let pixels = unsafe { ffi::sfImage_getPixelsPtr(self) };
        if pixels.is_null() {
            return &[];
        }

        unsafe { slice::from_raw_parts(pixels, (size.x * size.y * 4) as usize) }
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
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let mut image = Image::new_solid(Vector2u::new(2, 2), Color::RED);
    /// image.create_mask_from_color(Color::RED, 0);
    /// assert_eq!(image.pixel_at(Vector2u::new(0, 0)), Some(Color::rgba(255, 0, 0, 0)));
    /// ```
    pub fn create_mask_from_color(&mut self, color: Color, alpha: u8) {
        unsafe { ffi::sfImage_createMaskFromColor(self, color, alpha) }
    }

    /// Flip an image horizontally (left <-> right)
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let mut image = Image::new_solid(Vector2u::new(2, 1), Color::RED);
    /// image.set_pixel(Vector2u::new(0, 0), Color::BLUE).expect("in-bounds");
    /// image.flip_horizontally();
    /// assert_eq!(image.pixel_at(Vector2u::new(1, 0)), Some(Color::BLUE));
    /// ```
    pub fn flip_horizontally(&mut self) {
        unsafe { ffi::sfImage_flipHorizontally(self) }
    }

    /// Flip an image vertically (top <-> bottom)
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let mut image = Image::new_solid(Vector2u::new(1, 2), Color::RED);
    /// image.set_pixel(Vector2u::new(0, 0), Color::BLUE).expect("in-bounds");
    /// image.flip_vertically();
    /// assert_eq!(image.pixel_at(Vector2u::new(0, 1)), Some(Color::BLUE));
    /// ```
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
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image, IntRect};
    /// # use sfml::system::{Vector2i, Vector2u};
    /// let mut dest = Image::new_solid(Vector2u::new(2, 2), Color::BLACK);
    /// let source = Image::new_solid(Vector2u::new(1, 1), Color::GREEN);
    ///
    /// let whole = IntRect::new(Vector2i::new(0, 0), Vector2i::new(1, 1));
    /// dest.copy_image(&source, Vector2u::new(1, 1), whole, false).expect("failed to copy");
    /// assert_eq!(dest.pixel_at(Vector2u::new(1, 1)), Some(Color::GREEN));
    /// ```
    pub fn copy_image(
        &mut self,
        source: &Image,
        dest: Vector2u,
        source_rect: IntRect,
        apply_alpha: bool,
    ) -> SfResult<()> {
        unsafe { ffi::sfImage_copy(self, source, dest, source_rect, apply_alpha) }.into_sf_result()
    }
}

/// Etc.
impl Image {
    /// Resize the image and fill it with a unique color
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let mut image = Image::new();
    /// image.resize_with_color(Vector2u::new(3, 2), Color::BLUE);
    /// assert_eq!(image.size(), Vector2u::new(3, 2));
    /// assert_eq!(image.pixel_at(Vector2u::new(2, 1)), Some(Color::BLUE));
    /// ```
    pub fn resize_with_color(&mut self, size: Vector2u, color: Color) {
        unsafe {
            ffi::sfImage_resizeWithColor(self, size, color);
        }
    }
    /// Resize the image from an array of pixels
    ///
    /// The pixel array is assumed to contain 32-bits RGBA pixels,
    /// and have the given `size`. If not, this is an undefined behavior.
    /// If `pixels` is `nullptr`, an empty image is created.
    ///
    /// ```
    /// # use sfml::graphics::{Color, Image};
    /// # use sfml::system::Vector2u;
    /// let mut image = Image::new();
    /// let pixels = [255, 0, 0, 255, 0, 255, 0, 255];
    /// image.resize_with_pixels(Vector2u::new(2, 1), &pixels);
    /// assert_eq!(image.size(), Vector2u::new(2, 1));
    /// assert_eq!(image.pixel_at(Vector2u::new(0, 0)), Some(Color::RED));
    /// assert_eq!(image.pixel_at(Vector2u::new(1, 0)), Some(Color::GREEN));
    /// ```
    pub fn resize_with_pixels(&mut self, size: Vector2u, data: &[u8]) {
        unsafe { ffi::sfImage_resizeWithPixels(self, size, data.as_ptr()) }
    }
    /// Save an image to a file on disk
    ///
    /// The format of the image is automatically deduced from
    /// the extension. The supported image formats are bmp, png,
    /// tga, jpg and qoi. The destination file is overwritten
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

    /// Save the image to a buffer in memory
    ///
    /// The format of the image must be specified.
    /// The supported image formats are bmp, png, tga, jpg and qoi.
    /// This function fails if the image is empty, or if
    /// the format was invalid.
    ///
    /// # Returns
    ///
    /// The encoded image data on success.
    ///
    /// # Errors
    ///
    /// Returns [`SfError::CallFailed`](crate::SfError::CallFailed) if the
    /// image is empty or the format is invalid.
    #[must_use = "The returned buffer contains the saved data. Ignoring this result will discard the exported memory."]
    pub fn save_to_memory(&self, format: &str) -> SfResult<FBox<Buffer>> {
        let c_str = CString::new(format)?;
        FBox::new(unsafe { sfImage_saveToMemory(self, c_str.as_ptr()) }).into_sf_result()
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
        FBox::new(ptr).expect("Failed to copy image")
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
///
/// ```
/// # use sfml::graphics::{Color, Image, PixelAccessError};
/// # use sfml::system::Vector2u;
/// let mut image = Image::new_solid(Vector2u::new(1, 1), Color::BLACK);
/// match image.set_pixel(Vector2u::new(1, 0), Color::WHITE) {
///     Err(PixelAccessError::XTooLarge { x, .. }) => assert_eq!(x, 1),
///     other => panic!("expected XTooLarge, got {other:?}"),
/// }
/// ```
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
                write!(f, "x index out of bounds. x:{x} width:{width}")
            }
            Self::YTooLarge { y, height } => {
                write!(f, "y index out of bounds. y:{y} height:{height}")
            }
        }
    }
}
