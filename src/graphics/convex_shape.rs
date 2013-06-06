
use core::libc::{c_float, c_uint};
use graphics::color;
use graphics::texture;
use system::vector2;
use graphics::drawable;
use graphics::render_window::RenderWindow;
use graphics::rect::{FloatRect, IntRect};
use graphics::transform::Transform;

#[doc(hidden)]
pub mod csfml {

    use core::libc::{c_uint, c_void, c_float};
    use system::vector2;
    use graphics::color;
    use graphics::texture;
    use rsfml::sfTypes::sfBool;
    use graphics::rect::{FloatRect, IntRect};
    use graphics::transform::Transform;
    
    pub struct sfConvexShape {
        This : *c_void
    }

    pub extern "C" {
        fn sfConvexShape_create() -> *sfConvexShape;
        fn sfConvexShape_copy(shape : *sfConvexShape) -> *sfConvexShape;
        fn sfConvexShape_destroy(shape : *sfConvexShape) -> ();
        fn sfConvexShape_setPosition(shape : *sfConvexShape, position : vector2::Vector2f) -> ();
        fn sfConvexShape_setRotation(shape : *sfConvexShape, angle : c_float) -> ();
        fn sfConvexShape_setScale(shape : *sfConvexShape, scale : vector2::Vector2f) -> ();
        fn sfConvexShape_setOrigin(shape : *sfConvexShape, origin : vector2::Vector2f) -> ();
        fn sfConvexShape_getPosition(shape : *sfConvexShape) -> vector2::Vector2f;
        fn sfConvexShape_getRotation(shape : *sfConvexShape) -> c_float;
        fn sfConvexShape_getScale(shape : *sfConvexShape) -> vector2::Vector2f;
        fn sfConvexShape_getOrigin(shape : *sfConvexShape) -> vector2::Vector2f;
        fn sfConvexShape_move(shape : *sfConvexShape, offset : vector2::Vector2f) -> ();
        fn sfConvexShape_rotate(shape : *sfConvexShape, angle : c_float) -> ();
        fn sfConvexShape_scale(shape : *sfConvexShape, factors : vector2::Vector2f) -> ();
        fn sfConvexShape_getTransform(shape : *sfConvexShape) -> Transform;
        fn sfConvexShape_getInverseTransform(shape : *sfConvexShape) -> Transform;
        fn sfConvexShape_setTexture(shape : *sfConvexShape, texture : *texture::csfml::sfTexture, resetRect : sfBool) -> ();
        fn sfConvexShape_setTextureRect(shape : *sfConvexShape, rect : IntRect) -> ();
        fn sfConvexShape_setFillColor(shape : *sfConvexShape, color : color::Color) -> ();
        fn sfConvexShape_setOutlineColor(shape : *sfConvexShape, color : color::Color) -> ();
        fn sfConvexShape_setOutlineThickness(shape : *sfConvexShape, thickness : c_float) -> ();
        fn sfConvexShape_getTexture(shape : *sfConvexShape) -> *texture::csfml::sfTexture;
        fn sfConvexShape_getTextureRect(shape : *sfConvexShape) -> IntRect;
        fn sfConvexShape_getFillColor(shape : *sfConvexShape) -> color::Color;
        fn sfConvexShape_getOutlineColor(shape : *sfConvexShape) -> color::Color;
        fn sfConvexShape_getOutlineThickness(shape : *sfConvexShape) -> c_float;
        fn sfConvexShape_getPointCount(shape : *sfConvexShape) -> c_uint;
        fn sfConvexShape_getPoint(shape : *sfConvexShape, index : c_uint) -> vector2::Vector2f;
        fn sfConvexShape_setPointCount(shape : *sfConvexShape, count : c_uint) -> ();
        fn sfConvexShape_setPoint(shape : *sfConvexShape, index : c_uint, point : vector2::Vector2f) -> ();
        fn sfConvexShape_getLocalBounds(shape : *sfConvexShape) -> FloatRect;
        fn sfConvexShape_getGlobalBounds(shape : *sfConvexShape) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct ConvexShape {
    priv convexShape : *csfml::sfConvexShape
}

impl ConvexShape {
    
    pub fn new() -> ConvexShape {
        ConvexShape { convexShape : unsafe {csfml::sfConvexShape_create()} }
    }

    pub fn new_copy(shape : &ConvexShape) -> ConvexShape {
        ConvexShape { convexShape : unsafe {csfml::sfConvexShape_copy(shape.unwrap())} }
    }

    pub fn set_position(&self, position : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setPosition(self.convexShape, *position)
        }
    }

    pub fn set_scale(&self, scale : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setScale(self.convexShape, *scale)
        }
    }

