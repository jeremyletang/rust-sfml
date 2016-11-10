use csfml_audio_sys::*;
use csfml_system_sys::*;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::FromRaw;
use system::Time;
use std::panic;
use audio::SoundStatus;
use std::os::raw::c_void;

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

    if let Err(_) = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        (*stream).seek(Time::from_raw(offset))
    })) {
        use std::io::Write;
        let _ = writeln!(::std::io::stderr(),
                         "sound_stream: Failed to seek because `seek` panicked.");
    }
}

impl<'a, S: SoundStream> SoundStreamPlayer<'a, S> {
    /// Create a new `SoundStreamPlayer` with the specified `SoundStream`.
    pub fn new(sound_stream: &'a mut S) -> Self {
        SoundStreamPlayer {
            sf_sound_stream: unsafe {
                sfSoundStream_create(Some(get_data_callback::<S>),
                                     Some(seek_callback::<S>),
                                     sound_stream.channel_count(),
                                     sound_stream.sample_rate(),
                                     sound_stream as *mut SoundStream as *mut _)
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
    /// Get the current status of the stream (stopped, paused, playing)
    pub fn get_status(&self) -> SoundStatus {
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
    pub fn get_playing_offset(&self) -> Time {
        unsafe { Time::from_raw(sfSoundStream_getPlayingOffset(self.sf_sound_stream)) }
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
