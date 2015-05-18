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

use libc::c_uint;

use audio::{SoundBuffer, SoundRecorder};

use ffi::{Foreign, Ref};
use ffi::audio as ffi;

/// A sound recorder which stores the captured audio data in a sound buffer.
///
/// `SoundBufferRecorder` allows access to the recorded sound through a
/// SoundBuffer, so that it can be played, saved to a file, etc.
///
/// Capture can be controlled using `start()` and `stop()`, and the captured
/// buffer retrieved with `get_buffer()`.
///
/// See `SoundRecorder` for other important information on sound capture.
pub struct SoundBufferRecorder(Foreign<ffi::sfSoundBufferRecorder>);

impl SoundBufferRecorder {
    /// Create a new sound buffer recorder.
	///
	/// Returns Some(SoundBufferRecorder) or None on failure.
    pub fn new() -> Option<SoundBufferRecorder> {
        unsafe {
			Foreign::new(ffi::sfSoundBufferRecorder_create())
		}.map(SoundBufferRecorder)
    }

	fn raw(&self) -> &ffi::sfSoundBufferRecorder { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfSoundBufferRecorder { self.0.as_mut() }

	/// Start the capture.
    ///
    /// The `sample_rate` parameter defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the capture runs.
    /// Please note that only one capture can happen at the same time.
    pub fn start(&mut self, sample_rate: u32) -> bool {
        unsafe {
            ffi::sfSoundBufferRecorder_start(self.raw_mut(), sample_rate as c_uint)
        }.to_bool()
    }

    /// Stop the capture.
    pub fn stop(&mut self) -> () {
        unsafe {
            ffi::sfSoundBufferRecorder_stop(self.raw_mut())
        }
    }

    /// Get the sample rate.
    ///
    /// The sample rate defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    pub fn get_sample_rate(&self) -> u32 {
        unsafe {
            ffi::sfSoundBufferRecorder_getSampleRate(self.raw()) as u32
        }
    }

    /// Get the sound buffer containing the captured audio data.
    ///
    /// The sound buffer is valid only after the capture has ended.
    /// This function provides a read-only access to the internal
    /// sound buffer, but it can be copied if you need to
    /// make any modification to it.
    pub fn get_buffer(&self) -> Option<Ref<SoundBuffer>> {
        unsafe { Ref::new(ffi::sfSoundBufferRecorder_getBuffer(self.raw())) }
    }

	// TODO: add the methods that CSFML is currently missing support for

    /// Check if the system supports audio capture.
    ///
    /// This function should always be called before using
    /// the audio capture features. If it returns false, then
    /// any attempt to use sound recording will fail.
	#[inline]
    pub fn is_available() -> bool {
		SoundRecorder::is_available()
    }

	/// Get the name of the default audio capture device.
	///
	/// This function returns the name of the default audio capture device. If
	/// none is available, an empty string is returned.
	#[inline]
	pub fn get_default_device() -> String {
		SoundRecorder::get_default_device()
	}

	/// Get a list of the names of all available audio capture devices.
	///
	/// This function returns a vector of strings, containing the names of all
	/// available audio capture devices.
	#[inline]
	pub fn get_available_devices() -> Vec<String> {
		SoundRecorder::get_available_devices()
	}
}
