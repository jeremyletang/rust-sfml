/*!
* Specialized shape representing a circle.
*
* 
*
*/

use core::libc::{c_float, c_uint};
use graphics::color;
use graphics::rect::{IntRect, FloatRect};
use graphics::texture;
use graphics::drawable;
use graphics::render_window::RenderWindow;
use system::vector2;
use graphics::transform::Transform;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_void, c_float, c_uint};
    use graphics::texture;
    use rsfml::sfTypes::{sfBool};
    use graphics::rect;
    use system::vector2;
    use graphics::color;
    use graphics::transform::Transform;

    pub struct sfCircleShape {
        This : *c_void
    }
    
    pub extern "C" {
        fn sfCircleShape_create() -> *sfCircleShape;
        fn sfCircleShape_copy(shape : *sfCircleShape) -> *sfCircleShape;
        fn sfCircleShape_destroy(shape : *sfCircleShape) -> ();
        fn sfCircleShape_setPosition(shape : *sfCircleShape, position : vector2::Vector2f) -> ();
        fn sfCircleShape_setRotation(shape : *sfCircleShape, angle : c_float) -> ();
        fn sfCircleShape_setScale(shape : *sfCircleShape, scale : vector2::Vector2f) -> ();
        fn sfCircleShape_setOrigin(shape : *sfCircleShape, origin : vector2::Vector2f) -> ();
        fn sfCircleShape_getPosition(shape : *sfCircleShape) -> vector2::Vector2f;
        fn sfCircleShape_getRotation(shape : *sfCircleShape) -> c_float;
        fn sfCircleShape_getScale(shape : *sfCircleShape) -> vector2::Vector2f;
        fn sfCircleShape_getOrigin(shape : *sfCircleShape) -> vector2::Vector2f;
        fn sfCircleShape_move(shape : *sfCircleShape, offset : vector2::Vector2f) -> ();
        fn sfCircleShape_rotate(shape : *sfCircleShape, angle : c_float) -> ();
        fn sfCircleShape_scale(shape : *sfCircleShape, factors : vector2::Vector2f) -> ();
        fn sfCircleShape_getTransform(shape : *sfCircleShape) -> Transform;
        fn sfCircleShape_getInverseTransform(shape : *sfCircleShape) -> Transform;
        fn sfCircleShape_setTexture(shape : *sfCircleShape, texture : *texture::csfml::sfTexture, resetRect : sfBool) -> ();
        fn sfCircleShape_setTextureRect(shape : *sfCircleShape, rect : rect::IntRect) -> ();
        fn sfCircleShape_setFillColor(shape : *sfCircleShape, color : color::Color) -> ();
        fn sfCircleShape_setOutlineColor(shape : *sfCircleShape, color : color::Color) -> ();
        fn sfCircleShape_setOutlineThickness(shape : *sfCircleShape, thickness : c_float) -> ();
        fn sfCircleShape_getTexture(shape : *sfCircleShape) -> *texture::csfml::sfTexture;
        fn sfCircleShape_getTextureRect(shape : *sfCircleShape) -> rect::IntRect;
        fn sfCircleShape_getFillColor(shape : *sfCircleShape) -> color::Color;
        fn sfCircleShape_getOutlineColor(shape : *sfCircleShape) -> color::Color;
        fn sfCircleShape_getOutlineThickness(shape : *sfCircleShape) -> c_float;
        fn sfCircleShape_getPointCount(shape : *sfCircleShape) -> c_uint;
        fn sfCircleShape_getPoint(shape : *sfCircleShape, index : c_uint) -> ();
        fn sfCircleShape_setRadius(shape : *sfCircleShape, radius : c_float) -> ();
        fn sfCircleShape_getRadius(shape : *sfCircleShape) -> c_float;
        fn sfCircleShape_setPointCount(shape : *sfCircleShape, count : c_uint) -> ();
        fn sfCircleShape_getLocalBounds(shape : *sfCircleShape) -> rect::FloatRect;
        fn sfCircleShape_getGlobalBounds(shape : *sfCircleShape) -> rect::FloatRect;
    }
}

#[doc(hidden)]
pub struct CircleShape {
    priv circleShape : *csfml::sfCircleShape
}

impl CircleShape {
    pub fn new() -> CircleShape {
        CircleShape { circleShape : unsafe {csfml::sfCircleShape_create()} }
    }

