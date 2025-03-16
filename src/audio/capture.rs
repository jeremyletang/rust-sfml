use {
    crate::{
        IntoSfResult, SfResult,
        audio::SoundBuffer,
        cpp::{CppString, CppVector, FBox},
        ffi::audio as ffi,
    },
    std::{ffi::CString, os::raw::c_void, ptr::NonNull},
};

/// Trait for processing captured sound data.
///
/// `SoundRecorder` provides a simple interface to access the audio recording capabilities
/// of the computer (the microphone).
///
/// As a trait, it only cares about capturing sound samples,
/// the task of making something useful with them is left to the implementer.
/// Note that SFML provides a built-in implementation for saving the captured data to
/// a sound buffer (see [`SoundBufferRecorder`]).
///
/// Only one method is required to be implemented.
///
/// [`on_process_samples`] provides the new chunks of audio samples while the capture happens
/// Moreover, two additional methods can be overridden as well if necessary:
///
/// [`on_start`] is called before the capture happens, to perform custom initializations.
///
/// [`on_stop`] is called after the capture ends, to perform custom cleanup.
///
/// You can also control the frequency of the [`on_process_samples`] calls,
/// with [`SoundRecorderDriver::set_processing_interval`].
/// The default interval is chosen so that recording thread doesn't consume too much CPU,
/// but it can be changed to a smaller value if you need to process the recorded data in real time,
/// for example.
///
/// The audio capture feature may not be supported or activated on every platform,
/// thus it is recommended to check its availability with the [`is_available`] function.
/// If it returns `false`, then any attempt to use an audio recorder will fail.
///
/// If you have multiple sound input devices connected to your computer
/// (for example: microphone, external soundcard, webcam mic, ...)
/// you can get a list of all available devices through the [`available_devices`] function.
/// You can then select a device by calling [`SoundRecorderDriver::set_device`] with the
/// appropriate device. Otherwise the default capturing device will be used.
///
/// By default the recording is in 16-bit mono.
/// Using the [`SoundRecorderDriver::set_channel_count`] method you can change the number of
/// channels used by the audio capture device to record.
/// Note that you have to decide whether you want to record in mono or stereo before
/// starting the recording.
///
/// It is important to note that the audio capture happens in a separate thread,
/// so that it doesn't block the rest of the program. In particular,
/// the [`on_process_samples`] function (but not [`on_start`] and not [`on_stop`])
/// will be called from this separate thread.
/// It is important to keep this in mind, because you may have to take care of
/// synchronization issues if you share data between threads.
///
/// [`on_start`]: SoundRecorder::on_start
/// [`on_stop`]: SoundRecorder::on_stop
/// [`on_process_samples`]: SoundRecorder::on_process_samples
pub trait SoundRecorder: Send {
    /// Start capturing audio data.
    ///
    /// This method may be overridden by an implementer if something has
    /// to be done every time a new capture starts.
    /// If not, this function can be ignored; the default implementation does nothing.
    ///
    /// Returns `true` to start the capture, or `false` to abort it.
    fn on_start(&mut self) -> bool {
        true
    }
    /// Process a new chunk of recorded samples.
    ///
    /// This method is called every time a new chunk of recorded data is available.
    /// The implementer can then do whatever it
    /// wants with it (storing it, playing it, sending it over the network, etc.).
    ///
    /// Returns `true` to continue the capture, or `false` to stop it.
    fn on_process_samples(&mut self, samples: &[i16]) -> bool;
    /// Stop capturing audio data.
    ///
    /// This method may be overridden by an implementer if something has
    /// to be done every time the capture ends.
    /// If not, this function can be ignored; the default implementation does nothing.
    fn on_stop(&mut self) {}
}

/// Type that "drives" custom sound recorders.
///
/// It does the actual recording, and feeds the custom sound recorder with the recorded data.
#[derive(Debug)]
pub struct SoundRecorderDriver<'a, R: 'a> {
    handle: NonNull<ffi::sfCustomSoundRecorder>,
    recorder: &'a mut R,
}

