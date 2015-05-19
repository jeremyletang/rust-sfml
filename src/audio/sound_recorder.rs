use libc::{c_void, c_uint, size_t};
use std::mem::transmute;
use std::marker::PhantomData;
use std::ffi::CString;

use system::Time;

use ffi::{Foreign, SfBool};
use ffi::audio as ffi;

// TODO: trait-ify common interface of SoundBufferRecorder and SoundRecorder

/// A utility for capturing sound data from audio input devices.
///
/// It is constructed using a reference to a `SoundRecorderImpl`, which receives
/// chunks of audio samples while the capture is ongoing.
///
/// The audio capture feature may not be supported or activated on every
/// platform, so it is recommended to check its availability with the
/// `is_available()` function. If it returns false, then any attempt to use an
/// audio recorder will fail.
///
/// If you have multiple sound input devices connected to your computer (for
/// example: microphone, external soundcard, webcam mic, ...) you can get a list
/// of all available devices through the `get_available_devices()` function. You
/// can then select a device by calling `set_device()` with the appropriate
/// device. Otherwise the default capturing device will be used.
///
/// It is important to note that the audio capture happens in a separate thread,
/// so that it doesn't block the rest of the program. In particular,
/// `process()` (but not `start` or `stop`) will be called from this separate
/// thread. It is important to keep this in mind, because you may have to take
/// care of synchronization issues if you share data between threads.
pub struct SoundRecorder<'a>(Foreign<ffi::sfSoundRecorder>, PhantomData<&'a mut SoundRecorderImpl>);

impl<'a> SoundRecorder<'a> {
	/// Construct a new SoundStream.
	///
	/// A reference to the implementor, the channel count, and the sample rate
	/// (number of samples per second) must be provided.
	///
	/// Returns Some(SoundStream) or None on failure.
	pub fn new<T: SoundRecorderImpl>(recorder: &'a mut T) -> Option<SoundRecorder<'a>> {
		unsafe {
			Foreign::new(ffi::sfSoundRecorder_create(
				start_impl::<T>,
				process_impl::<T>,
				stop_impl::<T>,
				transmute(recorder)
			))
		}.map(|ptr| SoundRecorder(ptr, PhantomData))
	}

	fn raw(&self) -> &ffi::sfSoundRecorder { self.0.as_ref() }
	fn raw_mut(&mut self) -> &mut ffi::sfSoundRecorder { self.0.as_mut() }

	/// Start the capture.
    ///
    /// The `sample_rate` parameter defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the capture runs.
    /// Please note that only one capture can happen at the same time.
    pub fn start(&mut self, sample_rate: u32) -> bool {
        unsafe { ffi::sfSoundRecorder_start(self.raw_mut(), sample_rate as c_uint) }.to_bool()
    }

    /// Stop the capture.
    pub fn stop(&mut self) {
        unsafe { ffi::sfSoundRecorder_stop(self.raw_mut()) }
    }

    /// Get the sample rate.
    ///
    /// The sample rate defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    pub fn get_sample_rate(&self) -> u32 {
        unsafe { ffi::sfSoundRecorder_getSampleRate(self.raw()) as u32 }
    }

	/// Set the processing interval.
	///
	/// The processing interval controls the period between calls to the
	/// `process()` function. You may want to use a small interval if you want
	/// to process the recorded data in real time, for example.
	///
	/// Note: this is only a hint, the actual period may vary. So don't rely on
	/// this parameter to implement precise timing.
	///
	/// The default processing interval is 100 ms.
	pub fn set_processing_interval(&mut self, interval: Time) {
		unsafe { ffi::sfSoundRecorder_setProcessingInterval(self.raw_mut(), interval) }
	}

	/// Set the audio capture device.
	///
	/// This function sets the audio capture device to the device with the given
	/// name. It can be called on the fly (i.e: while recording). If you do so
	/// while recording and opening the device fails, it stops the recording.
	///
	/// Returns true on success or false on failure.
	pub fn set_device(&mut self, device: &str) -> bool {
        let c_str = match CString::new(device.as_bytes()) {
			Ok(c_str) => c_str,
			Err(_) => return false
		};
		unsafe { ffi::sfSoundRecorder_setDevice(self.raw_mut(), c_str.as_ptr()) }.to_bool()
	}

	/// Get the name of the current audio capture device.
	pub fn get_device(&self) -> String {
		unsafe { ::ffi::from_c_str(ffi::sfSoundRecorder_getDevice(self.raw())) }
	}

    /// Check if the system supports audio capture.
    ///
    /// This function should always be called before using
    /// the audio capture features. If it returns false, then
    /// any attempt to use sound recording will fail.
    pub fn is_available() -> bool {
        unsafe { ffi::sfSoundRecorder_isAvailable() }.to_bool()
    }

	/// Get the name of the default audio capture device.
	///
	/// This function returns the name of the default audio capture device. If
	/// none is available, an empty string is returned.
	pub fn get_default_device() -> String {
		unsafe { ::ffi::from_c_str(ffi::sfSoundRecorder_getDefaultDevice()) }
	}

	/// Get a list of the names of all available audio capture devices.
	///
	/// This function returns a vector of strings, containing the names of all
	/// available audio capture devices.
	pub fn get_available_devices() -> Vec<String> {
		let mut size: size_t = 0;
		let table = unsafe {
			ffi::sfSoundRecorder_getAvailableDevices(&mut size)
		};
		if size == 0 {
			Vec::new()
		} else {
			unsafe {
				::std::slice::from_raw_parts(table, size as usize)
					.into_iter().map(|&x| ::ffi::from_c_str(x)).collect()
			}
		}
	}
}

/// Types implementing this trait can be used as receivers for a SoundRecorder.
// TODO: double-check the Send and Sync bounds
pub trait SoundRecorderImpl: Send + Sync {
	/// Called to start capturing audio data.
	///
	/// Return true to start the capture, or false to abort it.
	fn start(&mut self) -> bool { true }

	/// Called to process a new chunk of recorded samples.
	///
	/// Return true to continue the capture, or false to stop it.
	fn process(&mut self, samples: &[i16]) -> bool;

	/// Called when a capture is stopped.
	fn stop(&mut self) {}
}

unsafe extern fn start_impl<T: SoundRecorderImpl>(user: *mut c_void) -> SfBool {
	let rec: &mut T = transmute(user);
	SfBool::from_bool(rec.start())
}

unsafe extern fn process_impl<T: SoundRecorderImpl>(buf: *const i16, size: size_t, user: *mut c_void) -> SfBool {
	let rec: &mut T = transmute(user);
	SfBool::from_bool(rec.process(::std::slice::from_raw_parts(buf, size as usize)))
}

unsafe extern fn stop_impl<T: SoundRecorderImpl>(user: *mut c_void) {
	let rec: &mut T = transmute(user);
	rec.stop();
}

/*pub trait SoundRecorder {
	fn start(&mut self, sample_rate: u32) -> bool;
	fn stop(&mut self);
	fn get_sample_rate(&self) -> u32;
	fn set_device(&mut self, device: &str) -> bool;
	fn get_device(&self) -> String;

	#[doc(hidden)]
	fn set_processing_interval(&mut self, interval: Time);
}*/

