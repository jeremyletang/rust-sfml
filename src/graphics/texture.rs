use {
    self::ffi::sfTexture_resize,
    crate::{
        IntoSfResult, SfError, SfResult,
        cpp::FBox,
        ffi::graphics::{self as ffi},
        graphics::{Image, IntRect, RenderWindow},
        system::{InputStream, Vector2u},
        window::Window,
    },
    std::{
        ffi::CString,
        io::{Read, Seek},
    },
};

decl_opaque! {
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
/// and then call [`Texture::load_from_image`].
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
pub Texture;
}

/// Creation and loading
impl Texture {
    /// Creates a new `Texture`
    pub fn new() -> SfResult<FBox<Texture>> {
        FBox::new(unsafe { ffi::sfTexture_new() }).into_sf_result()
    }

    /// Resize the texture
    ///
    /// If this function fails, the texture is left unchanged.
    ///
    /// # Arguments
    /// size - Width and height of the texture
    /// sRgb - `true` to enable sRGB conversion, `false` to disable it
    ///
    /// Returns `true` if resizing was successful, `false` if it failed
    pub fn resize(&mut self, size: Vector2u, srgb: bool) -> bool {
        unsafe { sfTexture_resize(self, size, srgb) }
    }

    /// Load texture from memory
    ///
    /// The `area` argument can be used to load only a sub-rectangle of the whole image.
    /// If you want the entire image then use a default [`IntRect`].
    /// If the area rectangle crosses the bounds of the image,
    /// it is adjusted to fit the image size.
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    /// * area - Area of the image to load
    pub fn load_from_memory(&mut self, mem: &[u8], srgb: bool, area: IntRect) -> SfResult<()> {
        unsafe {
            ffi::sfTexture_loadFromMemory(self, mem.as_ptr().cast(), mem.len(), srgb, area)
                .into_sf_result()
        }
    }

    /// Load texture from a stream (a struct implementing Read + Seek)
    ///
    /// The `area` argument can be used to load only a sub-rectangle of the whole image.
    /// If you want the entire image then use a default [`IntRect`].
    /// If the area rectangle crosses the bounds of the image,
    /// it is adjusted to fit the image size.
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    /// * area - Area of the image to load
    pub fn load_from_stream<T: Read + Seek>(
        &mut self,
        stream: &mut T,
        srgb: bool,
        area: IntRect,
    ) -> SfResult<()> {
        let mut input_stream = InputStream::new(stream);
        unsafe {
            ffi::sfTexture_loadFromStream(self, &raw mut *input_stream.stream, srgb, area)
                .into_sf_result()
        }
    }

    /// Load texture from a file
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    pub fn load_from_file(&mut self, filename: &str, srgb: bool, area: IntRect) -> SfResult<()> {
        let c_str = CString::new(filename)?;
        unsafe { ffi::sfTexture_loadFromFile(self, c_str.as_ptr(), srgb, area).into_sf_result() }
    }

    /// Convenience method to easily create and load a `Texture` from a file.
    pub fn from_file(filename: &str) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_file(filename, Default::default(), IntRect::default())?;
        Ok(new)
    }

    /// Convenience method to easily create and load a `Texture` from an iamge.
    pub fn from_image(image: &Image, srgb: bool, area: IntRect) -> SfResult<FBox<Self>> {
        let mut new = Self::new()?;
        new.load_from_image(image, srgb, area)?;
        Ok(new)
    }

    /// Load texture from an image
    ///
    /// # Arguments
    ///
    /// - `image`: Image to upload to the texture
    /// - `area`: Can be used to load only a sub-rectangle of the whole image.
    ///   If you want the entire image then use a default `IntRect`.
    ///   If the area rectangle crosses the bounds of the image,
    ///   it is adjusted to fit the image size.
    pub fn load_from_image(&mut self, image: &Image, srgb: bool, area: IntRect) -> SfResult<()> {
        unsafe { ffi::sfTexture_loadFromImage(self, image, srgb, area).into_sf_result() }
    }
}

/// Query properties
impl Texture {
    /// Return the size of the texture
    ///
    /// Return the Size in pixels
    #[must_use]
    pub fn size(&self) -> Vector2u {
        unsafe { ffi::sfTexture_getSize(self) }
    }
    /// Get the maximum texture size allowed
    ///
    /// Return the maximum size allowed for textures, in pixels
    #[must_use]
    pub fn maximum_size() -> u32 {
        unsafe { ffi::sfTexture_getMaximumSize() }
    }
    /// Tell whether the smooth filter is enabled or not for a texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfTexture_isSmooth(self) }
    }
    /// Tell whether a texture is repeated or not
    ///
    /// Return frue if repeat mode is enabled, false if it is disabled
    #[must_use]
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfTexture_isRepeated(self) }
    }
    /// Tell whether the texture source is converted from sRGB or not.
    #[must_use]
    pub fn is_srgb(&self) -> bool {
        unsafe { ffi::sfTexture_isSrgb(self) }
    }
}