// SAFETY: An `sfCustomSoundRecorder` isn't tied to a particular thread, so it can be sent between
// threads safely.
unsafe impl<R: Send> Send for SoundRecorderDriver<'_, R> {}

// SAFETY: An `&SoundRecorderDriver` only allows access to methods which read the status of the
// driver, which is fine to do from multiple threads at once. Thus it is safe to pass
// `&SoundRecorderDriver` between threads.
unsafe impl<R: Sync> Sync for SoundRecorderDriver<'_, R> {}

unsafe extern "C" fn on_start_callback<R: SoundRecorder>(user_data: *mut c_void) -> bool {
    let recorder: *mut R = user_data.cast();
    unsafe { (*recorder).on_start() }
}

unsafe extern "C" fn on_process_callback<R: SoundRecorder>(
    data: *const i16,
    len: usize,
    user_data: *mut c_void,
) -> bool {
    let recorder: *mut R = user_data.cast();
    unsafe { (*recorder).on_process_samples(std::slice::from_raw_parts(data, len)) }
}

unsafe extern "C" fn on_stop_callback<R: SoundRecorder>(user_data: *mut c_void) {
    let recorder: *mut R = user_data.cast();
    unsafe { (*recorder).on_stop() }
}

impl<'a, R: SoundRecorder> SoundRecorderDriver<'a, R> {
    /// Creates a new `SoundRecorderDriver` with the specified [`SoundRecorder`].
    ///
    /// # Panics
    ///
    /// Panics if a `SoundRecorderDriver` can't be created for whatever reason
    pub fn new(sound_recorder: &'a mut R) -> Self {
        Self {
            handle: unsafe {
                let ptr: *mut R = sound_recorder;
                NonNull::new(ffi::sfCustomSoundRecorder_new(
                    Some(on_start_callback::<R>),
                    Some(on_process_callback::<R>),
                    Some(on_stop_callback::<R>),
                    ptr.cast(),
                ))
                .expect("Failed to create SoundRecorderDriver")
            },
            recorder: sound_recorder,
        }
    }
    /// Start the capture.
    ///
    /// The `sample_rate` parameter defines the number of audio samples captured per second.
    /// The higher, the better the quality (for example, 44100 samples/sec is CD quality).
    /// This function uses its own thread so that it doesn't block the rest of the program
    /// while the capture runs.
    /// Please note that only one capture can happen at the same time.
    /// You can select which capture device will be used, by passing the name to the
    /// [`set_device`] method.
    /// If none was selected before, the default capture device will be used.
    /// You can get a list of the names of all available capture devices by
    /// calling [`available_devices`].
    ///
    /// [`set_device`]: SoundRecorderDriver::set_device
    pub fn start(&mut self, sample_rate: u32) -> SfResult<()> {
        unsafe { ffi::sfCustomSoundRecorder_start(self.handle.as_ptr(), sample_rate) }
            .into_sf_result()
    }
    /// Stop the capture, lending out the underlying [`SoundRecorder`].
    pub fn stop(&mut self) -> &mut R {
        unsafe {
            ffi::sfCustomSoundRecorder_stop(self.handle.as_ptr());
        }
        self.recorder
    }
    /// Get the sample rate.
    ///
    /// The sample rate defines the number of audio samples captured per second.
    /// The higher, the better the quality (for example, 44100 samples/sec is CD quality).
    #[must_use]
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::sfCustomSoundRecorder_getSampleRate(self.handle.as_ptr()) }
    }
    /// Set the channel count of the audio capture device.
    ///
    /// This method allows you to specify the number of channels used for recording.
    /// Currently only 16-bit mono (1) and 16-bit stereo (1) are supported.
    pub fn set_channel_count(&mut self, channel_count: u32) {
        unsafe { ffi::sfCustomSoundRecorder_setChannelCount(self.handle.as_ptr(), channel_count) }
    }
    /// Get the number of channels used by this recorder.
    ///
    /// Currently only mono and stereo are supported,
    /// so the value is either 1 (for mono) or 2 (for stereo).
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::sfCustomSoundRecorder_getChannelCount(self.handle.as_ptr()) }
    }
    /// Get the name of the current audio capture device.
    #[must_use]
    pub fn device(&self) -> &CppString {
        unsafe { &*ffi::sfCustomSoundRecorder_getDevice(self.handle.as_ptr()) }
    }

    /// Set the audio capture device.
    ///
    /// This function sets the audio capture device to the device with the given name.
    /// It can be called on the fly (i.e: while recording).
    /// If you do so while recording and opening the device fails, it stops the recording.
    pub fn set_device(&mut self, name: &str) -> SfResult<()> {
        let name = CString::new(name)?;
        let success =
            unsafe { ffi::sfCustomSoundRecorder_setDevice(self.handle.as_ptr(), name.as_ptr()) };
        success.into_sf_result()
    }
}

