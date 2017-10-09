use audio::csfml_audio_sys::*;
use csfml_system_sys::{sfBool, sfInt16};
use std::os::raw::c_void;
use sf_bool_ext::SfBoolExt;
use std::ffi::{CStr, CString};
use system::Time;

/// Trait for processing captured sound data.
///
/// `SoundRecorder` provides a simple interface to access the audio recording capabilities
/// of the computer (the microphone).
///
/// As a trait, it only cares about capturing sound samples,
/// the task of making something useful with them is left to the implementer.
/// Note that SFML provides a built-in implementation for saving the captured data to
/// a sound buffer (see `SoundBufferRecorder`).
///
/// Only one method is required to be implemented.
///
/// `on_process_samples` provides the new chunks of audio samples while the capture happens
/// Moreover, two additional methods can be overridden as well if necessary:
///
/// `on_start` is called before the capture happens, to perform custom initializations
/// `on_stop` is called after the capture ends, to perform custom cleanup
/// You can also control the frequency of the `on_process_samples` calls,
/// with the `set_processing_interval` method.
/// TODO: `set_processing_interval` is on `SoundRecorderDriver`.
/// The default interval is chosen so that recording thread doesn't consume too much CPU,
/// but it can be changed to a smaller value if you need to process the recorded data in real time,
/// for example.
///
/// The audio capture feature may not be supported or activated on every platform,
/// thus it is recommended to check its availability with the `is_available` function.
/// If it returns `false`, then any attempt to use an audio recorder will fail.
/// TODO: Add these methods
///
/// If you have multiple sound input devices connected to your computer
/// (for example: microphone, external soundcard, webcam mic, ...)
/// you can get a list of all available devices through the `get_available_devices` function.
/// You can then select a device by calling `set_device` with the appropriate device.
/// Otherwise the default capturing device will be used.
///
/// By default the recording is in 16-bit mono.
/// Using the `set_channel_count` method you can change the number of channels used by
/// the audio capture device to record.
/// Note that you have to decide whether you want to record in mono or stereo before
/// starting the recording.
///
/// It is important to note that the audio capture happens in a separate thread,
/// so that it doesn't block the rest of the program. In particular,
/// the `on_process_samples` function (but not `on_start` and not `on_stop`)
/// will be called from this separate thread.
/// It is important to keep this in mind, because you may have to take care of
/// synchronization issues if you share data between threads.
pub trait SoundRecorder {
    fn on_start(&mut self) -> bool {
        true
    }
    fn on_process_samples(&mut self, samples: &[i16]) -> bool;
    fn on_stop(&mut self) {}
}

#[derive(Debug)]
pub struct SoundRecorderDriver<'a, R: 'a> {
    sf_sound_recorder: *mut sfSoundRecorder,
    recorder: &'a mut R,
}

unsafe extern "C" fn on_start_callback<R: SoundRecorder>(user_data: *mut c_void) -> sfBool {
    let recorder = user_data as *mut R;
    sfBool::from_bool((*recorder).on_start())
}

unsafe extern "C" fn on_process_callback<R: SoundRecorder>(
    data: *const sfInt16,
    len: usize,
    user_data: *mut c_void,
) -> sfBool {
    let recorder = user_data as *mut R;
    sfBool::from_bool((*recorder).on_process_samples(::std::slice::from_raw_parts(data, len)))
}

unsafe extern "C" fn on_stop_callback<R: SoundRecorder>(user_data: *mut c_void) {
    let recorder = user_data as *mut R;
    (*recorder).on_stop()
}

impl<'a, R: SoundRecorder> SoundRecorderDriver<'a, R> {
    pub fn new(sound_recorder: &'a mut R) -> Self {
        let ptr: *mut R = sound_recorder;
        Self {
            sf_sound_recorder: unsafe {
                sfSoundRecorder_create(
                    Some(on_start_callback::<R>),
                    Some(on_process_callback::<R>),
                    Some(on_stop_callback::<R>),
                    ptr as *mut _,
                )
            },
            recorder: sound_recorder,
        }
    }
    pub fn start(&mut self, sample_rate: u32) -> bool {
        unsafe { sfSoundRecorder_start(self.sf_sound_recorder, sample_rate).to_bool() }
    }
    pub fn stop(&mut self) -> &mut R {
        unsafe {
            sfSoundRecorder_stop(self.sf_sound_recorder);
        }
        self.recorder
    }
    pub fn get_sample_rate(&self) -> u32 {
        unsafe { sfSoundRecorder_getSampleRate(self.sf_sound_recorder) }
    }
    pub fn set_device(&mut self, name: &str) -> bool {
        let name = CString::new(name).unwrap();
        unsafe { sfSoundRecorder_setDevice(self.sf_sound_recorder, name.as_ptr()).to_bool() }
    }
    pub fn get_device(&self) -> &str {
        unsafe {
            CStr::from_ptr(sfSoundRecorder_getDevice(self.sf_sound_recorder))
                .to_str()
                .unwrap()
        }
    }
    pub fn set_channel_count(&mut self, channel_count: u32) {
        unsafe { sfSoundRecorder_setChannelCount(self.sf_sound_recorder, channel_count) }
    }
    pub fn get_channel_count(&self) -> u32 {
        unsafe { sfSoundRecorder_getChannelCount(self.sf_sound_recorder) }
    }
    pub fn set_processing_interval(&mut self, interval: Time) {
        unsafe { sfSoundRecorder_setProcessingInterval(self.sf_sound_recorder, interval.raw()) }
    }
}

impl<'a, S> Drop for SoundRecorderDriver<'a, S> {
    fn drop(&mut self) {
        unsafe {
            // It seems there can be problems (e.g. "pure virtual method called") if the
            // recorder is not stopped before it's destroyed. So let's make sure it's stopped.
            sfSoundRecorder_stop(self.sf_sound_recorder);
            sfSoundRecorder_destroy(self.sf_sound_recorder);
        }
    }
}
