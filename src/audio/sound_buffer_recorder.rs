// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Store captured audio data in sound Buffer
//!
//! `SoundBufferRecorder` allows to access a recorded sound through a `SoundBuffer`,
//! so that it can be played, saved to a file, etc.

use libc::c_uint;

use raw_conv::FromRaw;
use audio::sound_buffer::SoundBufferRef;

use csfml_audio_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;

/// Store captured audio data in sound Buffer
///
/// `SoundBufferRecorder` allows to access a recorded sound through a `SoundBuffer`,
/// so that it can be played, saved to a file, etc.
pub struct SoundBufferRecorder {
    sound_buffer_recorder: *mut ffi::sfSoundBufferRecorder,
}

impl SoundBufferRecorder {
    /// Create a new sound buffer recorder
    pub fn new() -> SoundBufferRecorder {
        let buffer = unsafe { ffi::sfSoundBufferRecorder_create() };
        if buffer.is_null() {
            panic!("sfSoundBufferRecorder_create returned null.")
        } else {
            SoundBufferRecorder { sound_buffer_recorder: buffer }
        }
    }

    /// Start the capture of a sound buffer recorder
    ///
    /// The sampleRate parameter defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the capture runs.
    /// Please note that only one capture can happen at the same time.
    ///
    /// # Arguments
    /// * sample_rate - Desired capture rate, in number of samples per second
    pub fn start(&mut self, sample_rate: u32) {
        unsafe {
            ffi::sfSoundBufferRecorder_start(self.sound_buffer_recorder, sample_rate as c_uint)
        }
    }

    /// Stop the capture of a sound recorder
    pub fn stop(&mut self) {
        unsafe { ffi::sfSoundBufferRecorder_stop(self.sound_buffer_recorder) }
    }

    /// Get the sample rate of a sound buffer recorder
    ///
    /// The sample rate defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    ///
    /// Return the sample rate, in samples per second
    pub fn get_sample_rate(&self) -> u32 {
        unsafe { ffi::sfSoundBufferRecorder_getSampleRate(self.sound_buffer_recorder) as u32 }
    }

    /// Get the sound buffer containing the captured audio data
    ///
    /// The sound buffer is valid only after the capture has ended.
    /// This function provides a read-only access to the internal
    /// sound buffer, but it can be copied if you need to
    /// make any modification to it.
    ///
    /// Return Read-only access to the sound buffer
    pub fn get_buffer(&self) -> SoundBufferRef {
        let buff = unsafe { ffi::sfSoundBufferRecorder_getBuffer(self.sound_buffer_recorder) };
        if buff.is_null() {
            panic!("sound_buffer_recorder::get_buffer: buffer was null");
        } else {
            SoundBufferRef::from_raw(buff)
        }
    }

    /// Check if the system supports audio capture
    ///
    /// This function should always be called before using
    /// the audio capture features. If it returns false, then
    /// any attempt to use sfSoundRecorder will fail.
    ///
    /// Return true if audio capture is supported, false otherwise
    pub fn is_available() -> bool {
        unsafe { ffi::sfSoundRecorder_isAvailable() }.to_bool()
    }
}

impl Default for SoundBufferRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SoundBufferRecorder {
    fn drop(&mut self) {
        unsafe {
            ffi::sfSoundBufferRecorder_destroy(self.sound_buffer_recorder);
        }
    }
}
