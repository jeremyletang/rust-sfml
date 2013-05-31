
pub use core::libc::{c_void};

pub mod csfml {
    
    pub use core::libc::{c_void};
    
    pub struct sfThread {
        This : *c_void
    }

    pub extern "C" {
        fn sfThread_create(function : *u8, userData : *c_void) -> *sfThread;
        fn sfThread_destroy(thread : *sfThread) -> ();
        fn sfThread_launch(thread : *sfThread) -> ();
        fn sfThread_wait(thread : *sfThread) -> ();
        fn sfThread_terminate(thread : *sfThread) -> ();
    }
}

extern fn threadable_function(userData : *c_void) -> () {
   // let data : Thread = unsafe {cast::transmute(userData as *Thread) };
   // let func : @fn(x : *c_void) =  data.func;
   // func(data.param);
}

pub struct ThreadableStructData<T> {
    func : @fn(x : T) -> (),
    param : T
}

pub struct Thread {
    priv thread : *csfml::sfThread,
    func : @fn(x : *c_void),
    param : *c_void
}

impl Thread {
    pub fn new(func : @fn(x : *c_void), params : *c_void) -> Thread {
        let mut t : Thread = Thread{thread : unsafe {ptr::null()}, func : func, param : params};
        t.thread = unsafe {csfml::sfThread_create(threadable_function, ptr::null())};
        return t;
    }
}
