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

//! Define a set of one or more 2D primitives

use std::mem;
use std::ops::Index;

use sfml_system::raw_conv::{Raw, FromRaw};
use {Drawable, Vertex, FloatRect, primitive_type, PrimitiveType, RenderTarget, RenderStates};

use csfml_graphics_sys::*;
use csfml_graphics_sys::sfPrimitiveType::*;

/// Define a set of one or more 2D primitives
pub struct VertexArray {
    vertex_array: *mut sfVertexArray,
}

/// An iterator over the vertice of a `VertexArray`
pub struct Vertices<'a> {
    vertex_array: &'a VertexArray,
    pos: u32,
}

impl VertexArray {
    /// Create a new vertex array
    pub fn new() -> VertexArray {
        let ver = unsafe { sfVertexArray_create() };
        if ver.is_null() {
            panic!("sfVertexArray_create returned null.")
        } else {
            VertexArray { vertex_array: ver }
        }
    }

    /// Create a new initialized vertex array
    ///
    /// # Arguments
    /// * primitive_type - The type of the VertexArray
    /// * vertex_count - The maximal number of vertex
    pub fn new_init(primitive_type: PrimitiveType, vertex_count: u32) -> VertexArray {
        let ver = unsafe { sfVertexArray_create() };
        if ver.is_null() {
            panic!("sfVertexArray_create returned null.")
        } else {
            let mut tmp_vertex = VertexArray { vertex_array: ver };
            tmp_vertex.set_primitive_type(primitive_type);
            tmp_vertex.resize(vertex_count);
            tmp_vertex
        }
    }

    /// Return the vertex count of a vertex array
    ///
    /// Return the number of vertices in the array
    pub fn get_vertex_count(&self) -> u32 {
        unsafe { sfVertexArray_getVertexCount(self.vertex_array) as u32 }
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
    pub fn resize(&mut self, vertex_count: u32) {
        unsafe { sfVertexArray_resize(self.vertex_array, vertex_count as usize) }
    }

    /// Add a vertex to a vertex array array
    ///
    /// # Arguments
    /// * vertex - Vertex to add
    pub fn append(&mut self, vertex: &Vertex) {
        unsafe { sfVertexArray_append(self.vertex_array, vertex.0) }
    }

    /// Compute the bounding rectangle of a vertex array
    ///
    /// This function returns the axis-aligned rectangle that
    /// contains all the vertices of the array.
    ///
    /// Return the bounding rectangle of the vertex array
    pub fn get_bounds(&self) -> FloatRect {
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
            match primitive_type {
                primitive_type::Points => {
                    sfVertexArray_setPrimitiveType(self.vertex_array, sfPoints)
                }
                primitive_type::Lines => sfVertexArray_setPrimitiveType(self.vertex_array, sfLines),
                primitive_type::LineStrip => {
                    sfVertexArray_setPrimitiveType(self.vertex_array, sfLineStrip)
                }
                primitive_type::Triangles => {
                    sfVertexArray_setPrimitiveType(self.vertex_array, sfTriangles)
                }
                primitive_type::TriangleStrip => {
                    sfVertexArray_setPrimitiveType(self.vertex_array, sfTriangleStrip)
                }
                primitive_type::TriangleFan => {
                    sfVertexArray_setPrimitiveType(self.vertex_array, sfTriangleFan)
                }
                primitive_type::Quads => sfVertexArray_setPrimitiveType(self.vertex_array, sfQuads),
            }
        }
    }

    /// Get the type of primitives drawn by a vertex array
    ///
    /// Return the primitive type
    pub fn get_primitive_type(&self) -> PrimitiveType {
        match unsafe { sfVertexArray_getPrimitiveType(self.vertex_array) } {
            sfPoints => primitive_type::Points,
            sfLines => primitive_type::Lines,
            sfLineStrip => primitive_type::LineStrip,
            sfTriangles => primitive_type::Triangles,
            sfTriangleStrip => primitive_type::TriangleStrip,
            sfTriangleFan => primitive_type::TriangleFan,
            sfQuads => primitive_type::Quads,
        }
    }

    /// Get access to a vertex by its index
    ///
    /// This function doesn't check index, it must be in range
    /// [0, vertex count - 1]. The behaviour is undefined
    /// otherwise.
    ///
    /// # Arguments
    /// * index - Index of the vertex to get
    ///
    /// Return a mutable reference to the index-th vertex
    pub fn get_vertex(&mut self, index: u32) -> &mut Vertex {
        unsafe { &mut *(sfVertexArray_getVertex(self.vertex_array, index as usize) as *mut Vertex) }
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
        Self::new()
    }
}

impl Clone for VertexArray {
    /// Return a new Font or panic! if there is not enough memory
    fn clone(&self) -> VertexArray {
        let ver = unsafe { sfVertexArray_copy(self.vertex_array) };
        if ver.is_null() {
            panic!("Not enough memory to clone Font")
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

impl Index<u32> for VertexArray {
    type Output = Vertex;

    fn index(&self, rhs: u32) -> &Vertex {
        unsafe { &*(sfVertexArray_getVertex(self.vertex_array, rhs as usize) as *const Vertex) }
    }
}

impl Raw for VertexArray {
    type Raw = *mut sfVertexArray;
    fn raw(&self) -> Self::Raw {
        self.vertex_array
    }
}

impl Drawable for VertexArray {
    fn draw(&self, render_target: &mut RenderTarget, render_states: &mut RenderStates) {
        render_target.draw_vertex_array(self, render_states)
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { sfVertexArray_destroy(self.vertex_array) }
    }
}
