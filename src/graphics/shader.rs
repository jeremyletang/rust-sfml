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

use std::ptr;

//use graphics::transform::Transform;
use traits::wrappable::Wrappable;
use graphics::texture::Texture;
use system::vector2::Vector2f;
use system::vector3::Vector3f;
use graphics::color::Color;

#[doc(hidden)]
pub mod ffi {

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

    extern "C" {
        pub fn sfShader_createFromFile(vertexShaderFilename : *c_char, fragmentShaderFilename : *c_char) -> *sfShader;
        pub fn sfShader_createFromMemory(vertexShader : *c_char, fragmentShader : *c_char) -> *sfShader;
        //fn sfShader_createFromStream(vertexShaderStream : *sfInputStream, fragmentShaderStream : *sfInputStream) -> *sfShader;
        pub fn sfShader_destroy(shader : *sfShader)-> ();
        pub fn sfShader_setFloatParameter(shader : *sfShader, name : *c_char, x : c_float) -> ();
        pub fn sfShader_setFloat2Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float) -> ();
        pub fn sfShader_setFloat3Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float, z : c_float) -> ();
        pub fn sfShader_setFloat4Parameter(shader : *sfShader, name : *c_char, x : c_float, y : c_float, z : c_float, w : c_float) -> ();
        pub fn sfShader_setVector2Parameter(shader : *sfShader, name : *c_char, vector : Vector2f) -> ();
        pub fn sfShader_setVector3Parameter(shader : *sfShader, name : *c_char, vector : Vector3f) -> ();
        pub fn sfShader_setColorParameter(shader : *sfShader, name : *c_char, color : Color) -> (); 
        pub fn sfShader_setTransformParameter(shader : *sfShader, name : *c_char, transform : transform::Transform) -> ();
        pub fn sfShader_setTextureParameter(shader : *sfShader, name : *c_char, texture : *texture::ffi::sfTexture) -> ();
        pub fn sfShader_setCurrentTextureParameter(shader : *sfShader, name : *c_char) -> ();
        pub fn sfShader_bind(shader : *sfShader) -> ();
        pub fn sfShader_isAvailable() -> sfBool;
   }
}

#[doc(hidden)]
pub struct Shader {
    priv shader : *ffi::sfShader,
    priv texture : Option<@mut Texture>
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_from_file(vertex_shader_filename : ~str, fragment_shader_filename : ~str) -> Option<Shader> {
        let shader = unsafe { 
            let c_vertex_shader_filename = vertex_shader_filename.to_c_str().unwrap();
            let c_fragment_shader_filename = fragment_shader_filename.to_c_str().unwrap();
            ffi::sfShader_createFromFile(c_vertex_shader_filename, c_fragment_shader_filename) 
        };
        if ptr::is_null(shader) {
            None
            }
        else {
            Some(Shader { 
                shader : shader,
                texture : None
            })
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn new_from_memory(vertex_shader : ~str, fragment_shader : ~str) -> Option<Shader> {
        let shader = unsafe { 
            let c_vertex_shader = vertex_shader.to_c_str().unwrap();
            let c_fragment_shader = fragment_shader.to_c_str().unwrap();
            ffi::sfShader_createFromFile(c_vertex_shader, c_fragment_shader) 
        };
        if ptr::is_null(shader) {
            None
        }
        else {
            Some(Shader {
                shader : shader,
                texture : None
            })
        }
    }
    
    /**
    * Change a float parameter of a shader
    *
    * # Arguments
    * * name - Name of the parameter in the shader
    * * x - Value to assign
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_float_parameter(&mut self, name : ~str, x : f32) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setFloatParameter(self.shader, c_name, x)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_float_2_parameter(&mut self, name : ~str, x : f32, y : f32) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setFloat2Parameter(self.shader, c_name, x, y)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_float_3_parameter(&mut self, name : ~str, x : f32, y : f32, z : f32) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setFloat3Parameter(self.shader, c_name, x, y, z)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_float_4_parameter(&mut self, name : ~str, x : f32, y : f32, z : f32, w : f32) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setFloat4Parameter(self.shader, c_name, x, y, z, w)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_texture_parameter(&mut self, name : ~str, texture : @mut Texture) -> () {
        self.texture = Some(texture);
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setTextureParameter(self.shader, c_name, texture.unwrap())
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn set_current_texture_parameter(&self, name : ~str) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setCurrentTextureParameter(self.shader, c_name)
        }   
    }

    /**
    * Bind a shader for rendering (activate it)
    *
    * This function is not part of the graphics API, it mustn't be
    * used when drawing SFML entities. It must be used only if you
    * mix sfShader with OpenGL code.
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn bind(&mut self) -> () {
        unsafe {
            ffi::sfShader_bind(self.shader)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn is_available() -> bool {
        match unsafe { ffi::sfShader_isAvailable() } {
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
    #[fixed_stack_segment] #[inline(never)]
    fn set_vector2_parameter(&mut self, name : ~str, vector : &Vector2f) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setVector2Parameter(self.shader, c_name, *vector)
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
    #[fixed_stack_segment] #[inline(never)]
    fn set_vector3_parameter(&mut self, name : ~str, vector : &Vector3f) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setVector3Parameter(self.shader, c_name, *vector)
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
    #[fixed_stack_segment] #[inline(never)]
    fn set_color_parameter(&mut self, name : ~str, color : &Color) -> () {
        unsafe { 
            let c_name = name.to_c_str().unwrap();
            ffi::sfShader_setColorParameter(self.shader, c_name, *color)
        }
    }

}

#[doc(hidden)]
impl Wrappable<*ffi::sfShader> for Shader {
    fn wrap(shader : *ffi::sfShader) -> Shader {
        Shader {
            shader : shader,
            texture : None
        }
    }

    fn unwrap(&self) -> *ffi::sfShader {
        self.shader
    }
}

#[unsafe_destructor]
impl Drop for Shader {
    /**
    * Destroy an existing shader
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&self) -> () {
        unsafe {
            ffi::sfShader_destroy(self.shader)
        }
    }
}
