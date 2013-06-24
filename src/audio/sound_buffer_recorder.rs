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

use audio::sound_buffer::SoundBuffer;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_uint, c_void};
    use audio::sound_buffer;
    
    pub struct sfSoundBufferRecorder {
        This : *c_void
    }

    pub extern "C" {
        fn sfSoundBufferRecorder_create() -> *sfSoundBufferRecorder;
        fn sfSoundBufferRecorder_destroy(soundBufferRecorder : *sfSoundBufferRecorder) -> ();
        fn sfSoundBufferRecorder_start(soundBufferRecorder : *sfSoundBufferRecorder, sampleRate : c_uint) -> ();
        fn sfSoundBufferRecorder_stop(soundBufferRecorder : *sfSoundBufferRecorder) -> ();
        fn sfSoundBufferRecorder_getSampleRate(soundBufferRecorder : *sfSoundBufferRecorder) -> c_uint;
        fn sfSoundBufferRecorder_getBuffer(soundBufferRecorder : *sfSoundBufferRecorder) -> *sound_buffer::csfml::sfSoundBuffer;
    }
}

#[doc(hidden)]
pub struct SoundBufferRecorder {
    priv soundBufferRecorder : *csfml::sfSoundBufferRecorder
}

impl SoundBufferRecorder {
    
    /**
    * Create a new sound buffer recorder
    *
    * Return a new option to SoundBufferRecorder object or None if failed
    */
    pub fn new() -> Option<SoundBufferRecorder> {
        let buffer : *csfml::sfSoundBufferRecorder;
        unsafe { buffer = csfml::sfSoundBufferRecorder_create()};
        if buffer == ptr::null() {
            return None;
        }
        Some(SoundBufferRecorder{soundBufferRecorder : buffer})
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
    pub fn start(&mut self, sampleRate : uint) -> () {
        unsafe {
            csfml::sfSoundBufferRecorder_start(self.soundBufferRecorder, sampleRate as c_uint)
        }
    }

    /**
    * Stop the capture of a sound recorder
    */
    pub fn stop(&mut self) -> () {
        unsafe {
            csfml::sfSoundBufferRecorder_stop(self.soundBufferRecorder)
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
            csfml::sfSoundBufferRecorder_getSampleRate(self.soundBufferRecorder) as uint
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
    pub fn get_buffer(&self) -> SoundBuffer {
        unsafe {
            SoundBuffer::wrap(csfml::sfSoundBufferRecorder_getBuffer(self.soundBufferRecorder))
        }
    }

}

impl Drop for SoundBufferRecorder {
    /**
    *   Destructor for class SoundBufferRecorder. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfSoundBufferRecorder_destroy(self.soundBufferRecorder);
        }
    }
}
