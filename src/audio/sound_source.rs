use system::{Vector3f, Time};
use audio::SoundStatus;

/// Base trait for the properties of sounds in a scene.
///
/// All of the properties: pitch, volume, position, attenuation, etc. can be
/// changed at any time with no impact on performance.
pub trait SoundSource {
    /// Set the pitch of the sound.
    ///
    /// The pitch represents the perceived fundamental frequency
    /// of a sound; thus you can make a sound more acute or grave
    /// by changing its pitch. A side effect of changing the pitch
    /// is to modify the playing speed of the sound as well.
    /// The default value for the pitch is 1.
    fn set_pitch(&mut self, pitch: f32);

    /// Set the volume of the sound.
    ///
    /// The volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    fn set_volume(&mut self, volume: f32);

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be
    /// spatialized.
    /// The default position of a sound is (0, 0, 0).
    fn set_position(&mut self, position: &Vector3f);

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be
    /// spatialized.
    /// The default position of a sound is (0, 0, 0).
	#[inline]
    fn set_position3f(&mut self, x: f32, y: f32, z: f32) {
		self.set_position(&Vector3f::new(x, y, z))
	}

    /// Make the sounds's position relative to the listener or absolute.
    ///
    /// Making a sound relative to the listener will ensure that it will always
    /// be played the same way regardless of the position of the listener.
    /// This can be useful for non-spatialized sounds, sounds that are
    /// produced by the listener, or sounds attached to it.
    /// The default value is false (position is absolute).
    fn set_relative_to_listener(&mut self, relative: bool);

    /// Set the minimum distance of the sound.
    ///
    /// The "minimum distance" of a sound is the maximum
    /// distance at which it is heard at its maximum volume. Further
    /// than the minimum distance, it will start to fade out according
    /// to its attenuation factor. A value of 0 ("inside the head
    /// of the listener") is an invalid value and is forbidden.
    /// The default value of the minimum distance is 1.
    fn set_min_distance(&mut self, distance: f32);

    /// Set the attenuation factor of the sound.
    ///
    /// The attenuation is a multiplicative factor which makes
    /// the sound more or less loud according to its distance
    /// from the listener. An attenuation of 0 will produce a
    /// non-attenuated sound, i.e. its volume will always be the same
    /// whether it is heard from near or from far. On the other hand,
    /// an attenuation value such as 100 will make the sound fade out
    /// very quickly as it gets further from the listener.
    /// The default value of the attenuation is 1.
    fn set_attenuation(&mut self, attenuation: f32);

    /// Get the pitch of the sound.
    fn get_pitch(&self) -> f32;

    /// Get the volume of the sound, in the range [0, 100].
    fn get_volume(&self) -> f32;

    /// Get the 3D position of the sound in the audio scene.
    fn get_position(&self) -> Vector3f;

    /// Tell whether the sound's position is relative to the listener or not.
    ///
    /// Returns true if the position is relative, false if it's absolute.
    fn is_relative_to_listener(&self) -> bool;

    /// Get the minimum distance of the sound.
    fn get_min_distance(&self) -> f32;

    /// Get the attenuation factor of the sound.
    fn get_attenuation(&self) -> f32;
}

/// A `SoundSource` which also allows playback control.
pub trait PlayableSound: SoundSource {
    /// Get the current status of the sound (stopped, paused, playing).
    fn get_status(&self) -> SoundStatus;

    /// Set whether or not the sound should loop after reaching the end.
    ///
    /// If set, the sound will restart from beginning after
    /// reaching the end and so on, until it is stopped or
    /// `set_loop(false)` is called. The default looping state is false.
    fn set_loop(&mut self, lloop: bool);

    /// Tell whether or not the sound is in loop mode.
    fn get_loop(&self) -> bool;

    /// Start or resume playing the sound.
    ///
    /// This function starts the sound if it was stopped, resumes
    /// it if it was paused, and restarts it from the beginning if it
    /// was already playing.
    /// This function uses its own thread so that it doesn't block
    /// the rest of the program while the sound is played.
    fn play(&mut self);

    /// Pause the sound.
    ///
    /// This function pauses the sound if it was playing,
    /// otherwise (sound already paused or stopped) it has no effect.
    fn pause(&mut self);

    /// Stop playing the sound.
    ///
    /// This function stops the sound if it was playing or paused,
    /// and does nothing if it was already stopped.
    /// It also resets the playing position (unlike pause).
    fn stop(&mut self);

    /// Change the current playing position of the sound.
    ///
    /// The playing position can be changed when the sound is either paused
    /// or playing. Changing the playing position when the stream is stopped
	/// has no effect, since playing the stream would reset its position.
    fn set_playing_offset(&mut self, time_offset: Time);

    /// Get the current playing position of the sound.
    fn get_playing_offset(&self) -> Time;

    /// Get the number of channels of the sound.
    ///
    /// 1 channel means a mono sound, 2 means stereo, etc.
	fn get_channel_count(&self) -> u32;

    /// Get the sample rate of the sound.
    ///
    /// The sample rate is the number of audio samples played per
    /// second. The higher, the better the quality.
    fn get_sample_rate(&self) -> u32;
}
