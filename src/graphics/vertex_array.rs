use crate::{
    ffi::graphics::*,
    graphics::{Drawable, FloatRect, PrimitiveType, RenderStates, RenderTarget, Vertex},
};
use std::ops::{Index, IndexMut};

/// Define a set of one or more 2D primitives
#[derive(Debug)]
pub struct VertexArray {
    vertex_array: *mut sfVertexArray,
}

/// An iterator over the vertice of a [`VertexArray`].
#[derive(Debug)]
pub struct Vertices<'a> {
    vertex_array: &'a VertexArray,
    pos: u32,
}

impl VertexArray {
    /// Create a new initialized vertex array
    ///
    /// # Arguments
    /// * `primitive_type` - The type of the `VertexArray`
    /// * `vertex_count` - The initial number of vertex
    #[must_use]
    pub fn new(primitive_type: PrimitiveType, vertex_count: usize) -> VertexArray {
        let mut arr = Self::default();
        arr.set_primitive_type(primitive_type);
        arr.resize(vertex_count);
        arr
    }

    /// Return the vertex count of a vertex array
    ///
    /// Return the number of vertices in the array
    #[must_use]
    pub fn vertex_count(&self) -> usize {
        unsafe { sfVertexArray_getVertexCount(self.vertex_array) }
    }

    /// Clear a vertex array
    ///
    /// This function removes all the vertices from the array.
    /// It doesn't deallocate the corresponding memory, so that
    /// adding new vertices after clearing doesn't involve
    /// reallocating all the memory.
    pub fn clear(&mut self) {
        unsafe { sfVertexArray_clear(self.vertex_array) }
    }

    /// Resize the vertex array
    ///
    /// If vertexCount is greater than the current size, the previous
    /// vertices are kept and new (default-constructed) vertices are
    /// added.
    /// If vertexCount is less than the current size, existing vertices
    /// are removed from the array.
    ///
    /// # Arguments
    /// * `vertex_count` - New size of the array (number of vertices)
    pub fn resize(&mut self, vertex_count: usize) {
        unsafe { sfVertexArray_resize(self.vertex_array, vertex_count) }
    }

    /// Add a vertex to a vertex array
    ///
    /// # Arguments
    /// * vertex - Vertex to add
    pub fn append(&mut self, vertex: &Vertex) {
        unsafe { sfVertexArray_append(self.vertex_array, vertex.raw()) }
    }

    /// Compute the bounding rectangle of a vertex array
    ///
    /// This function returns the axis-aligned rectangle that
    /// contains all the vertices of the array.
    ///
    /// Return the bounding rectangle of the vertex array
    #[must_use]
    pub fn bounds(&self) -> FloatRect {
        unsafe { FloatRect::from_raw(sfVertexArray_getBounds(self.vertex_array)) }
    }

    /// Set the type of primitives of a vertex array
    ///
    /// This function defines how the vertices must be interpreted
    /// when it's time to draw them:
    /// As points
    /// As lines
    /// As triangles
    /// As quads
    /// The default primitive type is Points.
    ///
    /// # Arguments
    /// * type - Type of primitive
    pub fn set_primitive_type(&mut self, primitive_type: PrimitiveType) {
        unsafe {
            sfVertexArray_setPrimitiveType(self.vertex_array, primitive_type.0);
        }
    }

    /// Get the type of primitives drawn by a vertex array
    ///
    /// Return the primitive type
    #[must_use]
    pub fn primitive_type(&self) -> PrimitiveType {
        unsafe { PrimitiveType(sfVertexArray_getPrimitiveType(self.vertex_array)) }
    }

    /// Return an immutable iterator over all the vertice contained by the `VertexArray`
    #[must_use]
    pub fn vertices(&self) -> Vertices {
        Vertices {
            vertex_array: self,
            pos: 0,
        }
    }

    /// Returns an immutable reference to the `index`-th vertex in the `VertexArray`.
    ///
    /// # Safety
    /// The behaviour is undefined if `index >= self.vertex_count()`.
    #[must_use]
    pub unsafe fn get_vertex_unchecked(&self, index: usize) -> &Vertex {
        &*(sfVertexArray_getVertex(self.vertex_array, index) as *const _)
    }

    /// Returns a mutable reference to the `index`-th vertex in the `VertexArray`.
    ///
    /// # Safety
    /// The behaviour is undefined if `index >= self.vertex_count()`.
    #[must_use]
    pub unsafe fn get_vertex_mut_unchecked(&mut self, index: usize) -> &mut Vertex {
        &mut *(sfVertexArray_getVertex(self.vertex_array, index) as *mut _)
    }

    /// Sets the `index`-th vertex to `vertex`.
    ///
    /// # Safety
    /// The behaviour is undefined if `index >= self.vertex_count()`.
    pub unsafe fn set_vertex_unchecked(&mut self, index: usize, vertex: &Vertex) {
        let v = self.get_vertex_mut_unchecked(index);
        *v = *vertex;
    }

    pub(super) fn raw(&self) -> *const sfVertexArray {
        self.vertex_array
    }
}

impl Default for VertexArray {
    fn default() -> Self {
        let ver = unsafe { sfVertexArray_create() };
        assert!(!ver.is_null(), "Failed to create VertexArray");
        VertexArray { vertex_array: ver }
    }
}

impl Clone for VertexArray {
    /// Return a new `VertexArray` or panic! if there is not enough memory
    fn clone(&self) -> VertexArray {
        let ver = unsafe { sfVertexArray_copy(self.vertex_array) };
        if ver.is_null() {
            panic!("Not enough memory to clone VertexArray")
        } else {
            VertexArray { vertex_array: ver }
        }
    }
}

impl<'a> Iterator for Vertices<'a> {
    type Item = &'a Vertex;

    fn next(&mut self) -> Option<&'a Vertex> {
        use std::convert::TryInto;
        let point_count = unsafe {
            sfVertexArray_getVertexCount(self.vertex_array.vertex_array)
                .try_into()
                .unwrap()
        };
        if self.pos == point_count {
            None
        } else {
            self.pos += 1;
            unsafe {
                (sfVertexArray_getVertex(self.vertex_array.vertex_array, self.pos as usize)
                    as *const Vertex)
                    .as_ref()
            }
        }
    }
}

impl Index<usize> for VertexArray {
    type Output = Vertex;

    fn index(&self, idx: usize) -> &Vertex {
        assert!(
            idx < self.vertex_count(),
            "Out of bounds: {}, max {}",
            idx,
            self.vertex_count()
        );
        unsafe { self.get_vertex_unchecked(idx) }
    }
}

impl IndexMut<usize> for VertexArray {
    fn index_mut(&mut self, idx: usize) -> &mut Vertex {
        assert!(
            idx < self.vertex_count(),
            "Out of bounds: {}, max {}",
            idx,
            self.vertex_count()
        );
        unsafe { self.get_vertex_mut_unchecked(idx) }
    }
}

impl Drawable for VertexArray {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_vertex_array(self, states)
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { sfVertexArray_destroy(self.vertex_array) }
    }
}
