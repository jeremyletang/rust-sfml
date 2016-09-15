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

//! Storage of audio sample
//!
//! A sound buffer holds the data of a sound, which is an array of audio samples.

use std::ffi::CString;

use raw_conv::{Raw, FromRaw};
use system::Time;

use csfml_audio_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;
use std::marker::PhantomData;

/// Storage of audio sample
///
/// A sound buffer holds the data of a sound, which is an array of audio samples.
pub struct SoundBuffer {
    sound_buffer: *mut ffi::sfSoundBuffer,
}

/// An immutable reference to a `SoundBuffer`.
#[derive(Clone, Copy)]
pub struct SoundBufferRef<'a> {
    sound_buffer: *const ffi::sfSoundBuffer,
    _borrow: PhantomData<&'a SoundBuffer>,
}

impl<'a> SoundBufferRef<'a> {
    /// Save a sound buffer to an audio file
    ///
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * filename - Path of the sound file to write
    ///
    /// Return true if saving succeeded, false if it faileds
    pub fn save_to_file(&self, filename: &str) -> bool {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        unsafe { ffi::sfSoundBuffer_saveToFile(self.sound_buffer, c_str.as_ptr()) }.to_bool()
    }

    /// Get the number of samples stored in a sound buffer
    ///
    /// The array of samples can be accessed with the
    /// get_samples function.
    ///
    /// Return the number of samples
    pub fn get_sample_count(&self) -> i64 {
        unsafe { ffi::sfSoundBuffer_getSampleCount(self.sound_buffer) as i64 }
    }

    /// Get the number of channels used by a sound buffer
    ///
    /// If the sound is mono then the number of channels will
    /// be 1, 2 for stereo, etc.
    ///
    /// Return the number of channels
    pub fn get_channel_count(&self) -> u32 {
        unsafe { ffi::sfSoundBuffer_getChannelCount(self.sound_buffer) as u32 }
    }

    /// Get the total duration of a sound buffer
    ///
    /// Return the sound duration
    pub fn get_duration(&self) -> Time {
        Time::from_raw(unsafe { ffi::sfSoundBuffer_getDuration(self.sound_buffer) })
    }

    /// Get the sample rate of a sound buffer
    ///
    /// The sample rate is the number of samples played per second.
    /// The higher, the better the quality (for example, 44100
    /// samples/s is CD quality).
    ///
    /// Return the sample rate (number of samples per second)
    pub fn get_sample_rate(&self) -> u32 {
        unsafe { ffi::sfSoundBuffer_getSampleRate(self.sound_buffer) as u32 }
    }
}

impl SoundBuffer {
    /// Create a new sound buffer and load it from a file
    ///
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * filename - Path of the sound file to load
    ///
    /// Return an option to a SoundBuffer object or None.
    pub fn new(filename: &str) -> Option<SoundBuffer> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let sound_buffer: *mut ffi::sfSoundBuffer =
            unsafe { ffi::sfSoundBuffer_createFromFile(c_str.as_ptr()) };
        if sound_buffer.is_null() {
            None
        } else {
            Some(SoundBuffer { sound_buffer: sound_buffer })
        }
    }
    /// Get an immutable reference to the `SoundBuffer`.
    pub fn get_ref(&self) -> SoundBufferRef {
        SoundBufferRef::from_raw(self.sound_buffer)
    }
}

impl Clone for SoundBuffer {
    fn clone(&self) -> Self {
        let sound_buffer = unsafe { ffi::sfSoundBuffer_copy(self.sound_buffer) };
        if sound_buffer.is_null() {
            panic!("Sound buffer is null");
        } else {
            SoundBuffer { sound_buffer: sound_buffer }
        }
    }
}

impl Raw for SoundBuffer {
    type Raw = *mut ffi::sfSoundBuffer;
    fn raw(&self) -> Self::Raw {
        self.sound_buffer
    }
}

impl<'a> Raw for SoundBufferRef<'a> {
    type Raw = *const ffi::sfSoundBuffer;
    fn raw(&self) -> Self::Raw {
        self.sound_buffer
    }
}

impl<'a> FromRaw for SoundBufferRef<'a> {
    fn from_raw(raw: Self::Raw) -> Self {
        SoundBufferRef {
            sound_buffer: raw,
            _borrow: PhantomData,
        }
    }
}

impl Drop for SoundBuffer {
    /// Destructor for class SoundBuffer. Destroy all the ressource.
    fn drop(&mut self) {
        unsafe {
            ffi::sfSoundBuffer_destroy(self.sound_buffer);
        }
    }
}
