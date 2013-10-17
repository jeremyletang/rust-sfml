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
* Store captured audio data in sound Buffer
*
* SoundBufferRecorder allows to access a recorded sound through a sf::SoundBuffer, so that it can be played, saved to a file, etc.
*
*/

use std::libc::{c_uint};
use std::ptr;

use traits::wrappable::Wrappable;
use super::sound_buffer::SoundBuffer;

#[doc(hidden)]
pub mod ffi {

    use std::libc::{c_uint, c_void};
    use audio::sound_buffer;
    
    pub struct sfSoundBufferRecorder {
        This : *c_void
    }

    extern "C" {
        pub fn sfSoundBufferRecorder_create() -> *sfSoundBufferRecorder;
        pub fn sfSoundBufferRecorder_destroy(soundBufferRecorder : *sfSoundBufferRecorder) -> ();
        pub fn sfSoundBufferRecorder_start(soundBufferRecorder : *sfSoundBufferRecorder, sampleRate : c_uint) -> ();
        pub fn sfSoundBufferRecorder_stop(soundBufferRecorder : *sfSoundBufferRecorder) -> ();
        pub fn sfSoundBufferRecorder_getSampleRate(soundBufferRecorder : *sfSoundBufferRecorder) -> c_uint;
        pub fn sfSoundBufferRecorder_getBuffer(soundBufferRecorder : *sfSoundBufferRecorder) -> *sound_buffer::ffi::sfSoundBuffer;
    }
}

pub struct SoundBufferRecorder {
    #[doc(hidden)]
    priv sound_buffer_recorder : *ffi::sfSoundBufferRecorder
}

impl SoundBufferRecorder {
    
    /**
    * Create a new sound buffer recorder
    *
    * Return a new option to SoundBufferRecorder object or None if failed
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn new() -> Option<SoundBufferRecorder> {
        let buffer = unsafe { ffi::sfSoundBufferRecorder_create() };
        if ptr::is_null(buffer) {
            None
        }
        else {
            Some(SoundBufferRecorder{
                sound_buffer_recorder : buffer
            })
        }
    }
    
    /**
    * Start the capture of a sound buffer recorder 
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn start(&mut self, sampleRate : uint) -> () {
        unsafe {
            ffi::sfSoundBufferRecorder_start(self.sound_buffer_recorder, sampleRate as c_uint)
        }
    }

    /**
    * Stop the capture of a sound recorder
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn stop(&mut self) -> () {
        unsafe {
            ffi::sfSoundBufferRecorder_stop(self.sound_buffer_recorder)
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
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_sample_rate(&self) -> uint {
        unsafe {
            ffi::sfSoundBufferRecorder_getSampleRate(self.sound_buffer_recorder) as uint
        }
    }

    /**
    * Get the sound buffer containing the captured audio data
    *
    * The sound buffer is valid only after the capture has ended.
    * This function provides a read-only access to the internal
    * sound buffer, but it can be copied if you need to
    * make any modification to it.
    *
    * Return Read-only access to the sound buffer
    */
    #[fixed_stack_segment] #[inline(never)]
    pub fn get_buffer(&self) -> Option<SoundBuffer> {
        let buff = unsafe { ffi::sfSoundBufferRecorder_getBuffer(self.sound_buffer_recorder) };
        if ptr::is_null(buff) {
            None
        }
        else {
            Some(Wrappable::wrap(buff))
        }
    }

}

impl Drop for SoundBufferRecorder {
    /**
    *   Destructor for class SoundBufferRecorder. Destroy all the ressource.
    */
    #[fixed_stack_segment] #[inline(never)]
    fn drop(&mut self) {
        unsafe {
            ffi::sfSoundBufferRecorder_destroy(self.sound_buffer_recorder);
        }
    }
}
