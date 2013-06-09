/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
*
*
*
*
*
*/

use std::libc::{c_uint};
use std::ptr;

use system::vector2;
use graphics::view;
use graphics::sprite::Sprite;
use graphics::color;
use graphics::rect::IntRect;
use graphics::texture;
use graphics::text::Text;
use graphics::circle_shape::CircleShape;
use graphics::rectangle_shape::RectangleShape;
use graphics::vertex_array::VertexArray;
use graphics::convex_shape::ConvexShape;

use graphics::drawable;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_void, c_uint};

    use rsfml::sfTypes::sfBool;
    use graphics::view;
    use system::vector2;
    use graphics::render_states;
    use graphics::sprite;
    use graphics::color;
    use graphics::rect::IntRect;
    use graphics::texture;
    use graphics::text;
    use graphics::circle_shape;
    use graphics::rectangle_shape;
    use graphics::vertex_array;
    use graphics::convex_shape;

    pub struct sfRenderTexture {
        This : *c_void,
        Target : *texture::csfml::sfTexture,
        DefaultView : view::csfml::sfView,
        CurrentView : view::csfml::sfView
    }
    
    pub extern "C" {
        fn sfRenderTexture_create(width : c_uint, height : c_uint, depthBuffer : sfBool) -> *sfRenderTexture;
        fn sfRenderTexture_destroy(renderTexture : *sfRenderTexture) -> ();
        fn sfRenderTexture_getSize(renderTexture : *sfRenderTexture) -> vector2::Vector2u;
        fn sfRenderTexture_setActive(renderTexture : *sfRenderTexture, active : sfBool) -> sfBool;
        fn sfRenderTexture_display(renderTexture : *sfRenderTexture) -> ();
        fn sfRenderTexture_clear(renderTexture : *sfRenderTexture, color : color::Color) -> ();
        fn sfRenderTexture_setView(renderTexture : *sfRenderTexture, view : *view::csfml::sfView) -> ();
        fn sfRenderTexture_getView(renderTexture : *sfRenderTexture) -> *view::csfml::sfView;
        fn sfRenderTexture_getDefaultView(renderTexture : *sfRenderTexture) -> *view::csfml::sfView;
        fn sfRenderTexture_getViewport(renderTexture : *sfRenderTexture, view : *view::csfml::sfView) -> IntRect;
        fn sfRenderTexture_mapPixelToCoords(renderTexture : *sfRenderTexture, point : vector2::Vector2i, view : *view::csfml::sfView) -> vector2::Vector2f;
        fn sfRenderTexture_mapCoordsToPixel(renderTexture : *sfRenderTexture, point : vector2::Vector2f, view : *view::csfml::sfView) -> vector2::Vector2i;
        fn sfRenderTexture_drawSprite(renderTexture : *sfRenderTexture, object : *sprite::csfml::sfSprite, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderTexture_drawText(renderTexture : *sfRenderTexture, object : *text::csfml::sfText, states : *render_states::csfml::sfRenderStates) -> ();
        //fn sfRenderTexture_drawShape(renderTexture : *sfRenderTexture, object : *sfShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderTexture_drawCircleShape(renderTexture : *sfRenderTexture, object : *circle_shape::csfml::sfCircleShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderTexture_drawConvexShape(renderTexture : *sfRenderTexture, object : *convex_shape::csfml::sfConvexShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderTexture_drawRectangleShape(renderTexture : *sfRenderTexture, object : *rectangle_shape::csfml::sfRectangleShape, states : *render_states::csfml::sfRenderStates) -> ();
        fn sfRenderTexture_drawVertexArray(renderTexture : *sfRenderTexture, object : *vertex_array::csfml::sfVertexArray, states : *render_states::csfml::sfRenderStates) -> ();
        //fn sfRenderTexture_drawPrimitives(renderTexture : *sfRenderTexture) -> (); // a modifier
        fn sfRenderTexture_pushGLStates(renderTexture : *sfRenderTexture) -> ();
        fn sfRenderTexture_popGLStates(renderTexture : *sfRenderTexture) -> ();
        fn sfRenderTexture_resetGLStates(renderTexture : *sfRenderTexture) -> ();
        fn sfRenderTexture_getTexture(renderTexture : *sfRenderTexture) -> *texture::csfml::sfTexture;
        fn sfRenderTexture_setSmooth(renderTexture : *sfRenderTexture, smooth : sfBool) -> ();
        fn sfRenderTexture_isSmooth(renderTexture : *sfRenderTexture) -> sfBool;
    }
}

#[doc(hidden)]
pub struct RenderTexture {
    renderTexture : *csfml::sfRenderTexture
}

impl RenderTexture {
    pub fn new(width : uint, height : uint, depthBuffer : bool) -> RenderTexture {
        match depthBuffer {
            false       =>   RenderTexture { renderTexture : unsafe {csfml::sfRenderTexture_create(width as c_uint, height as c_uint, 0) }},
            true        =>   RenderTexture { renderTexture : unsafe {csfml::sfRenderTexture_create(width as c_uint, height as c_uint, 1) }}
        }
    }
    
