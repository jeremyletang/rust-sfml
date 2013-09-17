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

/*!
* Target for off-screen 2D rendering into a texture
*
*
*
*
*/

use std::libc::c_uint;
use std::ptr;

use traits::drawable::Drawable;
use traits::wrappable::Wrappable;
use system::vector2::{Vector2f, Vector2i, Vector2u};
use graphics::view::View;
use graphics::sprite::Sprite;
use graphics::color::Color;
use graphics::rect::IntRect;
use graphics::texture::Texture;
use graphics::text::Text;
use graphics::circle_shape::CircleShape;
use graphics::rectangle_shape::RectangleShape;
use graphics::vertex_array::VertexArray;
use graphics::convex_shape::ConvexShape;
use graphics::render_states::RenderStates;
use graphics::shape::Shape;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_void, c_uint};

    use sfml_types::SfBool;
    use graphics::view;
    use system::vector2::{Vector2f, Vector2i, Vector2u};
    use graphics::render_states;
    use graphics::sprite;
    use graphics::color::Color;
    use graphics::rect::IntRect;
    use graphics::texture;
    use graphics::text;
    use graphics::circle_shape;
    use graphics::rectangle_shape;
    use graphics::vertex_array;
    use graphics::convex_shape;
    use graphics::shape;

    pub struct sfRenderTexture {
        This :          *c_void,
        Target :        *texture::ffi::sfTexture,
        DefaultView :   view::ffi::sfView,
        CurrentView :   view::ffi::sfView
    }
    
    extern "C" {
        pub fn sfRenderTexture_create(width : c_uint, height : c_uint, depthBuffer : SfBool) -> *sfRenderTexture;
        pub fn sfRenderTexture_destroy(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_getSize(renderTexture : *sfRenderTexture) -> Vector2u;
        pub fn sfRenderTexture_setActive(renderTexture : *sfRenderTexture, active : SfBool) -> SfBool;
        pub fn sfRenderTexture_display(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_clear(renderTexture : *sfRenderTexture, color : Color) -> ();
        pub fn sfRenderTexture_setView(renderTexture : *sfRenderTexture, view : *view::ffi::sfView) -> ();
        pub fn sfRenderTexture_getView(renderTexture : *sfRenderTexture) -> *view::ffi::sfView;
        pub fn sfRenderTexture_getDefaultView(renderTexture : *sfRenderTexture) -> *view::ffi::sfView;
        pub fn sfRenderTexture_getViewport(renderTexture : *sfRenderTexture, view : *view::ffi::sfView) -> IntRect;
        pub fn sfRenderTexture_mapPixelToCoords(renderTexture : *sfRenderTexture, point : Vector2i, view : *view::ffi::sfView) -> Vector2f;
        pub fn sfRenderTexture_mapCoordsToPixel(renderTexture : *sfRenderTexture, point : Vector2f, view : *view::ffi::sfView) -> Vector2i;
        pub fn sfRenderTexture_drawSprite(renderTexture : *sfRenderTexture, object : *sprite::ffi::sfSprite, states : *render_states::ffi::sfRenderStates) -> ();
        pub fn sfRenderTexture_drawText(renderTexture : *sfRenderTexture, object : *text::ffi::sfText, states : *render_states::ffi::sfRenderStates) -> ();
        fn sfRenderTexture_drawShape(renderTexture : *sfRenderTexture, object : *shape::ffi::sfShape, states : *render_states::ffi::sfRenderStates) -> ();
        pub fn sfRenderTexture_drawCircleShape(renderTexture : *sfRenderTexture, object : *circle_shape::ffi::sfCircleShape, states : *render_states::ffi::sfRenderStates) -> ();
        pub fn sfRenderTexture_drawConvexShape(renderTexture : *sfRenderTexture, object : *convex_shape::ffi::sfConvexShape, states : *render_states::ffi::sfRenderStates) -> ();
        pub fn sfRenderTexture_drawRectangleShape(renderTexture : *sfRenderTexture, object : *rectangle_shape::ffi::sfRectangleShape, states : *render_states::ffi::sfRenderStates) -> ();
        pub fn sfRenderTexture_drawVertexArray(renderTexture : *sfRenderTexture, object : *vertex_array::ffi::sfVertexArray, states : *render_states::ffi::sfRenderStates) -> ();
        //fn sfRenderTexture_drawPrimitives(renderTexture : *sfRenderTexture) -> (); // a modifier
        pub fn sfRenderTexture_pushGLStates(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_popGLStates(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_resetGLStates(renderTexture : *sfRenderTexture) -> ();
        pub fn sfRenderTexture_getTexture(renderTexture : *sfRenderTexture) -> *texture::ffi::sfTexture;
        pub fn sfRenderTexture_setSmooth(renderTexture : *sfRenderTexture, smooth : SfBool) -> ();
        pub fn sfRenderTexture_isSmooth(renderTexture : *sfRenderTexture) -> SfBool;
    }
}

#[doc(hidden)]
pub struct RenderTexture {
    render_texture : *ffi::sfRenderTexture
}

impl RenderTexture {
    /**
    * Construct a new render texture
    *
    * # Arguments
    * * width - Width of the render texture
    * * height - Height of the render texture
    * * depthBuffer - Do you want a depth-buffer attached? (useful only if you're doing 3D OpenGL on the rendertexture)
    *
    * Return a new option on RenderTexture object, or None if it failed
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new(width : uint, height : uint, depth_buffer : bool) -> Option<RenderTexture> {
            let tex = match depth_buffer {
                false       => unsafe { ffi::sfRenderTexture_create(width as c_uint, height as c_uint, 0) },
                true        => unsafe { ffi::sfRenderTexture_create(width as c_uint, height as c_uint, 1) }
            };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(RenderTexture {
                render_texture : tex
            })
        }
    }
    
    /**
    * Get the size of the rendering region of a render texture
    *
    * Return the size in pixels
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfRenderTexture_getSize(self.render_texture)
        }
    }

    /**
    * Activate or deactivate a render texture as the current target for rendering
    *
    * # Arguments
    * * active - true to activate, false to deactivate
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_active(&mut self, active : bool) -> bool {
        match unsafe {
            match active {
                false       => ffi::sfRenderTexture_setActive(self.render_texture, 0),
                true        => ffi::sfRenderTexture_setActive(self.render_texture, 1)
            }
        } {
            0   => false,
            _   => true
        }
    }

    /**
    * Update the contents of the target texture
    *
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn display(&self) -> () {
        unsafe {
            ffi::sfRenderTexture_display(self.render_texture)
        }
    }

    /**
    * Clear the rendertexture with the given color
    *
    * # Arguments
    * * color - Fill color
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn clear(&mut self, color : &Color) -> () {
        unsafe {
            ffi::sfRenderTexture_clear(self.render_texture, *color)
        }
    }

    /**
    * Change the current active view of a render texture
    *
    * # Arguments
    * * view - the new view
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_view(&mut self, view : &View) -> () {
        unsafe {
            ffi::sfRenderTexture_setView(self.render_texture, view.unwrap())
        }
    }

    /**
    * Get the current active view of a render texture
    *
    * Return the current active view
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_view(&self) -> View {
        unsafe {
            Wrappable::wrap(ffi::sfRenderTexture_getView(self.render_texture))
        }
    }

    /**
    * Get the default view of a render texture
    *
    * Return the default view of the render texture
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_default_view(&self) -> View {
        unsafe {
            Wrappable::wrap(ffi::sfRenderTexture_getDefaultView(self.render_texture))
        }
    }

    /**
    * Get the viewport of a view applied to this target
    *
    * # Arguments
    * * view - Target view
    *
    * Return the viewport rectangle, expressed in pixels in the current target
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_viewport(&self, view : &View) -> IntRect {
        unsafe {
            ffi::sfRenderTexture_getViewport(self.render_texture, view.unwrap())
        }
    }
    
    /**
    * Convert a point from texture coordinates to world coordinates
    *
    * This function finds the 2D position that matches the
    * given pixel of the render-texture. In other words, it does
    * the inverse of what the graphics card does, to find the
    * initial position of a rendered pixel.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render texture, this assertion is not true anymore, ie. a point
    * located at (10, 50) in your render-texture may map to the point
    * (150, 75) in your 2D world -- if the view is translated by (140, 25).
    * 
    * This function is typically used to find which point (or object) is
    * located below the mouse cursor.
    * 
    * This version uses a custom view for calculations, see
    * map_pixel_to_coords if you want to use the current view of the
    * render-texture.
    *
    * # Arguments
    * * point - Pixel to convert
    * * view - The view to use for converting the point
    * 
    * Return the converted point, in "world" units
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn map_pixel_to_coords(&self, point : &Vector2i, view : &View) -> Vector2f {
        unsafe {
            ffi::sfRenderTexture_mapPixelToCoords(self.render_texture, *point, view.unwrap())
        }
    }

    /**
    * Convert a point from texture coordinates to world coordinates
    *
    * This function finds the 2D position that matches the
    * given pixel of the render-texture. In other words, it does
    * the inverse of what the graphics card does, to find the
    * initial position of a rendered pixel.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render texture, this assertion is not true anymore, ie. a point
    * located at (10, 50) in your render-texture may map to the point
    * (150, 75) in your 2D world -- if the view is translated by (140, 25).
    * 
    * This function is typically used to find which point (or object) is
    * located below the mouse cursor.
    * 
    * This version the current view for calculations, see
    * map_pixel_to_coordss if you want to use a custom view
    *
    * # Arguments
    * * point - Pixel to convert
    * 
    * Return the converted point, in "world" units
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn map_pixel_to_coords_current_view(&self, point : &Vector2i) -> Vector2f {
        let view = unsafe { ffi::sfRenderTexture_getView(self.render_texture) };
        unsafe {
            ffi::sfRenderTexture_mapPixelToCoords(self.render_texture, *point, view)
        }
    }

    /**
    * Convert a point from world coordinates to render texture coordinates
    *
    * This function finds the pixel of the render-texture that matches
    * the given 2D point. In other words, it goes through the same process
    * as the graphics card, to compute the final position of a rendered point.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render texture, this assertion is not true anymore, ie. a point
    * located at (150, 75) in your 2D world may map to the pixel
    * (10, 50) of your render-texture -- if the view is translated by (140, 25).
    * 
    * This version uses a custom view for calculations, see
    * map_coords_to_pixel_current_view if you want to use the current view of the
    * render-texture.
    *
    * # Arguments
    * * point - Point to convert
    * * view - The view to use for converting the point
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn map_coords_to_pixel(&self, point : &Vector2f, view : &View) -> Vector2i {
        unsafe {
            ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture, *point, view.unwrap())
        }
    }

    /**
    * Convert a point from world coordinates to render texture coordinates
    *
    * This function finds the pixel of the render-texture that matches
    * the given 2D point. In other words, it goes through the same process
    * as the graphics card, to compute the final position of a rendered point.
    *
    * Initially, both coordinate systems (world units and target pixels)
    * match perfectly. But if you define a custom view or resize your
    * render texture, this assertion is not true anymore, ie. a point
    * located at (150, 75) in your 2D world may map to the pixel
    * (10, 50) of your render-texture -- if the view is translated by (140, 25).
    * 
    * This version uses the default view for calculations, see
    * map_coords_to_pixel if you want to use as custom view.
    *
    * # Arguments
    * * point - Point to convert
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn map_coords_to_pixel_current_view(&self, point : &Vector2f) -> Vector2i {
        let view = unsafe { ffi::sfRenderTexture_getView(self.render_texture) };
        unsafe {
            ffi::sfRenderTexture_mapCoordsToPixel(self.render_texture, *point, view)
        }
    }

    /**
    * Draw a drawable object to the render-target
    *
    * # Arguments
    * * object - Object to draw
    */
    pub fn draw<T : Drawable>(&mut self, t : &T) -> () {
        t.draw_in_render_texture(self);
    }

    /**
    * Draw a drawable object to the render-target
    *
    * # Arguments
    * * object - Object to draw
    * * renderStates - The RenderStates to associate to the object
    */
    pub fn draw_with_renderstates<T : Drawable>(&mut self, t : &T, render_states : &mut RenderStates) -> () {
        t.draw_in_render_texture_rs(self, render_states);
    }

    /// Draw Text
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_text(&self, text : &Text) -> () {
        unsafe {
            ffi::sfRenderTexture_drawText(self.render_texture, text.unwrap(), ptr::null())
        }
    }

    /// Draw Text
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_shape(&self, shape : &Shape) -> () {
        unsafe {
            ffi::sfRenderTexture_drawShape(self.render_texture, shape.unwrap(), ptr::null())
        }
    }

    /// Draw Sprite
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_sprite(&self, sprite : &Sprite) -> () {
        unsafe {
            ffi::sfRenderTexture_drawSprite(self.render_texture, sprite.unwrap(), ptr::null())
        }
    }

    /// Draw CircleShape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_circle_shape(&self, circle_shape : &CircleShape) -> () {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.render_texture, circle_shape.unwrap(), ptr::null())
        }
    }

    /// Draw RectangleShape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_rectangle_shape(&self, rectangle_shape : &RectangleShape) -> () {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.render_texture, rectangle_shape.unwrap(), ptr::null())
        }
    }

    /// Draw ConvexShape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_convex_shape(&self, convex_shape : &ConvexShape) -> () {
        unsafe {
            ffi::sfRenderTexture_drawConvexShape(self.render_texture, convex_shape.unwrap(), ptr::null())
        }
    }

    /// Draw VertexArray
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_vertex_array(&self, vertex_array : &VertexArray) -> () {
        unsafe {
            ffi::sfRenderTexture_drawVertexArray(self.render_texture, vertex_array.unwrap(), ptr::null())
        }
    }
    
    /// Draw Text
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_text_rs(&self, text : &Text, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawText(self.render_texture, text.unwrap(), rs.unwrap())
        }
    }

    /// Draw Shape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_shape_rs(&self, shape : &Shape, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawShape(self.render_texture, shape.unwrap(), rs.unwrap())
        }
    }

    /// Draw Sprite
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_sprite_rs(&self, sprite : &Sprite, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawSprite(self.render_texture, sprite.unwrap(), rs.unwrap())
        }
    }

    /// Draw CircleShape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_circle_shape_rs(&self, circle_shape : &CircleShape, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawCircleShape(self.render_texture, circle_shape.unwrap(), rs.unwrap())
        }
    }

    /// Draw RectangleShape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_rectangle_shape_rs(&self, rectangle_shape : &RectangleShape, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawRectangleShape(self.render_texture, rectangle_shape.unwrap(), rs.unwrap())
        }
    }

    /// Draw ConvexShape
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_convex_shape_rs(&self, convex_shape : &ConvexShape, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawConvexShape(self.render_texture, convex_shape.unwrap(), rs.unwrap())
        }
    }

    /// Draw VertexArray
    #[fixed_stack_segment] #[inline(never)]
    pub fn draw_vertex_array_rs(&self, vertex_array : &VertexArray, rs : &mut RenderStates) -> () {
        unsafe {
            ffi::sfRenderTexture_drawVertexArray(self.render_texture, vertex_array.unwrap(), rs.unwrap())
        }
    }

    /**
    * Save the current OpenGL render states and matrices
    *
    * This function can be used when you mix SFML drawing
    * and direct OpenGL rendering. Combined with popGLStates,
    * it ensures that: 
    * SFML's internal states are not messed up by your OpenGL code
    * and that your OpenGL states are not modified by a call to a SFML function
    *
    * Note that this function is quite expensive: it saves all the
    * possible OpenGL states and matrices, even the ones you
    * don't care about. Therefore it should be used wisely.
    * It is provided for convenience, but the best results will
    * be achieved if you handle OpenGL states yourself (because
    * you know which states have really changed, and need to be
    * saved and restored). Take a look at the resetGLStates
    * function if you do so.
    *
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn push_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_pushGLStates(self.render_texture)
        }
    }
    
    /**
    * Restore the previously saved OpenGL render states and matrices
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn pop_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_popGLStates(self.render_texture)
        }
    }

    /**
    * Reset the internal OpenGL states so that the target is ready for drawing
    *
    * This function can be used when you mix SFML drawing
    * and direct OpenGL rendering, if you choose not to use
    * pushGLStates/popGLStates. It makes sure that all OpenGL
    * states needed by SFML are set, so that subsequent sfRenderWindow_draw*()
    * calls will work as expected.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn reset_GL_states(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_resetGLStates(self.render_texture)
        }
    }

    /**
    * Get the target texture of a render texture
    *
    * Return the target texture
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_texture(&self) -> Option<Texture> {
        let tex = unsafe { ffi::sfRenderTexture_getTexture(self.render_texture) };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Wrappable::wrap(tex))
        }
    }
    
    /**
    * Enable or disable the smooth filter on a render texture
    *
    * # Arguments
    * * smooth - true to enable smoothing, false to disable it
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_smooth(&mut self, smooth : bool) -> () {
        unsafe {
            match smooth {
                true        => ffi::sfRenderTexture_setSmooth(self.render_texture, 1),
                false       => ffi::sfRenderTexture_setSmooth(self.render_texture, 0)
            }
        }
    }
    
    /**
    * Tell whether the smooth filter is enabled or not for a render texture
    *
    * Return true if smoothing is enabled, false if it is disabled
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_smooth(&self) -> bool {
        match unsafe { ffi::sfRenderTexture_isSmooth(self.render_texture) } {
            0 => false,
            1  => true,
            _       => unreachable!()
        }
    }    
}

impl Drop for RenderTexture {
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfRenderTexture_destroy(self.render_texture)
        }
    }
}
