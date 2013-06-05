/*!
* Abstract base class for capturing sound data
*
*  !!! DON'T WORK !!!
*
*/

use core::libc::{c_uint};

#[doc(hidden)]
pub mod csfml{

    pub use core::libc::{c_void, c_uint};
    use rsfml::sfTypes::{sfBool};

    pub struct sfSoundRecorder {
        This : *c_void
    }

    pub extern "C" {
        fn sfSoundRecorder_create(onStart : *u8, onProcess : *u8, onStop : *u8, data : *c_void) -> *sfSoundRecorder;
        fn sfSoundRecorder_destroy(soundRecorder : *sfSoundRecorder) -> ();
        fn sfSoundRecorder_start(soundRecorder : *sfSoundRecorder, sampleRate : c_uint) -> ();
        fn sfSoundRecorder_stop(soundRecorder : *sfSoundRecorder) -> ();
        fn sfSoundRecorder_getSampleRate(soundRecorder : *sfSoundRecorder) -> c_uint;
        fn sfSoundRecorder_isAvailable() -> sfBool; // static

    }
}

#[doc(hidden)]
pub struct SoundRecorder {
    priv soundRecorder : *csfml::sfSoundRecorder
}

impl SoundRecorder {
   /* pub fn new(onStart : @fn(data : *c_void), onProcess : @fn(sample : *i16, sampleSize : size_t, data : *c_void), onStop : @fn(data : *c_void), data : *c_void) -> SoundRecorder {
        SoundRecorder {
           soundRecorder : unsafe {csfml::sfSoundRecorder_create(onStart, cast::transmute(onProcess as *u8), cast::transmute(onStop as *u8), data)} 
        }
    }*/
    
    /**
    * Start the capture of a sound recorder
    */
    pub fn start(&self, sampleRate : uint) -> () {
        unsafe {
            csfml::sfSoundRecorder_start(self.soundRecorder, sampleRate as c_uint)
        }
    }
    
    /**
    * Stop the capture of a sound recorder
    */
    pub fn stop(&self) -> () {
        unsafe {
            csfml::sfSoundRecorder_stop(self.soundRecorder)
        }
    }

    /**
    * Get the sample rate of a sound recorder
    */
    pub fn get_sample_rate(&self) -> uint {
        unsafe {
            csfml::sfSoundRecorder_getSampleRate(self.soundRecorder) as uint
        }
    }
    
    /*
    * Check if the system supports audio capture
    */
    pub fn is_available() -> bool {
        match unsafe {csfml::sfSoundRecorder_isAvailable()} {
            0 => false,
            _ => true
        }
    }

}


impl Drop for SoundRecorder {
    /**
    *   Destructor for class SoundRecorder. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfSoundRecorder_destroy(self.soundRecorder);
        }
    }
}
