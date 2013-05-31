/*!
* Drawable trait
*
* Implemented by each drawable object to specifiy them type to RenderWindow.
*
*/

use graphics::render_window;

/**
* The trait drawable is inherited by each object who can be drown by the RenderWindow
*/
pub trait Drawable {
    pub fn draw_in_render_window(&self, &render_window::RenderWindow) -> ();
}