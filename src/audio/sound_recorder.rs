use audio::csfml_audio_sys::*;
use csfml_system_sys::*;
use sf_bool_ext::SfBoolExt;

use super::SetDeviceError;

use std::ffi::{CString};
use std::ptr;
use std::os::raw::c_void;
use std::slice;

pub struct SoundRecorder<T1: Fn() -> bool, T2: Fn(&[i16]) -> bool, T3: Fn()> {
    sf_sound_recorder: *mut sfSoundRecorder,
    on_start: T1,
    on_data: T2,
    on_stop: T3
}

unsafe extern "C"
fn sound_recorder_on_start<T1: Fn() -> bool, T2: Fn(&[i16]) -> bool, T3: Fn()>(arg: *mut c_void) -> sfBool {
    let sr = arg as *mut SoundRecorder<T1, T2, T3>;
    let ref cb = (*sr).on_start;
    sfBool::from_bool(cb())
}

unsafe extern "C"
fn sound_recorder_on_data<T1: Fn() -> bool, T2: Fn(&[i16]) -> bool, T3: Fn()>(samples: *const sfInt16, len: usize, arg: *mut c_void) -> sfBool {
    let sr = arg as *mut SoundRecorder<T1, T2, T3>;
    let ref cb = (*sr).on_data;
    let sample_slice = slice::from_raw_parts(samples, len);
    sfBool::from_bool(cb(sample_slice))
}

unsafe extern "C"
fn sound_recorder_on_stop<T1: Fn() -> bool, T2: Fn(&[i16]) -> bool, T3: Fn()>(arg: *mut c_void) {
    let sr = arg as *mut SoundRecorder<T1, T2, T3>;
    let ref cb = (*sr).on_stop;
    cb();
}

impl<T1: Fn() -> bool, T2: Fn(&[i16]) -> bool, T3: Fn()> SoundRecorder<T1, T2, T3> {

    pub fn new(on_start: T1, on_data: T2, on_stop: T3) -> SoundRecorder<T1, T2, T3> {
        SoundRecorder{
            sf_sound_recorder: unsafe {
            sfSoundRecorder_create(
                Some(sound_recorder_on_start::<T1, T2, T3>),
                Some(sound_recorder_on_data::<T1, T2, T3>),
                Some(sound_recorder_on_stop::<T1, T2, T3>),
                ptr::null_mut()
            )
        }, on_start, on_data, on_stop }
    }

    pub fn start(&self, sample_rate: u32) -> bool {
        unsafe { sfSoundRecorder_start(self.sf_sound_recorder, sample_rate) == sfTrue }
    }

    pub fn stop(&self) {
        unsafe { sfSoundRecorder_stop(self.sf_sound_recorder) };
    }

    pub fn set_device(&mut self, name: &str) -> Result<(), SetDeviceError> {
        let name = CString::new(name).unwrap();
        let success = unsafe {
            sfSoundRecorder_setDevice(self.sf_sound_recorder, name.as_ptr()).to_bool()
        };
        if success {
            Ok(())
        } else {
            Err(SetDeviceError)
        }
    }

}

impl<T1: Fn() -> bool, T2: Fn(&[i16]) -> bool, T3: Fn()> Drop for SoundRecorder<T1, T2, T3> {
    fn drop(&mut self) {
        unsafe { sfSoundRecorder_destroy(self.sf_sound_recorder) };
    }
}
