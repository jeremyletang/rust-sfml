use crate::{
    ffi::graphics as ffi,
    graphics::{glsl, Texture},
    system::InputStream,
    LoadResult, ResourceLoadError,
};
use std::{
    ffi::CString,
    io::{Read, Seek},
    marker::PhantomData,
    ptr::{self, NonNull},
};

use super::ShaderType;

/// Shader type (vertex, geometry and fragment).
///
/// Shaders are programs written using a specific language, executed directly
/// by the graphics card and allowing to apply real-time operations to the rendered entities.
///
/// There are three kinds of shaders:
///
/// - Vertex shaders, that process vertices
/// - Geometry shaders, that process primitives
/// - Fragment (pixel) shaders, that process pixels
///
/// A `Shader` can be composed of either a vertex shader alone, a geometry shader alone,
/// a fragment shader alone, or any combination of them. (see the variants of the load functions).
///
/// Shaders are written in GLSL, which is a C-like language dedicated to OpenGL shaders.
/// You'll probably need to learn its basics before writing your own shaders for SFML.
///
/// Like any Rust program, a GLSL shader has its own variables called uniforms that you can set
/// from your Rust application. `Shader` handles different types of uniforms:
///
/// - scalars: `float`, `int`, `bool`
/// - vectors (2, 3 or 4 components)
/// - matrices (3x3 or 4x4)
/// - samplers (textures)
///
/// Some SFML-specific types can be converted:
///
/// - [`Color`] as a 4D vector (`vec4`)
/// - [`Transform`] as matrices (`mat3` or `mat4`)
/// Every uniform variable in a shader can be set through one of the
/// `set_uniform_*()` or `set_uniform_array_*()` methods.
/// For example, if you have a shader with the following uniforms:
///
/// ```glsl
/// uniform float offset;
/// uniform vec3 point;
/// uniform vec4 color;
/// uniform mat4 matrix;
/// uniform sampler2D overlay;
/// uniform sampler2D current;
/// ```
///
/// You can set their values from Rust code as follows,
/// using the types defined in the `glsl` module:
///
/// ```no_run
/// # use sfml::graphics::*;
/// # use sfml::system::*;
/// # use sfml::SfBox;
/// let texture: SfBox<Texture> = unimplemented!();
/// let mut shader: Shader = unimplemented!();
/// let color: Color = unimplemented!();
/// let transform: Transform = unimplemented!();
/// shader.set_uniform_float("offset", 2.);
/// shader.set_uniform_vec3("point", Vector3f::new(0.5, 0.8, 0.3));
/// shader.set_uniform_vec4("color", color);
/// shader.set_uniform_mat4("matrix", transform);
/// shader.set_uniform_texture("overlay", &texture);
/// shader.set_uniform_current_texture("current");
/// ```
///
/// To apply a shader to a drawable,
/// you must set the `shader` field of a [`RenderStates`] instance, and use
/// [`RenderTarget::draw_with_renderstates`]. Example:
///
/// ```no_run
/// # use sfml::graphics::*;
/// # let shader: Shader = unimplemented!();
/// # let mut window: RenderWindow = unimplemented!();
/// # let sprite: Sprite = unimplemented!();
/// let mut states = RenderStates::default();;
/// states.set_shader(Some(&shader));
/// window.draw_with_renderstates(&sprite, &states);
/// ```
///
/// Shaders can be used on any drawable, but some combinations are not interesting.
/// For example, using a vertex shader on a [`Sprite`] is limited
/// because there are only 4 vertices,
/// the sprite would have to be subdivided in order to apply wave effects.
/// Another bad example is a fragment shader with [`Text`]: the texture of the text is
/// not the actual text that you see on screen,
/// it is a big texture containing all the characters of the font in an arbitrary order;
/// thus, texture lookups on pixels other than the current one may not give you
/// the expected result.
///
/// Shaders can also be used to apply global post-effects to the current contents of the
/// target (like the old `sf::PostFx` class in SFML 1). This can be done in two different ways:
///
/// - draw everything to a [`RenderTexture`], then draw it to the main target using the shader
///
/// - draw everything directly to the main target, then use [`Texture::update_from_window`] to copy
///   its contents to a texture and draw it to the main target using the shader.
///
/// The first technique is more optimized because it doesn't involve retrieving the target's
/// pixels to system memory, but the second one doesn't impact the rendering process and can
/// be easily inserted anywhere without impacting all the code.
///
/// Like [`Texture`] that can be used as a raw OpenGL texture, `Shader` can also be used
/// directly as a raw shader for custom OpenGL geometry.
///
/// ```no_run
/// use sfml::graphics::*;
/// # let shader: Shader = unimplemented!();
/// Shader::bind(Some(&shader));
/// // ... render OpenGL geometry ...
/// Shader::bind(None);
/// ```
///
/// [`Color`]: crate::graphics::Color
/// [`Transform`]: crate::graphics::Transform
/// [`RenderStates`]: crate::graphics::RenderStates
/// [`RenderTarget::draw_with_renderstates`]: crate::graphics::RenderTarget::draw_with_renderstates
/// [`Sprite`]: crate::graphics::Sprite
/// [`Text`]: crate::graphics::Text
/// [`RenderTexture`]: crate::graphics::RenderTexture
///
#[derive(Debug)]
pub struct Shader<'texture> {
    shader: NonNull<ffi::sfShader>,
    texture: PhantomData<&'texture Texture>,
}

