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

use graphics::{Color, View, RenderStates, CircleShape, RectangleShape, Text, Sprite,
               IntRect, Vertex, PrimitiveType, BaseShape, Drawable};
use system::{Vector2f, Vector2i, Vector2u};
use ffi::Ref;

/// Base trait unifying all render targets (window, texture, ...).
///
/// `RenderTarget` defines the common behavior of the 2D render targets in the
/// graphics module. It makes it possible to draw 2D entities like sprites,
/// shapes, and text without using any OpenGL command directly.
///
/// A `RenderTarget` is also able to use `View`s, which are a kind of 2D
/// cameras. With views you can globally scroll, rotate or zoom everything that
/// is drawn, without having to transform every single entity. See the docs of
/// `View` for more details and sample pieces of code about this class.
///
/// On top of that, render targets are still able to render direct OpenGL stuff.
/// It is even possible to mix together OpenGL calls and regular SFML drawing
/// commands. When doing so, make sure that OpenGL states are not messed up by
/// calling the `push_gl_states()`/`pop_gl_states()` functions.
pub trait RenderTarget {
	/// Clear the entire target with a single color.
	///
	/// This function is usually called once every frame, to clear the previous
	/// contents of the target.
    fn clear(&mut self, color: &Color);

	/// Get the view currently in use in the render target.
    fn get_view(&self) -> Ref<View>;

	/// Get the default view of the render target.
	///
	/// The default view has the initial size of the render target, and never
	/// changes after the target has been created.
    fn get_default_view(&self) -> Ref<View>;

	/// Change the current active view.
	///
	/// The view is like a 2D camera: it controls which part of the 2D scene is
	/// visible, and how it is viewed in the render target. The new view will
	/// affect everything that is drawn until another view is set. The render
	/// target keeps its own copy of the view object, so it is not necessary to
	/// keep the original one alive after calling this function. To restore the
	/// original view of the target, you can pass the result of
	/// `get_default_view()` to this function.
    fn set_view(&mut self, view: &View);

	/// Get the viewport of a view, applied to this render target.
	///
	/// The viewport is defined in the view as a ratio. This function applies
	/// this ratio to the current dimensions of the render target to calculate
	/// the pixel rectangle that the viewport actually covers in the target.
	/// 
	/// Returns the viewport rectangle, expressed in pixels.
    fn get_viewport(&self, view: &View) -> IntRect;


    /// Convert a point from window coordinates to world coordinates.
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render target. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render target, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render target may map to the point
    /// (150, 75) in your 2D world if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
	/// See `map_pixel_to_coords_current_view` if you want to use the current
	/// view of the render target.
    ///
    /// Returns the converted point, in "world" units.
    fn map_pixel_to_coords(&self, point: &Vector2i, view: &View) -> Vector2f;

    /// Convert a point from target coordinates to world coordinates, using the
	/// current view.
	///
	/// Equivalent to calling `map_pixel_to_coords(point, get_view())`.
    ///
    /// Returns the converted point, in "world" units.
	#[inline]
    fn map_pixel_to_coords_current_view(&self, point: &Vector2i) -> Vector2f {
		self.map_pixel_to_coords(point, &self.get_view())
	}

    /// Convert a point from world coordinates to target coordinates.
    ///
    /// This function finds the pixel of the render target that matches
    /// the given 2D point. In other words, it goes through the same process
    /// as the graphics card, to compute the final position of a rendered point.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render target, this assertion is not true anymore, ie. a point
    /// located at (150, 75) in your 2D world may map to the pixel
    /// (10, 50) of your render target if the view is translated by (140, 25).
    ///
	/// See `map_coords_to_pixel_current_view` if you want to use the current
	/// view of the render target.
	///
    /// Returns the converted point, in target coordinates (pixels).
    fn map_coords_to_pixel(&self, point: &Vector2f, view: &View) -> Vector2i;

    /// Convert a point from world coordinates to target coordinates, using the
	/// current view.
	///
	/// Equivalent to calling `map_pixel_to_coords(point, get_view())`.
	/// 
    /// Returns the converted point, in target coordinates (pixels).
	#[inline]
    fn map_coords_to_pixel_current_view(&self, point: &Vector2f) -> Vector2i {
		self.map_coords_to_pixel(point, &self.get_view())
	}

	/// Draw a drawable object to the render target.
    fn draw(&mut self, object: &Drawable) where Self: Sized {
		object.draw(self, &RenderStates::default())
	}

	/// Draw a drawable object to the render target with a RenderStates.
    fn draw_rs(&mut self, object: &Drawable, render_states: &RenderStates) where Self: Sized {
		object.draw(self, render_states)
	}

    /// Get the size of the rendering region of the target, in pixels.
    fn get_size(&self) -> Vector2u;


    /// Save the current OpenGL render states and matrices.
    ///
    /// This function can be used when you mix SFML drawing and direct OpenGL
    /// rendering. Combined with `pop_gl_states()`, it ensures that SFML's
	/// internal states are not messed up by your OpenGL code and that your
    /// OpenGL states are not modified by a call to a SFML function.
    ///
    /// Note that this function is quite expensive: it saves all the
    /// possible OpenGL states and matrices, even the ones you
    /// don't care about. Therefore it should be used wisely.
    /// It is provided for convenience, but the best results will
    /// be achieved if you handle OpenGL states yourself (because
    /// you know which states have really changed, and need to be
    /// saved and restored). Take a look at the `reset_gl_states()`
    /// function if you do so.
    fn push_gl_states(&mut self);

    /// Restore the previously saved OpenGL render states and matrices.
    fn pop_gl_states(&mut self);

    /// Reset the internal OpenGL states so that the target is ready for
	/// drawing.
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering, if you choose not to use
    /// `push_gl_states()`/`pop_gl_states()`. It makes sure that all OpenGL
    /// states needed by SFML are set, so that subsequent `draw()`
    /// calls will work as expected.
    fn reset_gl_states(&mut self);


	#[doc(hidden)]
    fn draw_text_rs(&mut self, text: &Text, rs: &RenderStates);

	#[doc(hidden)]
    fn draw_shape_rs(&mut self, shape: &BaseShape, rs: &RenderStates);

	#[doc(hidden)]
    fn draw_sprite_rs(&mut self, sprite: &Sprite, rs: &RenderStates);

	#[doc(hidden)]
    fn draw_circle_shape_rs(&mut self, circle_shape: &CircleShape, rs: &RenderStates);

	#[doc(hidden)]
    fn draw_rectangle_shape_rs(&mut self, rectangle_shape: &RectangleShape, rs: &RenderStates);

	#[doc(hidden)]
    fn draw_primitives_rs(&mut self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates);
}
