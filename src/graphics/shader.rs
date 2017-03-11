// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

use std::ptr;
use std::ffi::CString;

use system::raw_conv::Raw;
use graphics::{Texture, Color};
use system::Vector2f;
use system::Vector3f;

use csfml_graphics_sys as ffi;

use std::io::{Read, Seek};
use inputstream::InputStream;
use ext::sf_bool_ext::SfBoolExt;

/// Shader type (vertex and fragment)
///
/// Shaders are programs written using a specific language,
/// executed directly by the graphics card and allowing to apply
/// real-time operations to the rendered entities.
pub struct Shader<'s> {
    shader: *mut ffi::sfShader,
    texture: Option<&'s Texture>,
}

macro_rules! cstring_then_ptr {
    ($cstring:ident, $shader:expr) => {
        match $shader {
            Some(s) => {
                $cstring = CString::new(s.as_bytes()).unwrap();
                $cstring.as_ptr()
            }
            None => ptr::null(),
        }
    }
}

impl<'s> Shader<'s> {
    /// Load both the vertex and fragment shaders from files
    ///
    /// This function can load both the vertex and the fragment
    /// shaders, or only one of them: pass None if you don't want to load
    /// either the vertex shader or the fragment shader.
    /// The sources must be text files containing valid shaders
    /// in GLSL language. GLSL is a C-like language dedicated to
    /// OpenGL shaders; you'll probably need to read a good documentation
    /// for it before writing your own shaders.
    ///
    /// # Arguments
    /// * vertexShaderFilename - Some(Path) of the vertex shader file to load,
    ///                          or None to skip this shader
    /// * fragmentShaderFilename - Some(Path) of the fragment shader file to load,
    ///                            or None to skip this shader
    ///
    /// Return Some(Shader) or None
    pub fn from_file(vertex: Option<&str>,
                     geometry: Option<&str>,
                     fragment: Option<&str>)
                     -> Option<Shader<'s>> {
        let cstring;
        let vert = cstring_then_ptr!(cstring, vertex);
        let cstring;
        let geom = cstring_then_ptr!(cstring, geometry);
        let cstring;
        let frag = cstring_then_ptr!(cstring, fragment);
        let shader = unsafe { ffi::sfShader_createFromFile(vert, geom, frag) };
        if shader.is_null() {
            None
        } else {
            Some(Shader {
                     shader: shader,
                     texture: None,
                 })
        }
    }

    /// Load both the vertex and fragment shaders from streams
    ///
    /// This function can load both the vertex and the fragment
    /// shaders, or only one of them: pass None if you don't want to load
    /// either the vertex shader or the fragment shader.
    /// The sources must be text files containing valid shaders
    /// in GLSL language. GLSL is a C-like language dedicated to
    /// OpenGL shaders; you'll probably need to read a good documentation
    /// for it before writing your own shaders.
    ///
    /// # Arguments
    /// * vertexShaderStream - Some(T: Read + Seek) of the vertex shader stream to load,
    ///                        or None to skip this shader
    /// * fragmentShaderStream - Some(T: Read + Seek) of the fragment shader stream to load,
    ///                          or None to skip this shader
    ///
    /// Return Some(Shader) or None
    pub fn from_stream<T: Read + Seek>(vertex_shader_stream: Option<&mut T>,
                                       geometry_shader_stream: Option<&mut T>,
                                       fragment_shader_stream: Option<&mut T>)
                                       -> Option<Shader<'s>> {
        let mut vertex_stream = vertex_shader_stream.map(InputStream::new);
        let mut geometry_stream = geometry_shader_stream.map(InputStream::new);
        let mut fragment_stream = fragment_shader_stream.map(InputStream::new);
        let vertex_ptr = vertex_stream.as_mut().map_or(ptr::null_mut(), |s| &mut s.0);
        let geometry_ptr = geometry_stream.as_mut().map_or(ptr::null_mut(), |s| &mut s.0);
        let fragment_ptr = fragment_stream.as_mut().map_or(ptr::null_mut(), |s| &mut s.0);
        let shader =
            unsafe { ffi::sfShader_createFromStream(vertex_ptr, geometry_ptr, fragment_ptr) };
        if shader.is_null() {
            None
        } else {
            Some(Shader {
                     shader: shader,
                     texture: None,
                 })
        }
    }

    /// Load both the vertex and fragment shaders from source codes in memory
    ///
    /// This function can load both the vertex and the fragment
    /// shaders, or only one of them: pass None if you don't want to load
    /// either the vertex shader or the fragment shader.
    /// The sources must be valid shaders in GLSL language. GLSL is
    /// a C-like language dedicated to OpenGL shaders; you'll
    /// probably need to read a good documentation for it before
    /// writing your own shaders.
    ///
    /// # Arguments
    /// * vertexShader - Some(String) containing the source code of the vertex shader,
    ///                  or None to skip this shader
    /// * fragmentShader - Some(String) containing the source code of the fragment shader,
    ///                    or None to skip this shader
    ///
    /// Return Some(Shader) or None
    pub fn from_memory(vertex: Option<&str>,
                       geometry: Option<&str>,
                       fragment: Option<&str>)
                       -> Option<Shader<'s>> {
        let cstring;
        let vert = cstring_then_ptr!(cstring, vertex);
        let cstring;
        let geom = cstring_then_ptr!(cstring, geometry);
        let cstring;
        let frag = cstring_then_ptr!(cstring, fragment);
        let shader = unsafe { ffi::sfShader_createFromMemory(vert, geom, frag) };
        if shader.is_null() {
            None
        } else {
            Some(Shader {
                     shader: shader,
                     texture: None,
                 })
        }
    }

    /// Change a f32 parameter of a shader
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * x - Value to assign
    pub fn set_float_parameter(&mut self, name: &str, x: f32) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setFloatParameter(self.shader, c_str.as_ptr(), x) }
    }

    /// Change a 2-components vector parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 2x1 vector
    /// (vec2 GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * x - First component of the value to assign
    /// * y - Second component of the value to assign
    pub fn set_float_2_parameter(&mut self, name: &str, x: f32, y: f32) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setFloat2Parameter(self.shader, c_str.as_ptr(), x, y) }
    }

    /// Change a 3-components vector parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 3x1 vector
    /// (vec3 GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * x - First component of the value to assign
    /// * y - Second component of the value to assign
    /// * z - Third component of the value to assign
    pub fn set_float_3_parameter(&mut self, name: &str, x: f32, y: f32, z: f32) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setFloat3Parameter(self.shader, c_str.as_ptr(), x, y, z) }
    }

    /// Change a 4-components vector parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 4x1 vector
    /// (vec4 GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * x - First component of the value to assign
    /// * y - Second component of the value to assign
    /// * z - Third component of the value to assign
    /// * w - Fourth component of the value to assign
    pub fn set_float_4_parameter(&mut self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setFloat4Parameter(self.shader, c_str.as_ptr(), x, y, z, w) }
    }

    /// Change a texture parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 2D texture
    /// (sampler2D GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the texture in the shader
    /// * texture - Texture to assign
    pub fn set_texture_parameter(&mut self, name: &str, texture: &'s Texture) {
        self.texture = Some(texture);
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setTextureParameter(self.shader, c_str.as_ptr(), texture.raw()) }
    }

    /// Change a texture parameter of a shader
    ///
    /// This function maps a shader texture variable to the
    /// texture of the object being drawn, which cannot be
    /// known in advance.
    /// The corresponding parameter in the shader must be a 2D texture
    /// (sampler2D GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the texture in the shader
    pub fn set_current_texture_parameter(&self, name: &str) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setCurrentTextureParameter(self.shader, c_str.as_ptr()) }
    }

    /// Bind a shader for rendering (activate it)
    ///
    /// This function is not part of the graphics API, it mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix sfShader with OpenGL code.
    pub fn bind(&mut self) {
        unsafe { ffi::sfShader_bind(self.shader) }
    }

    /// Tell whether or not the system supports shaders
    ///
    /// This function should always be called before using
    /// the shader features. If it returns false, then
    /// any attempt to use sfShader will fail.
    ///
    /// Return true if the system can use shaders, false otherwise
    pub fn is_available() -> bool {
        unsafe { ffi::sfShader_isAvailable() }.to_bool()
    }

    /// Change a 2-components vector parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 2x1 vector
    /// (vec2 GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * vector - Vector to assign
    pub fn set_vector2_parameter(&mut self, name: &str, vector: &Vector2f) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setVector2Parameter(self.shader, c_str.as_ptr(), vector.raw()) }
    }

    /// Change a 3-components vector parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 2x1 vector
    /// (vec2 GLSL type).
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * vector - Vector to assign
    pub fn set_vector3_parameter(&mut self, name: &str, vector: &Vector3f) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setVector3Parameter(self.shader, c_str.as_ptr(), vector.raw()) }
    }

    /// Change a color parameter of a shader
    ///
    /// name is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 4x1 vector
    /// (vec4 GLSL type).
    ///
    /// It is important to note that the components of the color are
    /// normalized before being passed to the shader. Therefore,
    /// they are converted from range [0 .. 255] to range [0 .. 1].
    /// For example, a sf::Color(255, 125, 0, 255) will be transformed
    /// to a vec4(1.0, 0.5, 0.0, 1.0) in the shader.
    ///
    /// # Arguments
    /// * name - Name of the parameter in the shader
    /// * color - Color to assign
    pub fn set_color_parameter(&mut self, name: &str, color: &Color) {
        let c_str = CString::new(name.as_bytes()).unwrap();
        unsafe { ffi::sfShader_setColorParameter(self.shader, c_str.as_ptr(), color.raw()) }
    }
}

impl<'s> Raw for Shader<'s> {
    type Raw = *const ffi::sfShader;
    fn raw(&self) -> Self::Raw {
        self.shader
    }
}

impl<'s> Drop for Shader<'s> {
    fn drop(&mut self) {
        unsafe { ffi::sfShader_destroy(self.shader) }
    }
}
