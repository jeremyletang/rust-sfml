use crate::{
    graphics::{
        CircleShape, Color, ConvexShape, CustomShape, Drawable, IntRect, PrimitiveType,
        RectangleShape, RenderStates, Sprite, Text, Vertex, VertexBuffer, View,
    },
    system::{Vector2f, Vector2i, Vector2u},
};

/// Trait for all render targets (window, texture, ...)
///
/// `RenderTarget` defines the common behavior of all the 2D render targets usable in
/// the graphics module.
///
/// It makes it possible to draw 2D entities like sprites, shapes, text
/// without using any OpenGL command directly.
///
/// A `RenderTarget` is also able to use views ([`View`]), which are a kind of 2D cameras.
/// With views you can globally scroll, rotate or zoom everything that is drawn,
/// without having to transform every single entity.
/// See the documentation of [`View`] for more details and sample pieces of code about this type.
///
/// On top of that, render targets are still able to render direct OpenGL stuff.
/// It is even possible to mix together OpenGL calls and regular SFML drawing commands.
/// When doing so, make sure that OpenGL states are not messed up by calling the
/// [`push_gl_states`]/[`pop_gl_states`] functions.
///
/// [`push_gl_states`]: RenderTarget::push_gl_states
/// [`pop_gl_states`]: RenderTarget::pop_gl_states
pub trait RenderTarget {
    /// clear the screen
    fn clear(&mut self, color: Color);

    /// return the current view
    fn view(&self) -> &View;

    /// get the default view for the render target
    fn default_view(&self) -> &View;

    /// set a new view to the target
    fn set_view(&mut self, view: &View);

    /// get the viewport of the render target
    fn viewport(&self, view: &View) -> IntRect;

    /// Convert a point from window coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-window. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-window may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version uses a custom view for calculations, see the
    /// [`map_pixel_to_coords_current_view`](RenderTarget::map_pixel_to_coords_current_view)
    /// function if you want to use the current view of the
    /// render-window.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    /// * view - The view to use for converting the point
    ///
    /// Return the converted point, in "world" units
    ///
    fn map_pixel_to_coords(&self, point: Vector2i, view: &View) -> Vector2f;

    /// Convert a point from window coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-window. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-window may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version uses the current view for calculations, see the
    /// [`map_pixel_to_coords`](RenderTarget::map_pixel_to_coords) function if
    /// you want to use a custom view.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    ///
    /// Return the converted point, in "world" units
    fn map_pixel_to_coords_current_view(&self, point: Vector2i) -> Vector2f;

    /// Convert a point from world coordinates to window coordinates
    ///
    /// This function finds the pixel of the render-window that matches
    /// the given 2D point. In other words, it goes through the same process
    /// as the graphics card, to compute the final position of a rendered point.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (150, 75) in your 2D world may map to the pixel
    /// (10, 50) of your render-window -- if the view is translated by (140, 25).
    ///
    /// This version uses a custom view for calculations, see
    /// [`map_coords_to_pixel_current_view`](RenderTarget::map_coords_to_pixel_current_view)
    /// if you want to use the current view of the render-window.
    ///
    /// # Arguments
    /// * point - Point to convert
    /// * view - The view to use for converting the point
    ///
    /// Return the converted point, in "world" units
    fn map_coords_to_pixel(&self, point: Vector2f, view: &View) -> Vector2i;

    /// Convert a point from window coordinates to world coordinates
    ///
    /// This function finds the 2D position that matches the
    /// given pixel of the render-window. In other words, it does
    /// the inverse of what the graphics card does, to find the
    /// initial position of a rendered pixel.
    ///
    /// Initially, both coordinate systems (world units and target pixels)
    /// match perfectly. But if you define a custom view or resize your
    /// render window, this assertion is not true anymore, ie. a point
    /// located at (10, 50) in your render-window may map to the point
    /// (150, 75) in your 2D world -- if the view is translated by (140, 25).
    ///
    /// This function is typically used to find which point (or object) is
    /// located below the mouse cursor.
    ///
    /// This version uses the current view for calculations, see the
    /// [`map_pixel_to_coords`](RenderTarget::map_pixel_to_coords) function if
    /// you want to use a custom view.
    ///
    /// # Arguments
    /// * point - Pixel to convert
    ///
    /// Return the converted point, in "world" units
    fn map_coords_to_pixel_current_view(&self, point: Vector2f) -> Vector2i;

    /// Draw a drawable object to the render target
    ///
    /// # Arguments
    /// * object - Object to draw
    fn draw(&mut self, object: &dyn Drawable);

    /// Draw a drawable object to the render-target with a [`RenderStates`]
    ///
    /// # Arguments
    /// * object - Object to draw
    /// * renderStates - The renderStates to associate to the object
    fn draw_with_renderstates(&mut self, object: &dyn Drawable, render_states: &RenderStates);

    /// Get the size of the rendering region of a window
    ///
    /// The size doesn't include the titlebar and borders of the window.
    ///
    /// Return the size in pixels
    fn size(&self) -> Vector2u;

    /// Save the current OpenGL render states and matrices
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering. Combined with `pop_gl_states`,
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
    /// saved and restored). Take a look at the [`reset_gl_states`]
    /// function if you do so.
    ///
    /// [`reset_gl_states`]: RenderTarget::reset_gl_states
    fn push_gl_states(&mut self);

    /// Restore the previously saved OpenGL render states and matrices
    fn pop_gl_states(&mut self);

    /// Reset the internal OpenGL states so that the target is ready for drawing
    ///
    /// This function can be used when you mix SFML drawing
    /// and direct OpenGL rendering, if you choose not to use
    /// [`push_gl_states`]/[`pop_gl_states`]. It makes sure that all OpenGL
    /// states needed by SFML are set, so that subsequent draw()
    /// calls will work as expected.
    ///
    /// [`push_gl_states`]: RenderTarget::push_gl_states
    /// [`pop_gl_states`]: RenderTarget::pop_gl_states
    fn reset_gl_states(&mut self);

    /// Draw Text
    fn draw_text(&self, text: &Text, rs: &RenderStates);

    /// Draw Shape
    fn draw_shape(&self, shape: &CustomShape, rs: &RenderStates);

    /// Draw Sprite
    fn draw_sprite(&self, sprite: &Sprite, rs: &RenderStates);

    /// Draw `CircleShape`
    fn draw_circle_shape(&self, circle_shape: &CircleShape, rs: &RenderStates);

    /// Draw `RectangleShape`
    fn draw_rectangle_shape(&self, rectangle_shape: &RectangleShape, rs: &RenderStates);

    /// Draw `ConvexShape`
    fn draw_convex_shape(&self, convex_shape: &ConvexShape, rs: &RenderStates);

    /// Draw `VertexBuffer`
    fn draw_vertex_buffer(&self, vertex_buffer: &VertexBuffer, rs: &RenderStates);

    /// draw primitives
    fn draw_primitives(&self, vertices: &[Vertex], ty: PrimitiveType, rs: &RenderStates);
}
