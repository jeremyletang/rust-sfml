use audio::csfml_audio_sys::*;
use csfml_system_sys::{sfBool, sfInt16};
use std::os::raw::c_void;
use sf_bool_ext::SfBoolExt;
use std::ffi::{CStr, CString};
use system::Time;

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