    pub fn new_copy(shape : &CircleShape) -> CircleShape {
        CircleShape { circleShape : unsafe {csfml::sfCircleShape_copy(shape.unwrap())} }
    }

    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfCircleShape_setRotation(self.circleShape, angle as c_float)
        }
    }

    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfCircleShape_getRotation(self.circleShape) as float
        }
    } 

    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfCircleShape_rotate(self.circleShape, angle as c_float)
        }
    }

    pub fn set_texture(&self, texture : &texture::Texture, resetRect : bool) -> () {
        match resetRect {
            true        => unsafe {csfml::sfCircleShape_setTexture(self.circleShape, texture.unwrap(), 1)},
            false       => unsafe {csfml::sfCircleShape_setTexture(self.circleShape, texture.unwrap(), 0)},
        }
    }
    
    pub fn set_texture_rect(&self, rect : &IntRect) -> () {
        unsafe {
            csfml::sfCircleShape_setTextureRect(self.circleShape, *rect)
        }
    }
    
    pub fn set_fill_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfCircleShape_setFillColor(self.circleShape, *color)
        }
    }

    pub fn set_outline_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfCircleShape_setOutlineColor(self.circleShape, *color)
        }
    }

    pub fn set_outline_thickness(&self, thickness : float) -> () {
        unsafe {
            csfml::sfCircleShape_setOutlineThickness(self.circleShape, thickness as c_float)
        }
    }

    pub fn get_texture(&self) -> texture::Texture {
        unsafe {
            texture::Texture::wrap(csfml::sfCircleShape_getTexture(self.circleShape))
        }
    }

    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            csfml::sfCircleShape_getTextureRect(self.circleShape)
        }
    }
    
    pub fn get_fill_color(&self) -> color::Color {
        unsafe {
            csfml::sfCircleShape_getFillColor(self.circleShape)
        }
    }

    pub fn get_outline_color(&self) -> color::Color {
        unsafe {
            csfml::sfCircleShape_getOutlineColor(self.circleShape)
        }
    }

    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            csfml::sfCircleShape_getOutlineThickness(self.circleShape) as float
        }
    }

    pub fn get_point_count(&self) -> uint {
        unsafe {
            csfml::sfCircleShape_getPointCount(self.circleShape) as uint
        }
    }

    pub fn get_point(&self, index : uint) -> () {
        unsafe {
            csfml::sfCircleShape_getPoint(self.circleShape, index as c_uint)
        }
    }
    
    pub fn set_radius(&self, radius : float) -> () {
        unsafe {
            csfml::sfCircleShape_setRadius(self.circleShape, radius as c_float)
        }
    }

    pub fn get_radius(&self) -> float {
        unsafe {
            csfml::sfCircleShape_getRadius(self.circleShape) as float
        }
    }
    
    pub fn set_point_count(&self, count : uint) -> () {
        unsafe {
            csfml::sfCircleShape_setPointCount(self.circleShape, count as c_uint)
        }
    }

    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe {csfml::sfCircleShape_getPosition(self.circleShape)}
    }

    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe {csfml::sfCircleShape_getScale(self.circleShape)}
    }

    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe {csfml::sfCircleShape_getOrigin(self.circleShape)}
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfCircleShape_move(self.circleShape, *offset)
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfCircleShape_scale(self.circleShape, *factors)
        }
    }

    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfCircleShape_setPosition(self.circleShape, *position)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfCircleShape_setScale(self.circleShape, *scale)
        }
    }

    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfCircleShape_setOrigin(self.circleShape, *origin)
        }
    }

    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfCircleShape_getLocalBounds(self.circleShape)
        }
    }
    
    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfCircleShape_getGlobalBounds(self.circleShape)
        }
    }

    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfCircleShape_getTransform(self.circleShape)
        }
    }

    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            csfml::sfCircleShape_getInverseTransform(self.circleShape)
        }
    }

    #[doc(hidden)]
    pub fn wrap(circleShape : *csfml::sfCircleShape) -> CircleShape {
        CircleShape { circleShape : circleShape}
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfCircleShape {
        self.circleShape
    }
}

impl drawable::Drawable for CircleShape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_circle_shape(self)
    }
}

impl Drop for CircleShape {
    /**
    * Destroy an existing CircleShape
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfCircleShape_destroy(self.circleShape)
        }
    }
}