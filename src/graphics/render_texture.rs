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

use system::raw_conv::{Raw, FromRaw};
use system::{Vector2f, Vector2i, Vector2u};
use graphics::{Drawable, View, ViewRef, Color, IntRect, TextureRef, CircleShape, RectangleShape,
               Text, RenderStates, Sprite, ConvexShape, VertexArray, RenderTarget, Vertex,
               PrimitiveType, CustomShape};

use csfml_system_sys::sfBool;
use csfml_graphics_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Target for off-screen 2D rendering into a texture
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
            Some(RenderTexture { render_texture: tex })
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
    pub fn texture(&self) -> &TextureRef {
        let tex = unsafe { ffi::sfRenderTexture_getTexture(self.render_texture) };
        if tex.is_null() {
            panic!("RenderTexture::texture: Texture is null")
        } else {
            unsafe { &*(tex as *const TextureRef) }
        }
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
    /// This function is similar to `Texture::generate_mipmap` and operates on the texture used as
    /// the target for drawing. Be aware that any draw operation may modify the base level
    /// image data. For this reason, calling this function only makes sense after all drawing
    /// is completed and display has been called. Not calling display after subsequent drawing
    /// will lead to __undefined behavior__ if a mipmap had been previously generated.
    pub unsafe fn generate_mipmap(&mut self) -> bool {
        ffi::sfRenderTexture_generateMipmap(self.render_texture).to_bool()
    }
}

impl RenderTarget for RenderTexture {
    /// Get the size of the rendering region of a render texture
    ///
    /// Return the size in pixels
    fn size(&self) -> Vector2u {
        unsafe { Vector2u::from_raw(ffi::sfRenderTexture_getSize(self.render_texture)) }
    }

    /// Clear the rendertexture with the given color
    ///
    /// # Arguments
    /// * color - Fill color
    fn clear(&mut self, color: &Color) {
        unsafe { ffi::sfRenderTexture_clear(self.render_texture, color.raw()) }
    }

