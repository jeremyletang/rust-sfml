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

use system::{Vector2f, Vector2i, Vector2u};
use graphics::{View, Color, IntRect, CircleShape, RectangleShape, Text,
               RenderStates, Sprite, Texture,
               RenderTarget, Vertex, PrimitiveType, BaseShape};

use ffi::{SfBool, Foreign, Ref};
use ffi::graphics as ffi;

/// Target for off-screen 2D rendering into a texture
pub struct RenderTexture(Foreign<ffi::sfRenderTexture>);

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
        unsafe {
            Foreign::new(ffi::sfRenderTexture_create(width as c_uint,
				height as c_uint, SfBool::from_bool(depth_buffer)))
        }.map(RenderTexture)
    }

	fn raw(&self) -> &ffi::sfRenderTexture { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfRenderTexture { self.0.as_mut() }

    /// Update the contents of the target texture
    pub fn display(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_display(self.raw_mut())
        }
    }

    /// Activate or deactivate a render texture as the current target for rendering
    ///
    /// # Arguments
    /// * active - true to activate, false to deactivate
    pub fn set_active(&mut self, active: bool) -> bool {
        unsafe {
            ffi::sfRenderTexture_setActive(self.raw_mut(), SfBool::from_bool(active))
        }.to_bool()
    }

    /// Get the target texture of a render texture
    ///
    /// Return the target texture
    pub fn get_texture(&self) -> Option<Ref<Texture>> {
		unsafe { Ref::new(ffi::sfRenderTexture_getTexture(self.raw())) }
    }

    /// Enable or disable the smooth filter on a render texture
    ///
    /// # Arguments
    /// * smooth - true to enable smoothing, false to disable it
    pub fn set_smooth(&mut self, smooth: bool) -> () {
        unsafe {
            ffi::sfRenderTexture_setSmooth(self.raw_mut(), SfBool::from_bool(smooth))
        }
    }

    /// Tell whether the smooth filter is enabled or not for a render texture
    ///
    /// Return true if smoothing is enabled, false if it is disabled
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isSmooth(self.raw()) }.to_bool()
    }
}

impl RenderTarget for RenderTexture {
    fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfRenderTexture_getSize(self.raw())
        }
    }

    fn clear(&mut self, color: &Color) -> () {
        unsafe {
            ffi::sfRenderTexture_clear(self.raw_mut(), *color)
        }
    }

    fn set_view(&mut self, view: &View) -> () {
        unsafe {
            ffi::sfRenderTexture_setView(self.raw_mut(), view.unwrap())
        }
    }

    fn get_view(&self) -> Ref<View> {
        unsafe {
            Ref::new(ffi::sfRenderTexture_getView(self.raw())).expect("Failed to wrap view")
        }
    }

    fn get_default_view(&self) -> Ref<View> {
        unsafe {
            Ref::new(ffi::sfRenderTexture_getDefaultView(self.raw())).expect("Failed to wrap view")
        }
    }

    fn get_viewport(&self, view: &View) -> IntRect {
        unsafe {
            ffi::sfRenderTexture_getViewport(self.raw(), view.unwrap())
        }
    }

    fn map_pixel_to_coords(&self,
                               point: &Vector2i,
                               view: &View) -> Vector2f {
        unsafe {
            ffi::sfRenderTexture_mapPixelToCoords(self.raw(),
                                                  *point,
                                                  view.unwrap())
        }
    }

    fn map_pixel_to_coords_current_view(&self, point: &Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderTexture_getView(self.raw()) };
        unsafe {
            ffi::sfRenderTexture_mapPixelToCoords(self.raw(),
                                                  *point,
                                                  view)
        }
    }

    fn map_coords_to_pixel(&self,
                               point: &Vector2f,
                               view: &View) -> Vector2i {
        unsafe {
            ffi::sfRenderTexture_mapCoordsToPixel(self.raw(),
                                                  *point,
                                                  view.unwrap())
        }
    }

    fn map_coords_to_pixel_current_view(&self, point: &Vector2f) -> Vector2i {
        let view = unsafe { ffi::sfRenderTexture_getView(self.raw()) };
        unsafe {
            ffi::sfRenderTexture_mapCoordsToPixel(self.raw(),
                                                  *point,
                                                  view)
        }
    }

    fn draw_text_rs(&mut self,
                        text: &Text,
                        rs: &RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawText(self.raw_mut(),
                                          text.unwrap(),
                                          &rs.unwrap())
        }
    }

    fn draw_shape_rs(&mut self,
                     shape: &BaseShape,
                     rs: &RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawShape(self.raw_mut(),
                                           shape.unwrap(),
                                           &rs.unwrap())
        }
    }

    fn draw_sprite_rs(&mut self,
                          sprite: &Sprite,
                          rs: &RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawSprite(self.raw_mut(),
                                            sprite.unwrap(),
                                            &rs.unwrap())
        }
    }

    fn draw_circle_shape_rs(&mut self,
                                circle_shape: &CircleShape,
                                rs: &RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.raw_mut(),
                                                 circle_shape.unwrap(),
                                                 &rs.unwrap())
        }
    }

    fn draw_rectangle_shape_rs(&mut self,
                                   rectangle_shape: &RectangleShape,
                                   rs: &RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.raw_mut(),
                                                    rectangle_shape.unwrap(),
                                                    &rs.unwrap())
        }
    }

    fn draw_primitives_rs(&mut self,
                          vertices: &[Vertex],
                          ty: PrimitiveType,
                          rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(self.raw_mut(),
                                                vertices.as_ptr(),
                                                vertices.len() as u32,
                                                ty,
                                                &rs.unwrap());
        }
    }

    fn push_gl_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_pushGLStates(self.raw_mut())
        }
    }

    fn pop_gl_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_popGLStates(self.raw_mut())
        }
    }

    fn reset_gl_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_resetGLStates(self.raw_mut())
        }
    }
}