    pub fn set_origin(&self, origin : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setOrigin(self.convexShape, *origin)
        }
    }

    pub fn move(&self, offset : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_move(self.convexShape, *offset)
        }
    }

    pub fn scale(&self, factors : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_scale(self.convexShape, *factors)
        }
    }

    pub fn set_point(&self, index : uint, point : &vector2::Vector2f) -> () {
        unsafe {
            csfml::sfConvexShape_setPoint(self.convexShape, index as c_uint, *point)
        }
    }

    pub fn get_position(&self) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getPosition(self.convexShape)}
    }
    
    pub fn get_scale(&self) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getScale(self.convexShape)}
    }
    
    pub fn get_origin(&self) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getOrigin(self.convexShape)}
    }

    pub fn get_point(&self, index : uint) -> vector2::Vector2f {
        unsafe {csfml::sfConvexShape_getPoint(self.convexShape, index as c_uint)}
    }
    
    pub fn set_rotation(&self, angle : float) -> () {
        unsafe {
            csfml::sfConvexShape_setRotation(self.convexShape, angle as c_float)
        }
    }

    pub fn get_rotation(&self) -> float {
        unsafe {
            csfml::sfConvexShape_getRotation(self.convexShape) as float
        }
    }

    pub fn rotate(&self, angle : float) -> () {
        unsafe {
            csfml::sfConvexShape_rotate(self.convexShape, angle as c_float)
        }
    }

    pub fn set_texture(&self, texture : &texture::Texture, resetRect : bool) -> () {
        match resetRect {
            true        => unsafe {csfml::sfConvexShape_setTexture(self.convexShape, texture.unwrap(), 1)},
            false       => unsafe {csfml::sfConvexShape_setTexture(self.convexShape, texture.unwrap(), 0)}
        }
    }

    pub fn set_fill_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfConvexShape_setFillColor(self.convexShape, *color)
        }
    }

    pub fn set_outline_color(&self, color : &color::Color) -> () {
        unsafe {
            csfml::sfConvexShape_setOutlineColor(self.convexShape, *color)
        }
    }

    pub fn set_outline_thickness(&self, thickness : float) -> () {
        unsafe {
            csfml::sfConvexShape_setOutlineThickness(self.convexShape, thickness as c_float)
        }
    }

    pub fn get_texture(&self) -> texture::Texture {
            texture::Texture::wrap(unsafe {csfml::sfConvexShape_getTexture(self.convexShape)})
    }
    
    pub fn get_fill_color(&self) -> color::Color {
        unsafe {csfml::sfConvexShape_getFillColor(self.convexShape)}
    }
    
    pub fn get_outline_color(&self) -> color::Color {
        unsafe {csfml::sfConvexShape_getOutlineColor(self.convexShape)}
    }
    
    pub fn get_outline_thickness(&self) -> float {
        unsafe {
            csfml::sfConvexShape_getOutlineThickness(self.convexShape) as float
        }
    }

    pub fn get_point_count(&self) -> uint {
        unsafe {
            csfml::sfConvexShape_getPointCount(self.convexShape) as uint
        }
    }

    pub fn set_point_count(&self, count : uint) -> () {
        unsafe {
            csfml::sfConvexShape_setPointCount(self.convexShape, count as c_uint)
        }
    }

    pub fn set_texture_rect(&self, rect : &IntRect) -> () {
        unsafe {
            csfml::sfConvexShape_setTextureRect(self.convexShape, *rect)
        }
    }

    pub fn get_local_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfConvexShape_getLocalBounds(self.convexShape)
        }
    }

    pub fn get_global_bounds(&self) -> FloatRect {
        unsafe {
            csfml::sfConvexShape_getGlobalBounds(self.convexShape)
        }
    }

    pub fn get_texture_rect(&self) -> IntRect {
        unsafe {
            csfml::sfConvexShape_getTextureRect(self.convexShape)
        }
    }
    
    pub fn get_transform(&self) -> Transform {
        unsafe {
            csfml::sfConvexShape_getTransform(self.convexShape)
        }
    }

    pub fn get_inverse_transform(&self) -> Transform {
        unsafe {
            csfml::sfConvexShape_getInverseTransform(self.convexShape)
        }
    }

    #[doc(hidden)]
    pub fn wrap(convexShape : *csfml::sfConvexShape) -> ConvexShape {
        ConvexShape { convexShape : convexShape}
    }
    
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfConvexShape {
        self.convexShape
    }
}

impl drawable::Drawable for ConvexShape {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_convex_shape(self)
    }
}

impl Drop for ConvexShape {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfConvexShape_destroy(self.convexShape)
        }
    }
}