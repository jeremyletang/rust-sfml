/*!
* Storage of audio sample
*
* A sound buffer holds the data of a sound, which is an array of audio samples.
*
*/

use system::time;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_char, size_t, c_void, c_uint};
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
    }
}

#[doc(hidden)]
pub struct SoundBuffer {
    priv soundBuffer : *csfml::sfSoundBuffer,
}

impl SoundBuffer {
    
    /**
    * Create a new sound buffer and load it from a file
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
    */
    pub fn new_copy(soundBuffer : SoundBuffer) -> SoundBuffer {
        SoundBuffer {soundBuffer :  unsafe {csfml::sfSoundBuffer_copy(soundBuffer.unwrap())}}
    }

    /**
    * Save a sound buffer to an audio file
    */
    pub fn save_to_file(&self, filename : ~str) -> bool {
        match do str::as_c_str(filename) |filename_buf| {
            unsafe {csfml::sfSoundBuffer_saveToFile(self.soundBuffer, filename_buf) }} {
            0 => false,
            _ => true
        }
    }
    
    /**
    * Get the number of samples stored in a sound buffer
    */
    pub fn get_sample_count(&self) -> i64 {
        unsafe {
            csfml::sfSoundBuffer_getSampleCount(self.soundBuffer) as i64
        }
    }

    /*
    * Get the number of channels used by a sound buffer
    */
    pub fn get_channel_count(&self) -> uint {
        unsafe {
            csfml::sfSoundBuffer_getChannelCount(self.soundBuffer) as uint
        }
    }

    /*
    * Get the total duration of a sound buffer
    */
    pub fn get_duration(&self) -> time::Time {
        time::Time::wrap(unsafe {csfml::sfSoundBuffer_getDuration(self.soundBuffer)})
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

