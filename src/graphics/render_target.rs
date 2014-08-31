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

//Authored on 2014-08-30 by Brandon Sanderson

use graphics::{Color, Text, Shape, Sprite, VertexArray, View, RenderStates,
               ConvexShape, RectangleShape, CircleShape, IntRect, rc};
use traits::{Drawable, Wrappable};
use system::vector2::{Vector2f, Vector2i, Vector2u};

/// Trait which is the equivalent of the sf::RenderTarget class in SFML.
/// This is implemented by RenderTarget and RenderWindow structs to provide
/// a unified interface for rendering.
pub trait RenderTarget{
    fn clear(&mut self, color: &Color);
    
    fn get_view(&self) -> View;
    fn get_default_view(&self) -> View;

    fn set_view(&mut self, view:&View);
    
    fn get_viewport(&self, view: &View) -> IntRect;
  
    fn map_pixel_to_coords(&self, point: &Vector2i, view: &View) -> Vector2f;
    fn map_pixel_to_coords_current_view(&self, point: &Vector2i) -> Vector2f;
  
    fn map_coords_to_pixel(&self, point: &Vector2f, view: &View) -> Vector2i;
    fn map_coords_to_pixel_current_view(&self, point: &Vector2f) -> Vector2i;
  
    fn draw<T: Drawable>(&mut self, object: &T);
  
    fn draw_with_renderstates<T: Drawable>(&mut self,
                                           object: &T,
                                           render_states: &mut RenderStates);
  
    fn draw_with_renderstates_rc<T: Drawable>(&mut self,
                                              object: &T,
                                              render_states: &mut rc::RenderStates);
  
    fn get_size(&self) -> Vector2u;
  
    fn push_GL_states(&mut self);
    fn pop_GL_states(&mut self);
    fn reset_GL_states(&mut self);


    /// Draw Text
    fn draw_text(&self, text: &Text);

    /// Draw Text
    fn draw_text_rc(&self, text: &rc::Text);

    /// Draw Shape
    fn draw_shape(&self, shape: &Shape);

    /// Draw Shape
    fn draw_shape_rc(&self, shape: &rc::Shape);
    
    /// Draw Sprite
    fn draw_sprite(&self, sprite: &Sprite);

    /// Draw Sprite
    fn draw_sprite_rc(&self, sprite: &rc::Sprite);

    /// Draw CircleShape
    fn draw_circle_shape(&self, circle_shape: &CircleShape);

    /// Draw CircleShape
    fn draw_circle_shape_rc(&self, circle_shape: &rc::CircleShape);

    /// Draw RectangleShape
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape);

    /// Draw RectangleShape
    fn draw_rectangle_shape_rc(&self, rectangle_shape: &rc::RectangleShape);

    /// Draw ConvexShape
    fn draw_convex_shape(&self, convex_shape: &ConvexShape);

    /// Draw ConvexShape
    fn draw_convex_shape_rc(&self, convex_shape: &rc::ConvexShape);

    /// Draw VertexArray
    fn draw_vertex_array(&self, vertex_array: &VertexArray);

    /// Draw Text
    fn draw_text_rs(&self,
                    text: &Text,
                    rs: &mut RenderStates);

    /// Draw Text
    fn draw_text_rs_rc(&self,
                       text: &rc::Text,
                       rs: &mut rc::RenderStates);

    /// Draw Shape
    fn draw_shape_rs(&self,
                     shape: &Shape,
                     rs: &mut RenderStates);

    /// Draw Shape
    fn draw_shape_rs_rc(&self,
                        shape: &rc::Shape,
                        rs: &mut rc::RenderStates);

    /// Draw Sprite
    fn draw_sprite_rs(&self,
                      sprite: &Sprite,
                      rs: &mut RenderStates);

    /// Draw Sprite
    fn draw_sprite_rs_rc(&self,
                         sprite: &rc::Sprite,
                         rs: &mut rc::RenderStates);

    /// Draw CircleShape
    fn draw_circle_shape_rs(&self,
                            circle_shape: &CircleShape,
                            rs: &mut RenderStates);

    /// Draw CircleShape
    fn draw_circle_shape_rs_rc(&self,
                               circle_shape: &rc::CircleShape,
                               rs: &mut rc::RenderStates);

    /// Draw RectangleShape
    fn draw_rectangle_shape_rs(&self,
                               rectangle_shape: &RectangleShape,
                               rs: &mut RenderStates);

    /// Draw RectangleShape
    fn draw_rectangle_shape_rs_rc(&self,
                                  rectangle_shape: &rc::RectangleShape,
                                  rs: &mut rc::RenderStates);

    /// Draw ConvexShape
    fn draw_convex_shape_rs(&self,
                            convex_shape: &ConvexShape,
                            rs: &mut RenderStates);

    /// Draw ConvexShape
    fn draw_convex_shape_rs_rc(&self,
                               convex_shape: &rc::ConvexShape,
                               rs: &mut rc::RenderStates);

    /// Draw VertexArray
    fn draw_vertex_array_rs(&self,
                            vertex_array: &VertexArray,
                            rs: &mut RenderStates);

    /// Draw VertexArray
    fn draw_vertex_array_rs_rc(&self,
                               vertex_array: &VertexArray,
                               rs: &mut rc::RenderStates);


}
