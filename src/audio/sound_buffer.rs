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

use libc::{size_t, c_uint};
use std::ffi::CString;

use system::Time;

use ffi::{Foreign, ForeignHolder};
use ffi::audio as ffi;

/// Storage for audio samples defining a sound.
///
/// A sound buffer holds the data of a sound, which is an array of audio
/// samples.
///
/// A sample is a 16 bits signed integer that defines the amplitude of the sound
/// at a given time. The sound is then restituted by playing these samples at a
/// high rate (for example, 44100 samples per second is the standard rate used
/// for playing CDs). In short, audio samples are like texture pixels, and a
/// `SoundBuffer` is similar to a `Texture`.
///
/// A sound buffer can be loaded from a file, from memory, or directly from an
/// array of samples. It can also be saved back to a file. The supported formats
/// are: ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam, w64, mat4,
/// mat5, pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
///
/// Sound buffers alone are not very useful: they hold the audio data but cannot
/// be played. To do so, you need to use the `Sound` type, which provides
/// functions to play/pause/stop the sound as well as changing the way it is
/// outputted (volume, pitch, 3D position, ...). This separation allows more
/// flexibility and better performances: indeed a `SoundBuffer` is a heavy
/// resource, and any operation on it is slow (often too slow for real-time
/// applications). On the other side, a `Sound` is a lightweight object, which
/// can use the audio data of a sound buffer and change the way it is played
/// without actually modifying that data. Note that it is also possible to bind
/// several `Sound` instances to the same `SoundBuffer`.
pub struct SoundBuffer(Foreign<ffi::sfSoundBuffer>);

impl SoundBuffer {
    /// Create a new sound buffer and load it from a file.
	///
	/// Returns Some(SoundBuffer) or None on failure.
    pub fn new(filename: &str) -> Option<SoundBuffer> {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return None
		};
        unsafe {
            Foreign::new(ffi::sfSoundBuffer_createFromFile(c_str.as_ptr()))
        }.map(SoundBuffer)
    }

	/// Create a new sound buffer and load it from a file already in memory.
	///
	/// Returns Some(SoundBuffer) or None on failure.
	pub fn new_from_memory(contents: &[u8]) -> Option<SoundBuffer> {
		unsafe {
			Foreign::new(ffi::sfSoundBuffer_createFromMemory(contents.as_ptr(), contents.len() as size_t))
		}.map(SoundBuffer)
	}

	/// Create a new sound buffer from an array of audio samples.
	///
	/// The assumed format of the audio samples is 16-bit signed integers.
	/// The number of channels (1 = mono, 2 = stereo, ...) and the sample rate
	/// (number of samples to play per second) must be specified.
	///
	/// Returns Some(SoundBuffer) or None on failure.
	pub fn new_from_samples(samples: &[i16], channel_count: u32, sample_rate: u32) -> Option<SoundBuffer> {
		unsafe {
			Foreign::new(ffi::sfSoundBuffer_createFromSamples(samples.as_ptr(), samples.len() as size_t, channel_count as c_uint, sample_rate as c_uint))
		}.map(SoundBuffer)
	}

	fn raw(&self) -> &ffi::sfSoundBuffer { self.0.as_ref() }
	#[doc(hidden)]
	pub fn unwrap(&self) -> &ffi::sfSoundBuffer { self.raw() }

    /// Create a new sound buffer by copying an existing one.
    ///
	/// Returns Some(SoundBuffer) or None.
    pub fn clone_opt(&self) -> Option<SoundBuffer> {
        unsafe {
			Foreign::new(ffi::sfSoundBuffer_copy(self.raw()))
		}.map(SoundBuffer)
    }

    /// Save a sound buffer to an audio file.
	///
	/// Returns true if the save succeeded, or false if it failed.
    pub fn save_to_file(&self, filename: &str) -> bool {
        let c_str = match CString::new(filename.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return false
		};
        unsafe {
			ffi::sfSoundBuffer_saveToFile(self.raw(), c_str.as_ptr())
		}.to_bool()
    }

    /// Get the number of samples stored in the buffer.
    ///
    /// The array of samples can be accessed with the
    /// `get_samples` function.
    pub fn get_sample_count(&self) -> u64 {
        unsafe {
            ffi::sfSoundBuffer_getSampleCount(self.raw()) as u64
        }
    }

	/// Get the array of audio samples stored in the buffer.
	pub fn get_samples(&self) -> &[i16] {
		unsafe {
			::std::slice::from_raw_parts(
				ffi::sfSoundBuffer_getSamples(self.raw()),
				ffi::sfSoundBuffer_getSampleCount(self.raw()) as usize
			)
		}
	}

    /// Get the number of channels used by the sound.
    ///
    /// If the sound is mono then the number of channels will
    /// be 1, 2 for stereo, etc.
    pub fn get_channel_count(&self) -> u32 {
        unsafe {
            ffi::sfSoundBuffer_getChannelCount(self.raw()) as u32
        }
    }

    /// Get the total duration of the sound.
    pub fn get_duration(&self) -> Time {
        unsafe { ffi::sfSoundBuffer_getDuration(self.raw()) }
    }

    /// Get the sample rate of the sound.
    ///
    /// The sample rate is the number of samples played per second.
    /// The higher, the better the quality (for example, 44100
    /// samples/s is CD quality).
    pub fn get_sample_rate(&self) -> u32 {
        unsafe {
            ffi::sfSoundBuffer_getSampleRate(self.raw()) as u32
        }
    }
}

impl Clone for SoundBuffer {
	fn clone(&self) -> SoundBuffer {
		self.clone_opt().expect("Failed to clone SoundBuffer")
	}
}

#[doc(hidden)]
unsafe impl ForeignHolder for SoundBuffer {
	type Inner = ffi::sfSoundBuffer;
}
