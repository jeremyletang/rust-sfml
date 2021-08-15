use std::ptr::NonNull;

use crate::graphics::{Drawable, PrimitiveType, RenderStates, RenderTarget, Vertex};
use csfml_graphics_sys::*;

/// Usage specifiers for a [`VertexBuffer`]
///
/// If data is going to be updated once or more every frame,
/// set the usage to `Stream`.  If data is going to be set once and used
/// for a long time without being modified, set the usage to `Static`.
/// For everything else `Dynamic` should be a good compromise.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct VertexBufferUsage(pub(super) sfVertexBufferUsage);

impl VertexBufferUsage {
    /// Constantly changing data.
    pub const STREAM: Self = Self(sfVertexBufferUsage_sfVertexBufferStream);
    /// Occasionally changing data.
    pub const DYNAMIC: Self = Self(sfVertexBufferUsage_sfVertexBufferDynamic);
    /// Rarely changing data.
    pub const STATIC: Self = Self(sfVertexBufferUsage_sfVertexBufferStatic);
}

/// Define a set of one or more 2D primitives stored in graphics memory
#[derive(Debug)]
pub struct VertexBuffer {
    vertex_buffer: NonNull<sfVertexBuffer>,
}

impl VertexBuffer {
    /// Create a new initialized vertex buffer
    ///
    /// # Arguments
    /// * `primitive_type` - The type of the `VertexBuffer`
    /// * `vertex_count` - The maximal number of vertex
    #[must_use]
    pub fn new(
        primitive_type: PrimitiveType,
        vertex_count: u32,
        usage: VertexBufferUsage,
    ) -> VertexBuffer {
        let vertex_buffer =
            unsafe { sfVertexBuffer_create(vertex_count, primitive_type.0, usage.0) };
        VertexBuffer {
            vertex_buffer: NonNull::new(vertex_buffer).expect("Failed to create VertexBuffer"),
        }
    }

    /// Return the vertex count of a vertex buffer
    ///
    /// Return the number of vertices in the buffer
    #[must_use]
    pub fn vertex_count(&self) -> u32 {
        unsafe { sfVertexBuffer_getVertexCount(self.vertex_buffer.as_ptr()) }
    }

    /// Update a part of the buffer from an array of vertices.
    ///
    /// `offset` is specified as the number of vertices to skip from the beginning of the buffer.
    ///
    /// If `offset` is `0` and `vertices.len()` is equal to the size of the currently created buffer, its
    /// whole contents are replaced.
    ///
    /// If `offset` is `0` and `vertices.len()` is greater than the size of the currently created buffer, a
    /// new buffer is created containing the vertex data.
    ///
    /// If `offset` is `0` and `vertices.len()` is less than the size of the currently created buffer, only
    /// the corresponding region is updated.
    ///
    /// If `offset` is not `0` and `offset + vertices.len()` is greater than the size of the currently
    /// created buffer, the update fails.
    ///
    /// No additional check is performed on the size of the vertex array, passing invalid arguments
    /// will lead to undefined behavior.
    ///
    /// # Arguments
    /// * vertices - Array of vertices to copy in the buffer
    /// * offset - Offset in the buffer to copy to
    ///
    /// Return True if the update was successful
    pub fn update(&mut self, vertices: &[Vertex], offset: u32) -> bool {
        use std::convert::TryInto;
        unsafe {
            sfVertexBuffer_update(
                self.vertex_buffer.as_ptr(),
                vertices.as_ptr() as *const _,
                vertices.len().try_into().unwrap(),
                offset,
            ) != 0
        }
    }

    /// Copy the contents of another buffer into this buffer.
    ///
    /// # Arguments
    /// * other - Vertex buffer whose contents to copy into this vertex buffer
    ///
    /// Return True if the update was successful
    pub fn update_from_vertex_buffer(&mut self, other: &VertexBuffer) -> bool {
        unsafe {
            sfVertexBuffer_updateFromVertexBuffer(
                self.vertex_buffer.as_ptr(),
                other.vertex_buffer.as_ptr(),
            ) != 0
        }
    }

    /// Swap the contents of this vertex buffer with those of another.
    ///
    /// # Arguments
    /// * other - Instance to swap with
    pub fn swap(&mut self, other: &mut VertexBuffer) {
        unsafe {
            sfVertexBuffer_swap(self.vertex_buffer.as_ptr(), other.vertex_buffer.as_ptr());
        }
    }

