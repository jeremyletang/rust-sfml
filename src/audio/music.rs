use {
    crate::{
        audio::{SoundSource, SoundStatus, TimeSpan},
        ffi::{self},
        system::{InputStream, Time, Vector3f},
        IntoSfResult, SfError, SfResult,
    },
    std::{
        ffi::CString,
        io::{Read, Seek},
        marker::PhantomData,
    },
};

/// Streamed music played from an audio file.
///
/// `Music`s are sounds that are streamed rather than completely loaded in memory.
///
/// This is especially useful for compressed musics that usually take hundreds of MB when they are
/// uncompressed: by streaming it instead of loading it entirely, you avoid saturating the memory
/// and have almost no loading delay. This implies that the underlying resource
/// (file, stream or memory buffer) must remain valid for the lifetime of the `Music` object.
///
/// Apart from that, a `Music` has almost the same features as the
/// [`SoundBuffer`] / [`Sound`] pair: you can play/pause/stop it, request its parameters
/// (channels, sample rate), change the way it is played (pitch, volume, 3D position, ...), etc.
///
/// As a sound stream, a music is played in its own thread in order not to block the rest of the
/// program. This means that you can leave the music alone after calling [`play`],
/// it will manage itself very well.
///
/// # Usage example
///
/// ```no_run
/// use sfml::audio::{Music, SoundSource};
///
/// // Open a new music from an audio file
/// let mut music = Music::from_file("music.ogg").unwrap();
///
/// // Change some parameters
/// music.set_position((0., 1., 10.)); // change its 3D position
/// music.set_pitch(2.); // increase the pitch
/// music.set_volume(50.); // reduce the volume
/// music.set_looping(true); // make it loop
///
/// // Play it
/// music.play();
/// ```
///
/// [`play`]: Music::play
/// [`SoundBuffer`]: crate::audio::SoundBuffer
/// [`Sound`]: crate::audio::Sound
#[derive(Debug)]
pub struct Music<'stream> {
    music: *mut ffi::audio::sfMusic,
    _stream: PhantomData<&'stream mut ()>,
}

/// Creating and loading
impl<'stream> Music<'stream> {
    /// Create a new music and load it from a file
    ///
    /// This function doesn't start playing the music (call [`play`] to do so).
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * filename - Path of the music file to open
    ///
    /// [`play`]: Music::play
    pub fn from_file(filename: &str) -> SfResult<Self> {
        let c_str = CString::new(filename).into_sf_result()?;
        let music_tmp: *mut ffi::audio::sfMusic =
            unsafe { ffi::audio::sfMusic_createFromFile(c_str.as_ptr()) };
        if music_tmp.is_null() {
            Err(SfError::CallFailed)
        } else {
            Ok(Self {
                music: music_tmp,
                _stream: PhantomData,
            })
        }
    }

    /// Create a new music and load it from a stream (a struct implementing Read and Seek)
    ///
    /// This function doesn't start playing the music (call [`play`] to do so).
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * stream - Your struct, implementing Read and Seek
    ///
    /// [`play`]: Music::play
    pub fn from_stream<T: Read + Seek>(stream: &'stream mut InputStream<T>) -> SfResult<Self> {
        let music_tmp: *mut ffi::audio::sfMusic =
            unsafe { ffi::audio::sfMusic_createFromStream(&mut *stream.stream) };
        if music_tmp.is_null() {
            Err(SfError::CallFailed)
        } else {
            Ok(Music {
                music: music_tmp,
                _stream: PhantomData,
            })
        }
    }

    /// Create a new music and load it from memory
    ///
    /// This function doesn't start playing the music (call [`play`] to do so).
    /// Here is a complete list of all the supported audio formats:
    /// ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    /// w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    ///
    /// # Arguments
    /// * mem - Pointer to the file data in memory
    ///
    /// [`play`]: Music::play
    pub fn from_memory(mem: &[u8]) -> SfResult<Self> {
        let music_tmp =
            unsafe { ffi::audio::sfMusic_createFromMemory(mem.as_ptr() as *const _, mem.len()) };
        if music_tmp.is_null() {
            Err(SfError::CallFailed)
        } else {
            Ok(Music {
                music: music_tmp,
                _stream: PhantomData,
            })
        }
    }
}

/// Playback
impl Music<'_> {
    /// Start or resume playing a music
    ///
    /// This function starts the music if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the music is played.
    pub fn play(&mut self) {
        unsafe { ffi::audio::sfMusic_play(self.music) }
    }

    /// Pause a music
    ///
    /// This function pauses the music if it was playing,
    /// otherwise (music already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::audio::sfMusic_pause(self.music) }
    }

    /// Stop playing a music
    ///
    /// This function stops the music if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::audio::sfMusic_stop(self.music) }
    }
}

