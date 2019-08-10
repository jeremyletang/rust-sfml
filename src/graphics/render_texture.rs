use crate::graphics::csfml_graphics_sys as ffi;
use crate::graphics::{
    CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType, RectangleShape,
    RenderStates, RenderTarget, Sprite, Text, Texture, Vertex, VertexArray, View,
};
use crate::sf_bool_ext::SfBoolExt;
use crate::system::{Vector2f, Vector2i, Vector2u};
use csfml_system_sys::sfBool;

/// Target for off-screen 2D rendering into a texture
#[derive(Debug)]
pub struct RenderTexture {
    render_texture: *mut ffi::sfRenderTexture,
}

impl RenderTexture {
    /// Construct a new render texture
    ///
    /// # Arguments
    /// * width - Width of the render texture
    /// * height - Height of the render texture
    /// * depthBuffer - Do you want a depth-buffer attached?
    ///                 (useful only if you're doing 3D OpenGL on the rendertexture)
    ///
    /// Return Some(RenderTexture) or None
    pub fn new(width: u32, height: u32, depth_buffer: bool) -> Option<RenderTexture> {
        let tex =
            unsafe { ffi::sfRenderTexture_create(width, height, sfBool::from_bool(depth_buffer)) };
        if tex.is_null() {
            None
        } else {
            Some(RenderTexture {
                render_texture: tex,
            })
        }
    }

    /// Update the contents of the target texture
    pub fn display(&self) {
        unsafe { ffi::sfRenderTexture_display(self.render_texture) }
    }

    /// Activate or deactivate a render texture as the current target for rendering
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    pub fn set_active(&mut self, active: bool) -> bool {
        unsafe { ffi::sfRenderTexture_setActive(self.render_texture, sfBool::from_bool(active)) }
            .to_bool()
    }

    /// Get the target texture of a render texture
    ///
    /// Return the target texture
    pub fn texture(&self) -> &Texture {
        let tex = unsafe { ffi::sfRenderTexture_getTexture(self.render_texture) };
        assert!(!tex.is_null(), "sfRenderTexture_getTexture failed");
        unsafe { &*(tex as *const Texture) }
    }

    /// Enable or disable the smooth filter on a render texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe { ffi::sfRenderTexture_setSmooth(self.render_texture, sfBool::from_bool(smooth)) }
    }

    /// Tell whether the smooth filter is enabled or not for a render texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isSmooth(self.render_texture) }.to_bool()
    }
    /// Enable or disable texture repeating.
    ///
    /// This function is similar to Texture::setRepeated. This parameter is disabled by default.
    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe {
            ffi::sfRenderTexture_setRepeated(self.render_texture, SfBoolExt::from_bool(repeated))
        }
    }
    /// Tell whether the texture is repeated or not.
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isRepeated(self.render_texture).to_bool() }
    }
    /// Generate a mipmap using the current texture data.
    ///
    /// This function is similar to [`Texture::generate_mipmap`] and
    /// operates on the texture used as the target for drawing.
    /// Be aware that any draw operation may modify the base level image data.
    /// For this reason, calling this function only makes sense after all drawing is
    /// completed and display has been called. Not calling display after subsequent drawing
    /// will lead to __undefined behavior__ if a mipmap had been previously generated.
    pub unsafe fn generate_mipmap(&mut self) -> bool {
        ffi::sfRenderTexture_generateMipmap(self.render_texture).to_bool()
    }
}

impl RenderTarget for RenderTexture {
    fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfRenderTexture_getSize(self.render_texture)) }
    }
    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderTexture_clear(self.render_texture, color.raw()) }
    }
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderTexture_setView(self.render_texture, view.raw()) }
    }
    fn view(&self) -> &View {
        unsafe { &*(ffi::sfRenderTexture_getView(self.render_texture) as *const View) }
    }
    fn default_view(&self) -> &View {
        unsafe { &*(ffi::sfRenderTexture_getDefaultView(self.render_texture) as *const View) }
    }
    fn viewport(&self, view: &View) -> IntRect {
        unsafe {
            IntRect::from_raw(ffi::sfRenderTexture_getViewport(
                self.render_texture,
                view.raw(),
            ))
        }
    }
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRenderTexture_mapPixelToCoords(
                self.render_texture,
                point.raw(),
                view.raw(),
            ))
        }
    }
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderTexture_getView(self.render_texture) };
        unsafe {
            Vector2f::from_raw(ffi::sfRenderTexture_mapPixelToCoords(
                self.render_texture,
                point.raw(),
                view,
            ))
        }
    }
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfRenderTexture_mapCoordsToPixel(
                self.render_texture,
                point.raw(),
                view.raw(),
            ))
        }
    }
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i {
        let view = unsafe { ffi::sfRenderTexture_getView(self.render_texture) };
        unsafe {
            Vector2i::from_raw(ffi::sfRenderTexture_mapCoordsToPixel(
                self.render_texture,
                point.raw(),
                view,
            ))
        }
    }
    fn draw(&mut self, object: &dyn Drawable) {
        object.draw(self, RenderStates::default());
    }
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: RenderStates) {
        object.draw(self, render_states);
    }
    fn draw_text(&self, text: &Text, rs: RenderStates) {
        unsafe { ffi::sfRenderTexture_drawText(self.render_texture, text.raw(), &rs.raw()) }
    }
    fn draw_shape(&self, shape: &CustomShape, rs: RenderStates) {
        unsafe { ffi::sfRenderTexture_drawShape(self.render_texture, shape.raw(), &rs.raw()) }
    }
    fn draw_sprite(&self, sprite: &Sprite, rs: RenderStates) {
        unsafe { ffi::sfRenderTexture_drawSprite(self.render_texture, sprite.raw(), &rs.raw()) }
    }
    fn draw_circle_shape(&self, circle_shape: &CircleShape, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.render_texture, circle_shape.raw(), &rs.raw())
        }
    }
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(
                self.render_texture,
                rectangle_shape.raw(),
                &rs.raw(),
            )
        }
    }
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawConvexShape(self.render_texture, convex_shape.raw(), &rs.raw())
        }
    }
    fn draw_vertex_array(&self, vertex_array: &VertexArray, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawVertexArray(self.render_texture, vertex_array.raw(), &rs.raw())
        }
    }
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: RenderStates) {
        let len = vertices.len();
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(
                self.render_texture,
                vertices.as_ptr() as *const _,
                len,
                ty.raw(),
                &rs.raw(),
            );
        }
    }
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_pushGLStates(self.render_texture) }
    }
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_popGLStates(self.render_texture) }
    }
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_resetGLStates(self.render_texture) }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::sfRenderTexture_destroy(self.render_texture) }
    }
}