macro_rules! shader_create {
    ($shader:ident, $load_block:block) => {{
        let $shader =
            NonNull::new(unsafe { ffi::sfShader_defaultConstruct() }).ok_or(ResourceLoadError)?;
        unsafe {
            if !$load_block {
                ffi::sfShader_destroy($shader.as_ptr());
                return Err(ResourceLoadError);
            }
        }
        Ok(Self {
            shader: $shader,
            texture: PhantomData,
        })
    }};
}

fn c_string(source: &str) -> LoadResult<CString> {
    CString::new(source).map_err(|_| ResourceLoadError)
}

impl<'texture> Shader<'texture> {
    /// Load the vertex, geometry or fragment shader from a file.
    ///
    /// This function loads a single shader, vertex, geometry or fragment,
    /// identified by the second argument.
    /// The source must be a text file containing a valid shader in GLSL language.
    /// GLSL is a C-like language dedicated to OpenGL shaders; you'll probably need to read a good
    /// documentation for it before writing your own shaders.
    pub fn from_file(path: &str, type_: ShaderType) -> LoadResult<Self> {
        shader_create!(shader, {
            let path = c_string(path)?;
            ffi::sfShader_loadFromFile_1(shader.as_ptr(), path.as_ptr(), type_)
        })
    }

    /// Load both the vertex and fragment shaders from files.
    ///
    /// This function loads both the vertex and the fragment shaders.
    /// The sources must be text files containing valid shaders in GLSL language.
    /// GLSL is a C-like language dedicated to OpenGL shaders;
    /// you'll probably need to read a good documentation for it before writing your own shaders.
    pub fn from_file_vert_frag(vert: &str, frag: &str) -> LoadResult<Self> {
        shader_create!(shader, {
            let vert = c_string(vert)?;
            let frag = c_string(frag)?;
            ffi::sfShader_loadFromFile_vert_frag(shader.as_ptr(), vert.as_ptr(), frag.as_ptr())
        })
    }

    /// Load the vertex, geometry and fragment shaders from files.
    ///
    /// This function loads the vertex, geometry and fragment shaders.
    /// The sources must be text files containing valid shaders in GLSL language.
    /// GLSL is a C-like language dedicated to OpenGL shaders; you'll probably need to
    /// read a good documentation for it before writing your own shaders.
    pub fn from_file_all(vert: &str, geom: &str, frag: &str) -> LoadResult<Self> {
        shader_create!(shader, {
            let vert = c_string(vert)?;
            let geom = c_string(geom)?;
            let frag = c_string(frag)?;
            ffi::sfShader_loadFromFile_all(
                shader.as_ptr(),
                vert.as_ptr(),
                geom.as_ptr(),
                frag.as_ptr(),
            )
        })
    }

    /// Load the vertex, geometry or fragment shader from a source code in memory.
    ///
    /// This function loads a single shader, vertex, geometry or fragment, identified by the second
    /// argument.
    /// The source code must be a valid shader in GLSL language.
    /// GLSL is a C-like language dedicated to OpenGL shaders; you'll probably need to read a
    /// good documentation for it before writing your own shaders.
    pub fn from_memory(contents: &str, type_: ShaderType) -> LoadResult<Self> {
        shader_create!(shader, {
            let contents = c_string(contents)?;
            ffi::sfShader_loadFromMemory_1(shader.as_ptr(), contents.as_ptr(), type_)
        })
    }