    /// Change the current active view of a render texture
    ///
    /// # Arguments
    /// * view - the new view
    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderTexture_setView(self.render_texture, view.raw()) }
    }

    /// Get the current active view of a render texture
    ///
    /// Return the current active view
    fn view(&self) -> &ViewRef {
        unsafe { &*(ffi::sfRenderTexture_getView(self.render_texture) as *const ViewRef) }
    }

    /// Get the default view of a render texture
    ///
    /// Return the default view of the render texture
    fn default_view(&self) -> &ViewRef {
        unsafe { &*(ffi::sfRenderTexture_getDefaultView(self.render_texture) as *const ViewRef) }
    }

    /// Get the viewport of a view applied to this target
    ///
    /// # Arguments
    /// * view - Target view
    ///
    /// Return the viewport rectangle, expressed in pixels in the current target
    fn viewport(&self, view: &View) -> IntRect {
        unsafe {
            IntRect::from_raw(ffi::sfRenderTexture_getViewport(self.render_texture, view.raw()))
        }
    }

    /// Convert a point from texture coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-texture. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render texture, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-texture may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version uses a custom view for calculations, see
    /// map_pixel_to_coords if you want to use the current view of the
    /// render-texture.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    /// * view - The view to use for converting the point
    ///
    /// Return the converted point, in "world" units
    fn map_pixel_to_coords(&self, point: &Vector2i, view: &View) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRenderTexture_mapPixelToCoords(self.render_texture,
                                                                     point.raw(),
                                                                     view.raw()))
        }
    }

    /// Convert a point from texture coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-texture. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render texture, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-texture may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version the current view for calculations, see
    /// map_pixel_to_coordss if you want to use a custom view
    ///
    /// # Arguments
    /// * point - Pixel to convert
    ///
    /// Return the converted point, in "world" units
    fn map_pixel_to_coords_current_view(&self, point: &Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderTexture_getView(self.render_texture) };
        unsafe {
            Vector2f::from_raw(ffi::sfRenderTexture_mapPixelToCoords(self.render_texture,
                                                                     point.raw(),
                                                                     view))
        }
    }

    /// Convert a point from world coordinates to render texture coordinates
    ///
    /// This function finds the pixel of the render-texture that matches
    /// the given 2D point. In other words, it goes through the same process
    /// as the graphics card, to compute the final position of a rendered point.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render texture, this assertion is not true anymore, ie. a point
    /// located at (150, 75) in your 2D world may map to the pixel
    /// (10, 50) of your render-texture -- if the view is translated by (140, 25).
    ///
    /// This version uses a custom view for calculations, see
    /// map_coords_to_pixel_current_view if you want to use the current view of the
    /// render-texture.
    ///
    /// # Arguments
    /// * point - Point to convert
    /// * view - The view to use for converting the point
    fn map_coords_to_pixel(&self, point: &Vector2f, view: &View) -> Vector2i {
        unsafe {
            Vector2i::from_raw(ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture,
                                                                     point.raw(),
                                                                     view.raw()))
        }
    }

    /// Convert a point from world coordinates to render texture coordinates
    ///
    /// This function finds the pixel of the render-texture that matches
    /// the given 2D point. In other words, it goes through the same process
    /// as the graphics card, to compute the final position of a rendered point.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render texture, this assertion is not true anymore, ie. a point
    /// located at (150, 75) in your 2D world may map to the pixel
    /// (10, 50) of your render-texture -- if the view is translated by (140, 25).
    ///
    /// This version uses the default view for calculations, see
    /// map_coords_to_pixel if you want to use as custom view.
    ///
    /// # Arguments
    /// * point - Point to convert
    fn map_coords_to_pixel_current_view(&self, point: &Vector2f) -> Vector2i {
        let view = unsafe { ffi::sfRenderTexture_getView(self.render_texture) };
        unsafe {
            Vector2i::from_raw(ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture,
                                                                     point.raw(),
                                                                     view))
        }
    }

    /// Draw a drawable object to the render-target
    ///
    /// # Arguments
    /// * object - Object to draw
    fn draw(&mut self, object: &Drawable) {
        object.draw(self, RenderStates::default());
    }

    /// Draw a drawable object to the render-target
    ///
    /// # Arguments
    /// * object - Object to draw
    /// * renderStates - The RenderStates to associate to the object
    fn draw_with_renderstates(&mut self, object: &Drawable, render_states: RenderStates) {
        object.draw(self, render_states);
    }

    /// Draw Text
    fn draw_text(&self, text: &Text, rs: RenderStates) {
        unsafe { ffi::sfRenderTexture_drawText(self.render_texture, text.raw(), &rs.raw()) }
    }

    /// Draw Shape
    fn draw_shape(&self, shape: &CustomShape, rs: RenderStates) {
        unsafe { ffi::sfRenderTexture_drawShape(self.render_texture, shape.raw(), &rs.raw()) }
    }

    /// Draw Sprite
    fn draw_sprite(&self, sprite: &Sprite, rs: RenderStates) {
        unsafe { ffi::sfRenderTexture_drawSprite(self.render_texture, sprite.raw(), &rs.raw()) }
    }

    /// Draw CircleShape
    fn draw_circle_shape(&self, circle_shape: &CircleShape, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.render_texture, circle_shape.raw(), &rs.raw())
        }
    }

    /// Draw RectangleShape
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.render_texture,
                                                    rectangle_shape.raw(),
                                                    &rs.raw())
        }
    }

    /// Draw ConvexShape
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawConvexShape(self.render_texture, convex_shape.raw(), &rs.raw())
        }
    }

    /// Draw VertexArray
    fn draw_vertex_array(&self, vertex_array: &VertexArray, rs: RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawVertexArray(self.render_texture, vertex_array.raw(), &rs.raw())
        }
    }

    /// draw primitives
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: RenderStates) {

        let len = vertices.len();
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(self.render_texture,
                                                &vertices[0] as *const _ as *const _,
                                                len,
                                                ty,
                                                &rs.raw());
        }
    }

    /// Save the current OpenGL render states and matrices
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering. Combined with popGLStates,
    /// it ensures that:
    /// SFML's internal states are not messed up by your OpenGL code
    /// and that your OpenGL states are not modified by a call to a SFML function
    ///
    /// Note that this function is quite expensive: it saves all the
    /// possible OpenGL states and matrices, even the ones you
    /// don't care about. Therefore it should be used wisely.
    /// It is provided for convenience, but the best results will
    /// be achieved if you handle OpenGL states yourself (because
    /// you know which states have really changed, and need to be
    /// saved and restored). Take a look at the resetGLStates
    /// function if you do so.
    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_pushGLStates(self.render_texture) }
    }

    /// Restore the previously saved OpenGL render states and matrices
    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_popGLStates(self.render_texture) }
    }

    /// Reset the internal OpenGL states so that the target is ready for drawing
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering, if you choose not to use
    /// pushGLStates/popGLStates. It makes sure that all OpenGL
    /// states needed by SFML are set, so that subsequent sfRenderWindow_draw*()
    /// calls will work as expected.
    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_resetGLStates(self.render_texture) }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { ffi::sfRenderTexture_destroy(self.render_texture) }
    }
}
