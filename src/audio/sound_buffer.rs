use audio::csfml_audio_sys as ffi;
use inputstream::InputStream;
use sf_bool_ext::SfBoolExt;
use std::borrow::ToOwned;
use std::ffi::CString;
use std::io::{Read, Seek};
use std::slice;
use system::{Dispose, SfBox, Time};

/// Storage for audio samples defining a sound.
///
/// A sound buffer holds the data of a sound, which is an array of audio samples.
///
/// A sample is a 16 bits signed integer that defines the amplitude of the sound at a given time.
/// The sound is then reconstituted by playing these samples at a high rate
/// (for example, 44100 samples per second is the standard rate used for playing CDs).
/// In short, audio samples are like texture pixels, and a `SoundBuffer` is
/// similar to a [`::graphics::Texture`].
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
#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum SoundBuffer {}

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
    pub fn save_to_file(&self, filename: &str) -> bool {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        unsafe { ffi::sfSoundBuffer_saveToFile(self.raw(), c_str.as_ptr()) }.to_bool()
    }

    /// Get the number of samples stored in a sound buffer
    ///
    /// The array of samples can be accessed with [`samples`](SoundBuffer::samples).
    ///
    /// Return the number of samples
    pub fn sample_count(&self) -> u64 {
        unsafe { ffi::sfSoundBuffer_getSampleCount(self.raw()) }
    }

    /// Get the samples stored in the buffer
    ///
    /// Panic if the sample count exceeds usize range
    pub fn samples(&self) -> &[i16] {
        let len = self.sample_count();
        // TODO: Replace with TryFrom, or a similar standard library API, once available
        #[cfg(target_pointer_width = 32)]
        {
            if len > usize::max_value() as u64 {
                panic!("Sample count {} too big to fit into usize", len);
            }
        }
        unsafe { slice::from_raw_parts(ffi::sfSoundBuffer_getSamples(self.raw()), len as usize) }
    }

    /// Get the number of channels used by a sound buffer
    ///
    /// If the sound is mono then the number of channels will
    /// be 1, 2 for stereo, etc.
    ///
    /// Return the number of channels
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::sfSoundBuffer_getChannelCount(self.raw()) }
    }

    /// Get the total duration of a sound buffer
    ///
    /// Return the sound duration
    pub fn duration(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfSoundBuffer_getDuration(self.raw())) }
    }

    /// Get the sample rate of a sound buffer
    ///
    /// The sample rate is the number of samples played per second.
    /// The higher, the better the quality (for example, 44100
    /// samples/s is CD quality).
    ///
    /// Return the sample rate (number of samples per second)
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::sfSoundBuffer_getSampleRate(self.raw()) }
    }
    fn raw(&self) -> *const ffi::sfSoundBuffer {
        let ptr: *const Self = self;
        ptr as *const ffi::sfSoundBuffer
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
    pub fn from_file(filename: &str) -> Option<SfBox<SoundBuffer>> {
        let c_str = CString::new(filename.as_bytes()).unwrap();
        let sound_buffer: *mut ffi::sfSoundBuffer =
            unsafe { ffi::sfSoundBuffer_createFromFile(c_str.as_ptr()) };
        if sound_buffer.is_null() {
            None
        } else {
            Some(SfBox(sound_buffer as _))
        }
    }
    /// Load the sound buffer from a file in memory.
    pub fn from_memory(data: &[u8]) -> Option<SfBox<SoundBuffer>> {
        let sound_buffer =
            unsafe { ffi::sfSoundBuffer_createFromMemory(data.as_ptr() as _, data.len()) };
        if sound_buffer.is_null() {
            None
        } else {
            Some(SfBox(sound_buffer as _))
        }
    }
    /// Load the sound buffer from a custom stream.
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<SfBox<SoundBuffer>> {
        let mut stream = InputStream::new(stream);
        let buffer = unsafe { ffi::sfSoundBuffer_createFromStream(&mut stream.0) };
        if buffer.is_null() {
            None
        } else {
            Some(SfBox(buffer as _))
        }
    }
    /// Load the sound buffer from a slice of audio samples.
    ///
    /// The assumed format of the audio samples is 16 bits signed integer.
    pub fn from_samples(
        samples: &[i16],
        channel_count: u32,
        sample_rate: u32,
    ) -> Option<SfBox<SoundBuffer>> {
        let buffer = unsafe {
            ffi::sfSoundBuffer_createFromSamples(
                samples.as_ptr(),
                samples.len() as _,
                channel_count,
                sample_rate,
            )
        };
        if buffer.is_null() {
            None
        } else {
            Some(SfBox(buffer as _))
        }
    }
}

impl ToOwned for SoundBuffer {
    type Owned = SfBox<Self>;

    fn to_owned(&self) -> Self::Owned {
        let sound_buffer = unsafe { ffi::sfSoundBuffer_copy(self.raw()) };
        assert!(!sound_buffer.is_null(), "Failed to copy SoundBuffer");
        SfBox(sound_buffer as _)
    }
}

impl Dispose for SoundBuffer {
    unsafe fn dispose(&mut self) {
        let ptr: *mut Self = self;
        ffi::sfSoundBuffer_destroy(ptr as _);
    }
}