/// Set properties
impl Texture {
    /// Enable or disable the smooth filter on a texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfTexture_setSmooth(self, smooth) }
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
        unsafe { ffi::sfTexture_setRepeated(self, repeated) }
    }
}

/// OpenGL interop
impl Texture {
    /// Get the underlying OpenGL handle of the texture.
    ///
    /// You shouldn't need to use this function, unless you have very specific stuff to implement
    /// that SFML doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn native_handle(&self) -> u32 {
        unsafe { ffi::sfTexture_getNativeHandle(self) }
    }

    /// Bind a texture for rendering
    ///
    /// This function is not part of the graphics API, it mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix `Texture` with OpenGL code.
    pub fn bind(&self) {
        unsafe { ffi::sfTexture_bind(self) }
    }
}

/// Copying and updating
impl Texture {
    /// Copy a texture's pixels to an image
    ///
    /// Return an image containing the texture's pixels
    pub fn copy_to_image(&self) -> SfResult<FBox<Image>> {
        let img = unsafe { ffi::sfTexture_copyToImage(self) };
        FBox::new(img).ok_or(SfError::CallFailed)
    }

    /// Update a part of the texture from the contents of a window.
    ///
    /// This function does nothing if either the texture or the window was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the window, passing an invalid combination
    /// of window size and offset will lead to an _undefined behavior_.
    pub unsafe fn update_from_window(&mut self, window: &Window, dest: Vector2u) {
        unsafe { ffi::sfTexture_updateFromWindow(self, window, dest) }
    }

    /// Update a part of the texture from the contents of a render window.
    ///
    /// This function does nothing if either the texture or the window was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the window, passing an invalid combination
    /// of window size and offset will lead to an _undefined behavior_.
    pub unsafe fn update_from_render_window(
        &mut self,
        render_window: &RenderWindow,
        dest: Vector2u,
    ) {
        unsafe { ffi::sfTexture_updateFromRenderWindow(self, render_window, dest) }
    }

    /// Update a part of the texture from an image.
    ///
    /// This function does nothing if the texture was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the image, passing an invalid combination
    /// of image size and offset will lead to an _undefined behavior_.
    pub unsafe fn update_from_image(&mut self, image: &Image, dest: Vector2u) {
        unsafe { ffi::sfTexture_updateFromImage(self, image, dest) }
    }

    /// Update a part of this texture from another texture.
    ///
    /// This function does nothing if either texture was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the texture,
    /// passing an invalid combination of texture size and offset will
    /// lead to an _undefined behavior_.
    pub unsafe fn update_from_texture(&mut self, texture: &Texture, dest: Vector2u) {
        unsafe { ffi::sfTexture_updateFromTexture(self, texture, dest) }
    }

    /// Update a part of the texture from an array of pixels.
    ///
    /// `pixels` must contain at least `width * height` 32 bit RGBA pixels.
    ///
    /// This function does nothing if the texture was not previously created.
    ///
    /// # Panics
    ///
    /// Panics the provided parameters would result in out of bounds access.
    pub fn update_from_pixels(&mut self, pixels: &[u8], size: Vector2u, dest: Vector2u) {
        let my_dims = self.size();
        assert!(
            dest.x + size.x <= my_dims.x
                && dest.y + size.y <= my_dims.y
                && pixels.len() >= (size.x * size.y * 4) as usize
        );
        unsafe { ffi::sfTexture_updateFromPixels(self, pixels.as_ptr(), size, dest) }
    }

    /// Swap the contents of this texture with those of another.
    pub fn swap(&mut self, other: &mut Texture) {
        unsafe { ffi::sfTexture_swap(self, other) }
    }
}

/// Mipmapping
impl Texture {
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
    pub fn generate_mipmap(&mut self) -> SfResult<()> {
        unsafe { ffi::sfTexture_generateMipmap(self) }.into_sf_result()
    }
}

impl ToOwned for Texture {
    type Owned = FBox<Texture>;

    fn to_owned(&self) -> Self::Owned {
        FBox::new(unsafe { ffi::sfTexture_cpy(self) }).expect("Failed to copy texture")
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { ffi::sfTexture_del(self) }
    }
}
