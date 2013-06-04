
use graphics::vertex::Vertex;
use core::libc::c_uint;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_uint, c_void};
    use graphics::vertex;

    pub struct sfVertexArray {
        This : *c_void
    }

    pub extern "C" {
        fn sfVertexArray_create() -> *sfVertexArray;
        fn sfVertexArray_copy(vertexArray : *sfVertexArray) -> *sfVertexArray;
        fn sfVertexArray_destroy(vertexArray : *sfVertexArray) -> ();
        fn sfVertexArray_getVertexCount(vertexArray : *sfVertexArray) -> c_uint;
        //fn sfVertexArray_getVertex(vertexArray : *sfVertexArray, index : c_uint) -> *csfml::sfVertex;
        fn sfVertexArray_clear(vertexArray : *sfVertexArray) -> ();
        fn sfVertexArray_resize(vertexArray : *sfVertexArray, vertexCount : c_uint) -> ();
        fn sfVertexArray_append(vertexArray : *sfVertexArray, vertex : vertex::Vertex) -> ();
        // fn sfVertexArray_setPrimitiveType(vertexArray : *sfVertexArray, stype : sfPrimitiveType) -> ();
        // fn sfVertexArray_getPrimitiveType(vertexArray : *sfVertexArray) -> sfPrimitiveType;
        //fn sfVertexArray_getBounds(vertexArray : *sfVertexArray) -> sfFloatRect;
    }
}

#[doc(hidden)]
pub struct VertexArray {
    priv vertexArray : *csfml::sfVertexArray
}

impl VertexArray {
    pub fn new() -> VertexArray {
        unsafe {
            VertexArray { vertexArray : csfml::sfVertexArray_create()}
        }
    }

    pub fn new_copy(vertexArray : &VertexArray) -> VertexArray {
        unsafe {
            VertexArray { vertexArray : csfml::sfVertexArray_copy(vertexArray.unwrap())}
        }
    }

    pub fn get_vertex_count(&self) -> uint {
        unsafe {
            csfml::sfVertexArray_getVertexCount(self.vertexArray) as uint
        }
    }

    pub fn clear(&self) -> () {
        unsafe {
            csfml::sfVertexArray_clear(self.vertexArray)
        }
    }
    
    pub fn resize(&self, vertexCount : uint) -> () {
        unsafe {
            csfml::sfVertexArray_resize(self.vertexArray, vertexCount as c_uint)
        }
    }

    pub fn append(&self, vertex : &Vertex) -> () {
        unsafe {
            csfml::sfVertexArray_append(self.vertexArray, *vertex)
        }
    }
    
    pub fn wrap(vertexArray : *csfml::sfVertexArray) -> VertexArray {
        VertexArray {vertexArray : vertexArray}
    }

    pub fn unwrap(&self) -> *csfml::sfVertexArray {
        self.vertexArray
    }
}

impl Drop for VertexArray {
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfVertexArray_destroy(self.vertexArray)
        }
    }
}