use audio::SoundSource;
use audio::SoundStatus;
use audio::csfml_audio_sys::*;
use csfml_system_sys::*;
use sf_bool_ext::SfBoolExt;
use std::os::raw::c_void;
use std::panic;
use system::{Time, Vector3f};

/// Trait for streamed audio sources.
pub trait SoundStream {
    /// Request a new chunk of audio samples from the stream source.
    ///
    /// Returns (`chunk`, `keep_playing`), where `chunk` is the chunk of audio samples,
    /// and `keep_playing` tells the streaming loop whether to keep playing or to stop.
    fn get_data(&mut self) -> (&mut [i16], bool);
    /// Change the current playing position in the stream source.
    fn seek(&mut self, offset: Time);
    /// Return the number of channels of the stream.
    fn channel_count(&self) -> u32;
    /// Get the stream sample rate of the stream.
    fn sample_rate(&self) -> u32;
}

/// Player for custom streamed audio sources. See `SoundStream`.
#[derive(Debug)]
pub struct SoundStreamPlayer<'a, S: SoundStream + 'a> {
    sf_sound_stream: *mut sfSoundStream,
    stream: &'a mut S,
}

unsafe extern "C" fn get_data_callback<S: SoundStream>(chunk: *mut sfSoundStreamChunk,
                                                       user_data: *mut c_void)
                                                       -> sfBool {
    let stream = user_data as *mut S;
    let (data, keep_playing) =
        match panic::catch_unwind(panic::AssertUnwindSafe(|| (*stream).get_data())) {
            Ok(ret) => ret,
            Err(_) => {
                use std::io::Write;
                let _ = writeln!(::std::io::stderr(),
                                 "sound_stream: Stopping playback beacuse `get_data` panicked.");
                (&mut [][..], false)
            }
        };
    (*chunk).samples = data.as_mut_ptr();
    (*chunk).sampleCount = data.len() as u32;
    sfBool::from_bool(keep_playing)
}

unsafe extern "C" fn seek_callback<S: SoundStream>(offset: sfTime, user_data: *mut c_void) {
    let stream = user_data as *mut S;

    let result =
        panic::catch_unwind(panic::AssertUnwindSafe(|| (*stream).seek(Time::from_raw(offset))));
    if result.is_err() {
        use std::io::Write;
        let _ = writeln!(::std::io::stderr(),
                         "sound_stream: Failed to seek because `seek` panicked.");
    }
}

