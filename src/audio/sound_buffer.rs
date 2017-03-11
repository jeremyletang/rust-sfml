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

use std::ffi::CString;
use std::borrow::{Borrow, ToOwned};

use system::raw_conv::{Raw, FromRaw};
use system::Time;

use csfml_audio_sys as ffi;
use ext::sf_bool_ext::SfBoolExt;
use std::ops::Deref;
use inputstream::InputStream;
use std::io::{Read, Seek};


/// Storage for audio samples defining a sound.
///
/// A sound buffer holds the data of a sound, which is an array of audio samples.
///
/// A sample is a 16 bits signed integer that defines the amplitude of the sound at a given time.
/// The sound is then reconstituted by playing these samples at a high rate
/// (for example, 44100 samples per second is the standard rate used for playing CDs).
/// In short, audio samples are like texture pixels, and a `SoundBuffer` is similar to a `Texture`.
///
/// A sound buffer can be loaded from a file (see `from_file()` for the complete list of
/// supported formats), from memory, from a custom stream or directly from an array of samples.
/// It can also be saved back to a file.
///
/// Sound buffers alone are not very useful: they hold the audio data but cannot be played.
/// To do so, you need to use the `Sound` type, which provides functions to play/pause/stop
/// the sound as well as changing the way it is outputted (volume, pitch, 3D position, ...).
/// This separation allows more flexibility and better performances: indeed a `SoundBuffer` is
/// a heavy resource, and any operation on it is slow (often too slow for real-time applications).
/// On the other side, a `Sound` is a lightweight object, which can use the audio data of a sound
/// buffer and change the way it is played without actually modifying that data.
/// Note that it is also possible to bind several `Sound` instances to the same `SoundBuffer`.
///
/// It is important to note that the `Sound` instance doesn't copy the buffer that it uses,
/// it only keeps a reference to it. Thus, a `SoundBuffer` can not be destructed while it is
/// borrowed by a `Sound`.
///
/// # Usage example
///
/// ```no_run
/// use sfml::audio::{Sound, SoundBuffer, SoundSource};
///
/// // Load a new sound buffer
/// let buffer = SoundBuffer::from_file("sound.wav").unwrap();
///
/// // Create a sound source and bind it to the buffer
/// let mut sound_1 = Sound::with_buffer(&buffer);
///
/// // Play the sound
/// sound_1.play();
///
/// // Create another sound source bound to the same buffer
/// let mut sound_2 = Sound::with_buffer(&buffer);
///
/// // Play it with a higher pitch -- the first sound remains unchanged
/// sound_2.set_pitch(2.0);
/// sound_2.play();
/// ```
pub struct SoundBuffer {
    sound_buffer: *mut ffi::sfSoundBuffer,
}

impl Deref for SoundBuffer {
    type Target = SoundBufferRef;

    fn deref(&self) -> &SoundBufferRef {
        unsafe { &*(self.sound_buffer as *const SoundBufferRef) }
    }
}

/// A non-owning `SoundBuffer`.
pub enum SoundBufferRef {}

impl SoundBufferRef {
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
        unsafe { ffi::sfSoundBuffer_saveToFile(self as *const _ as _, c_str.as_ptr()) }.to_bool()
    }

    /// Get the number of samples stored in a sound buffer
    ///
    /// The array of samples can be accessed with the
    /// get_samples function.
    ///
    /// Return the number of samples
    pub fn sample_count(&self) -> i64 {
        unsafe { ffi::sfSoundBuffer_getSampleCount(self as *const _ as _) as i64 }
    }

    /// Get the number of channels used by a sound buffer
    ///
    /// If the sound is mono then the number of channels will
    /// be 1, 2 for stereo, etc.
    ///
    /// Return the number of channels
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::sfSoundBuffer_getChannelCount(self as *const _ as _) as u32 }
    }

    /// Get the total duration of a sound buffer
    ///
    /// Return the sound duration
    pub fn duration(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfSoundBuffer_getDuration(self as *const _ as _)) }
    }

    /// Get the sample rate of a sound buffer
    ///
    /// The sample rate is the number of samples played per second.
    /// The higher, the better the quality (for example, 44100
    /// samples/s is CD quality).
    ///
    /// Return the sample rate (number of samples per second)
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::sfSoundBuffer_getSampleRate(self as *const _ as _) as u32 }
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
    pub fn from_file(filename: &str) -> Option<SoundBuffer> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let sound_buffer: *mut ffi::sfSoundBuffer =
            unsafe { ffi::sfSoundBuffer_createFromFile(c_str.as_ptr()) };
        if sound_buffer.is_null() {
            None
        } else {
            Some(SoundBuffer { sound_buffer: sound_buffer })
        }
    }
    /// Load the sound buffer from a file in memory.
    pub fn from_memory(data: &[u8]) -> Option<Self> {
        let sound_buffer =
            unsafe { ffi::sfSoundBuffer_createFromMemory(data.as_ptr() as _, data.len()) };
        if sound_buffer.is_null() {
            None
        } else {
            Some(SoundBuffer { sound_buffer: sound_buffer })
        }
    }
    /// Load the sound buffer from a custom stream.
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<Self> {
        let mut stream = InputStream::new(stream);
        let buffer = unsafe { ffi::sfSoundBuffer_createFromStream(&mut stream.0) };
        if buffer.is_null() {
            None
        } else {
            Some(SoundBuffer { sound_buffer: buffer })
        }
    }
    /// Load the sound buffer from a slice of audio samples.
    ///
    /// The assumed format of the audio samples is 16 bits signed integer.
    pub fn from_samples(samples: &[i16], channel_count: u32, sample_rate: u32) -> Option<Self> {
        let buffer = unsafe {
            ffi::sfSoundBuffer_createFromSamples(samples.as_ptr(),
                                                 samples.len() as _,
                                                 channel_count,
                                                 sample_rate)
        };
        if buffer.is_null() {
            None
        } else {
            Some(SoundBuffer { sound_buffer: buffer })
        }
    }
}

impl Borrow<SoundBufferRef> for SoundBuffer {
    fn borrow(&self) -> &SoundBufferRef {
        &*self
    }
}

impl ToOwned for SoundBufferRef {
    type Owned = SoundBuffer;

    fn to_owned(&self) -> Self::Owned {
        let sound_buffer = unsafe { ffi::sfSoundBuffer_copy(self as *const _ as _) };
        if sound_buffer.is_null() {
            panic!("Sound buffer is null");
        } else {
            SoundBuffer { sound_buffer: sound_buffer }
        }
    }
}

impl Clone for SoundBuffer {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl Raw for SoundBuffer {
    type Raw = *mut ffi::sfSoundBuffer;
    fn raw(&self) -> Self::Raw {
        self.sound_buffer
    }
}

impl Drop for SoundBuffer {
    fn drop(&mut self) {
        unsafe {
            ffi::sfSoundBuffer_destroy(self.sound_buffer);
        }
    }
}
