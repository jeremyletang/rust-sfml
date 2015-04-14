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

//! Define a set of one or more 2D primitives

use libc::c_uint;
use std::mem;
use std::ops::Index;

use traits::{Drawable, Wrappable};
use graphics::{Vertex, FloatRect, primitive_type, PrimitiveType, RenderTarget, RenderStates, rc};

use ffi::graphics::vertex_array as ffi;

/// Define a set of one or more 2D primitives
pub struct VertexArray {
    vertex_array: *mut ffi::sfVertexArray
}

/// An iterator over the vertice of a VertexArray
pub struct Vertices {
    vertex_array: *mut ffi::sfVertexArray,
    pos: u32
}

impl VertexArray {
    /// Create a new vertex array
    ///
    /// Return Some(VertexArray) or None
    pub fn new() -> Option<VertexArray> {
        let ver = unsafe { ffi::sfVertexArray_create() };
        if ver.is_null() {
            None
        } else {
            Some(VertexArray {
                    vertex_array: ver
                })
        }
    }

    /// Create a new initialized vertex array
    ///
    /// # Arguments
    /// * primitive_type - The type of the VertexArray
    /// * vertex_count - The maximal number of vertex
    ///
    /// Return Some(VertexArray) or None
    pub fn new_init(primitive_type: PrimitiveType,
                    vertex_count: u32) -> Option<VertexArray> {
        let ver = unsafe { ffi::sfVertexArray_create() };
        if ver.is_null() {
            None
        } else {
            let mut tmp_vertex = VertexArray {
                vertex_array: ver
            };
            tmp_vertex.set_primitive_type(primitive_type);
            tmp_vertex.resize(vertex_count);
            Some(tmp_vertex)
        }
    }

    /// Copy an existing vertex array
    ///
    /// # Arguments
    /// * vertexArray - Vertex array to copy
    ///
    /// Return Some(VertexArray) or None
    pub fn clone_opt(&self) -> Option<VertexArray> {
        let ver = unsafe { ffi::sfVertexArray_copy(self.vertex_array) };
        if ver.is_null() {
            None
        } else {
            Some(VertexArray {
                    vertex_array: ver
                })
        }
    }

    /// Return the vertex count of a vertex array
    ///
    /// Return the number of vertices in the array
    pub fn get_vertex_count(&self) -> u32 {
        unsafe {
            ffi::sfVertexArray_getVertexCount(self.vertex_array) as u32
        }
    }

