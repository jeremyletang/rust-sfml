/*!
 * Drawing context
 *
 * Class holding a valid drawing context.
 */

use rsfml::sfTypes::{sfBool};

#[doc(hidden)]
pub mod csfml {

    use core::libc::{c_void};
    use rsfml::sfTypes::{sfBool};

    pub struct sfContext {
        This: *c_void
    }

    pub extern "C" {
        fn sfContext_create() -> *sfContext;
        fn sfContext_destroy(context : *sfContext) -> ();
        fn sfContext_setActive(context : *sfContext, active : sfBool) -> ();
    }
}

#[doc(hidden)]
pub struct Context {
    priv cont : *csfml::sfContext
}

impl Context {

    /**
    * Constructor for class Context. Create the context and active it.
    */
    pub fn new() -> Context {
        Context{cont : unsafe{csfml::sfContext_create()}}
    }

    /**
    * Set if the constructor is active or not.
    */
    pub fn set_active(&self, active : bool) -> () {
        let act : sfBool = 
            match active {
                true    => 0,
                _       => 1
            };
        unsafe {
            csfml::sfContext_setActive(self.cont, act);
        }
    }

}

impl Drop for Context {
    /*
    *   Destructor for class Context. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfContext_destroy(self.cont);
        }
    }
}