/// Query properties
impl Music<'_> {
    /// Tell whether or not a music is in loop mode
    ///
    /// Return true if the music is looping, false otherwise
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { ffi::audio::sfMusic_getLoop(self.music) }
    }
    /// Get the total duration of a music
    ///
    /// Return Music duration
    #[must_use]
    pub fn duration(&self) -> Time {
        unsafe { Time::from_raw(ffi::audio::sfMusic_getDuration(self.music)) }
    }
    /// Return the number of channels of a music
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
    ///
    /// Return the number of channels
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::audio::sfMusic_getChannelCount(self.music) }
    }
    /// Get the sample rate of a music
    ///
    /// The sample rate is the number of audio samples played per
    /// second. The higher, the better the quality.
    ///
    /// Return the sample rate, in number of samples per second
    #[must_use]
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::audio::sfMusic_getSampleRate(self.music) }
    }

    /// Get the current status of a music (stopped, paused, playing)
    ///
    /// Return current status
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { SoundStatus(ffi::audio::sfMusic_getStatus(self.music)) }
    }

    /// Get the current playing position of a music
    ///
    /// Return the current playing position
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(ffi::audio::sfMusic_getPlayingOffset(self.music)) }
    }

    /// Get the positions of the of the music's looping sequence.
    ///
    /// # Warning
    /// Since [`set_loop_points`] performs some adjustments on the provided values and
    /// rounds them to internal samples, a call to [`loop_points`] is not guaranteed to
    /// return the same times passed into a previous call to [`set_loop_points`].
    /// However, it is guaranteed to return times that will map to the
    /// valid internal samples of this [`Music`] if they are later passed to [`set_loop_points`].
    ///
    /// [`set_loop_points`]: Music::set_loop_points
    /// [`loop_points`]: Music::loop_points
    #[must_use]
    pub fn loop_points(&self) -> TimeSpan {
        TimeSpan::from_raw(unsafe { ffi::audio::sfMusic_getLoopPoints(self.music) })
    }
}

/// Set properties
impl Music<'_> {
    /// Sets whether this music should loop or not.
    ///
    /// If `true`, the music will restart from beginning after
    /// reaching the end and so on, until it is stopped or
    /// `set_looping(false)` is called.
    ///
    /// By default, the music will *not* loop.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { ffi::audio::sfMusic_setLoop(self.music, looping) }
    }
    /// Change the current playing position of a music
    ///
    /// The playing position can be changed when the music is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::audio::sfMusic_setPlayingOffset(self.music, time_offset.raw()) }
    }
    /// Sets the beginning and end of the music's looping sequence.
    ///
    /// Loop points allow one to specify a pair of positions such that, when the music is
    /// enabled for looping, it will seamlessly seek to the beginning whenever it encounters
    /// the end. Valid ranges for timePoints.offset and timePoints.length are
    /// `[0, Dur)` and `(0, Dur-offset]` respectively, where `Dur` is the value returned by
    /// `duration`. Note that the EOF "loop point" from the end to the beginning of the
    /// stream is still honored, in case the caller seeks to a point after the end of the
    /// loop range. This function can be safely called at any point after a stream is opened,
    /// and will be applied to a playing sound without affecting the current playing offset.
    pub fn set_loop_points(&mut self, time_points: TimeSpan) {
        unsafe { ffi::audio::sfMusic_setLoopPoints(self.music, time_points.into_raw()) }
    }
}

impl SoundSource for Music<'_> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::audio::sfMusic_setPitch(self.music, pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::audio::sfMusic_setVolume(self.music, volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { ffi::audio::sfMusic_setPosition(self.music, position.into()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::audio::sfMusic_setRelativeToListener(self.music, relative) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::audio::sfMusic_setMinDistance(self.music, distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::audio::sfMusic_setAttenuation(self.music, attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { ffi::audio::sfMusic_getPitch(self.music) }
    }
    fn volume(&self) -> f32 {
        unsafe { ffi::audio::sfMusic_getVolume(self.music) }
    }
    fn position(&self) -> Vector3f {
        unsafe { ffi::audio::sfMusic_getPosition(self.music) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::audio::sfMusic_isRelativeToListener(self.music) }
    }
    fn min_distance(&self) -> f32 {
        unsafe { ffi::audio::sfMusic_getMinDistance(self.music) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { ffi::audio::sfMusic_getAttenuation(self.music) }
    }
}

impl Drop for Music<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::audio::sfMusic_destroy(self.music);
        }
    }
}