    /// Get the underlying OpenGL handle of the vertex buffer.
    ///
    /// You shouldn't need to use this function, unless you have very specific stuff to implement
    /// that SFML doesn't support, or implement a temporary workaround until a bug is fixed.
    ///
    /// Return OpenGL handle of the vertex buffer or 0 if not yet created
    #[must_use]
    pub fn native_handle(&self) -> u32 {
        unsafe { sfVertexBuffer_getNativeHandle(self.vertex_buffer.as_ptr()) }
    }

    /// Get the type of primitives drawn by the vertex buffer.
    ///
    /// Return Primitive type
    #[must_use]
    pub fn primitive_type(&self) -> PrimitiveType {
        unsafe { PrimitiveType(sfVertexBuffer_getPrimitiveType(self.vertex_buffer.as_ptr())) }
    }

    /// Set the type of primitives to draw.
    ///
    /// This function defines how the vertices must be interpreted when it's time to draw them.
    ///
    /// The default primitive type is `Points`.
    ///
    /// # Arguments
    /// * `primitive_type` - Type of primitive
    pub fn set_primitive_type(&mut self, primitive_type: PrimitiveType) {
        unsafe {
            sfVertexBuffer_setPrimitiveType(self.vertex_buffer.as_ptr(), primitive_type.0);
        }
    }

    /// Get the usage specifier of this vertex buffer.
    ///
    /// Return Usage specifier
    #[must_use]
    pub fn usage(&self) -> VertexBufferUsage {
        unsafe { VertexBufferUsage(sfVertexBuffer_getUsage(self.vertex_buffer.as_ptr())) }
    }

    /// Set the usage specifier of this vertex buffer.
    ///
    /// This function provides a hint about how this vertex buffer is going to be used in terms of
    /// data update frequency.
    ///
    /// After changing the usage specifier, the vertex buffer has to be updated with new data for
    /// the usage specifier to take effect.
    ///
    /// The default primitive type is `Stream`.
    ///
    /// # Arguments
    /// * usage - Usage specifier
    pub fn set_usage(&mut self, usage: VertexBufferUsage) {
        unsafe { sfVertexBuffer_setUsage(self.vertex_buffer.as_ptr(), usage.0) }
    }

    /// Bind a vertex buffer for rendering.
    ///
    /// This function is not part of the graphics API, it mustn't be used when drawing SFML
    /// entities. It must be used only if you mix sf::VertexBuffer with OpenGL code.
    ///
    #[cfg_attr(feature = "ci-headless", doc = "```no_run")]
    #[cfg_attr(not(feature = "ci-headless"), doc = "```")]
    /// use sfml::graphics::{PrimitiveType, VertexBuffer, VertexBufferUsage};
    ///
    /// let mut vb1 = VertexBuffer::new(PrimitiveType::TRIANGLES, 32, VertexBufferUsage::STATIC);
    /// let mut vb2 = VertexBuffer::new(PrimitiveType::QUADS, 12, VertexBufferUsage::DYNAMIC);
    ///
    /// // ...
    ///
    /// VertexBuffer::bind(Some(&vb1));
    /// // draw OpenGL stuff that use vb1...
    /// VertexBuffer::bind(Some(&vb2));
    /// // draw OpenGL stuff that use vb2...
    /// VertexBuffer::bind(None);
    /// // draw OpenGL stuff that use no vertex buffer...
    /// ```
    ///
    /// # Arguments
    /// * vb - Vertex buffer to use; None to use no vertex buffer.
    pub fn bind(vb: Option<&VertexBuffer>) {
        unsafe {
            if let Some(&VertexBuffer { vertex_buffer }) = vb {
                sfVertexBuffer_bind(vertex_buffer.as_ptr());
            } else {
                sfVertexBuffer_bind(std::ptr::null());
            }
        }
    }

    /// Tell whether or not the system supports vertex buffers.

    /// This  function should always be called before using the vertex buffer features. If it
    /// returns false, then any attempt to use `VertexBuffer` will fail.
    ///
    /// Return True if vertex buffers are supported, false otherwise
    #[must_use]
    pub fn available() -> bool {
        unsafe { sfVertexBuffer_isAvailable() != 0 }
    }

    pub(super) fn raw(&self) -> *const sfVertexBuffer {
        self.vertex_buffer.as_ptr()
    }
}

impl Clone for VertexBuffer {
    /// Return a new `VertexBuffer` or panic! if the copy fails
    fn clone(&self) -> VertexBuffer {
        let vertex_buffer = unsafe { sfVertexBuffer_copy(self.vertex_buffer.as_ptr()) };
        VertexBuffer {
            vertex_buffer: NonNull::new(vertex_buffer).expect("Failed to clone VertexBuffer"),
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            sfVertexBuffer_destroy(self.vertex_buffer.as_ptr());
        }
    }
}

impl Drawable for VertexBuffer {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_vertex_buffer(self, states)
    }
}