    /// Load both the vertex and fragment shaders from source codes in memory.
    ///
    /// This function loads both the vertex and the fragment shaders.
    /// The sources must be valid shaders in GLSL language. GLSL is a C-like language dedicated
    /// to OpenGL shaders; you'll probably need to read a good documentation
    /// for it before writing your own shaders.
    pub fn from_memory_vert_frag(vert: &str, frag: &str) -> LoadResult<Self> {
        shader_create!(shader, {
            let vert = c_string(vert)?;
            let frag = c_string(frag)?;
            ffi::sfShader_loadFromMemory_vert_frag(shader.as_ptr(), vert.as_ptr(), frag.as_ptr())
        })
    }

    /// Load the vertex, geometry and fragment shaders from source codes in memory.
    ///
    /// This function loads the vertex, geometry and fragment shaders.
    /// The sources must be valid shaders in GLSL language. GLSL is a C-like language dedicated to
    /// OpenGL shaders; you'll probably need to read a good documentation for it
    /// before writing your own shaders.
    pub fn from_memory_all(vert: &str, geom: &str, frag: &str) -> LoadResult<Self> {
        shader_create!(shader, {
            let vert = c_string(vert)?;
            let geom = c_string(geom)?;
            let frag = c_string(frag)?;
            ffi::sfShader_loadFromMemory_all(
                shader.as_ptr(),
                vert.as_ptr(),
                geom.as_ptr(),
                frag.as_ptr(),
            )
        })
    }

    /// Load the vertex, geometry or fragment shader from a custom stream.
    ///
    /// This function loads a single shader, vertex, geometry or fragment, identified by the second
    /// argument. The source code must be a valid shader in GLSL language.
    /// GLSL is a C-like language dedicated to OpenGL shaders; you'll probably need to read a good
    /// documentation for it before writing your own shaders.
    pub fn from_stream<T: Read + Seek>(mut source: T, type_: ShaderType) -> LoadResult<Self> {
        shader_create!(shader, {
            let source = InputStream::new(&mut source);
            ffi::sfShader_loadFromStream_1(shader.as_ptr(), source.stream.0.as_ptr(), type_)
        })
    }

    /// Load both the vertex and fragment shaders from custom streams.
    ///
    /// This function loads both the vertex and the fragment shaders.
    /// The source codes must be valid shaders in GLSL language. GLSL is a C-like
    /// language dedicated to OpenGL shaders; you'll probably need to read a good documentation
    /// for it before writing your own shaders.
    pub fn from_stream_vert_frag<T, U>(mut vert: T, mut frag: U) -> LoadResult<Self>
    where
        T: Read + Seek,
        U: Read + Seek,
    {
        shader_create!(shader, {
            let vert = InputStream::new(&mut vert);
            let frag = InputStream::new(&mut frag);
            ffi::sfShader_loadFromStream_vert_frag(
                shader.as_ptr(),
                vert.stream.0.as_ptr(),
                frag.stream.0.as_ptr(),
            )
        })
    }

    /// Load the vertex, geometry and fragment shaders from custom streams.
    ///
    /// This function loads the vertex, geometry and fragment shaders.
    /// The source codes must be valid shaders in GLSL language. GLSL is a C-like language
    /// dedicated to OpenGL shaders; you'll probably need to read a good documentation for it
    /// before writing your own shaders.
    pub fn from_stream_all<T, U, V>(mut vert: T, mut geom: U, mut frag: V) -> LoadResult<Self>
    where
        T: Read + Seek,
        U: Read + Seek,
        V: Read + Seek,
    {
        shader_create!(shader, {
            let vert = InputStream::new(&mut vert);
            let geom = InputStream::new(&mut geom);
            let frag = InputStream::new(&mut frag);
            ffi::sfShader_loadFromStream_all(
                shader.as_ptr(),
                vert.stream.0.as_ptr(),
                geom.stream.0.as_ptr(),
                frag.stream.0.as_ptr(),
            )
        })
    }

    /// Bind a shader for rendering.
    ///
    /// This function is not part of the graphics API,
    /// it mustn't be used when drawing SFML entities.
    /// It must be used only if you mix `Shader` with OpenGL code.
    pub fn bind(shader: Option<&Self>) {
        unsafe { ffi::sfShader_bind(shader.map_or(ptr::null_mut(), |s| s.shader.as_ptr())) }
    }

    /// Tell whether or not the system supports shaders
    ///
    /// This function should always be called before using
    /// the shader features. If it returns false, then
    /// any attempt to use `Shader` will fail.
    #[must_use]
    pub fn is_available() -> bool {
        unsafe { ffi::sfShader_isAvailable() }
    }

