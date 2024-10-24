use {
    crate::{
        cpp::FBox,
        graphics::{Image, IntRect, RenderWindow, Texture},
        system::Vector2u,
        window::Window,
        SfResult,
    },
    std::{
        cell::RefCell,
        io::{Read, Seek},
        rc::Rc,
    },
};

/// [`Image`] living on the graphics card that can be used for drawing. (reference counted)
///
/// `RcTexture` stores pixels that can be drawn, with a `RcSprite` for example.
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
/// `RcTexture` makes it easy to convert from/to [`Image`],
/// but keep in mind that these calls require transfers between the graphics card and
/// the central memory, therefore they are slow operations.
///
/// A texture can be loaded from an image, but also directly from a file/memory/stream.
/// The necessary shortcuts are defined so that you don't need an image first for the
/// most common cases.
/// However, if you want to perform some modifications on the pixels before creating the
/// final texture, you can load your file to a [`Image`], do whatever you need with the pixels,
/// and then call [`RcTexture::load_from_image`].
///
/// Since they live in the graphics card memory,
/// the pixels of a texture cannot be accessed without a slow copy first.
/// And they cannot be accessed individually.
/// Therefore, if you need to read the texture's pixels (like for pixel-perfect collisions),
/// it is recommended to store the collision information separately,
/// for example in an array of booleans.
///
/// Like [`Image`], `RcTexture` can handle a unique internal representation of pixels,
/// which is RGBA 32 bits.
/// This means that a pixel must be composed of
/// 8 bits red, green, blue and alpha channels – just like a [`Color`].
///
/// [`Color`]: crate::graphics::Color
#[derive(Debug)]
pub struct RcTexture {
    texture: Rc<RefCell<FBox<Texture>>>,
}

