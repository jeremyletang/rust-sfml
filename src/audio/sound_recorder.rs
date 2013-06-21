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
* Abstract base class for capturing sound data
*
*  !!! DON'T WORK !!!
*
*/

use std::libc::{c_uint};

#[doc(hidden)]
pub mod csfml{

    pub use std::libc::{c_void, c_uint};
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
    *
    * The sampleRate parameter defines the number of audio samples
    * captured per second. The higher, the better the quality
    * (for example, 44100 samples/sec is CD quality).
    * This function uses its own thread so that it doesn't block
    * the rest of the program while the capture runs.
    * Please note that only one capture can happen at the same time.
    *
    * # Arguments
    * * ampleRate - Desired capture rate, in number of samples per second
    */
    pub fn start(&mut self, sampleRate : uint) -> () {
        unsafe {
            csfml::sfSoundRecorder_start(self.soundRecorder, sampleRate as c_uint)
        }
    }
    
    /**
    * Stop the capture of a sound recorder
    */
    pub fn stop(&mut self) -> () {
        unsafe {
            csfml::sfSoundRecorder_stop(self.soundRecorder)
        }
    }

    /**
    * Get the sample rate of a sound buffer recorder
    *
    * The sample rate defines the number of audio samples
    * captured per second. The higher, the better the quality
    * (for example, 44100 samples/sec is CD quality).
    *
    * Return the sample rate, in samples per second
    */
    pub fn get_sample_rate(&self) -> uint {
        unsafe {
            csfml::sfSoundRecorder_getSampleRate(self.soundRecorder) as uint
        }
    }
    
    /*
    * Check if the system supports audio capture
    *
    * This function should always be called before using
    * the audio capture features. If it returns false, then
    * any attempt to use sfSoundRecorder will fail.
    *
    * Return true if audio capture is supported, false otherwise
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