impl<S> Drop for SoundRecorderDriver<'_, S> {
    fn drop(&mut self) {
        unsafe {
            // It seems there can be problems (e.g. "pure virtual method called") if the
            // recorder is not stopped before it's destroyed. So let's make sure it's stopped.
            ffi::sfCustomSoundRecorder_stop(self.handle.as_ptr());
            ffi::sfCustomSoundRecorder_del(self.handle.as_ptr());
        }
    }
}

/// Specialized [`SoundRecorder`] which stores the captured audio data into a sound buffer.
///
/// `SoundBufferRecorder` allows to access a recorded sound through a [`SoundBuffer`],
/// so that it can be played, saved to a file, etc.
///
/// As usual, don't forget to call the [`is_available`] function before using this type
/// (see [`SoundRecorder`] for more details about this).
#[derive(Debug)]
pub struct SoundBufferRecorder {
    handle: NonNull<ffi::sfSoundBufferRecorder>,
}

// SAFETY: An `sfSoundBufferRecorder` isn't tied to a particular thread, so it can be sent between
// threads safely.
unsafe impl Send for SoundBufferRecorder {}

// SAFETY: An `&SoundBufferRecorder` only allows access to methods which read the status of the
// recorder, which is fine to do from multiple threads at once. Thus it is safe to pass
// `&SoundBufferRecorder` between threads.
unsafe impl Sync for SoundBufferRecorder {}

impl SoundBufferRecorder {
    /// Create a new sound buffer recorder
    ///
    /// # Panics
    ///
    /// Panics if a `SoundBufferRecorder` can't be created for whatever reason
    #[must_use]
    pub fn new() -> SoundBufferRecorder {
        let buffer = unsafe { ffi::sfSoundBufferRecorder_new() };
        SoundBufferRecorder {
            handle: NonNull::new(buffer).expect("Failed to create SoundBufferRecorder"),
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
    /// * `sample_rate` - Desired capture rate, in number of samples per second
    pub fn start(&mut self, sample_rate: u32) -> SfResult<()> {
        unsafe { ffi::sfSoundBufferRecorder_start(self.handle.as_ptr(), sample_rate) }
            .into_sf_result()
    }

    /// Stop the capture of a sound recorder
    pub fn stop(&mut self) {
        unsafe { ffi::sfSoundBufferRecorder_stop(self.handle.as_ptr()) }
    }

    /// Get the sample rate of a sound buffer recorder
    ///
    /// The sample rate defines the number of audio samples
    /// captured per second. The higher, the better the quality
    /// (for example, 44100 samples/sec is CD quality).
    ///
    /// Return the sample rate, in samples per second
    #[must_use]
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::sfSoundBufferRecorder_getSampleRate(self.handle.as_ptr()) }
    }

