/*!
* Handle concurrents access
*
* Blocks concurrent access to shared resources from multiple threads.
*
*/

#[doc(hidden)]
pub mod csfml {
    
    pub use core::libc::{c_void};
    
    pub struct sfMutex {
        This : *c_void
    }
    
    pub extern "C" {
        fn sfMutex_create() -> *sfMutex;
        fn sfMutex_destroy(mutex : *sfMutex) -> ();
        fn sfMutex_lock(mutex : *sfMutex) -> ();
        fn sfMutex_unlock(mutex : *sfMutex) -> ();
    }
}

#[doc(hidden)]
pub struct Mutex {
    priv mutex : *csfml::sfMutex
}

impl Mutex {

    /**
    * Create a new Mutex
    */
    pub fn new() -> Mutex {
        Mutex {mutex : unsafe {csfml::sfMutex_create()}}
    }

    
    /**
    * Lock a Mutex
    */
    pub fn lock(&self) -> () {
        unsafe {
            csfml::sfMutex_lock(self.mutex)
        }
    }
    
    /**
    * Unlock a Mutex
    */
    pub fn unlock(&self) -> () {
        unsafe {
            csfml::sfMutex_unlock(self.mutex)
        }
    }
}

impl Drop for Mutex {

    /**
    * Destroy a Mutex
    */
    fn finalize(&self) -> () {
        unsafe {
            csfml::sfMutex_destroy(self.mutex)
        }
    }
}