use crate::{
    ffi::graphics as ffi,
    graphics::{
        CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType, RcSprite,
        RcText, RectangleShape, RenderStates, RenderTarget, Sprite, Text, Texture, Vertex,
        VertexBuffer, View,
    },
    sf_box::Dispose,
    system::{Vector2f, Vector2i, Vector2u},
    window::ContextSettings,
    IntoSfResult, SfBox, SfError, SfResult,
};

decl_opaque! {
    /// Target for off-screen 2D rendering into a texture
    RenderTexture;
}

impl RenderTexture {
    /// Construct a new render texture
    ///
    /// # Arguments
    /// * width - Width of the render texture
    /// * height - Height of the render texture
    pub fn new(width: u32, height: u32) -> SfResult<SfBox<Self>> {
        Self::with_settings(width, height, &ContextSettings::default())
    }

    /// Create a `RenderTexture` with the given `ContextSettings`.
    ///
    /// Useful if you want to enable multi-sampling or use the render-texture for
    /// OpenGL rendering that requires a depth or stencil buffer.
    /// Otherwise it is unnecessary, and you should call [`RenderTexture::new`].
    ///
    /// # Parameters
    /// * width - Width of the render-texture
    /// * height - Height of the render-texture
    /// * settings - Additional settings for the underlying OpenGL texture and context
    pub fn with_settings(
        width: u32,
        height: u32,
        settings: &ContextSettings,
    ) -> SfResult<SfBox<Self>> {
        let tex = unsafe { ffi::sfRenderTexture_createWithSettings(width, height, settings) };
        SfBox::new(tex).ok_or(SfError::CallFailed)
    }

    /// Update the contents of the target texture
    pub fn display(&mut self) {
        unsafe { ffi::sfRenderTexture_display(self) }
    }

    /// Tell if the render texture will use sRGB encoding when drawing it
    ///
    /// Returns true if sRGB encoding is enabled, false if sRGB encoding is disabled
    #[must_use]
    pub fn is_srgb(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isSrgb(self) }
    }

    /// Activate or deactivate a render texture as the current target for rendering
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    pub fn set_active(&mut self, active: bool) -> bool {
        unsafe { ffi::sfRenderTexture_setActive(self, active) }
    }

    /// Get the target texture of a render texture
    ///
    /// Return the target texture
    #[must_use]
    pub fn texture(&self) -> &Texture {
        unsafe {
            ffi::sfRenderTexture_getTexture(self)
                .as_ref()
                .expect("sfRenderTexture_getTexture failed")
        }
    }

    /// Enable or disable the smooth filter on a render texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfRenderTexture_setSmooth(self, smooth) }
    }

    /// Tell whether the smooth filter is enabled or not for a render texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    #[must_use]
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isSmooth(self) }
    }
    /// Enable or disable texture repeating.
    ///
    /// This function is similar to `Texture::setRepeated`. This parameter is disabled by default.
    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe { ffi::sfRenderTexture_setRepeated(self, repeated) }
    }
    /// Tell whether the texture is repeated or not.
    #[must_use]
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isRepeated(self) }
    }
    /// Generate a mipmap using the current texture data.
    ///
    /// This function is similar to [`Texture::generate_mipmap`] and
    /// operates on the texture used as the target for drawing.
    ///
    /// # Safety
    ///
    /// Be aware that any draw operation may modify the base level image data.
    /// For this reason, calling this function only makes sense after all drawing is
    /// completed and display has been called. Not calling display after subsequent drawing
    /// will lead to __undefined behavior__ if a mipmap had been previously generated.
    pub unsafe fn generate_mipmap(&mut self) -> SfResult<()> {
        unsafe { ffi::sfRenderTexture_generateMipmap(self) }.into_sf_result()
    }

    /// Get the maximum anti-aliasing level supported by the system.
    #[must_use]
    pub fn maximum_antialiasing_level() -> u32 {
        unsafe { ffi::sfRenderTexture_getMaximumAntialiasingLevel() }
    }
}

impl RenderTarget for RenderTexture {
    fn size(&self) -> Vector2u {
        unsafe { ffi::sfRenderTexture_getSize(self) }
    }
    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderTexture_clear(self, color) }
    }
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderTexture_setView(self, view) }
    }
    fn view(&self) -> &View {
        unsafe { &*(ffi::sfRenderTexture_getView(self)) }
    }
    fn default_view(&self) -> &View {
        unsafe { &*(ffi::sfRenderTexture_getDefaultView(self)) }
    }
    fn viewport(&self, view: &View) -> IntRect {
        unsafe { ffi::sfRenderTexture_getViewport(self, view) }
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe { ffi::sfRenderTexture_mapPixelToCoords_View(self, point, view) }
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        unsafe { ffi::sfRenderTexture_mapPixelToCoords(self, point) }
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe { ffi::sfRenderTexture_mapCoordsToPixel_View(self, point, view) }
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        unsafe { ffi::sfRenderTexture_mapCoordsToPixel(self, point) }
    }
    fn draw(&mut self, object: &dyn Drawable) {
        object.draw(self, &RenderStates::DEFAULT);
    }
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: &RenderStates) {
        object.draw(self, render_states);
    }
    fn draw_text(&mut self, text: &Text, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawText(self, text.raw(), rs) }
    }
    fn draw_rc_text(&mut self, text: &RcText, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawText(self, text.raw(), rs) }
    }
    fn draw_shape(&mut self, shape: &CustomShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawShape(self, shape.raw(), rs) }
    }
    fn draw_sprite(&mut self, sprite: &Sprite, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawSprite(self, sprite.raw(), rs) }
    }
    fn draw_rc_sprite(&mut self, sprite: &RcSprite, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawSprite(self, sprite.raw(), rs) }
    }
    fn draw_circle_shape(&mut self, circle_shape: &CircleShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawCircleShape(self, circle_shape.raw(), rs) }
    }
    fn draw_rectangle_shape(&mut self, rectangle_shape: &RectangleShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawRectangleShape(self, rectangle_shape.raw(), rs) }
    }
    fn draw_convex_shape(&mut self, convex_shape: &ConvexShape, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawConvexShape(self, convex_shape.raw(), rs) }
    }
    fn draw_vertex_buffer(&mut self, vertex_buffer: &VertexBuffer, rs: &RenderStates) {
        unsafe { ffi::sfRenderTexture_drawVertexBuffer(self, vertex_buffer, rs) }
    }
    fn draw_primitives(&mut self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        let len = vertices.len();
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(self, vertices.as_ptr().cast(), len, ty.0, rs);
        }
    }
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_pushGLStates(self) }
    }
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_popGLStates(self) }
    }
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_resetGLStates(self) }
    }
}

impl Dispose for RenderTexture {
    unsafe fn dispose(&mut self) {
        unsafe { ffi::sfRenderTexture_destroy(self) }
    }
}