    /// Tell whether or not the system supports geometry shaders.
    ///
    /// This function should always be called before using the geometry shader features.
    /// If it returns `false`,
    /// then any attempt to use `Shader` geometry shader features will fail.
    ///
    /// This function can only return true if [`Shader::is_available`] would also return `true`,
    /// since shaders in general have to be supported in order for geometry shaders
    /// to be supported as well.
    ///
    /// Note: The first call to this function, whether by your code or SFML will result
    /// in a context switch.
    #[must_use]
    pub fn is_geometry_available() -> bool {
        unsafe { ffi::sfShader_isGeometryAvailable() }
    }

    /// Specify value for `float` uniform.
    pub fn set_uniform_float(&mut self, name: &str, value: f32) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setFloatUniform(self.shader.as_ptr(), name, value);
        }
    }

    /// Specify value for `vec2` uniform.
    pub fn set_uniform_vec2(&mut self, name: &str, value: glsl::Vec2) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setVec2Uniform(self.shader.as_ptr(), name, value.raw());
        }
    }

    /// Specify value for `vec3` uniform.
    pub fn set_uniform_vec3(&mut self, name: &str, value: glsl::Vec3) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setVec3Uniform(self.shader.as_ptr(), name, value.raw());
        }
    }

    /// Specify value for vec4 uniform.
    ///
    /// This function can also be called with [`Color`] objects that are converted to
    /// [`glsl::Vec4`].
    ///
    /// [`Color`]: crate::graphics::Color
    ///
    /// It is important to note that the components of the color are normalized before being
    /// passed to the shader. Therefore, they are converted from range `[0 .. 255]` to range
    /// `[0 .. 1]`. For example, a `Color{r: 255, g: 127, b: 0, a: 255}`
    /// will be transformed to a `Vec4{x: 1.0, y: 0.5, z: 0.0, w: 1.0}` in the shader.
    pub fn set_uniform_vec4<V>(&mut self, name: &str, value: V)
    where
        V: Into<glsl::Vec4>,
    {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setVec4Uniform(self.shader.as_ptr(), name, value.into().raw());
        }
    }

    /// Specify value for `int` uniform.
    pub fn set_uniform_int(&mut self, name: &str, value: i32) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setIntUniform(self.shader.as_ptr(), name, value);
        }
    }

    /// Specify value for `ivec2` uniform.
    pub fn set_uniform_ivec2(&mut self, name: &str, value: glsl::IVec2) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setIvec2Uniform(self.shader.as_ptr(), name, value.raw());
        }
    }

    /// Specify value for `ivec3` uniform.
    pub fn set_uniform_ivec3(&mut self, name: &str, value: glsl::IVec3) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setIvec3Uniform(self.shader.as_ptr(), name, value.into());
        }
    }

    /// Specify value for `ivec4` uniform.
    ///
    /// This overload can also be called with [`Color`] objects that are
    /// converted to [`glsl::IVec4`].
    ///
    /// [`Color`]: crate::graphics::Color
    ///
    /// If color conversions are used, the `ivec4` uniform in GLSL will hold the same values
    /// as the original [`Color`] instance. For example, `Color{r: 255, g: 127, b: 0, a: 255}`
    /// is mapped to `IVec4{x: 255, y: 127, z: 0, w: 255}`.
    pub fn set_uniform_ivec4<V>(&mut self, name: &str, value: V)
    where
        V: Into<glsl::IVec4>,
    {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setIvec4Uniform(self.shader.as_ptr(), name, value.into().raw());
        }
    }

    /// Specify value for `bool` uniform.
    pub fn set_uniform_bool(&mut self, name: &str, value: bool) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setBoolUniform(self.shader.as_ptr(), name, value);
        }
    }

    /// Specify value for `bvec2` uniform.
    pub fn set_uniform_bvec2(&mut self, name: &str, value: glsl::BVec2) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setBvec2Uniform(self.shader.as_ptr(), name, value.into());
        }
    }

    /// Specify value for `bvec3` uniform.
    pub fn set_uniform_bvec3(&mut self, name: &str, value: glsl::BVec3) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setBvec3Uniform(self.shader.as_ptr(), name, value.into());
        }
    }

    /// Specify value for `bvec4` uniform.
    pub fn set_uniform_bvec4(&mut self, name: &str, value: glsl::BVec4) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setBvec4Uniform(self.shader.as_ptr(), name, value.into());
        }
    }

    /// Specify value for `mat3` matrix.
    pub fn set_uniform_mat3<V>(&mut self, name: &str, value: V)
    where
        V: Into<glsl::Mat3>,
    {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let value = value.into();
            let ptr: *const _ = &value.0;
            ffi::sfShader_setMat3Uniform(self.shader.as_ptr(), name, ptr as *const _);
        }
    }

    /// Specify value for `mat4` matrix.
    pub fn set_uniform_mat4<V>(&mut self, name: &str, value: V)
    where
        V: Into<glsl::Mat4>,
    {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let value = value.into();
            let ptr: *const _ = &value.0;
            ffi::sfShader_setMat4Uniform(self.shader.as_ptr(), name, ptr as *const _);
        }
    }

    /// Specify a texture as `sampler2D` uniform.
    ///
    /// `name` is the name of the variable to change in the shader.
    /// The corresponding parameter in the shader must be a 2D texture (`sampler2D` GLSL type).
    ///
    /// To use the texture of the object being drawn, which cannot be known in advance,
    /// use [`Shader::set_uniform_current_texture`].
    pub fn set_uniform_texture(&mut self, name: &str, value: &'texture Texture) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setTextureUniform(self.shader.as_ptr(), name, value.raw());
        }
    }

    /// Specify current texture as `sampler2D` uniform.
    ///
    /// This function maps a shader texture variable to the texture of the object being drawn,
    /// which cannot be known in advance.
    /// The corresponding parameter in the shader must be a 2D texture (`sampler2D` GLSL type).
    pub fn set_uniform_current_texture(&mut self, name: &str) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            ffi::sfShader_setCurrentTextureUniform(self.shader.as_ptr(), name);
        }
    }

    /// Specify values for `float[]` array uniform.
    pub fn set_uniform_array_float(&mut self, name: &str, array: &[f32]) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let len = array.len();
            ffi::sfShader_setFloatUniformArray(self.shader.as_ptr(), name, array.as_ptr(), len);
        }
    }

    /// Specify values for `vec2[]` array uniform.
    pub fn set_uniform_array_vec2(&mut self, name: &str, array: &[glsl::Vec2]) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let len = array.len();
            let ptr = array.as_ptr() as *const ffi::sfGlslVec2;
            ffi::sfShader_setVec2UniformArray(self.shader.as_ptr(), name, ptr, len);
        }
    }

    /// Specify values for `vec3[]` array uniform.
    pub fn set_uniform_array_vec3(&mut self, name: &str, array: &[glsl::Vec3]) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let len = array.len();
            let ptr = array.as_ptr();
            ffi::sfShader_setVec3UniformArray(self.shader.as_ptr(), name, ptr, len);
        }
    }

    /// Specify values for `vec4[]` array uniform.
    pub fn set_uniform_array_vec4(&mut self, name: &str, array: &[glsl::Vec4]) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let len = array.len();
            let ptr = array.as_ptr() as *const ffi::sfGlslVec4;
            ffi::sfShader_setVec4UniformArray(self.shader.as_ptr(), name, ptr, len);
        }
    }

    /// Specify values for `mat3[]` array uniform.
    pub fn set_uniform_array_mat3(&mut self, name: &str, array: &[glsl::Mat3]) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let len = array.len();
            let ptr = array.as_ptr() as *const ffi::sfGlslMat3;
            ffi::sfShader_setMat3UniformArray(self.shader.as_ptr(), name, ptr, len);
        }
    }

    /// Specify values for `mat4[]` array uniform.
    pub fn set_uniform_array_mat4(&mut self, name: &str, array: &[glsl::Mat4]) {
        unsafe {
            let cstring = CString::new(name).unwrap();
            let name = cstring.as_ptr();
            let len = array.len();
            let ptr = array.as_ptr() as *const ffi::sfGlslMat4;
            ffi::sfShader_setMat4UniformArray(self.shader.as_ptr(), name, ptr, len);
        }
    }
    /// Get the underlying OpenGL handle of the shader.
    ///
    /// You shouldn't need to use this function, unless you have very specific stuff to implement
    /// that SFML doesn't support, or implement a temporary workaround until a bug is fixed.
    #[must_use]
    pub fn native_handle(&self) -> u32 {
        unsafe { ffi::sfShader_getNativeHandle(self.shader.as_ptr()) }
    }
    pub(super) fn raw(&self) -> *const ffi::sfShader {
        self.shader.as_ptr()
    }
}

impl<'texture> Drop for Shader<'texture> {
    fn drop(&mut self) {
        unsafe { ffi::sfShader_destroy(self.shader.as_ptr()) }
    }
}
