/*!
* Image used for drawing
*
* Texture stores pixels that can be drawn, with a sprite for example.
*
*/

use core::libc::{c_uint};
use system::vector2;
use window::window;
use graphics::render_window;
use graphics::image;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_uint, c_void, c_char};
    use rsfml::sfTypes::{sfBool};
    use system::vector2::csfml::sfVector2u;
    use window::window::csfml::sfWindow;
    use graphics::render_window::csfml::sfRenderWindow;
    use graphics::rect::IntRect;
    use graphics::image;

    pub struct sfTexture {
        this : *c_void
    }

    pub extern "C" {
        fn sfTexture_create(width : c_uint, height : c_uint) -> *sfTexture;
        fn sfTexture_createFromFile(filename : *c_char, area : *IntRect) -> *sfTexture;
        //fn sfTexture_createFromMemory(data : *c_void, sizeInBytes : size_t , area : *sfIntRect) -> *sfTexture;
        //fn sfTexture_createFromStream(strea; : *sfInputStream, area : *sfIntRect) -> *sfTexture;
        //fn sfTexture_createFromImage(image :*sfImage, area : *sfIntRect) -> *sfTexture;
        fn sfTexture_copy(texture : *sfTexture) -> *sfTexture;
        fn sfTexture_destroy(texture : *sfTexture) -> ();
        fn sfTexture_getSize(texture : *sfTexture) -> sfVector2u;
        fn sfTexture_copyToImage(texture : *sfTexture) -> *image::csfml::sfImage;
        //fn sfTexture_updateFromPixels(texture : *sfTexture, pixels : *u8, width : c_uint, height : c_uint, x : c_uint, y : c_uint) -> ();
        //fn sfTexture_updateFromImage(texture : *sfTexture, image : *sfImage, x : c_uint, y : c_uint) -> ();
        fn sfTexture_updateFromWindow(texture : *sfTexture, window : *sfWindow, x : c_uint, y : c_uint) -> ();
        fn sfTexture_updateFromRenderWindow(texture : *sfTexture, renderWindow : *sfRenderWindow, x : c_uint, y : c_uint) -> ();
        fn sfTexture_setSmooth(texture : *sfTexture, smooth : sfBool) -> ();
        fn sfTexture_isSmooth(texture : *sfTexture) -> sfBool;
        fn sfTexture_setRepeated(texture : *sfTexture, repeated : sfBool);
        fn sfTexture_isRepeated(texture : *sfTexture) -> sfBool;
        fn sfTexture_bind(texture : *sfTexture) -> ();
        fn sfTexture_getMaximumSize() -> c_uint;
    }
}

#[doc(hidden)]
pub struct Texture {
    priv texture : *csfml::sfTexture
}

impl Texture {
    /**
    * Create a new texture
    */
    pub fn new(width: uint, height : uint) -> Texture {
        Texture { texture : unsafe {csfml::sfTexture_create(width as c_uint, height as c_uint)}}
    }
    
    /**
    * Create a new texture from a file
    */
    pub fn new_from_file(filename : ~str) -> Texture {
        do str::as_c_str(filename) |filebuf| {
            Texture { texture : unsafe {csfml::sfTexture_createFromFile(filebuf, ptr::null())} }
        }
    }
    
    /**
    * Create a new texture by copying a exitant one
    */
    pub fn new_copy(texture : &Texture) -> Texture {
        Texture { texture : unsafe {csfml::sfTexture_copy(texture.unwrap_texture())}}
    }
    
    /**
    * Return the size of the texture
    */
    pub fn get_size(&self) -> vector2::Vector2u {
        vector2::wrap_vector2u(unsafe {
            csfml::sfTexture_getSize(self.texture)
        })
    }
    
    /**
    * Update a texture from the contents of a window
    */
    pub fn update_from_window(&self, window : window::Window, x : uint, y : uint) -> () {
        unsafe {
            csfml::sfTexture_updateFromWindow(self.texture, window.get_sfWindow(), x as c_uint, y as c_uint)
        }
    }

    /**
    * Update a texture from the contents of a render window
    */
    pub fn update_from_render_window(&self, renderWindow : render_window::RenderWindow, x : uint, y : uint) -> () {
        unsafe {
            csfml::sfTexture_updateFromRenderWindow(self.texture, renderWindow.get_sfRenderWindow(), x as c_uint, y as c_uint)
        }
    }

    /**
    * Enable or disable the smooth filter on a texture
    */
    pub fn set_smooth(&self, smooth : bool) -> () {
        match smooth {
            true        => unsafe {csfml::sfTexture_setSmooth(self.texture, 1)},
            false       => unsafe {csfml::sfTexture_setSmooth(self.texture, 0)}
        }
    }

    /**
    * Tell whether the smooth filter is enabled or not for a texture
    */
    pub fn is_smooth(&self) -> bool {
        match unsafe {csfml::sfTexture_isSmooth(self.texture)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Enable or disable repeating for a texture
    */
    pub fn set_repeated(&self, repeated : bool) -> () {
        match repeated {
            true        => unsafe {csfml::sfTexture_setRepeated(self.texture, 1)},
            false       => unsafe {csfml::sfTexture_setRepeated(self.texture, 0)}
        }
    }
    
    /**
    * Tell whether a texture is repeated or not
    */
    pub fn is_repeated(&self) -> bool {
        match unsafe {csfml::sfTexture_isRepeated(self.texture)} {
            0   => false,
            _   => true
        }
    }

    /**
    * Bind a texture for rendering
    */
    pub fn bind(&self) -> () {
        unsafe {
            csfml::sfTexture_bind(self.texture)
        }
    }
    
    /**
    * Get the maximum texture size allowed
    */
    pub fn get_maximum_size() -> uint {
        unsafe {
            csfml::sfTexture_getMaximumSize() as uint
        }
    }

    /**
    * Copy a texture's pixels to an image
    */
    pub fn copy_to_image(&self) -> image::Image {
        unsafe {
            image::Image::wrap_image(csfml::sfTexture_copyToImage(self.texture))
        }
    }
    
    #[doc(hidden)]
    pub fn unwrap_texture(&self) -> *csfml::sfTexture {
        self.texture
    }
    
    #[doc(hidden)]
    pub fn wrap_texture(texture : *csfml::sfTexture) -> Texture {
        Texture { texture : texture}
    }
}

impl Drop for Texture {
    /**
    * Destroy an existing texture
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfTexture_destroy(self.texture)
        }
    }
}