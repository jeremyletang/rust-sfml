use crate::{
    audio::{SoundSource, SoundStatus, TimeSpan},
    ffi::{self, sfBool},
    inputstream::InputStream,
    sf_bool_ext::SfBoolExt,
    system::{Time, Vector3f},
};
use std::{
    ffi::CString,
    io::{Read, Seek},
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

///
#[derive(Debug)]
pub struct Music {
    music: *mut ffi::sfMusic,
}

impl Music {
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
    /// Returns `None` if loading fails.
    ///
    /// [`play`]: Music::play
    #[must_use]
    pub fn from_file(filename: &str) -> Option<Music> {
        let c_str = CString::new(filename).unwrap();
        let music_tmp: *mut ffi::sfMusic = unsafe { ffi::sfMusic_createFromFile(c_str.as_ptr()) };
        if music_tmp.is_null() {
            None
        } else {
            Some(Music { music: music_tmp })
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
    /// Returns `None` if loading fails.
    ///
    /// [`play`]: Music::play
    pub fn from_stream<T: Read + Seek>(stream: &mut T) -> Option<Music> {
        let mut input_stream = InputStream::new(stream);
        let music_tmp: *mut ffi::sfMusic =
            unsafe { ffi::sfMusic_createFromStream(&mut input_stream.0) };
        if music_tmp.is_null() {
            None
        } else {
            Some(Music { music: music_tmp })
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
    /// Returns `None` if loading fails.
    ///
    /// [`play`]: Music::play
    #[must_use]
    pub fn from_memory(mem: &[u8]) -> Option<Music> {
        let music_tmp =
            unsafe { ffi::sfMusic_createFromMemory(mem.as_ptr() as *const _, mem.len()) };
        if music_tmp.is_null() {
            None
        } else {
            Some(Music { music: music_tmp })
        }
    }

    /// Sets whether this music should loop or not.
    ///
    /// If `true`, the music will restart from beginning after
    /// reaching the end and so on, until it is stopped or
    /// `set_looping(false)` is called.
    ///
    /// By default, the music will *not* loop.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { ffi::sfMusic_setLoop(self.music, sfBool::from_bool(looping)) }
    }

    /// Tell whether or not a music is in loop mode
    ///
    /// Return true if the music is looping, false otherwise
    #[must_use]
    pub fn is_looping(&self) -> bool {
        unsafe { ffi::sfMusic_getLoop(self.music) }.into_bool()
    }

    /// Get the total duration of a music
    ///
    /// Return Music duration
    #[must_use]
    pub fn duration(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfMusic_getDuration(self.music)) }
    }

    /// Start or resume playing a music
    ///
    /// This function starts the music if it was stopped, resumes
    /// it if it was paused, and restarts it from beginning if it
    /// was it already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the music is played.
    pub fn play(&mut self) {
        unsafe { ffi::sfMusic_play(self.music) }
    }

    /// Pause a music
    ///
    /// This function pauses the music if it was playing,
    /// otherwise (music already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe { ffi::sfMusic_pause(self.music) }
    }

    /// Stop playing a music
    ///
    /// This function stops the music if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    pub fn stop(&mut self) {
        unsafe { ffi::sfMusic_stop(self.music) }
    }

    /// Return the number of channels of a music
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
    ///
    /// Return the number of channels
    #[must_use]
    pub fn channel_count(&self) -> u32 {
        unsafe { ffi::sfMusic_getChannelCount(self.music) }
    }

    /// Get the sample rate of a music
    ///
    /// The sample rate is the number of audio samples played per
    /// second. The higher, the better the quality.
    ///
    /// Return the sample rate, in number of samples per second
    #[must_use]
    pub fn sample_rate(&self) -> u32 {
        unsafe { ffi::sfMusic_getSampleRate(self.music) }
    }

    /// Get the current status of a music (stopped, paused, playing)
    ///
    /// Return current status
    #[must_use]
    pub fn status(&self) -> SoundStatus {
        unsafe { SoundStatus(ffi::sfMusic_getStatus(self.music)) }
    }

    /// Get the current playing position of a music
    ///
    /// Return the current playing position
    #[must_use]
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(ffi::sfMusic_getPlayingOffset(self.music)) }
    }

    /// Change the current playing position of a music
    ///
    /// The playing position can be changed when the music is
    /// either paused or playing.
    ///
    /// # Arguments
    /// * timeOffset - New playing position
    pub fn set_playing_offset(&mut self, time_offset: Time) {
        unsafe { ffi::sfMusic_setPlayingOffset(self.music, time_offset.raw()) }
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
        TimeSpan::from_raw(unsafe { ffi::sfMusic_getLoopPoints(self.music) })
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
        unsafe { ffi::sfMusic_setLoopPoints(self.music, time_points.into_raw()) }
    }
}

impl SoundSource for Music {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { ffi::sfMusic_setPitch(self.music, pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { ffi::sfMusic_setVolume(self.music, volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { ffi::sfMusic_setPosition(self.music, position.into().raw()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe { ffi::sfMusic_setRelativeToListener(self.music, sfBool::from_bool(relative)) }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { ffi::sfMusic_setMinDistance(self.music, distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { ffi::sfMusic_setAttenuation(self.music, attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { ffi::sfMusic_getPitch(self.music) }
    }
    fn volume(&self) -> f32 {
        unsafe { ffi::sfMusic_getVolume(self.music) }
    }
    fn position(&self) -> Vector3f {
        unsafe { Vector3f::from_raw(ffi::sfMusic_getPosition(self.music)) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { ffi::sfMusic_isRelativeToListener(self.music).into_bool() }
    }
    fn min_distance(&self) -> f32 {
        unsafe { ffi::sfMusic_getMinDistance(self.music) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { ffi::sfMusic_getAttenuation(self.music) }
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe {
            ffi::sfMusic_destroy(self.music);
        }
    }
}
