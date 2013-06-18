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
* Storage of audio sample
*
* A sound buffer holds the data of a sound, which is an array of audio samples.
*
*/

use system::time;
use std::ptr;
use std::str;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_char, size_t, c_void, c_uint};
    use system::time;
    use rsfml::sfTypes::{sfBool};

    pub struct sfSoundBuffer {
        This : *c_void
    }

    pub extern "C" {
        fn sfSoundBuffer_createFromFile(filename : *c_char) -> *sfSoundBuffer;
        fn sfSoundBuffer_copy(soundBuffer : *sfSoundBuffer) -> *sfSoundBuffer;
        fn sfSoundBuffer_destroy(soundBuffer : *sfSoundBuffer) -> ();
        fn sfSoundBuffer_saveToFile(soundBuffer : *sfSoundBuffer, filename : *c_char) -> sfBool;
       // fn sfSoundBuffer_getSamples(soundBuffer : *sfSoundBuffer) -> *i16;
        fn sfSoundBuffer_getSampleCount(soundBuffer : *sfSoundBuffer) -> size_t;
        fn sfSoundBuffer_getChannelCount(soundBuffer : *sfSoundBuffer) -> c_uint;
        fn sfSoundBuffer_getDuration(soundBuffer : *sfSoundBuffer) -> time::csfml::sfTime;
        fn sfSoundBuffer_getSampleRate(soundBuffer : *sfSoundBuffer) -> c_uint;
    }
}

#[doc(hidden)]
pub struct SoundBuffer {
    priv soundBuffer : *csfml::sfSoundBuffer,
}

impl SoundBuffer {
    
    /**
    * Create a new sound buffer and load it from a file
    *
    * Here is a complete list of all the supported audio formats:
    * ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    * w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    *
    * # Arguments
    * * filename - Path of the sound file to load
    *
    * Return an option to a SoundBuffer object or None.
    */
    pub fn new(filename : ~str) -> Option<SoundBuffer> {
        let mut soundBuffer : *csfml::sfSoundBuffer = ptr::null();
        do str::as_c_str(filename) |filename_buf| {
            unsafe { soundBuffer = csfml::sfSoundBuffer_createFromFile(filename_buf); }
        };
        if soundBuffer == ptr::null() {
            return None;
        }
        Some(SoundBuffer{soundBuffer : soundBuffer})
    }

    /**
    * Create a new sound buffer by copying an existing one
    *
    * # Arguments
    * * soundBuffer - Sound buffer to copy
    */
    pub fn new_copy(soundBuffer : SoundBuffer) -> SoundBuffer {
        SoundBuffer {soundBuffer :  unsafe {csfml::sfSoundBuffer_copy(soundBuffer.unwrap())}}
    }

    /**
    * Save a sound buffer to an audio file
    *
    * Here is a complete list of all the supported audio formats:
    * ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    * w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    *
    * # Arguments
    * * filename - Path of the sound file to write
    *
    * Return true if saving succeeded, false if it faileds
    */
    pub fn save_to_file(&self, filename : ~str) -> bool {
        match do str::as_c_str(filename) |filename_buf| {
            unsafe {csfml::sfSoundBuffer_saveToFile(self.soundBuffer, filename_buf) }} {
            0 => false,
            _ => true
        }
    }
    
    /**
    * Get the array of audio samples stored in a sound buffer
    *
    * The format of the returned samples is 16 bits signed integer
    * (sfInt16). The total number of samples in this array
    * is given by the get_cample_count function.
    *
    * return Read-only pointer to the array of sound samples
    */


    /**
    * Get the number of samples stored in a sound buffer
    *
    * The array of samples can be accessed with the
    * get_samples function.
    * 
    * Return the number of samples
    */
    pub fn get_sample_count(&self) -> i64 {
        unsafe {
            csfml::sfSoundBuffer_getSampleCount(self.soundBuffer) as i64
        }
    }

    /*
    * Get the number of channels used by a sound buffer
    *
    * If the sound is mono then the number of channels will
    * be 1, 2 for stereo, etc.
    *
    * Return the number of channels
    */
    pub fn get_channel_count(&self) -> uint {
        unsafe {
            csfml::sfSoundBuffer_getChannelCount(self.soundBuffer) as uint
        }
    }

    /*
    * Get the total duration of a sound buffer
    *
    * Return the sound duration
    */
    pub fn get_duration(&self) -> time::Time {
        time::Time::wrap(unsafe {csfml::sfSoundBuffer_getDuration(self.soundBuffer)})
    }

    /**
    * Get the sample rate of a sound buffer
    *
    * The sample rate is the number of samples played per second.
    * The higher, the better the quality (for example, 44100
    * samples/s is CD quality).
    *
    * Return the sample rate (number of samples per second)
    */
    pub fn get_sample_rate(&self) -> uint {
        unsafe {
            csfml::sfSoundBuffer_getSampleRate(self.soundBuffer) as uint
        }
    }

    #[doc(hidden)]
    pub fn wrap(buffer : *csfml::sfSoundBuffer) -> SoundBuffer {
        SoundBuffer {soundBuffer : buffer}
    }

    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfSoundBuffer {
        self.soundBuffer
    }

}

impl Drop for SoundBuffer {
    /**
    *   Destructor for class SoundBuffer. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfSoundBuffer_destroy(self.soundBuffer);
        }
    }
}

