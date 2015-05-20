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

/// Target for off-screen 2D rendering into a texture.
///
/// `RenderTexture` implements the same 2D drawing and OpenGL-related functions
/// as `RenderWindow` (see `RenderTarget` for details), with the difference that
/// the result is stored in an off-screen texture rather than being shown in a
/// window.
///
/// Rendering to a texture can be useful in a variety of situations, such as:
///
/// * Precomputing a complex static texture (like a level's background from
///   multiple tiles).
/// * Applying post-effects to the whole scene with shaders.
/// * Creating a sprite from a 3D object rendered with OpenGL.
pub struct RenderTexture(Foreign<ffi::sfRenderTexture>);

impl RenderTexture {
	/// Construct a new render texture.
	///
	/// The width and height of the texture should be specified. The last
	/// parameter, `depth_buffer`, is useful if you want to use the texture for
	/// 3D OpenGL rendering that requires a depth buffer. Otherwise it is
	/// unnecessary, and should be left false.
    ///
    /// Returns Some(RenderTexture) or None on failure.
    pub fn new(width: u32, height: u32, depth_buffer: bool) -> Option<RenderTexture> {
        unsafe {
            Foreign::new(ffi::sfRenderTexture_create(width as c_uint,
				height as c_uint, SfBool::from_bool(depth_buffer)))
        }.map(RenderTexture)
    }

	fn raw(&self) -> &ffi::sfRenderTexture { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfRenderTexture { self.0.as_mut() }

    /// Update the contents of the target texture.
	///
	/// This function updates the target texture with what has been drawn so
	/// far. Like for windows, calling this function is mandatory at the end of
	/// rendering. Not calling it may leave the texture in an undefined state. 
    pub fn display(&mut self) {
        unsafe {
            ffi::sfRenderTexture_display(self.raw_mut())
        }
    }

    /// Activate of deactivate the render-texture for rendering.
	///
	/// This function makes the render-texture's context current for future
	/// OpenGL rendering operations (so you shouldn't care about it if you're
	/// not doing direct OpenGL stuff). Only one context can be current in a
	/// thread, so if you want to draw OpenGL geometry to another render target
	/// (like a `RenderWindow`) don't forget to activate it again.
	///
	/// Returns true on success, or false on failure.
    pub fn set_active(&mut self, active: bool) -> bool {
        unsafe {
            ffi::sfRenderTexture_setActive(self.raw_mut(), SfBool::from_bool(active))
        }.to_bool()
    }

	/// Get a read-only reference to the target texture.
	///
	/// After drawing to the render texture and calling `display()`, you can
	/// retrieve the updated texture using this function, and draw it using a
	/// sprite (for example).
    pub fn get_texture(&self) -> Option<Ref<Texture>> {
		unsafe { Ref::new(ffi::sfRenderTexture_getTexture(self.raw())) }
    }

    /// Enable or disable texture smoothing.
	///
	/// This function is similar to `Texture::setSmooth`. Smoothing is disabled
	/// by default.
    pub fn set_smooth(&mut self, smooth: bool) {
        unsafe {
            ffi::sfRenderTexture_setSmooth(self.raw_mut(), SfBool::from_bool(smooth))
        }
    }

	/// Tell whether the smooth filtering is enabled or not.
    pub fn is_smooth(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isSmooth(self.raw()) }.to_bool()
    }

	/// Enable or disbale texture repeating.
	///
	/// This funtion is similar to `Texture::setRepeated`. Repeating is disabled
	/// by default.
    pub fn set_repeated(&mut self, repeated: bool) {
        unsafe {
            ffi::sfRenderTexture_setRepeated(self.raw_mut(), SfBool::from_bool(repeated))
        }
    }

    /// Tell whether the texture is repeated or not.
    pub fn is_repeated(&self) -> bool {
        unsafe { ffi::sfRenderTexture_isRepeated(self.raw()) }.to_bool()
    }
}

impl RenderTarget for RenderTexture {
    fn get_size(&self) -> Vector2u {
        unsafe { ffi::sfRenderTexture_getSize(self.raw()) }
    }

    fn clear(&mut self, color: Color) {
        unsafe { ffi::sfRenderTexture_clear(self.raw_mut(), color) }
    }

    fn set_view(&mut self, view: &View) {
        unsafe { ffi::sfRenderTexture_setView(self.raw_mut(), view.unwrap()) }
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
        unsafe { ffi::sfRenderTexture_getViewport(self.raw(), view.unwrap()) }
    }

    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f {
        unsafe {
            ffi::sfRenderTexture_mapPixelToCoords(self.raw(), point, view.unwrap())
        }
    }

    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i {
        unsafe {
            ffi::sfRenderTexture_mapCoordsToPixel(self.raw(), point, view.unwrap())
        }
    }

    fn draw_text_rs(&mut self, text: &Text, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawText(self.raw_mut(), text.unwrap(), &rs.unwrap())
        }
    }

    fn draw_shape_rs(&mut self, shape: &BaseShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawShape(self.raw_mut(), shape.unwrap(), &rs.unwrap())
        }
    }

    fn draw_sprite_rs(&mut self, sprite: &Sprite, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawSprite(self.raw_mut(), sprite.unwrap(), &rs.unwrap())
        }
    }

    fn draw_circle_shape_rs(&mut self, circle: &CircleShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.raw_mut(), circle.unwrap(), &rs.unwrap())
        }
    }

    fn draw_rectangle_shape_rs(&mut self, rect: &RectangleShape, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.raw_mut(), rect.unwrap(), &rs.unwrap())
        }
    }

    fn draw_primitives_rs(&mut self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates) {
        unsafe {
            ffi::sfRenderTexture_drawPrimitives(self.raw_mut(),
                                                vertices.as_ptr(),
                                                vertices.len() as u32,
                                                ty,
                                                &rs.unwrap());
        }
    }

    fn push_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_pushGLStates(self.raw_mut()) }
    }

    fn pop_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_popGLStates(self.raw_mut()) }
    }

    fn reset_gl_states(&mut self) {
        unsafe { ffi::sfRenderTexture_resetGLStates(self.raw_mut()) }
    }
}
