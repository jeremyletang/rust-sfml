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
* Shader class (vertex and fragment)
*
* Shaders are programs written using a specific language, executed directly by the graphics card and allowing to apply real-time operations to the rendered entities.
*
*/

use std::str;
use std::ptr;

//use graphics::transform::Transform;
use graphics::texture::Texture;
use system::vector2::Vector2f;
use system::vector3::Vector3f;
use graphics::color::Color;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void, c_float, c_char};

    use rsfml::sfTypes::{sfBool};
    use graphics::transform;
    use graphics::texture;
    use system::vector2::Vector2f;
    use system::vector3::Vector3f;
    use graphics::color::Color;

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
        fn sfShader_setVector2Parameter(shader : *sfShader, name : *c_char, vector : Vector2f) -> ();
        fn sfShader_setVector3Parameter(shader : *sfShader, name : *c_char, vector : Vector3f) -> ();
        fn sfShader_setColorParameter(shader : *sfShader, name : *c_char, color : Color) -> (); 
        fn sfShader_setTransformParameter(shader : *sfShader, name : *c_char, transform : transform::Transform) -> ();
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
    *
    * This function can load both the vertex and the fragment
    * shaders, or only one of them: pass NULL if you don't want to load
    * either the vertex shader or the fragment shader.
    * The sources must be text files containing valid shaders
    * in GLSL language. GLSL is a C-like language dedicated to
    * OpenGL shaders; you'll probably need to read a good documentation
    * for it before writing your own shaders.
    *
    * # Arguments
    * * vertexShaderFilename - Path of the vertex shader file to load, or NULL to skip this shader
    * * fragmentShaderFilename - Path of the fragment shader file to load, or NULL to skip this shader
    *
    * Return a new Shader object
    */
    pub fn new_from_file(vertexShaderFilename : ~str, fragmentShaderFilename : ~str) -> Option<Shader> {
        do str::as_c_str(vertexShaderFilename) |vertex| {
            do str::as_c_str(fragmentShaderFilename) |fragment| {
                let shader = unsafe { csfml::sfShader_createFromFile(vertex, fragment)};
                if shader == ptr::null() {
                    None
                }
                else {
                    Some(Shader { shader : shader})
                }
            }
        }
    }
    
    /**
    * Load both the vertex and fragment shaders from source codes in memory
    *
    * This function can load both the vertex and the fragment
    * shaders, or only one of them: pass NULL if you don't want to load
    * either the vertex shader or the fragment shader.
    * The sources must be valid shaders in GLSL language. GLSL is
    * a C-like language dedicated to OpenGL shaders; you'll
    * probably need to read a good documentation for it before
    * writing your own shaders.
    *
    * # Arguments
    * * vertexShader - String containing the source code of the vertex shader, or NULL to skip this shader
    * * fragmentShader - String containing the source code of the fragment shader, or NULL to skip this shader
    *
    * Return a new Shader object
    */
    pub fn new_from_memory(vertexShader : ~str, fragmentShader : ~str) -> Option<Shader> {
        do str::as_c_str(vertexShader) |vertex| {
            do str::as_c_str(fragmentShader) |fragment| {
                let shader = unsafe { csfml::sfShader_createFromFile(vertex, fragment)};
                if shader == ptr::null() {
                    None
                }
                else {
                    Some(Shader { shader :shader})
                }
            }
        }
    }
    
    /**
    * Change a float parameter of a shader
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * x - Value to assign
    */
    pub fn set_float_parameter(&self, name : ~str, x : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloatParameter(self.shader, shader, x)}
        }
    }

    /**
    * Change a 2-components vector parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 2x1 vector
    * (vec2 GLSL type).
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * x - First component of the value to assign
    * * y - Second component of the value to assign
    */
    pub fn set_float_2_parameter(&self, name : ~str, x : f32, y : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloat2Parameter(self.shader, shader, x, y)}
        }
    }

    /**
    * Change a 3-components vector parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 3x1 vector
    * (vec3 GLSL type).
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * x - First component of the value to assign
    * * y - Second component of the value to assign
    * * z - Third component of the value to assign
    */
    pub fn set_float_3_parameter(&self, name : ~str, x : f32, y : f32, z : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloat3Parameter(self.shader, shader, x, y, z)}
        }
    }
    
    /**
    * Change a 4-components vector parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 4x1 vector
    * (vec4 GLSL type).
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * x - First component of the value to assign
    * * y - Second component of the value to assign
    * * z - Third component of the value to assign
    * * w - Fourth component of the value to assign
    */
    pub fn set_float_4_parameter(&self, name : ~str, x : f32, y : f32, z : f32, w : f32) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setFloat4Parameter(self.shader, shader, x, y, z, w)}
        }
    }
    
    /**
    * Change a texture parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 2D texture
    * (sampler2D GLSL type).
    *
    * # Arguments
    * * name - Name of the texture in the shader
    * * texture - Texture to assign
    */
    pub fn set_texture_parameter(&self, name : ~str, texture : &Texture) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setTextureParameter(self.shader, shader, texture.unwrap())}
        }
    }
    
    /**
    * Change a texture parameter of a shader
    *
    * This function maps a shader texture variable to the
    * texture of the object being drawn, which cannot be
    * known in advance.
    * The corresponding parameter in the shader must be a 2D texture
    * (sampler2D GLSL type).
    *
    * # Arguments
    * * name - Name of the texture in the shader
    */
    pub fn set_current_texture_parameter(&self, name : ~str) -> () {
        do str::as_c_str(name) |shader| {
            unsafe { csfml::sfShader_setCurrentTextureParameter(self.shader, shader)}
        }   
    }

    /**
    * Bind a shader for rendering (activate it)
    *
    * This function is not part of the graphics API, it mustn't be
    * used when drawing SFML entities. It must be used only if you
    * mix sfShader with OpenGL code.
    */
    pub fn bind(&self) -> () {
        unsafe {
            csfml::sfShader_bind(self.shader)
        }
    }

    /**
    * Tell whether or not the system supports shaders
    *
    * This function should always be called before using
    * the shader features. If it returns false, then
    * any attempt to use sfShader will fail.
    * 
    * Return true if the system can use shaders, false otherwise
    */
    pub fn is_available() -> bool {
        match unsafe {csfml::sfShader_isAvailable()} {
            0   => false,
            _   => true
        }
    }

    /**
    * Change a 2-components vector parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 2x1 vector
    * (vec2 GLSL type).
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * vector - Vector to assign
    */
    fn set_vector2_parameter(&self, name : ~str, vector : &Vector2f) -> () {
        unsafe {
            do str::as_c_str(name) |namebuf| {
                csfml::sfShader_setVector2Parameter(self.shader, namebuf, *vector)
            }
        }
    }

    /**
    * Change a 3-components vector parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 2x1 vector
    * (vec2 GLSL type).
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * vector - Vector to assign
    */
    fn set_vector3_parameter(&self, name : ~str, vector : &Vector3f) -> () {
        unsafe {
            do str::as_c_str(name) |namebuf| {
                csfml::sfShader_setVector3Parameter(self.shader, namebuf, *vector)
            }
        }
    }

    /**
    * Change a color parameter of a shader
    *
    * name is the name of the variable to change in the shader.
    * The corresponding parameter in the shader must be a 4x1 vector
    * (vec4 GLSL type).
    * 
    * It is important to note that the components of the color are
    * normalized before being passed to the shader. Therefore,
    * they are converted from range [0 .. 255] to range [0 .. 1].
    * For example, a sf::Color(255, 125, 0, 255) will be transformed
    * to a vec4(1.0, 0.5, 0.0, 1.0) in the shader.
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * color - Color to assign
    */
    fn set_color_parameter(&self, name : ~str, color : &Color) -> () {
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
    fn drop(&self) -> () {
        unsafe {
            csfml::sfShader_destroy(self.shader)
        }
    }
}