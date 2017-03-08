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

use sound_buffer::SoundBufferRef;

use csfml_audio_sys as ffi;
use csfml_system_sys::*;
use sfml::system::SfBoolExt;
use std::ffi::{CStr, CString};

/// Store captured audio data in sound Buffer
///
/// `SoundBufferRecorder` allows to access a recorded sound through a `SoundBuffer`,
/// so that it can be played, saved to a file, etc.
pub struct SoundBufferRecorder {
    sound_buffer_recorder: *mut ffi::sfSoundBufferRecorder,
}

#[derive(Debug)]
pub struct SetDeviceError;

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
    pub fn start(&mut self, sample_rate: u32) -> bool {
        unsafe {
            ffi::sfSoundBufferRecorder_start(self.sound_buffer_recorder, sample_rate) == sfTrue
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
    pub fn get_buffer(&self) -> &SoundBufferRef {
        let buff = unsafe { ffi::sfSoundBufferRecorder_getBuffer(self.sound_buffer_recorder) };
        if buff.is_null() {
            panic!("sound_buffer_recorder::get_buffer: buffer was null");
        } else {
            unsafe { &*(buff as *const SoundBufferRef) }
        }
    }

    /// Get the name of the current audio capture device.
    pub fn device(&self) -> String {
        unsafe {
            let c_str_ptr = ffi::sfSoundRecorder_getDevice(self.sound_buffer_recorder as _);
            CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
        }
    }

    /// Set the audio capture device.
    ///
    /// This function sets the audio capture device to the device with the given name.
    /// It can be called on the fly (i.e: while recording).
    /// If you do so while recording and opening the device fails, it stops the recording.
    pub fn set_device(&mut self, name: &str) -> Result<(), SetDeviceError> {
        let name = CString::new(name).unwrap();
        let success = unsafe {
            ffi::sfSoundRecorder_setDevice(self.sound_buffer_recorder as _, name.as_ptr()).to_bool()
        };
        if success { Ok(()) } else { Err(SetDeviceError) }
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

    /// Get the name of the default audio capture device.
    ///
    /// This function returns the name of the default audio capture device.
    /// If none is available, an empty string is returned.
    pub fn default_device() -> String {
        unsafe {
            let c_str_ptr = ffi::sfSoundRecorder_getDefaultDevice();
            CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
        }
    }

    /// Get a list of the names of all available audio capture devices.
    ///
    /// This function returns a vector of strings, containing the names of all available
    /// audio capture devices.
    pub fn available_devices() -> Vec<String> {
        unsafe {
            let mut count = 0;
            let device_names = ffi::sfSoundRecorder_getAvailableDevices(&mut count);
            let device_names = ::std::slice::from_raw_parts(device_names, count as usize);
            let mut names = Vec::new();
            for c_str_ptr in device_names {
                let name = CStr::from_ptr(*c_str_ptr).to_string_lossy().into_owned();
                names.push(name);
            }
            names
        }
    }
}

#[test]
fn test_devices() {
    let default = SoundBufferRecorder::default_device();
    println!("Default device: {}", default);
    println!("Available devices:");
    let devices = SoundBufferRecorder::available_devices();
    for device in &devices {
        println!("{}", device);
    }
    let mut recorder = SoundBufferRecorder::new();
    assert_eq!(recorder.device(), default);
    if let Some(device) = devices.last() {
        recorder.set_device(device).unwrap();
        assert_eq!(&recorder.device(), device);
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
