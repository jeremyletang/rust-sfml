/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
*
* 3. This notice may not be removed or altered from any source distribution.
*/

//! Target for off-screen 2D rendering into a texture

use libc::c_uint;

use traits::Wrappable;
use system::vector2::{Vector2f, Vector2i, Vector2u};
use graphics::{Drawable, View, Color, IntRect, Texture, CircleShape, RectangleShape, Text,
               RenderStates, Sprite, ConvexShape, VertexArray,
               RenderTarget, Vertex, PrimitiveType, CustomShape};

use ffi::sfml_types::{SFTRUE, SFFALSE};
use ffi::graphics::render_texture as ffi;

/// Target for off-screen 2D rendering into a texture
pub struct RenderTexture {
    render_texture: *mut ffi::sfRenderTexture
}

impl RenderTexture {
    /// Construct a new render texture
    ///
    /// # Arguments
    /// * width - Width of the render texture
    /// * height - Height of the render texture
    /// * depthBuffer - Do you want a depth-buffer attached? (useful only if you're doing 3D OpenGL on the rendertexture)
    ///
    /// Return Some(RenderTexture) or None
    pub fn new(width: u32,
               height: u32,
               depth_buffer: bool) -> Option<RenderTexture> {

        let tex = match depth_buffer {
            false       => unsafe { ffi::sfRenderTexture_create(width as c_uint,
                                                                height as c_uint, 
                                                                SFFALSE) },
            true        => unsafe { ffi::sfRenderTexture_create(width as c_uint, 
                                                                height as c_uint, 
                                                                SFTRUE) }
        };
        if tex.is_null() {
            None
        } else {
            Some(RenderTexture {
                    render_texture: tex
                })
        }
    }

    /// Update the contents of the target texture
    pub fn display(&self) -> () {
        unsafe {
            ffi::sfRenderTexture_display(self.render_texture)
        }
    }

    /// Activate or deactivate a render texture as the current target for rendering
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    pub fn set_active(&mut self, active: bool) -> bool {
        let ret = unsafe {
            match active {
                false => ffi::sfRenderTexture_setActive(self.render_texture,
                                                        SFFALSE),
                true  => ffi::sfRenderTexture_setActive(self.render_texture,
                                                        SFTRUE)
            }
        };
        match ret {
            SFFALSE => false,
            SFTRUE  => true
        }
    }

    /// Get the target texture of a render texture
    ///
    /// Return the target texture
    pub fn get_texture(&self) -> Option<Texture> {
        let tex = unsafe { ffi::sfRenderTexture_getTexture(self.render_texture) };
        if tex.is_null() {
            None
        }
        else {
            Some(Wrappable::wrap(tex))
        }
    }

    /// Enable or disable the smooth filter on a render texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) -> () {
        unsafe {
            match smooth {
                true        => ffi::sfRenderTexture_setSmooth(self.render_texture,
                                                              SFTRUE),
                false       => ffi::sfRenderTexture_setSmooth(self.render_texture,
                                                              SFFALSE)
            }
        }
    }

    /// Tell whether the smooth filter is enabled or not for a render texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    pub fn is_smooth(&self) -> bool {
        match unsafe { ffi::sfRenderTexture_isSmooth(self.render_texture) } {
            SFFALSE => false,
            SFTRUE  => true
        }
    }
}

impl RenderTarget for RenderTexture {