impl RcTexture {
    /// Return the size of the texture
    ///
    /// Return the Size in pixels
    #[must_use]
    pub fn size(&self) -> Vector2u {
        self.texture.borrow().size()
    }
    /// Tell whether the smooth filter is enabled or not for a texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        self.texture.borrow().is_smooth()
    }
    /// Tell whether a texture is repeated or not
    ///
    /// Return frue if repeat mode is enabled, false if it is disabled
    #[must_use]
    pub fn is_repeated(&self) -> bool {
        self.texture.borrow().is_repeated()
    }
    /// Copy a texture's pixels to an image
    ///
    /// Return an image containing the texture's pixels
    pub fn copy_to_image(&self) -> SfResult<FBox<Image>> {
        self.texture.borrow().copy_to_image()
    }
    /// Tell whether the texture source is converted from sRGB or not.
    #[must_use]
    pub fn is_srgb(&self) -> bool {
        self.texture.borrow().is_srgb()
    }
    /// Get the underlying OpenGL handle of the texture.
    ///
    /// You shouldn't need to use this function, unless you have very specific stuff to implement
    /// that SFML doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn native_handle(&self) -> u32 {
        self.texture.borrow().native_handle()
    }

    /// Bind a texture for rendering
    ///
    /// This function is not part of the graphics API, it mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix `Texture` with OpenGL code.
    pub fn bind(&self) {
        self.texture.borrow().bind()
    }

    /// Create a new `RcTexture`
    pub fn new() -> SfResult<RcTexture> {
        Ok(RcTexture {
            texture: Rc::new(RefCell::new(Texture::new()?)),
        })
    }

    /// Create the texture.
    ///
    /// If this function fails, the texture is left unchanged.
    ///
    /// Returns whether creation was successful.
    #[must_use = "Check if texture was created successfully"]
    pub fn create(&mut self, width: u32, height: u32) -> SfResult<()> {
        self.texture.borrow_mut().create(width, height)
    }

    /// Get the source texture of an [`RcTexture`]
    ///
    /// It let's you temporarily borrow the [`Texture`] being held by an [`RcTexture`].
    /// This may be useful for many things, for example, using vertex arrays with
    /// [`RcTexture`].
    ///
    /// Returns a [`Texture`]
    #[must_use]
    pub fn raw_texture(&self) -> &Texture {
        unsafe { &*self.texture.as_ptr() }
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
    pub fn load_from_memory(&mut self, mem: &[u8], area: IntRect) -> SfResult<()> {
        self.texture.borrow_mut().load_from_memory(mem, area)
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
        area: IntRect,
    ) -> SfResult<()> {
        self.texture.borrow_mut().load_from_stream(stream, area)
    }

    /// Load texture from a file
    ///
    /// # Arguments
    /// * filename - Path of the image file to load
    pub fn load_from_file(&mut self, filename: &str, area: IntRect) -> SfResult<()> {
        self.texture.borrow_mut().load_from_file(filename, area)
    }

    /// Convenience method to easily create and load a `RcTexture` from a file.
    pub fn from_file(filename: &str) -> SfResult<RcTexture> {
        Ok(RcTexture {
            texture: Rc::new(RefCell::new(Texture::from_file(filename)?)),
        })
    }

    /// Convenience method to easily move a [`Texture`] into a `RcTexture`
    #[must_use]
    pub fn from_texture(texture: FBox<Texture>) -> RcTexture {
        RcTexture {
            texture: Rc::new(RefCell::new(texture)),
        }
    }

    /// Load texture from an image
    ///
    /// # Arguments
    /// * image - Image to upload to the texture
    pub fn load_from_image(&mut self, image: &Image, area: IntRect) -> SfResult<()> {
        self.texture.borrow_mut().load_from_image(image, area)
    }

    /// Update a part of the texture from the contents of a window.
    ///
    /// This function does nothing if either the texture or the window was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the window, passing an invalid combination
    /// of window size and offset will lead to an _undefined behavior_.
    pub unsafe fn update_from_window(&mut self, window: &Window, x: u32, y: u32) {
        unsafe { self.texture.borrow_mut().update_from_window(window, x, y) }
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
        x: u32,
        y: u32,
    ) {
        unsafe {
            self.texture
                .borrow_mut()
                .update_from_render_window(render_window, x, y)
        }
    }

    /// Update a part of the texture from an image.
    ///
    /// This function does nothing if the texture was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the image, passing an invalid combination
    /// of image size and offset will lead to an _undefined behavior_.
    pub unsafe fn update_from_image(&mut self, image: &Image, x: u32, y: u32) {
        unsafe { self.texture.borrow_mut().update_from_image(image, x, y) }
    }

    /// Update a part of this texture from another texture.
    ///
    /// This function does nothing if either texture was not previously created.
    ///
    /// # Safety
    /// No additional check is performed on the size of the texture,
    /// passing an invalid combination of texture size and offset will
    /// lead to an _undefined behavior_.
    pub unsafe fn update_from_texture(&mut self, texture: &Texture, x: u32, y: u32) {
        unsafe { self.texture.borrow_mut().update_from_texture(texture, x, y) }
    }

    /// Update a part of the texture from an array of pixels.
    ///
    /// The size of the pixel array must match the width and height arguments,
    /// and it must contain 32-bits RGBA pixels.
    ///
    /// This function does nothing if the texture was not previously created.
    ///
    /// # Panics
    ///
    /// Panics the provided parameters would result in out of bounds access.
    pub fn update_from_pixels(&mut self, pixels: &[u8], width: u32, height: u32, x: u32, y: u32) {
        self.texture
            .borrow_mut()
            .update_from_pixels(pixels, width, height, x, y)
    }

    /// Enable or disable the smooth filter on a texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        self.texture.borrow_mut().set_smooth(smooth)
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
        self.texture.borrow_mut().set_repeated(repeated)
    }

    /// Get the maximum texture size allowed
    ///
    /// Return the maximum size allowed for textures, in pixels
    #[must_use]
    pub fn maximum_size() -> u32 {
        Texture::maximum_size()
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
        self.texture.borrow_mut().set_srgb(srgb)
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
    pub fn generate_mipmap(&mut self) -> SfResult<()> {
        self.texture.borrow_mut().generate_mipmap()
    }
    /// Swap the contents of this texture with those of another.
    pub fn swap(&mut self, other: &mut Texture) {
        self.texture.borrow_mut().swap(other)
    }

    /// INTERNAL FUNCTION ONLY
    /// Allows other rc variants to request a weak pointer to the texture
    pub(super) fn downgrade(&self) -> std::rc::Weak<RefCell<FBox<Texture>>> {
        Rc::downgrade(&self.texture)
    }
}

impl ToOwned for RcTexture {
    type Owned = Self;

    fn to_owned(&self) -> Self {
        RcTexture {
            texture: Rc::new(RefCell::new(self.texture.borrow().to_owned())),
        }
    }
}
