use {
    crate::{
        ffi,
        sf_box::Dispose,
        system::{InputStream, Time},
        SfBox, SfError, SfResult,
    },
    std::{
        ffi::CString,
        io::{Read, Seek},
        slice,
    },
};

decl_opaque! {
/// Storage for audio samples defining a sound.
///
/// A sound buffer holds the data of a sound, which is an array of audio samples.
///
/// A sample is a 16 bits signed integer that defines the amplitude of the sound at a given time.
/// The sound is then reconstituted by playing these samples at a high rate
/// (for example, 44100 samples per second is the standard rate used for playing CDs).
/// In short, audio samples are like texture pixels, and a `SoundBuffer` is
/// similar to a [`crate::graphics::Texture`].
///
/// A sound buffer can be loaded from a file (see [`from_file`] for the complete list of
/// supported formats), from memory, from a custom stream or directly from an array of samples.
/// It can also be saved back to a file.
///
/// [`from_file`]: SoundBuffer::from_file
///
/// Sound buffers alone are not very useful: they hold the audio data but cannot be played.
/// To do so, you need to use the [`Sound`] type, which provides functions to play/pause/stop
/// the sound as well as changing the way it is outputted (volume, pitch, 3D position, ...).
/// This separation allows more flexibility and better performances: indeed a `SoundBuffer` is
/// a heavy resource, and any operation on it is slow (often too slow for real-time applications).
/// On the other side, a [`Sound`] is a lightweight object, which can use the audio data of a sound
/// buffer and change the way it is played without actually modifying that data.
/// Note that it is also possible to bind several [`Sound`] instances to the same `SoundBuffer`.
///
/// It is important to note that the [`Sound`] instance doesn't copy the buffer that it uses,
/// it only keeps a reference to it. Thus, a `SoundBuffer` can not be destructed while it is
/// borrowed by a [`Sound`].
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
///
/// [`Sound`]: crate::audio::Sound
SoundBuffer;
}

impl SoundBuffer {
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
    #[must_use]
    pub fn save_to_file(&self, filename: &str) -> bool {
        let c_str = CString::new(filename).unwrap();
        unsafe { ffi::audio::sfSoundBuffer_saveToFile(self, c_str.as_ptr()) }
    }

    /// Get the number of samples stored in a sound buffer
    ///
    /// The array of samples can be accessed with [`samples`](SoundBuffer::samples).
    ///
    /// Return the number of samples
    #[must_use]
    pub fn sample_count(&self) -> u64 {
        unsafe { ffi::audio::sfSoundBuffer_getSampleCount(self) }
    }

    /// Get the samples stored in the buffer
    ///
    /// Panic if the sample count exceeds usize range
    #[must_use]
    pub fn samples(&self) -> &[i16] {
        let len: usize = self
            .sample_count()
            .try_into()
            .expect("Overflow when casting sample count to usize");
        unsafe { slice::from_raw_parts(ffi::audio::sfSoundBuffer_getSamples(self), len) }
    }

    /// Get the number of channels used by a sound buffer
    ///
    /// If the sound is mono then the number of channels will
    /// be 1, 2 for stereo, etc.
    ///
    /// Return the number of channels
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::audio::sfSoundBuffer_getChannelCount(self) }
    }

    /// Get the total duration of a sound buffer
    ///
    /// Return the sound duration
    #[must_use]
    pub fn duration(&self) -> Time {
        unsafe { Time::from_raw(ffi::audio::sfSoundBuffer_getDuration(self)) }
    }

    /// Get the sample rate of a sound buffer
    ///
    /// The sample rate is the number of samples played per second.
    /// The higher, the better the quality (for example, 44100
    /// samples/s is CD quality).
    ///
    /// Return the sample rate (number of samples per second)
    #[must_use]
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::audio::sfSoundBuffer_getSampleRate(self) }
    }
    /// Create a new sound buffer and load it from a file
    ///
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * filename - Path of the sound file to load
    ///
    /// Returns `None` on failure.
    pub fn from_file(filename: &str) -> SfResult<SfBox<Self>> {
        let c_str = CString::new(filename).unwrap();
        let sound_buffer: *mut ffi::audio::sfSoundBuffer =
            unsafe { ffi::audio::sfSoundBuffer_createFromFile(c_str.as_ptr()) };
        SfBox::new(sound_buffer).ok_or(SfError::CallFailed)
    }
    /// Load the sound buffer from a file in memory.
    pub fn from_memory(data: &[u8]) -> SfResult<SfBox<Self>> {
        let sound_buffer =
            unsafe { ffi::audio::sfSoundBuffer_createFromMemory(data.as_ptr() as _, data.len()) };
        SfBox::new(sound_buffer).ok_or(SfError::CallFailed)
    }
    /// Load the sound buffer from a custom stream.
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> SfResult<SfBox<Self>> {
        let mut stream = InputStream::new(stream);
        let buffer = unsafe { ffi::audio::sfSoundBuffer_createFromStream(&mut *stream.stream) };
        SfBox::new(buffer).ok_or(SfError::CallFailed)
    }
    /// Load the sound buffer from a slice of audio samples.
    ///
    /// The assumed format of the audio samples is 16 bits signed integer.
    pub fn from_samples(
        samples: &[i16],
        channel_count: u32,
        sample_rate: u32,
    ) -> SfResult<SfBox<Self>> {
        let buffer = unsafe {
            ffi::audio::sfSoundBuffer_createFromSamples(
                samples.as_ptr(),
                samples.len() as _,
                channel_count,
                sample_rate,
            )
        };
        SfBox::new(buffer).ok_or(SfError::CallFailed)
    }
}

impl ToOwned for SoundBuffer {
    type Owned = SfBox<Self>;

    fn to_owned(&self) -> Self::Owned {
        let sound_buffer = unsafe { ffi::audio::sfSoundBuffer_copy(self) };
        SfBox::new(sound_buffer).expect("Failed to copy SoundBuffer")
    }
}

impl Dispose for SoundBuffer {
    unsafe fn dispose(&mut self) {
        unsafe {
            ffi::audio::sfSoundBuffer_destroy(self);
        }
    }
}
