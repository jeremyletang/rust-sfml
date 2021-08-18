use crate::{
    ffi::{graphics as ffi, system::sfBool},
    graphics::{Image, IntRect, RenderWindow},
    inputstream::InputStream,
    sf_bool_ext::SfBoolExt,
    sf_box::{Dispose, SfBox},
    system::Vector2u,
    window::Window,
};
use std::{
    borrow::ToOwned,
    ffi::CString,
    io::{Read, Seek},
    ptr,
};

/// [`Image`] living on the graphics card that can be used for drawing.
///
/// `Texture` stores pixels that can be drawn, with a sprite for example.
///
/// A texture lives in the graphics card memory, therefore it is very fast to draw a
/// texture to a render target,
/// or copy a render target to a texture (the graphics card can access both directly).
///
/// Being stored in the graphics card memory has some drawbacks.
/// A texture cannot be manipulated as freely as a [`Image`],
/// you need to prepare the pixels first and then upload them to the texture in a
/// single operation (see the various update methods below).
///
/// `Texture` makes it easy to convert from/to [`Image`],
/// but keep in mind that these calls require transfers between the graphics card and
/// the central memory, therefore they are slow operations.
///
/// A texture can be loaded from an image, but also directly from a file/memory/stream.
/// The necessary shortcuts are defined so that you don't need an image first for the
/// most common cases.
/// However, if you want to perform some modifications on the pixels before creating the
/// final texture, you can load your file to a [`Image`], do whatever you need with the pixels,
/// and then call [`Texture::from_image`].
///
/// Since they live in the graphics card memory,
/// the pixels of a texture cannot be accessed without a slow copy first.
/// And they cannot be accessed individually.
/// Therefore, if you need to read the texture's pixels (like for pixel-perfect collisions),
/// it is recommended to store the collision information separately,
/// for example in an array of booleans.
///
/// Like [`Image`], `Texture` can handle a unique internal representation of pixels,
/// which is RGBA 32 bits.
/// This means that a pixel must be composed of
/// 8 bits red, green, blue and alpha channels – just like a [`Color`].
///
/// [`Color`]: crate::graphics::Color
#[derive(Debug)]
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Texture {
    _opaque: [u8; 0],
}