    pub fn get_size(&self) -> vector2::Vector2u {
        unsafe {
            csfml::sfRenderTexture_getSize(self.renderTexture)
        }
    }

    pub fn set_active(&self, active : bool) -> bool {
        match match active {
            false       => unsafe {csfml::sfRenderTexture_setActive(self.renderTexture, 0)},
            true        => unsafe {csfml::sfRenderTexture_setActive(self.renderTexture, 1)}
        } {
            0   => false,
            _   => true
            
        }
    }

    pub fn display(&self) -> () {
        unsafe {
            csfml::sfRenderTexture_display(self.renderTexture)
        }
    }

    pub fn clear(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfRenderTexture_clear(self.renderTexture, *color)
        }
    }

    pub fn set_view(&self, view : &view::View) -> () {
        unsafe {
            csfml::sfRenderTexture_setView(self.renderTexture, view.unwrap())
        }
    }

    pub fn get_view(&self) -> view::View {
        unsafe {
            view::View::wrap(csfml::sfRenderTexture_getView(self.renderTexture))
        }
    }

    pub fn get_default_view(&self) -> view::View {
        unsafe {
            view::View::wrap(csfml::sfRenderTexture_getDefaultView(self.renderTexture))
        }
    }

    pub fn get_viewport(&self, view : &view::View) -> IntRect {
        unsafe {
            csfml::sfRenderTexture_getViewport(self.renderTexture, view.unwrap())
        }
    }
    
    pub fn map_pixel_to_coords(&self, point : &vector2::Vector2i, view : &view::View) -> vector2::Vector2f {
        unsafe {
            csfml::sfRenderTexture_mapPixelToCoords(self.renderTexture, *point, view.unwrap())
        }
    }

    pub fn map_coords_to_pixel(&self, point : &vector2::Vector2f, view : &view::View) -> vector2::Vector2i {
        unsafe {
            csfml::sfRenderTexture_mapCoordsToPixel(self.renderTexture, *point, view.unwrap())
        }
    }

    /**
    * Drawing functions
    */
    pub fn draw<T : drawable::Drawable>(&self, t : &T) -> () {
        t.draw_in_render_texture(self);
    }

    /// Draw Text
    pub fn draw_text(&self, text : &Text) -> () {
        unsafe {
            csfml::sfRenderTexture_drawText(self.renderTexture, text.unwrap(), ptr::null())
        }
    }

    pub fn draw_sprite(&self, sprite : &Sprite) -> () {
        unsafe {
            csfml::sfRenderTexture_drawSprite(self.renderTexture, sprite.unwrap(), ptr::null())
        }
    }

    pub fn draw_circle_shape(&self, circleShape : &CircleShape) -> () {
        unsafe {
            csfml::sfRenderTexture_drawCircleShape(self.renderTexture, circleShape.unwrap(), ptr::null())
        }
    }

    pub fn draw_rectangle_shape(&self, rectangleShape : &RectangleShape) -> () {
        unsafe {
            csfml::sfRenderTexture_drawRectangleShape(self.renderTexture, rectangleShape.unwrap(), ptr::null())
        }
    }

    pub fn draw_convex_shape(&self, convexShape : &ConvexShape) -> () {
        unsafe {
            csfml::sfRenderTexture_drawConvexShape(self.renderTexture, convexShape.unwrap(), ptr::null())
        }
    }

    pub fn draw_vertex_array(&self, vertexArray : &VertexArray) -> () {
        unsafe {
            csfml::sfRenderTexture_drawVertexArray(self.renderTexture, vertexArray.unwrap(), ptr::null())
        }
    }

    pub fn push_GL_states(&self) -> () {
        unsafe {
            csfml::sfRenderTexture_pushGLStates(self.renderTexture)
        }
    }
    
    pub fn pop_GL_states(&self) -> () {
        unsafe {
            csfml::sfRenderTexture_popGLStates(self.renderTexture)
        }
    }

    pub fn reset_GL_states(&self) -> () {
        unsafe {
            csfml::sfRenderTexture_resetGLStates(self.renderTexture)
        }
    }

    pub fn get_texture(&self) -> texture::Texture {
        unsafe {
            texture::Texture::wrap(csfml::sfRenderTexture_getTexture(self.renderTexture))
        }
    }
    
    pub fn set_smooth(&self, smooth : bool) -> () {
        match smooth {
            true        => unsafe {csfml::sfRenderTexture_setSmooth(self.renderTexture, 1)},
            false       => unsafe {csfml::sfRenderTexture_setSmooth(self.renderTexture, 0)}
        }
    }
    
    pub fn is_smooth(&self) -> bool {
        match unsafe {csfml::sfRenderTexture_isSmooth(self.renderTexture)} {
            0 => false,
            _ => true
        }
    }    
}

impl Drop for RenderTexture {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfRenderTexture_destroy(self.renderTexture)
        }
    }
}