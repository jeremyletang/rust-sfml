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

use std::ptr;
use std::ffi::CString;
use std::marker::PhantomData;
use std::io::{Read, Seek};

use graphics::{Texture, Color, Transform};
use system::{Vector2f, Vector3f, InputStream};

use ffi::Foreign;
use ffi::graphics as ffi;

/// Container for a vertex and/or fragment shader.
///
/// Shaders are programs written using a specific language which are executed
/// directly by the graphics card and allow applying real-time operations to
/// rendered entities.
///
/// There are two kinds of shaders: vertex shaders, that process vertices of
/// geometry, and fragment (pixel) shaders, that process pixels on the screen.
///
/// A `Shader` can be composed of one or both of a vertex and fragment shader.
///
/// Shaders are written in GLSL, which is a C-like language dedicated to OpenGL
/// shaders. You'll probably need to learn its basics before writing your own
/// shaders for SFML.
///
/// Like any program, a shader has its own variables that you can set from your
/// application. `Shader` handles 5 different types of variables: floats,
/// vectors (2, 3, or 4 components), colors, textures, and transforms. The value
/// of variables can be changed at any time with the `set_X_parameter()` family
/// of functions. The `set_current_texture_parameter()` function maps the given
/// texture variable to the current texture of the object being drawn, which
/// cannot be known in advance.
///
/// To apply a shader to a drawable, you must include it in the `RenderStates`
/// passed to the `draw()` call for that object.
///
/// Shaders can be used on any drawable, but some combinations are not
/// interesting. For example, using a vertex shader on a `Sprite` is limited
/// because there are only 4 vertices, the sprite would have to be subdivided in
/// order to apply wave effects. Another bad example is a fragment shader with
/// `Text`: the texture of the text is not the actual text that you see on
/// screen, it is a big texture containing all the characters of the font in an
/// arbitrary order; thus, texture lookups on pixels other than the current one
/// may not give you the expected result.
///
/// Shaders can also be used to apply global post-effects to the current
/// contents of the target. This can be done in two different ways:
///
/// * Draw everything to a `RenderTexture`, then draw it to the main target
///   using the shader.
/// * Draw everything directly to the main target, then use
///   `Texture::update(Window)` to copy its contents to a texture and draw it to
///   the main target using the shader.
///
/// The first technique is more optimized because it doesn't involve retrieving
/// the target's pixels to system memory, but the second one doesn't impact the
/// rendering process and can be easily inserted anywhere without impacting all
/// the code.
///
/// Like `Texture` that can be used as a raw OpenGL texture, `Shader` can also
/// be used for custom OpenGL geometry by calling `bind()`.
pub struct Shader<'s>(Foreign<ffi::sfShader>, PhantomData<&'s Texture>);

// The PhantomData represents arguments to set_texture_parameter which must
// outlive the Shader.

macro_rules! try_string {
	($val:expr, $fallback:expr) => ({
		match CString::new($val) {
			Ok(str) => str,
			Err(_) => return $fallback
		}
	})
}

impl<'s> Shader<'s> {
	/// Load a vertex and/or fragment shader from files.
    ///
    /// This function can load both the vertex and the fragment
    /// shaders, or only one of them: pass None if you don't want to load
    /// either the vertex shader or the fragment shader.
    /// The sources must be text files containing valid shaders
    /// in GLSL language. GLSL is a C-like language dedicated to
    /// OpenGL shaders; you'll probably need to read a good documentation
    /// for it before writing your own shaders.
    ///
    /// Returns Some(Shader) or None on failure.
    pub fn new_from_file(vertex_shader_filename: Option<&str>,
                         fragment_shader_filename: Option<&str>)
                         -> Option<Shader<'s>> {
		let vertex_fname;
		let vertex_ptr = match vertex_shader_filename {
			None => ptr::null(),
			Some(string) => {
				vertex_fname = try_string!(string, None);
				vertex_fname.as_ptr()
			}
		};

