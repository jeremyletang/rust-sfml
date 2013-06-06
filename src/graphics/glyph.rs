/*!
* Glyph describes a glyph (a visual character)
*
*
*
*/

use graphics::rect::IntRect;

pub struct Glyph {
    advance : i32,
    bounds : IntRect,
    textureRect : IntRect
}