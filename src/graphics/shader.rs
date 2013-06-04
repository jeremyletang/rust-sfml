/*!
* Shader class (vertex and fragment)
*
* Shaders are programs written using a specific language, executed directly by the graphics card and allowing to apply real-time operations to the rendered entities.
*
*/

//use graphics::transform::Transform;
use graphics::texture::Texture;
use system::vector2;
use system::vector3;
use graphics::color;

#[doc(hidden)]
pub mod csfml {

    use core::libc::{c_void, c_float, c_char};
    use rsfml::sfTypes::{sfBool};
    use graphics::transform;
    use graphics::texture;
    use system::vector2;
    use system::vector3;
    use graphics::color;

    pub struct sfShader {
        This : *c_void
    }

    pub extern "C" {
        fn sfShader_createFromFile(vertexShaderFilename : *c_char, fragmentShaderFilename : *c_char) -> *sfShader;
        fn sfShader_createFromMemory(vertexShader : *c_char, fragmentShader : *c_char) -> *sfShader;
        //fn sfShader_createFromStream(vertexShaderStream : *sfInputStream, fragmentShaderStream : *sfInputStream) -> *sfShader;
        fn sfShader_destroy(shader : *sfShader)-> ();
        fn sfShader_setFloatParameter(shader : *sfShader, name : *c_char, x : c_float) -> ();
        fn sfShader_setFloat2Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float) -> ();
        fn sfShader_setFloat3Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float, z : c_float) -> ();
        fn sfShader_setFloat4Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float, z : c_float, w : c_float) -> ();
        fn sfShader_setVector2Parameter(shader : *sfShader, name : *c_char, vector : vector2::Vector2f) -> ();
        fn sfShader_setVector3Parameter(shader : *sfShader, name : *c_char, vector : vector3::Vector3f) -> ();
        fn sfShader_setColorParameter(shader : *sfShader, name : *c_char, color : color::Color) -> (); 
        fn sfShader_setTransformParameter(shader : *sfShader, name : *c_char, transform : transform::csfml::sfTransform) -> ();
        fn sfShader_setTextureParameter(shader : *sfShader, name : *c_char, texture : *texture::csfml::sfTexture) -> ();
        fn sfShader_setCurrentTextureParameter(shader : *sfShader, name : *c_char) -> ();
        fn sfShader_bind(shader : *sfShader) -> ();
        fn sfShader_isAvailable() -> sfBool;
   }
}

#[doc(hidden)]
pub struct Shader {
    priv shader : *csfml::sfShader
}

impl Shader {
    /**
    *  Load both the vertex and fragment shaders from files
    */
    pub fn new_from_file(vertexShaderFilename : ~str, fragmentShaderFilename : ~str) -> Shader {
        do str::as_c_str(vertexShaderFilename) |vertex| {
            do str::as_c_str(fragmentShaderFilename) |fragment| {
                Shader { shader : unsafe { csfml::sfShader_createFromFile(vertex, fragment)}}
            }
        }
    }
    
    /**
    * Load both the vertex and fragment shaders from source codes in memory
    */
    pub fn new_from_memory(vertexShader : ~str, fragmentShader : ~str) -> Shader {
        do str::as_c_str(vertexShader) |vertex| {
            do str::as_c_str(fragmentShader) |fragment| {
                Shader { shader : unsafe { csfml::sfShader_createFromFile(vertex, fragment)}}
            }
        }
    }
    
    /**
    * Change a float parameter of a shader
    */
    pub fn set_float_parameter(&self, name : ~str, x : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloatParameter(self.shader, shader, x)}
        }
    }

    /**
    * Change a 2-components vector parameter of a shader
    */
    pub fn set_float_2_parameter(&self, name : ~str, x : f32, y : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloat2Parameter(self.shader, shader, x, y)}
        }
    }

    /**
    * Change a 3-components vector parameter of a shader
    */
    pub fn set_float_3_parameter(&self, name : ~str, x : f32, y : f32, z : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloat3Parameter(self.shader, shader, x, y, z)}
        }
    }
    
    /**
    * Change a 4-components vector parameter of a shader
    */
    pub fn set_float_4_parameter(&self, name : ~str, x : f32, y : f32, z : f32, w : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloat4Parameter(self.shader, shader, x, y, z, w)}
        }
    }
    
    /**
    * Change a texture parameter of a shader
    */
    pub fn set_texture_parameter(&self, name : ~str, texture : &Texture) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setTextureParameter(self.shader, shader, texture.unwrap_texture())}
        }
    }
    
    /**
    * Change a texture parameter of a shader
    */
    pub fn set_current_texture_parameter(&self, name : ~str) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setCurrentTextureParameter(self.shader, shader)}
        }   
    }

    /**
    * Bind a shader for rendering (activate it)
    */
    pub fn bind(&self) -> () {
        unsafe {
            csfml::sfShader_bind(self.shader)
        }
    }

    /**
    * Tell whether or not the system supports shaders
    */
    pub fn is_available() -> bool {
        match unsafe {csfml::sfShader_isAvailable()} {
            0   => false,
            _   => true
        }
    }

    fn set_vector2_parameter(&self, name : ~str, vector : &vector2::Vector2f) -> () {
        unsafe {
            do str::as_c_str(name) |namebuf| {
                csfml::sfShader_setVector2Parameter(self.shader, namebuf, *vector)
            }
        }
    }

    fn set_vector3_parameter(&self, name : ~str, vector : &vector3::Vector3f) -> () {
        unsafe {
            do str::as_c_str(name) |namebuf| {
                csfml::sfShader_setVector3Parameter(self.shader, namebuf, *vector)
            }
        }
    }

    fn set_color_parameter(&self, name : ~str, color : &color::Color) -> () {
        unsafe {
            do str::as_c_str(name) |namebuf| {
                csfml::sfShader_setColorParameter(self.shader, namebuf, *color)
            }
        }
    }

}

impl Drop for Shader {
    /**
    * Destroy an existing shader
    */
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfShader_destroy(self.shader)
        }
    }
}