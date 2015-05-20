use libc::{c_void, c_uint, c_float};
use std::mem::transmute;
use std::marker::PhantomData;

use audio::{SoundSource, PlayableSound, SoundStatus};
use system::{Time, Vector3f};

use ffi::{Foreign, SfBool};
use ffi::audio as ffi;

/// Custom streamed audio source.
///
/// Unlike audio buffers (such as `SoundBuffer`), audio streams are never
/// completely loaded in memory. Instead, the audio data is acquired
/// continuously while the stream is playing. This behavior allows playing of a
/// sound with no loading delay, and keeps the memory consumption very low.
///
/// A `SoundStream` is constructed from an implementor of `SoundStreamImpl`,
/// which provides methods to read data and seek within the stream.
///
/// It is important to note that each `SoundStream` is played in its own
/// separate thread, so that the streaming loop doesn't block the rest of the
/// program. In particular, the `SoundStreamImpl`'s methods may sometimes be
/// called from this separate thread. It is important to keep this in mind,
/// because you may have to take care of synchronization issues if you share
/// data between threads.
pub struct SoundStream<'a>(Foreign<ffi::sfSoundStream>, PhantomData<&'a mut SoundStreamImpl>);

impl<'a> SoundStream<'a> {
	/// Construct a new SoundStream.
	///
	/// A reference to the implementor, the channel count, and the sample rate
	/// (number of samples per second) must be provided.
	///
	/// Returns Some(SoundStream) or None on failure.
	pub fn new<T: SoundStreamImpl>(stream_impl: &'a mut T, channel_count: u32, sample_rate: u32) -> Option<SoundStream<'a>> {
		unsafe {
			Foreign::new(ffi::sfSoundStream_create(
				get_data_impl::<T>,
				seek_impl::<T>,
				channel_count as c_uint,
				sample_rate as c_uint,
				transmute(stream_impl)
			))
		}.map(|ptr| SoundStream(ptr, PhantomData))
	}

	fn raw(&self) -> &ffi::sfSoundStream { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfSoundStream { self.0.as_mut() }
}

impl<'a> SoundSource for SoundStream<'a> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfSoundStream_setPitch(self.raw_mut(), pitch as c_float) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfSoundStream_setVolume(self.raw_mut(), volume as c_float) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfSoundStream_setRelativeToListener(self.raw_mut(), SfBool::from_bool(relative)) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfSoundStream_setMinDistance(self.raw_mut(), distance as c_float) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfSoundStream_setAttenuation(self.raw_mut(), attenuation as c_float) }
    }
    fn get_position(&self) -> Vector3f {
        unsafe { ffi::sfSoundStream_getPosition(self.raw()) }
    }
    fn set_position(&mut self, position: Vector3f) {
        unsafe { ffi::sfSoundStream_setPosition(self.raw_mut(), position) }
    }
    fn get_pitch(&self) -> f32 {
        unsafe { ffi::sfSoundStream_getPitch(self.raw()) as f32 }
    }
    fn get_volume(&self) -> f32 {
        unsafe { ffi::sfSoundStream_getVolume(self.raw()) as f32 }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfSoundStream_isRelativeToListener(self.raw()) }.to_bool()
    }
    fn get_min_distance(&self) -> f32 {
        unsafe { ffi::sfSoundStream_getMinDistance(self.raw()) as f32 }
    }
    fn get_attenuation(&self) -> f32 {
        unsafe { ffi::sfSoundStream_getAttenuation(self.raw()) as f32 }
    }
}

impl<'a> PlayableSound for SoundStream<'a> {
    fn set_loop(&mut self, lloop: bool) {
        unsafe { ffi::sfSoundStream_setLoop(self.raw_mut(), SfBool::from_bool(lloop)) }
    }
    fn get_loop(&self) -> bool {
        unsafe { ffi::sfSoundStream_getLoop(self.raw()) }.to_bool()
    }
    fn play(&mut self) {
        unsafe { ffi::sfSoundStream_play(self.raw_mut()) }
    }
    fn pause(&mut self) {
        unsafe { ffi::sfSoundStream_pause(self.raw_mut()) }
    }
    fn stop(&mut self) {
        unsafe { ffi::sfSoundStream_stop(self.raw_mut()) }
    }
    fn get_status(&self) -> SoundStatus {
        unsafe { ffi::sfSoundStream_getStatus(self.raw()) }
    }
    fn get_playing_offset(&self) -> Time {
        unsafe { ffi::sfSoundStream_getPlayingOffset(self.raw()) }
    }
    fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfSoundStream_setPlayingOffset(self.raw_mut(), time_offset) }
    }
	fn get_channel_count(&self) -> u32 {
		unsafe { ffi::sfSoundStream_getChannelCount(self.raw()) as u32 }
	}
	fn get_sample_rate(&self) -> u32 {
		unsafe { ffi::sfSoundStream_getSampleRate(self.raw()) as u32 }
	}
}

/// Types implementing this trait can be used as providers for a SoundStream.
// TODO: double-check the Send and Sync bounds
pub trait SoundStreamImpl: Send + Sync {
	/// Read a chunk of sound data from the stream.
	///
	/// If an empty sample array is returned, the sound is considered ended.
	fn get_data(&mut self) -> &[i16];

	/// Seek to the specified offset in the stream.
	fn seek(&mut self, time: Time);
}

unsafe extern fn get_data_impl<T: SoundStreamImpl>(chunk: *mut ffi::sfSoundStreamChunk, user: *mut c_void) -> SfBool {
	let stream: &mut T = transmute(user);
	let slice = stream.get_data();
	if slice.len() == 0 {
		SfBool::SFFALSE
	} else {
		*chunk = ffi::sfSoundStreamChunk {
			samples: slice.as_ptr(),
			sample_count: slice.len() as c_uint
		};
		SfBool::SFTRUE
	}
}

unsafe extern fn seek_impl<T: SoundStreamImpl>(time: Time, user: *mut c_void) {
	let stream: &mut T = transmute(user);
	stream.seek(time);
}
