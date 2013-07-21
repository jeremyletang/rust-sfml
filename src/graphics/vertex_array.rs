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
* Define a set of one or more 2D primitives
*
*
*
*/

use std::libc::c_uint;
use std::ptr;

use traits::drawable::Drawable;
use traits::wrappable::Wrappable;
use graphics::vertex::Vertex;
use graphics::rect::FloatRect;
use graphics::primitive_type;
use graphics::primitive_type::PrimitiveType;
use graphics::render_window::RenderWindow;
use graphics::render_texture::RenderTexture;
use graphics::render_states::RenderStates;

#[doc(hidden)]
pub mod ffi {
    
    use std::libc::{c_uint, c_void};

    use graphics::vertex;
    use graphics::rect::FloatRect;
    
    pub type sfPrimitiveType = c_uint;
    pub static SFPOINTS : sfPrimitiveType = 0;
    pub static SFLINES : sfPrimitiveType = 1;
    pub static SFLINESSTRIP : sfPrimitiveType = 2;
    pub static SFTRIANGLES : sfPrimitiveType = 3;
    pub static SFTRIANGLESSTRIP : sfPrimitiveType = 4;
    pub static SFTRIANGLESFAN : sfPrimitiveType = 5;
    pub static SFQUADS : sfPrimitiveType = 6;

    pub struct sfVertexArray {
        This : *c_void
    }

    extern "C" {
        pub fn sfVertexArray_create() -> *sfVertexArray;
        pub fn sfVertexArray_copy(vertexArray : *sfVertexArray) -> *sfVertexArray;
        pub fn sfVertexArray_destroy(vertexArray : *sfVertexArray) -> ();
        pub fn sfVertexArray_getVertexCount(vertexArray : *sfVertexArray) -> c_uint;
        pub fn sfVertexArray_getVertex(vertexArray : *sfVertexArray, index : c_uint) -> *vertex::Vertex;
        pub fn sfVertexArray_clear(vertexArray : *sfVertexArray) -> ();
        pub fn sfVertexArray_resize(vertexArray : *sfVertexArray, vertexCount : c_uint) -> ();
        pub fn sfVertexArray_append(vertexArray : *sfVertexArray, vertex : vertex::Vertex) -> ();
        pub fn sfVertexArray_setPrimitiveType(vertexArray : *sfVertexArray, stype : sfPrimitiveType) -> ();
        pub fn sfVertexArray_getPrimitiveType(vertexArray : *sfVertexArray) -> sfPrimitiveType;
        pub fn sfVertexArray_getBounds(vertexArray : *sfVertexArray) -> FloatRect;
    }
}

#[doc(hidden)]
pub struct VertexArray {
    priv vertexArray : *ffi::sfVertexArray
}

impl VertexArray {
    /**
    * Create a new vertex array
    *
    * Return a new VertexArray object
    */
    pub fn new() -> Option<VertexArray> {
        let ver = unsafe { ffi::sfVertexArray_create() };
        if ptr::is_null(ver) {
            None
        }
        else {
            Some(VertexArray {
                vertexArray : ver
            })
        }    
    }

    /**
    * Copy an existing vertex array
    *
    * # Arguments
    * * vertexArray - Vertex array to copy
    *
    * Return the copied object
    */
    pub fn clone(&self) -> Option<VertexArray> {
        let ver = unsafe { ffi::sfVertexArray_copy(self.vertexArray) };
        if ptr::is_null(ver) {
            None
        }
        else {
            Some(VertexArray {
                vertexArray : ver
            })
        }
    }

    /**
    * Return the vertex count of a vertex array
    *
    * Return the number of vertices in the array
    */
    pub fn get_vertex_count(&self) -> uint {
        unsafe {
            ffi::sfVertexArray_getVertexCount(self.vertexArray) as uint
        }
    }

    /**
    * Clear a vertex array
    *
    * This function removes all the vertices from the array.
    * It doesn't deallocate the corresponding memory, so that
    * adding new vertices after clearing doesn't involve
    * reallocating all the memory.
    */
    pub fn clear(&mut self) -> () {
        unsafe {
            ffi::sfVertexArray_clear(self.vertexArray)
        }
    }
    
    /**
    * Resize the vertex array
    *
    * If vertexCount is greater than the current size, the previous
    * vertices are kept and new (default-constructed) vertices are
    * added.
    * If vertexCount is less than the current size, existing vertices
    * are removed from the array.
    *
    * # Arguments
    * * vertexCount - New size of the array (number of vertices)
    */
    pub fn resize(&mut self, vertexCount : uint) -> () {
        unsafe {
            ffi::sfVertexArray_resize(self.vertexArray, vertexCount as c_uint)
        }
    }

