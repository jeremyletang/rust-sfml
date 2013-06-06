/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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
use core::libc::{c_uint};
use audio::sound_buffer;

#[doc(hidden)]
pub mod csfml {

    use core::libc::{c_uint, c_void};
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
    * Start the capture of a sound recorder recorder 
    */
    pub fn start(&self, sampleRate : uint) -> () {
        unsafe {
            csfml::sfSoundBufferRecorder_start(self.soundBufferRecorder, sampleRate as c_uint)
        }
    }

    /**
    * Stop the capture of a sound recorder
    */
    pub fn stop(&self) -> () {
        unsafe {
            csfml::sfSoundBufferRecorder_stop(self.soundBufferRecorder)
        }
    }

    /**
    * Get the sample rate of a sound buffer recorder
    */
    pub fn get_sample_rate(&self) -> uint {
        unsafe {
            csfml::sfSoundBufferRecorder_getSampleRate(self.soundBufferRecorder) as uint
        }
    }

    /**
    * Get the sound buffer containing the captured audio data
    */
    pub fn get_buffer(&self) -> sound_buffer::SoundBuffer {
        unsafe {
            sound_buffer::SoundBuffer::wrap(csfml::sfSoundBufferRecorder_getBuffer(self.soundBufferRecorder))
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