		let fragment_fname;
		let fragment_ptr = match fragment_shader_filename {
			None => ptr::null(),
			Some(string) => {
				fragment_fname = try_string!(string, None);
				fragment_fname.as_ptr()
			}
		};

        unsafe {
            Foreign::new(ffi::sfShader_createFromFile(vertex_ptr, fragment_ptr))
        }.map(|shader| Shader(shader, PhantomData))
    }

    /// Load a vertex and/or fragment shader from source code in memory.
    ///
    /// This function can load both the vertex and the fragment
    /// shaders, or only one of them: pass None if you don't want to load
    /// either the vertex shader or the fragment shader.
    /// The sources must be valid shaders in GLSL language. GLSL is
    /// a C-like language dedicated to OpenGL shaders; you'll
    /// probably need to read a good documentation for it before
    /// writing your own shaders.
    ///
    /// Returns Some(Shader) or None on failure.
    pub fn new_from_memory(vertex_shader: Option<&str>,
                           fragment_shader: Option<&str>)
                           -> Option<Shader<'s>> {
		let vertex_fname;
		let vertex_ptr = match vertex_shader {
			None => ptr::null(),
			Some(string) => {
				vertex_fname = try_string!(string, None);
				vertex_fname.as_ptr()
			}
		};

		let fragment_fname;
		let fragment_ptr = match fragment_shader {
			None => ptr::null(),
			Some(string) => {
				fragment_fname = try_string!(string, None);
				fragment_fname.as_ptr()
			}
		};

        unsafe {
            Foreign::new(ffi::sfShader_createFromMemory(vertex_ptr, fragment_ptr))
        }.map(|shader| Shader(shader, PhantomData))
    }

    /// Load a vertex and/or fragment shader from source code in streams.
    ///
    /// This function can load both the vertex and the fragment
    /// shaders, or only one of them: pass None if you don't want to load
    /// either the vertex shader or the fragment shader.
    /// The sources must be valid shaders in GLSL language. GLSL is
    /// a C-like language dedicated to OpenGL shaders; you'll
    /// probably need to read a good documentation for it before
    /// writing your own shaders.
    ///
    /// Returns Some(Shader) or None on failure.
	pub fn new_from_stream<T: Read + Seek, U: Read + Seek>(vertex_shader_stream: Option<&mut T>, fragment_shader_stream: Option<&mut U>) -> Option<Shader<'s>> {
		let mut v_stream;
		let v_stream_ptr = match vertex_shader_stream {
			None => ptr::null_mut(),
			Some(stream) => {
				v_stream = InputStream::new(stream);
				&mut v_stream as *mut InputStream
			}
		};

		let mut f_stream;
		let f_stream_ptr = match fragment_shader_stream {
			None => ptr::null_mut(),
			Some(stream) => {
				f_stream = InputStream::new(stream);
				&mut f_stream as *mut InputStream
			}
		};

		unsafe {
			Foreign::new(ffi::sfShader_createFromStream(v_stream_ptr, f_stream_ptr))
		}.map(|shader| Shader(shader, PhantomData))
	}

	fn raw(&self) -> &ffi::sfShader { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfShader { self.0.as_mut() }
	#[doc(hidden)]
    pub fn unwrap(&self) -> &ffi::sfShader { self.raw() }

	/// Change a float parameter of the shader.
    pub fn set_float_parameter(&mut self, name: &str, x: f32) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setFloatParameter(self.raw_mut(), c_str.as_ptr(), x)
        }
    }

	/// Change a 2-component vector parameter of the shader.
	///
	/// The corresponding parameter in the shader must be a 2x1 vector
	/// (`vec2` GLSL type).
    pub fn set_float_2_parameter(&mut self, name: &str, x: f32, y: f32) {
        let c_str = try_string!(name, ());
        unsafe {
			ffi::sfShader_setFloat2Parameter(self.raw_mut(), c_str.as_ptr(), x, y)
        }
    }

    /// Change a 3-component vector parameter of the shader.
    ///
    /// The corresponding parameter in the shader must be a 3x1 vector
    /// (`vec3` GLSL type).
    pub fn set_float_3_parameter(&mut self, name: &str, x: f32, y: f32, z: f32) {
		let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setFloat3Parameter(self.raw_mut(), c_str.as_ptr(), x, y, z)
        }
    }

    /// Change a 4-component vector parameter of the shader.
    ///
    /// The corresponding parameter in the shader must be a 4x1 vector
    /// (`vec4` GLSL type).
    pub fn set_float_4_parameter(&mut self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setFloat4Parameter(self.raw_mut(), c_str.as_ptr(), x, y, z, w)
        }
    }

    /// Change a texture parameter of the shader.
    ///
    /// The corresponding parameter in the shader must be a 2D texture
    /// (`sampler2D` GLSL type).
    pub fn set_texture_parameter(&mut self, name: &str, texture: &'s Texture) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setTextureParameter(self.raw_mut(), c_str.as_ptr(), texture.unwrap())
        }
    }

    /// Change a texture paramater of the shader to the current texture.
    ///
    /// This function maps a shader texture variable to the
    /// texture of the object being drawn, which cannot be
    /// known in advance.
    /// The corresponding parameter in the shader must be a 2D texture
    /// (`sampler2D` GLSL type).
    pub fn set_current_texture_parameter(&mut self, name: &str) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setCurrentTextureParameter(self.raw_mut(), c_str.as_ptr())
        }
    }

	/// Change a 2-component vector parameter of the shader.
	///
	/// The corresponding parameter in the shader must be a 2x1 vector
	/// (`vec2` GLSL type).
    pub fn set_vector2_parameter(&mut self, name: &str, vector: Vector2f) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setVector2Parameter(self.raw_mut(), c_str.as_ptr(), vector)
        }
    }

    /// Change a 3-component vector parameter of the shader.
    ///
    /// The corresponding parameter in the shader must be a 3x1 vector
    /// (`vec3` GLSL type).
    pub fn set_vector3_parameter(&mut self, name: &str, vector: Vector3f) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setVector3Parameter(self.raw_mut(), c_str.as_ptr(), vector)
        }
    }

    /// Change a color parameter of a shader.
	///
    /// The corresponding parameter in the shader must be a 4x1 vector
    /// (`vec4` GLSL type).
    ///
    /// It is important to note that the components of the color are
    /// normalized before being passed to the shader. Therefore,
    /// they are converted from range [0 .. 255] to range [0 .. 1].
    /// For example, a `Color(255, 128, 0, 255)` will be transformed
    /// to a `vec4(1.0, 0.5, 0.0, 1.0)` in the shader.
    pub fn set_color_parameter(&mut self, name: &str, color: Color) {
        let c_str = try_string!(name, ());
        unsafe {
            ffi::sfShader_setColorParameter(self.raw_mut(), c_str.as_ptr(), color)
        }
    }

	/// Change a matrix parameter of the shader.
	///
	/// The corresponding parameter in the shader must be a 4x4 matrix
	/// (`mat4` GLSL type).
	pub fn set_transform_parameter(&mut self, name: &str, transform: &Transform) {
        let c_str = try_string!(name, ());
		unsafe {
			ffi::sfShader_setTransformParameter(self.raw_mut(), c_str.as_ptr(), *transform)
		}
	}

    /// Bind a shader for rendering.
    ///
    /// This function is not part of the graphics API, and mustn't be
    /// used when drawing SFML entities. It must be used only if you
    /// mix `Shader` with OpenGL code.
    pub fn bind(&mut self) {
        unsafe {
            ffi::sfShader_bind(self.raw_mut())
        }
    }

    /// Tell whether or not the system supports shaders.
    ///
    /// This function should always be called before using
    /// the shader features. If it returns false, then
    /// any attempt to use `Shader` will fail.
    ///
    /// Returns true if shaders are supported, false otherwise.
    pub fn is_available() -> bool {
        unsafe { ffi::sfShader_isAvailable() }.to_bool()
    }
}