    /// Clear a vertex array
    ///
    /// This function removes all the vertices from the array.
    /// It doesn't deallocate the corresponding memory, so that
    /// adding new vertices after clearing doesn't involve
    /// reallocating all the memory.
    pub fn clear(&mut self) -> () {
        unsafe {
            ffi::sfVertexArray_clear(self.vertex_array)
        }
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
    pub fn resize(&mut self, vertex_count: u32) -> () {
        unsafe {
            ffi::sfVertexArray_resize(self.vertex_array, vertex_count as c_uint)
        }
    }

    /// Add a vertex to a vertex array array
    ///
    /// # Arguments
    /// * vertex - Vertex to add
    pub fn append(&mut self, vertex: &Vertex) -> () {
        unsafe {
            ffi::sfVertexArray_append(self.vertex_array, *vertex)
        }
    }

    /// Compute the bounding rectangle of a vertex array
    ///
    /// This function returns the axis-aligned rectangle that
    /// contains all the vertices of the array.
    ///
    /// Return the bounding rectangle of the vertex array
    pub fn get_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfVertexArray_getBounds(self.vertex_array)
        }
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
    pub fn set_primitive_type(&mut self, primitive_type: PrimitiveType) -> () {
        unsafe {
            match primitive_type {
                primitive_type::Points              =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFPOINTS),
                primitive_type::Lines               =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFLINES),
                primitive_type::LinesStrip          =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFLINESSTRIP),
                primitive_type::Triangles           =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFTRIANGLES),
                primitive_type::TrianglesStrip      =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFTRIANGLESSTRIP),
                primitive_type::TrianglesFan        =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFTRIANGLESFAN),
                primitive_type::Quads               =>
                    ffi::sfVertexArray_setPrimitiveType(self.vertex_array,
                                                        ffi::SFQUADS)
            }
        }
    }

    /// Get the type of primitives drawn by a vertex array
    ///
    /// Return the primitive type
    pub fn get_primitive_type(&self) -> PrimitiveType {
        match unsafe { ffi::sfVertexArray_getPrimitiveType(self.vertex_array) } {
            ffi::SFPOINTS             => primitive_type::Points,
            ffi::SFLINES              => primitive_type::Lines,
            ffi::SFLINESSTRIP         => primitive_type::LinesStrip,
            ffi::SFTRIANGLES          => primitive_type::Triangles,
            ffi::SFTRIANGLESSTRIP     => primitive_type::TrianglesStrip,
            ffi::SFTRIANGLESFAN       => primitive_type::TrianglesFan,
            ffi::SFQUADS              => primitive_type::Quads,
            _                         => primitive_type::Points
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
    pub fn get_vertex(&self, index: u32) -> &mut Vertex {
        unsafe {
            mem::transmute(ffi::sfVertexArray_getVertex(self.vertex_array,
                                                         index as c_uint))
        }
    }

    /// Return an immutable iterator over all the vertice contained by the VertexArray
    pub fn vertices(&self) -> Vertices {
        Vertices {
            vertex_array: self.vertex_array.clone(),
            pos: 0
        }
    }
}

impl Clone for VertexArray {
    /// Return a new Font or panic! if there is not enough memory
    fn clone(&self) -> VertexArray {
        let ver = unsafe { ffi::sfVertexArray_copy(self.vertex_array) };
        if ver.is_null() {
            panic!("Not enough memory to clone Font")
        } else {
            VertexArray {
                vertex_array: ver
            }
        }
    }
}

impl<'s> Iterator for Vertices {
    type Item = &'s Vertex;

    fn next(&mut self) -> Option<&'s Vertex> {
        let point_count =
            unsafe { ffi::sfVertexArray_getVertexCount(self.vertex_array) as u32 };
        if self.pos == point_count {
            None
        } else {
            self.pos += 1;
            unsafe {
                mem::transmute(ffi::sfVertexArray_getVertex(self.vertex_array,
                                                             self.pos as c_uint))
            }
        }
    }
}

impl Index<u32> for VertexArray {
    type Output = Vertex;

    fn index<'s>(&'s self, _rhs: u32) -> &'s Vertex {
        unsafe {
            mem::transmute::<*const Vertex, &'s Vertex>
                (ffi::sfVertexArray_getVertex(self.vertex_array,
                                              _rhs as c_uint) as *const Vertex)
        }
    }
}

impl Wrappable<*mut ffi::sfVertexArray> for VertexArray {
    fn wrap(vertex_array: *mut ffi::sfVertexArray) -> VertexArray {
        VertexArray {
            vertex_array: vertex_array
        }
    }

    fn unwrap(&self) -> *mut ffi::sfVertexArray {
        self.vertex_array
    }
}

impl Drawable for VertexArray {
    fn draw<RT: RenderTarget>(&self, render_target: &mut RT) -> () {
        render_target.draw_vertex_array(self)
    }

    fn draw_rs<RT: RenderTarget>(&self,
                                 render_target: &mut RT,
                                 render_states: &mut RenderStates) -> () {
        render_target.draw_vertex_array_rs(self, render_states)
    }

    fn draw_rs_rc<RT: RenderTarget>(&self,
                                    render_target: &mut RT,
                                    render_states: &mut rc::RenderStates) -> () {
        render_target.draw_vertex_array_rs_rc(self, render_states)
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) -> () {
        unsafe {
            ffi::sfVertexArray_destroy(self.vertex_array)
        }
    }
}