impl<'a, S: SoundStream> SoundStreamPlayer<'a, S> {
    /// Create a new `SoundStreamPlayer` with the specified `SoundStream`.
    pub fn new(sound_stream: &'a mut S) -> Self {
        let ptr: *mut S = sound_stream;
        SoundStreamPlayer {
            sf_sound_stream: unsafe {
                sfSoundStream_create(Some(get_data_callback::<S>),
                                     Some(seek_callback::<S>),
                                     sound_stream.channel_count(),
                                     sound_stream.sample_rate(),
                                     ptr as *mut _)
            },
            stream: sound_stream,
        }
    }
    /// Start or resume playing the audio stream.
    pub fn play(&mut self) {
        unsafe {
            sfSoundStream_play(self.sf_sound_stream);
        }
    }
    /// Pause the audio stream.
    ///
    /// This function pauses the stream if it was playing,
    /// otherwise (stream already paused or stopped) it has no effect.
    pub fn pause(&mut self) {
        unsafe {
            sfSoundStream_pause(self.sf_sound_stream);
        }
    }
    /// Get the current status of the stream (stopped, paused, playing)
    pub fn status(&self) -> SoundStatus {
        unsafe { ::std::mem::transmute(sfSoundStream_getStatus(self.sf_sound_stream)) }
    }
    /// Stop playing, lending out the underlying `SoundStream`.
    ///
    /// This function stops the stream if it was playing or paused, and does nothing if it was
    /// already stopped. It also resets the playing position (unlike pause()).
    ///
    /// It lends out the underlying `SoundStream`, allowing it to be manipulated.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut music_stream = MusicStream::load("cool_song.ogg");
    /// let mut player = SoundStreamPlayer::new(&mut music_stream);
    /// player.play();
    /// // ...
    /// // Let's say we want to change the song being played.
    /// // We can't just simply reassign `music_stream`, since it's being borrowed by `player`.
    /// // Manipulating the stream while it's being played is _unsafe_, so it's not allowed.
    /// //
    /// // Instead, let's stop the player first, reassign the stream, then restart the player.
    /// {
    ///    let stream = player.stop();
    ///    *stream = MusicStream::load("another_cool_song.ogg");
    /// }
    /// player.play();
    /// ```
    pub fn stop(&mut self) -> &mut S {
        unsafe {
            sfSoundStream_stop(self.sf_sound_stream);
        }
        &mut self.stream
    }
    /// Get the current playing position, from the beginning of the stream
    pub fn playing_offset(&self) -> Time {
        unsafe { Time::from_raw(sfSoundStream_getPlayingOffset(self.sf_sound_stream)) }
    }
    /// Change the current playing position of the stream.
    ///
    /// The playing position can be changed when the stream is either paused or playing.
    /// Changing the playing position when the stream is stopped has no effect,
    /// since playing the stream would reset its position.
    pub fn set_playing_offset(&mut self, offset: Time) {
        unsafe { sfSoundStream_setPlayingOffset(self.sf_sound_stream, offset.raw()) }
    }
    /// Return the number of channels of the stream.
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
    pub fn channel_count(&self) -> u32 {
        unsafe { sfSoundStream_getChannelCount(self.sf_sound_stream) }
    }
    /// Get the stream sample rate of the stream.
    ///
    /// The sample rate is the number of audio samples played per second.
    /// The higher, the better the quality.
    pub fn sample_rate(&self) -> u32 {
        unsafe { sfSoundStream_getSampleRate(self.sf_sound_stream) }
    }
    /// Tell whether or not the stream is in loop mode.
    pub fn is_looping(&self) -> bool {
        unsafe { sfSoundStream_getLoop(self.sf_sound_stream).to_bool() }
    }
    /// Set whether or not the stream should loop after reaching the end.
    ///
    /// If set, the stream will restart from beginning after reaching the end and so on,
    /// until it is stopped or setLoop(false) is called.
    /// The default looping state for streams is false.
    pub fn set_looping(&mut self, looping: bool) {
        unsafe { sfSoundStream_setLoop(self.sf_sound_stream, SfBoolExt::from_bool(looping)) }
    }
}

impl<'a, S: SoundStream> SoundSource for SoundStreamPlayer<'a, S> {
    fn set_pitch(&mut self, pitch: f32) {
        unsafe { sfSoundStream_setPitch(self.sf_sound_stream, pitch) }
    }
    fn set_volume(&mut self, volume: f32) {
        unsafe { sfSoundStream_setVolume(self.sf_sound_stream, volume) }
    }
    fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        unsafe { sfSoundStream_setPosition(self.sf_sound_stream, position.into().raw()) }
    }
    fn set_relative_to_listener(&mut self, relative: bool) {
        unsafe {
            sfSoundStream_setRelativeToListener(self.sf_sound_stream,
                                                SfBoolExt::from_bool(relative))
        }
    }
    fn set_min_distance(&mut self, distance: f32) {
        unsafe { sfSoundStream_setMinDistance(self.sf_sound_stream, distance) }
    }
    fn set_attenuation(&mut self, attenuation: f32) {
        unsafe { sfSoundStream_setAttenuation(self.sf_sound_stream, attenuation) }
    }
    fn pitch(&self) -> f32 {
        unsafe { sfSoundStream_getPitch(self.sf_sound_stream) }
    }
    fn volume(&self) -> f32 {
        unsafe { sfSoundStream_getVolume(self.sf_sound_stream) }
    }
    fn position(&self) -> Vector3f {
        unsafe { Vector3f::from_raw(sfSoundStream_getPosition(self.sf_sound_stream)) }
    }
    fn is_relative_to_listener(&self) -> bool {
        unsafe { sfSoundStream_isRelativeToListener(self.sf_sound_stream).to_bool() }
    }
    fn min_distance(&self) -> f32 {
        unsafe { sfSoundStream_getMinDistance(self.sf_sound_stream) }
    }
    fn attenuation(&self) -> f32 {
        unsafe { sfSoundStream_getAttenuation(self.sf_sound_stream) }
    }
}

impl<'a, S: SoundStream> Drop for SoundStreamPlayer<'a, S> {
    fn drop(&mut self) {
        unsafe {
            // It seems there can be problems (e.g. "pure virtual method called") if the
            // stream is not stopped before it's destroyed. So let's make sure it's stopped.
            sfSoundStream_stop(self.sf_sound_stream);
            sfSoundStream_destroy(self.sf_sound_stream);
        }
    }
}