    /// Get the sound buffer containing the captured audio data
    ///
    /// The sound buffer is valid only after the capture has ended.
    /// This function provides a read-only access to the internal
    /// sound buffer, but it can be copied if you need to
    /// make any modification to it.
    ///
    /// Return Read-only access to the sound buffer
    #[must_use]
    pub fn buffer(&self) -> &SoundBuffer {
        let buff = unsafe { ffi::sfSoundBufferRecorder_getBuffer(self.handle.as_ptr()) };
        // Safety: getBuffer returns a reference on C++ side, it can never be null or dangling.
        unsafe { &*(buff) }
    }
    /// Get the name of the current audio capture device.
    #[must_use]
    pub fn device(&self) -> &CppString {
        unsafe { &*ffi::sfSoundBufferRecorder_getDevice(self.handle.as_ptr()) }
    }

    /// Set the audio capture device.
    ///
    /// This function sets the audio capture device to the device with the given name.
    /// It can be called on the fly (i.e: while recording).
    /// If you do so while recording and opening the device fails, it stops the recording.
    pub fn set_device(&mut self, name: &str) -> SfResult<()> {
        let name = CString::new(name)?;
        let success =
            unsafe { ffi::sfSoundBufferRecorder_setDevice(self.handle.as_ptr(), name.as_ptr()) };
        success.into_sf_result()
    }

    /// Get the number of channels used by this recorder
    ///
    /// Currently only mono and stereo are supported, so the
    /// value is either 1 (for mono) or 2 (for stereo).
    ///
    /// Number of channels
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::sfSoundBufferRecorder_getChannelCount(self.handle.as_ptr()) }
    }

    /// Set the channel count of the audio capture device
    ///
    /// This method allows you to specify the number of channels
    /// used for recording. Currently only 16-bit mono and
    /// 16-bit stereo are supported.
    ///
    /// # Arguments
    /// channelCount - Number of channels. Currently only
    ///                     mono (1) and stereo (2) are supported.
    pub fn set_channel_count(&mut self, channel_count: u32) {
        unsafe {
            ffi::sfSoundBufferRecorder_setChannelCount(self.handle.as_ptr(), channel_count);
        }
    }
}

#[cfg_attr(not(feature = "ci-headless"), test)]
fn test_devices() {
    let default = default_device();
    println!("Default device: {}", *default);
    println!("Available devices:");
    let devices = available_devices();
    for device in devices.iter() {
        println!("{device}");
    }
    let mut recorder = SoundBufferRecorder::new();
    assert_eq!(*recorder.device(), *default);
    if let Some(device) = devices.last() {
        recorder.set_device(device.to_str().unwrap()).unwrap();
        assert_eq!(recorder.device().to_str().unwrap(), device);
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
            ffi::sfSoundBufferRecorder_del(self.handle.as_ptr());
        }
    }
}

/// Check if the system supports audio capture
///
/// This function should always be called before using
/// the audio capture features. If it returns false, then
/// any attempt to use `SoundRecorder` will fail.
///
/// Return true if audio capture is supported, false otherwise
#[must_use]
pub fn is_available() -> bool {
    unsafe { ffi::sfSoundRecorder_isAvailable() }
}
/// Get the name of the default audio capture device.
///
/// This function returns the name of the default audio capture device.
/// If none is available, an empty string is returned.
///
/// # Panics
///
/// Panics on allocation failure.
#[must_use]
pub fn default_device() -> FBox<CppString> {
    unsafe {
        FBox::new(ffi::sfSoundRecorder_getDefaultDevice()).expect("Failed to create sfStdString")
    }
}

/// Get a list of the names of all available audio capture devices.
///
/// This function returns a vector of strings, containing the names of all available
/// audio capture devices.
///
/// # Panics
///
/// Panics on allocation failure.
#[must_use]
pub fn available_devices() -> FBox<CppVector<CppString>> {
    unsafe {
        FBox::new(ffi::sfSoundRecorder_getAvailableDevices())
            .expect("Failed to create sfStdStringVector")
    }
}