impl Texture {
    /// Return the size of the texture
    ///
    /// Return the Size in pixels
    #[must_use]
    pub fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfTexture_getSize(self.raw())) }
    }
    /// Tell whether the smooth filter is enabled or not for a texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfTexture_isSmooth(self.raw()) }.into_bool()
    }
    /// Tell whether a texture is repeated or not
    ///
    /// Return frue if repeat mode is enabled, false if it is disabled
    #[must_use]
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfTexture_isRepeated(self.raw()) }.into_bool()
    }
    /// Copy a texture's pixels to an image
    ///
    /// Return an image containing the texture's pixels
    #[must_use]
    pub fn copy_to_image(&self) -> Option<Image> {
        let img = unsafe { ffi::sfTexture_copyToImage(self.raw()) };
        if img.is_null() {
            None
        } else {
            Some(unsafe { Image::from_raw(img) })
        }
    }
    /// Tell whether the texture source is converted from sRGB or not.
    #[must_use]
    pub fn is_srgb(&self) -> bool {
        unsafe { ffi::sfTexture_isSrgb(self.raw()).into_bool() }
    }
    /// Get the underlying OpenGL handle of the texture.
    ///
    /// You shouldn't need to use this function, unless you have very specific stuff to implement
    /// that SFML doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn native_handle(&self) -> u32 {
        unsafe { ffi::sfTexture_getNativeHandle(self.raw()) }
    }

    /// Bind a texture for rendering
    ///
    /// This function is not part of the graphics API, it mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix `Texture` with OpenGL code.
    pub fn bind(&self) {
        unsafe { ffi::sfTexture_bind(self.raw()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfTexture {
        let ptr: *const Self = self;
        ptr as _
    }
    fn raw_mut(&mut self) -> *mut ffi::sfTexture {
        let ptr: *mut Self = self;
        ptr as _
    }
    /// Create a new texture
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Option<SfBox<Texture>> {
        let tex = unsafe { ffi::sfTexture_create(width, height) };
        SfBox::new(tex as *mut Self)
    }

    /// Create a new texture from memory
    ///
    /// The `area` argument can be used to load only a sub-rectangle of the whole image.
    /// If you want the entire image then use a default [`IntRect`].
    /// If the area rectangle crosses the bounds of the image,
    /// it is adjusted to fit the image size.
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    /// * area - Area of the image to load
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn from_memory(mem: &[u8], area: &IntRect) -> Option<SfBox<Texture>> {
        let tex = unsafe {
            ffi::sfTexture_createFromMemory(mem.as_ptr() as *const _, mem.len(), &area.raw())
        };
        SfBox::new(tex as *mut Self)
    }

    /// Create a new texture from a stream (a struct implementing Read + Seek)
    ///
    /// The `area` argument can be used to load only a sub-rectangle of the whole image.
    /// If you want the entire image then use a default [`IntRect`].
    /// If the area rectangle crosses the bounds of the image,
    /// it is adjusted to fit the image size.
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    /// * area - Area of the image to load
    ///
    /// Returns `None` on failure.
    pub fn from_stream<T: Read + Seek>(
        stream: &mut T,
        area: &mut IntRect,
    ) -> Option<SfBox<Texture>> {
        let mut input_stream = InputStream::new(stream);
        let tex = unsafe { ffi::sfTexture_createFromStream(&mut input_stream.0, &area.raw()) };
        SfBox::new(tex as *mut Self)
    }

    /// Create a new texture from a file
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn from_file(filename: &str) -> Option<SfBox<Texture>> {
        let c_str = CString::new(filename).unwrap();
        let tex = unsafe { ffi::sfTexture_createFromFile(c_str.as_ptr(), ptr::null()) };
        SfBox::new(tex as *mut Self)
    }

    /// Create a new texture from a file with a given area
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    /// * area - Area of the source image to load
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn from_file_with_rect(filename: &str, area: &IntRect) -> Option<SfBox<Texture>> {
        let c_str = CString::new(filename).unwrap();
        let tex = unsafe { ffi::sfTexture_createFromFile(c_str.as_ptr(), &area.raw()) };
        SfBox::new(tex as *mut Self)
    }

    /// Create a new texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    /// * area - Area of the source image to load
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn from_image_with_rect(image: &Image, area: &IntRect) -> Option<SfBox<Texture>> {
        let tex = unsafe { ffi::sfTexture_createFromImage(image.raw(), &area.raw()) };
        SfBox::new(tex as *mut Self)
    }

    /// Create a new texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    ///
    /// Returns `None` on failure.
    #[must_use]
    pub fn from_image(image: &Image) -> Option<SfBox<Texture>> {
        let tex = unsafe { ffi::sfTexture_createFromImage(image.raw(), ptr::null()) };
        SfBox::new(tex as *mut Self)
    }

    /// Update a texture from the contents of a window
    ///
    /// # Arguments
    /// * window - Window to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_window(&mut self, window: &Window, x: u32, y: u32) {
        unsafe { ffi::sfTexture_updateFromWindow(self.raw_mut(), window.raw(), x, y) }
    }

    /// Update a texture from the contents of a render window
    ///
    /// # Arguments
    /// * renderWindow - Render-window to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_render_window(&mut self, render_window: &RenderWindow, x: u32, y: u32) {
        unsafe { ffi::sfTexture_updateFromRenderWindow(self.raw_mut(), render_window.raw(), x, y) }
    }

    /// Update a texture from the contents of an image
    ///
    /// # Arguments
    /// * image - Image to copy to the texture
    /// * x - X offset in the texture where to copy the source pixels
    /// * y - Y offset in the texture where to copy the source pixels
    pub fn update_from_image(&mut self, image: &Image, x: u32, y: u32) {
        unsafe { ffi::sfTexture_updateFromImage(self.raw_mut(), image.raw(), x, y) }
    }

    /// Update a part of the texture from an array of pixels.
    ///
    /// The size of the pixel array must match the width and height arguments,
    /// and it must contain 32-bits RGBA pixels.
    ///
    /// This function does nothing if pixels is null or if the texture was not previously created.
    ///
    /// # Safety
    ///
    /// No additional check is performed on the size of the pixel array or the bounds of the
    /// area to update, passing invalid arguments will lead to an _undefined behavior_.
    pub unsafe fn update_from_pixels(
        &mut self,
        pixels: &[u8],
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) {
        ffi::sfTexture_updateFromPixels(self.raw_mut(), pixels.as_ptr(), width, height, x, y)
    }

    /// Enable or disable the smooth filter on a texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfTexture_setSmooth(self.raw_mut(), sfBool::from_bool(smooth)) }
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
        unsafe { ffi::sfTexture_setRepeated(self.raw_mut(), sfBool::from_bool(repeated)) }
    }

    /// Get the maximum texture size allowed
    ///
    /// Return the maximum size allowed for textures, in pixels
    #[must_use]
    pub fn maximum_size() -> u32 {
        unsafe { ffi::sfTexture_getMaximumSize() }
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
        unsafe { ffi::sfTexture_setSrgb(self.raw_mut(), SfBoolExt::from_bool(srgb)) }
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
        unsafe { ffi::sfTexture_generateMipmap(self.raw_mut()).into_bool() }
    }
}

impl ToOwned for Texture {
    type Owned = SfBox<Texture>;

    fn to_owned(&self) -> Self::Owned {
        let tex = unsafe { ffi::sfTexture_copy(self.raw()) };
        SfBox::new(tex as *mut Self).expect("Failed to copy texture")
    }
}

impl Dispose for Texture {
    unsafe fn dispose(&mut self) {
        let ptr: *mut Self = self;
        ffi::sfTexture_destroy(ptr as _)
    }
}