    /// Get the size of the rendering region of a render texture
    ///
    /// Return the size in pixels
    fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfRenderTexture_getSize(self.render_texture)
        }
    }

    /// Clear the rendertexture with the given color
    ///
    /// # Arguments
    /// * color - Fill color
    fn clear(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfRenderTexture_clear(self.render_texture, *color)
        }
    }

    /// Change the current active view of a render texture
    ///
    /// # Arguments
    /// * view - the new view
    fn set_view(&mut self, view: &View) -> () {
        unsafe {
            ffi::sfRenderTexture_setView(self.render_texture, view.unwrap())
        }
    }

    /// Get the current active view of a render texture
    ///
    /// Return the current active view
    fn get_view(&self) -> View {
        unsafe {
            Wrappable::wrap(ffi::sfRenderTexture_getView(self.render_texture))
        }
    }

    /// Get the default view of a render texture
    ///
    /// Return the default view of the render texture
    fn get_default_view(&self) -> View {
        unsafe {
            Wrappable::wrap(ffi::sfRenderTexture_getDefaultView(self.render_texture))
        }
    }

    /// Get the viewport of a view applied to this target
    ///
    /// # Arguments
    /// * view - Target view
    ///
    /// Return the viewport rectangle, expressed in pixels in the current target
    fn get_viewport(&self, view: &View) -> IntRect {
        unsafe {
            ffi::sfRenderTexture_getViewport(self.render_texture, view.unwrap())
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
    fn map_pixel_to_coords(&self,
                               point: &Vector2i,
                               view: &View) -> Vector2f {
        unsafe {
            ffi::sfRenderTexture_mapPixelToCoords(self.render_texture,
                                                  *point,
                                                  view.unwrap())
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
            ffi::sfRenderTexture_mapPixelToCoords(self.render_texture,
                                                  *point,
                                                  view)
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
    fn map_coords_to_pixel(&self,
                               point: &Vector2f,
                               view: &View) -> Vector2i {
        unsafe {
            ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture,
                                                  *point,
                                                  view.unwrap())
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
            ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture,
                                                  *point,
                                                  view)
        }
    }

    /// Draw a drawable object to the render-target
    ///
    /// # Arguments
    /// * object - Object to draw
    fn draw<T: Drawable>(&mut self, object: &T) -> () {
        object.draw(self, &mut RenderStates::default());
    }

    /// Draw a drawable object to the render-target
    ///
    /// # Arguments
    /// * object - Object to draw
    /// * renderStates - The RenderStates to associate to the object
    fn draw_with_renderstates<T: Drawable>(&mut self,
                                                object: &T,
                                                render_states: &mut RenderStates) {
        object.draw(self, render_states);
    }

    /// Draw Text
    fn draw_text(&self,
                        text: &Text,
                        rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawText(self.render_texture,
                                          text.unwrap(),
                                          rs.unwrap())
        }
    }

    /// Draw Shape
    fn draw_shape(&self,
                     shape: &CustomShape,
                     rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawShape(self.render_texture,
                                           shape.unwrap(),
                                           rs.unwrap())
        }
    }

    /// Draw Sprite
    fn draw_sprite(&self,
                          sprite: &Sprite,
                          rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawSprite(self.render_texture,
                                            sprite.unwrap(),
                                            rs.unwrap())
        }
    }

    /// Draw CircleShape
    fn draw_circle_shape(&self,
                                circle_shape: &CircleShape,
                                rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.render_texture,
                                                 circle_shape.unwrap(),
                                                 rs.unwrap())
        }
    }

    /// Draw RectangleShape
    fn draw_rectangle_shape(&self,
                                   rectangle_shape: &RectangleShape,
                                   rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.render_texture,
                                                    rectangle_shape.unwrap(),
                                                    rs.unwrap())
        }
    }

    /// Draw ConvexShape
    fn draw_convex_shape(&self,
                                convex_shape: &ConvexShape,
                                rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawConvexShape(self.render_texture,
                                                 convex_shape.unwrap(),
                                                 rs.unwrap())
        }
    }

    /// Draw VertexArray
    fn draw_vertex_array(&self,
                                vertex_array: &VertexArray,
                                rs: &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawVertexArray(self.render_texture,
                                                 vertex_array.unwrap(),
                                                 rs.unwrap())
        }
    }

    /// draw primitives
    fn draw_primitives(&self,
                          vertices: &[Vertex],
                          ty: PrimitiveType,
                          rs: &mut RenderStates) {

        let len = vertices.len() as u32;
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(self.render_texture,
                                                &vertices[0],
                                                len,
                                                ty,
                                                rs.unwrap());
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
    fn push_gl_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_pushGLStates(self.render_texture)
        }
    }

    /// Restore the previously saved OpenGL render states and matrices
    fn pop_gl_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_popGLStates(self.render_texture)
        }
    }

    /// Reset the internal OpenGL states so that the target is ready for drawing
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering, if you choose not to use
    /// pushGLStates/popGLStates. It makes sure that all OpenGL
    /// states needed by SFML are set, so that subsequent sfRenderWindow_draw*()
    /// calls will work as expected.
    fn reset_gl_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_resetGLStates(self.render_texture)
        }
    }


}

impl Drop for RenderTexture {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_destroy(self.render_texture)
        }
    }
}
