use csfml_audio_sys::*;
use csfml_system_sys::*;
use ext::sf_bool_ext::SfBoolExt;
use raw_conv::FromRaw;
use system::Time;
use std::panic;
use std::marker::PhantomData;
use audio::SoundStatus;

/// Trait for streamed audio sources.
pub trait SoundStream {
    /// Request a new chunk of audio samples from the stream source.
    ///
    /// Returns (`chunk`, `keep_playing`), where `chunk` is the chunk of audio samples,
    /// and `keep_playing` tells the streaming loop whether to keep playing or to stop.
    fn get_data(&mut self) -> (&mut [i16], bool);
    /// Change the current playing position in the stream source.
    fn seek(&mut self, offset: Time);
}

/// Player for custom streamed audio sources. See `SoundStream`.
pub struct SoundStreamPlayer<'a, S: SoundStream + 'a> {
    sf_sound_stream: *mut sfSoundStream,
    _borrow: PhantomData<&'a mut S>,
}

unsafe extern "C" fn get_data_callback<S: SoundStream>(chunk: *mut sfSoundStreamChunk,
                                                       stream: *mut S)
                                                       -> sfBool {
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

unsafe extern "C" fn seek_callback<S: SoundStream>(offset: sfTime, stream: *mut S) {
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
    pub fn new(sound_stream: &mut S, channels: u32, sample_rate: u32) -> Self {
        let get_data_callback =
            get_data_callback::<S> as unsafe extern "C" fn(*mut sfSoundStreamChunk, *mut S) -> sfBool;
        let seek_callback = seek_callback::<S> as unsafe extern "C" fn(sfTime, *mut S);
        SoundStreamPlayer {
            sf_sound_stream: unsafe {
                sfSoundStream_create(Some(::std::mem::transmute(get_data_callback)),
                                     Some(::std::mem::transmute(seek_callback)),
                                     channels, // channel count
                                     sample_rate, // sample rate
                                     sound_stream as *mut SoundStream as *mut _)
            },
            _borrow: PhantomData,
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
    /// Stop playing the audio stream.
    ///
    /// This function stops the stream if it was playing or paused, and does nothing if it was
    /// already stopped. It also resets the playing position (unlike pause()).
    pub fn stop(&mut self) {
        unsafe {
            sfSoundStream_stop(self.sf_sound_stream);
        }
    }
}

impl<'a, S: SoundStream> Drop for SoundStreamPlayer<'a, S> {
    fn drop(&mut self) {
        unsafe {
            sfSoundStream_destroy(self.sf_sound_stream);
        }
    }
}
