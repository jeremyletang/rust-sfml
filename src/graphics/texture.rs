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
* Image used for drawing
*
* Texture stores pixels that can be drawn, with a sprite for example.
*
*/

use std::libc::{c_uint};
use std::vec;
use std::ptr;

use traits::wrappable::Wrappable;
use system::vector2::Vector2u;
use window::window::Window;
use graphics::render_window::RenderWindow;
use graphics::image::Image;
use graphics::rect::IntRect;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_uint, c_void, c_char};

    use rsfml::sfTypes::{sfBool};
    use system::vector2::Vector2u;
    use window::window::ffi::sfWindow;
    use graphics::render_window::ffi::sfRenderWindow;
    use graphics::rect::IntRect;
    use graphics::image;

    pub struct sfTexture {
        this : *c_void
    }

    extern "C" {
        pub fn sfTexture_create(width : c_uint, height : c_uint) -> *sfTexture;
        pub fn sfTexture_createFromFile(filename : *c_char, area : *IntRect) -> *sfTexture;
        //fn sfTexture_createFromMemory(data : *c_void, sizeInBytes : size_t , area : *sfIntRect) -> *sfTexture;
        //fn sfTexture_createFromStream(strea; : *sfInputStream, area : *sfIntRect) -> *sfTexture;
        pub fn sfTexture_createFromImage(image :*image::ffi::sfImage, area : *IntRect) -> *sfTexture;
        pub fn sfTexture_copy(texture : *sfTexture) -> *sfTexture;
        pub fn sfTexture_destroy(texture : *sfTexture) -> ();
        pub fn sfTexture_getSize(texture : *sfTexture) -> Vector2u;
        pub fn sfTexture_copyToImage(texture : *sfTexture) -> *image::ffi::sfImage;
        pub fn sfTexture_updateFromPixels(texture : *sfTexture, pixels : *u8, width : c_uint, height : c_uint, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_updateFromImage(texture : *sfTexture, image : *image::ffi::sfImage, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_updateFromWindow(texture : *sfTexture, window : *sfWindow, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_updateFromRenderWindow(texture : *sfTexture, renderWindow : *sfRenderWindow, x : c_uint, y : c_uint) -> ();
        pub fn sfTexture_setSmooth(texture : *sfTexture, smooth : sfBool) -> ();
        pub fn sfTexture_isSmooth(texture : *sfTexture) -> sfBool;
        pub fn sfTexture_setRepeated(texture : *sfTexture, repeated : sfBool);
        pub fn sfTexture_isRepeated(texture : *sfTexture) -> sfBool;
        pub fn sfTexture_bind(texture : *sfTexture) -> ();
        pub fn sfTexture_getMaximumSize() -> c_uint;
    }
}

#[doc(hidden)]
pub struct Texture {
    priv texture : *ffi::sfTexture,
    priv dropable : bool
}

impl Texture {
    /**
    * Create a new texture
    *
    * # Arguments
    * * width - Texture width
    * * height - Texture height
    *
    * Return a new Option to Texture object or None
    */
    pub fn new(width: uint, height : uint) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_create(width as c_uint, height as c_uint) };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Texture {
                texture : tex,
                dropable : true
            })
        }
    }
    
    /**
    * Create a new texture from a file
    *
    * # Arguments
    * * filename - Path of the image file to load
    *
    * Return a new Option to Texture object or None
    */
    pub fn new_from_file(filename : ~str) -> Option<Texture> {
        let mut tex = ptr::null();
        do filename.as_c_str |filebuf| {
            tex = unsafe { ffi::sfTexture_createFromFile(filebuf, ptr::null()) };
        }
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Texture {
                texture : tex,
                dropable : true
            })
        }
    }

     /**
    * Create a new texture from a file with a given area
    *
    * # Arguments
    * * filename - Path of the image file to load
    * * area - Area of the source image to load
    *
    * Return a new Option to Texture object or None
    */
    pub fn new_from_file_with_rect(filename : ~str, area : &IntRect) -> Option<Texture> {
        let mut tex = ptr::null();
        do filename.as_c_str |filebuf| {
            tex = unsafe { ffi::sfTexture_createFromFile(filebuf, &*area)};
        }
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Texture {
                texture : tex,
                dropable : true
            })
        }
    }
    
    /**
    * Create a new texture by copying a exitant one
    *
    * # Arguments
    * * texture - Texture to copy
    *
    * Return an option to the copied texture or None
    */
    pub fn clone(&self) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_copy(self.texture) };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Texture {
                texture : tex,
                dropable : true
            })
        }
    }

    /**
    * Create a new texture from an image
    *
    * # Arguments
    * * image - Image to upload to the texture
    * * area - Area of the source image to load
    *
    * Return a new Option to Texture object or None
    */
    pub fn new_from_image_with_rect(image : &Image, area : &IntRect) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_createFromImage(image.unwrap(), &*area) };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Texture {
                texture : tex,
                dropable : true
            })
        }
    }

    /**
    * Create a new texture from an image
    *
    * # Arguments
    * * image - Image to upload to the texture
    *
    * Return a new Option to Texture object or None
    */
    pub fn new_from_image(image : &Image) -> Option<Texture> {
        let tex = unsafe { ffi::sfTexture_createFromImage(image.unwrap(), ptr::null()) };
        if ptr::is_null(tex) {
            None
        }
        else {
            Some(Texture {
                texture : tex,
                dropable : true
            })
        }
    }
    
    /**
    * Return the size of the texture
    *
    * Return the Size in pixels
    */
    pub fn get_size(&self) -> Vector2u {
        unsafe {
            ffi::sfTexture_getSize(self.texture)
        }
    }
    
    /**
    * Update a texture from the contents of a window
    *
    * # Arguments
    * * window - Window to copy to the texture
    * * x - X offset in the texture where to copy the source pixels
    * * y - Y offset in the texture where to copy the source pixels
    */
    pub fn update_from_window(&mut self, window : Window, x : uint, y : uint) -> () {
        unsafe {
            ffi::sfTexture_updateFromWindow(self.texture, window.unwrap(), x as c_uint, y as c_uint)
        }
    }

    /**
    * Update a texture from the contents of a render window
    *
    * # Arguments
    * * renderWindow - Render-window to copy to the texture
    * * x - X offset in the texture where to copy the source pixels
    * * y - Y offset in the texture where to copy the source pixels
    */
    pub fn update_from_render_window(&mut self, renderWindow : RenderWindow, x : uint, y : uint) -> () {
        unsafe {
            ffi::sfTexture_updateFromRenderWindow(self.texture, renderWindow.unwrap(), x as c_uint, y as c_uint)
        }
    }

    /**
    * Update a texture from the contents of an image
    *
    * # Arguments
    * * image - Image to copy to the texture
    * * x - X offset in the texture where to copy the source pixels
    * * y - Y offset in the texture where to copy the source pixels
    */
    pub fn update_from_image(&mut self, image : &Image, x : uint, y : uint) -> () {
        unsafe {
            ffi::sfTexture_updateFromImage(self.texture, image.unwrap(), x as c_uint, y as c_uint)
        }
    }
    
    /**
    * Update a texture from the contents of a Vector of pixels
    *
    * # Arguments
    * * pixels - Pixels to copy to the texture
    * * x - X offset in the texture where to copy the source pixels
    * * y - Y offset in the texture where to copy the source pixels
    */
    pub fn update_from_pixels(&mut self, pixels : ~[u8], width : uint, height : uint, x : uint, y : uint) -> () {
        unsafe {
            ffi::sfTexture_updateFromPixels(self.texture, vec::raw::to_ptr(pixels), width as c_uint, height as c_uint, x as c_uint, y as c_uint)
        }
    }

    /**
    * Enable or disable the smooth filter on a texture
    *
    * # Arguments
    * * smooth - true to enable smoothing, false to disable it
    */
    pub fn set_smooth(&mut self, smooth : bool) -> () {
        unsafe {
            match smooth {
                true        => ffi::sfTexture_setSmooth(self.texture, 1),
                false       => ffi::sfTexture_setSmooth(self.texture, 0)
            }
        }
    }

    /**
    * Tell whether the smooth filter is enabled or not for a texture
    *
    * Return true if smoothing is enabled, false if it is disabled
    */
    pub fn is_smooth(&self) -> bool {
        match unsafe { ffi::sfTexture_isSmooth(self.texture) } {
            0 => false,
            _ => true
        }
    }

    /**
    * Enable or disable repeating for a texture
    *
    * epeating is involved when using texture coordinates
    * outside the texture rectangle [0, 0, width, height].
    * In this case, if repeat mode is enabled, the whole texture
    * will be repeated as many times as needed to reach the
    * coordinate (for example, if the X texture coordinate is
    * 3 * width, the texture will be repeated 3 times).
    * If repeat mode is disabled, the "extra space" will instead
    * be filled with border pixels.
    * Warning: on very old graphics cards, white pixels may appear
    * when the texture is repeated. With such cards, repeat mode
    * can be used reliably only if the texture has power-of-two
    * dimensions (such as 256x128).
    * Repeating is disabled by default.
    * 
    * # Arguments
    * * repeated  - true to repeat the texture, false to disable repeating
    */
    pub fn set_repeated(&mut self, repeated : bool) -> () {
        unsafe {
            match repeated {
                true        => ffi::sfTexture_setRepeated(self.texture, 1),
                false       => ffi::sfTexture_setRepeated(self.texture, 0)
            }
        }
    }
    
    /**
    * Tell whether a texture is repeated or not
    *
    * Return frue if repeat mode is enabled, false if it is disabled
    */
    pub fn is_repeated(&self) -> bool {
        match unsafe {ffi::sfTexture_isRepeated(self.texture)} {
            0   => false,
            _   => true
        }
    }

    /**
    * Bind a texture for rendering
    *
    * This function is not part of the graphics API, it mustn't be
    * used when drawing SFML entities. It must be used only if you
    * mix sfTexture with OpenGL code.
    *
    */
    pub fn bind(&mut self) -> () {
        unsafe {
            ffi::sfTexture_bind(self.texture)
        }
    }
    
    /**
    * Get the maximum texture size allowed
    *
    * Return the maximum size allowed for textures, in pixels
    */
    pub fn get_maximum_size() -> uint {
        unsafe {
            ffi::sfTexture_getMaximumSize() as uint
        }
    }

    /**
    * Copy a texture's pixels to an image
    *
    * Return an image containing the texture's pixels
    */
    pub fn copy_to_image(&self) -> Option<Image> {
        let img = unsafe {ffi::sfTexture_copyToImage(self.texture)};
        if ptr::is_null(img) {
            None
        }
        else {
            Some(Wrappable::wrap(img))
        }
    }
}

#[doc(hidden)]
impl Wrappable<*ffi::sfTexture> for Texture {
    pub fn unwrap(&self) -> *ffi::sfTexture {
        self.texture
    }
    
    pub fn wrap(texture : *ffi::sfTexture) -> Texture {
        Texture { 
            texture : texture,
            dropable : false
        }
    }
}

impl Drop for Texture {
    /**
    * Destroy an existing texture
    */
    fn drop(&self) {
        if self.dropable {
            unsafe {
                ffi::sfTexture_destroy(self.texture)
            }
        }
    }
}