    /**
    * Add a vertex to a vertex array array
    *
    * # Arguments
    * * vertex - Vertex to add
    */
    pub fn append(&mut self, vertex : &Vertex) -> () {
        unsafe {
            ffi::sfVertexArray_append(self.vertexArray, *vertex)
        }
    }

    /**
    * Compute the bounding rectangle of a vertex array
    *
    * This function returns the axis-aligned rectangle that
    * contains all the vertices of the array.
    *
    * Return the bounding rectangle of the vertex array
    */
    pub fn get_bounds(&self) -> FloatRect {
        unsafe {
            ffi::sfVertexArray_getBounds(self.vertexArray)
        }
    }
    
    /**
    * Set the type of primitives of a vertex array
    *
    * This function defines how the vertices must be interpreted
    * when it's time to draw them:
    * As points
    * As lines
    * As triangles
    * As quads
    * The default primitive type is Points.
    *
    * # Arguments
    * * type - Type of primitive
    */
    pub fn set_primitive_type(&mut self, primitiveType : PrimitiveType) -> () {
        unsafe {
            match primitiveType {
                primitive_type::Points              => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFPOINTS),
                primitive_type::Lines               => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFLINES),
                primitive_type::LinesStrip          => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFLINESSTRIP),
                primitive_type::Triangles           => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFTRIANGLES),
                primitive_type::TrianglesStrip      => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFTRIANGLESSTRIP),
                primitive_type::TrianglesFan        => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFTRIANGLESFAN),
                primitive_type::Quads               => ffi::sfVertexArray_setPrimitiveType(self.vertexArray, ffi::SFQUADS)
            }
        }
    }
    
    /**
    * Get the type of primitives drawn by a vertex array
    *
    * Return the primitive type
    */
    pub fn get_primitive_type(&self) -> PrimitiveType {
        match unsafe { ffi::sfVertexArray_getPrimitiveType(self.vertexArray) } {
            ffi::SFPOINTS             => primitive_type::Points,
            ffi::SFLINES              => primitive_type::Lines,
            ffi::SFLINESSTRIP         => primitive_type::LinesStrip,
            ffi::SFTRIANGLES          => primitive_type::Triangles,
            ffi::SFTRIANGLESSTRIP     => primitive_type::TrianglesStrip,
            ffi::SFTRIANGLESFAN       => primitive_type::TrianglesFan,
            ffi::SFQUADS              => primitive_type::Quads,
            _                           => primitive_type::Points   
        }
    }

    /**
    * Get access to a vertex by its index
    *
    * This function doesn't check \a index, it must be in range
    * [0, vertex count - 1]. The behaviour is undefined
    * otherwise.
    *
    * # Arguments
    * * index - Index of the vertex to get
    *
    * Return the index-th vertex
    */
    pub fn get_vertex(&self, index : uint) -> Vertex {
        unsafe {
            *ffi::sfVertexArray_getVertex(self.vertexArray, index as c_uint)
        }
    }
}

impl Wrappable<*ffi::sfVertexArray> for VertexArray {
    pub fn wrap(vertexArray : *ffi::sfVertexArray) -> VertexArray {
        VertexArray {
            vertexArray : vertexArray
        }
    }

    pub fn unwrap(&self) -> *ffi::sfVertexArray {
        self.vertexArray
    }
}

#[doc(hidden)]
impl Drawable for VertexArray {
    pub fn draw_in_render_window(&self, renderWindow : &RenderWindow) -> () {
        renderWindow.draw_vertex_array(self)
    }

    pub fn draw_in_render_window_rs(&self, renderWindow : &RenderWindow, renderStates : &mut RenderStates) -> () {
        renderWindow.draw_vertex_array_rs(self, renderStates)
    }

    pub fn draw_in_render_texture(&self, renderTexture : &RenderTexture) -> () {
        renderTexture.draw_vertex_array(self)
    }

    pub fn draw_in_render_texture_rs(&self, renderTexture : &RenderTexture, renderStates : &mut RenderStates) -> () {
        renderTexture.draw_vertex_array_rs(self, renderStates)
    }
}

impl Drop for VertexArray {
    fn drop(&self) -> () {
        unsafe {
            ffi::sfVertexArray_destroy(self.vertexArray)
        }
    }
}
