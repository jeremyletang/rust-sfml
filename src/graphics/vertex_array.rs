use graphics::{Drawable, FloatRect, PrimitiveType, RenderStates, RenderTarget, Vertex};
use graphics::csfml_graphics_sys::*;
use std::mem;
use std::ops::{Index, IndexMut};
use system::raw_conv::{FromRaw, Raw};

/// Define a set of one or more 2D primitives
#[derive(Debug)]
pub struct VertexArray {
    vertex_array: *mut sfVertexArray,
}

/// An iterator over the vertice of a `VertexArray`
#[derive(Debug)]
pub struct Vertices<'a> {
    vertex_array: &'a VertexArray,
    pos: u32,
}

impl VertexArray {
    /// Create a new initialized vertex array
    ///
    /// # Arguments
    /// * primitive_type - The type of the VertexArray
    /// * vertex_count - The maximal number of vertex
    pub fn new(primitive_type: PrimitiveType, vertex_count: usize) -> VertexArray {
        let mut arr = Self::default();
        arr.set_primitive_type(primitive_type);
        arr.resize(vertex_count);
        arr
    }

    /// Return the vertex count of a vertex array
    ///
    /// Return the number of vertices in the array
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
    /// * vertex_count - New size of the array (number of vertices)
    pub fn resize(&mut self, vertex_count: usize) {
        unsafe { sfVertexArray_resize(self.vertex_array, vertex_count) }
    }

    /// Add a vertex to a vertex array array
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
            sfVertexArray_setPrimitiveType(self.vertex_array, primitive_type.raw());
        }
    }

    /// Get the type of primitives drawn by a vertex array
    ///
    /// Return the primitive type
    pub fn primitive_type(&self) -> PrimitiveType {
        unsafe { FromRaw::from_raw(sfVertexArray_getPrimitiveType(self.vertex_array)) }
    }

    /// Return an immutable iterator over all the vertice contained by the VertexArray
    pub fn vertices(&self) -> Vertices {
        Vertices {
            vertex_array: self,
            pos: 0,
        }
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
    /// Return a new Font or panic! if there is not enough memory
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
        let point_count =
            unsafe { sfVertexArray_getVertexCount(self.vertex_array.vertex_array) as u32 };
        if self.pos == point_count {
            None
        } else {
            self.pos += 1;
            unsafe {
                mem::transmute(sfVertexArray_getVertex(self.vertex_array.vertex_array,
                                                       self.pos as usize))
            }
        }
    }
}

impl Index<usize> for VertexArray {
    type Output = Vertex;

    fn index(&self, idx: usize) -> &Vertex {
        assert!(idx < self.vertex_count(),
                "Out of bounds: {}, max {}",
                idx,
                self.vertex_count());
        unsafe { &*(sfVertexArray_getVertex(self.vertex_array, idx) as *const Vertex) }
    }
}

impl IndexMut<usize> for VertexArray {
    fn index_mut(&mut self, idx: usize) -> &mut Vertex {
        assert!(idx < self.vertex_count(),
                "Out of bounds: {}, max {}",
                idx,
                self.vertex_count());
        unsafe { &mut *(sfVertexArray_getVertex(self.vertex_array, idx) as *mut Vertex) }
    }
}

impl Raw for VertexArray {
    type Raw = *const sfVertexArray;
    fn raw(&self) -> Self::Raw {
        self.vertex_array
    }
}

impl Drawable for VertexArray {
    fn draw<'se, 'tex, 'sh, 'shte>(&'se self,
                                   target: &mut RenderTarget,
                                   states: RenderStates<'tex, 'sh, 'shte>)
        where 'se: 'sh
    {
        target.draw_vertex_array(self, states)
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { sfVertexArray_destroy(self.vertex_array) }
    }
}
