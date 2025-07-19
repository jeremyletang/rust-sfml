use crate::system::{Angle, Vector3f};

use std::f32::consts::PI;

/// Structure defining the properties of a directional cone
///
/// Sounds will play at gain 1 when they are positioned
/// within the inner angle of the cone. Sounds will play
/// at `outerGain` when they are positioned outside the
/// outer angle of the cone. The gain declines linearly
/// from 1 to `outerGain` as the sound moves from the inner
/// angle to the outer angle.
#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cone {
    /// Inner angle, in degrees
    pub inner_angle: Angle,
    /// Outer angle, in degrees
    pub outer_angle: Angle,
    /// Outer gain
    pub outer_gain: f32,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            inner_angle: Angle::degrees(2. * PI),
            outer_angle: Angle::degrees(2. * PI),
            outer_gain: 1.,
        }
    }
}

/// Enumeration of statuses for sounds and musics
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    /// Sound is not playing
    #[default]
    Stopped,
    /// Sound is paused
    Paused,
    /// Sound is playing
    Playing,
}

/// Callable that is provided with sound data for processing
///
/// When the audio engine sources sound data from sound
/// sources it will pass the data through an effects
/// processor if one is set. The sound data will already be
/// converted to the internal floating point format.
///
/// Sound data that is processed this way is provided in
/// frames. Each frame contains 1 floating point sample per
/// channel. If e.g. the data source provides stereo data,
/// each frame will contain 2 floats.
///
/// The effects processor function takes 4 parameters:
///   - The input data frames, channels interleaved
///   - The number of input data frames available
///   - The buffer to write output data frames to, channels interleaved
///   - The number of output data frames that the output buffer can hold
///   - The channel count
///
/// The input and output frame counts are in/out parameters.
///
/// When this function is called, the input count will
/// contain the number of frames available in the input
/// buffer. The output count will contain the size of the
/// output buffer i.e. the maximum number of frames that
/// can be written to the output buffer.
///
/// Attempting to read more frames than the input frame
/// count or write more frames than the output frame count
/// will result in undefined behaviour.
///
/// It is important to note that the channel count of the
/// audio engine currently sourcing data from this sound
/// will always be provided in `frameChannelCount`. This can
/// be different from the channel count of the sound source
/// so make sure to size necessary processing buffers
/// according to the engine channel count and not the sound
/// source channel count.
///
/// When done processing the frames, the input and output
/// frame counts must be updated to reflect the actual
/// number of frames that were read from the input and
/// written to the output.
///
/// The processing function should always try to process as
/// much sound data as possible i.e. always try to fill the
/// output buffer to the maximum. In certain situations for
/// specific effects it can be possible that the input frame
/// count and output frame count aren't equal. As long as
/// the frame counts are updated accordingly this is
/// perfectly valid.
///
/// If the audio engine determines that no audio data is
/// available from the data source, the input data frames
/// pointer is set to `None` and the input frame count is
/// set to 0. In this case it is up to the function to
/// decide how to handle the situation. For specific effects
/// e.g. Echo/Delay buffered data might still be able to be
/// written to the output buffer even if there is no longer
/// any input data.
///
/// An important thing to remember is that this function is
/// directly called by the audio engine. Because the audio
/// engine runs on an internal thread of its own, make sure
/// access to shared data is synchronized appropriately.
///
/// Because this function is stored by the `SoundSource`
/// object it will be able to be called as long as the
/// `SoundSource` object hasn't yet been destroyed. Make sure
/// that any data this function references outlives the
/// `SoundSource` object otherwise use-after-free errors will
/// occur.
pub type EffectProcessor =
    Option<Box<dyn FnMut(&[f32], &mut [f32], &mut u32, &mut u32, u32) + Send + 'static>>;

/// Base trait defining a sound's properties.
pub trait SoundSource {
    /// Set the pitch of the sound.
    ///
    /// The pitch represents the perceived fundamental frequency of a sound;
    /// thus you can make a sound more acute or grave by changing its pitch.
    /// A side effect of changing the pitch is to modify the playing speed of the sound as well.
    /// The default value for the pitch is 1.
    ///
    /// # Parameters
    /// pitch - New pitch to apply to the sound
    fn set_pitch(&mut self, pitch: f32);

    /// Set the pan of the sound
    ///
    /// Using panning, a mono sound can be panned between
    /// stereo channels. When the pan is set to -1, the sound
    /// is played only on the left channel, when the pan is set
    /// to +1, the sound is played only on the right channel.
    ///
    /// # Parameters
    /// pan - New pan to apply to the sound [-1, +1]
    fn set_pan(&mut self, pan: f32);

    /// Set the volume of the sound.
    ///
    /// The volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    ///
    /// # Parameters
    /// volume - Volume of the sound
    fn set_volume(&mut self, volume: f32);

    /// Set whether spatialization of the sound is enabled
    ///
    /// Spatialization is the application of various effects to
    /// simulate a sound being emitted at a virtual position in
    /// 3D space and exhibiting various physical phenomena such as
    /// directional attenuation and doppler shift.
    ///
    /// # Parameters
    /// enabled - `true` to enable spatialization, `false` to disable
    fn set_spatialization_enabled(&mut self, enabled: bool);

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be spatialized.
    /// The default position of a sound is (0, 0, 0).
    ///
    /// # Parameters
    /// position - Position of the sound in the scene
    fn set_position<P: Into<Vector3f>>(&mut self, position: P);

    /// The direction defines where the sound source is facing
    /// in 3D space. It will affect how the sound is attenuated
    /// if facing away from the listener.
    /// The default direction of a sound is (0, 0, -1).
    ///
    /// # Parameters
    /// direction - Direction of the sound in the scene
    fn set_direction<P: Into<Vector3f>>(&mut self, direction: P);

    /// Set the cone properties of the sound in the audio scene
    ///
    /// The cone defines how directional attenuation is applied.
    /// The default cone of a sound is (2 * PI, 2 * PI, 1).
    ///
    /// # Parameters
    /// cone - Cone properties of the sound in the scene
    fn set_cone(&mut self, cone: Cone);

    /// Set the 3D velocity of the sound in the audio scene
    ///
    /// The velocity is used to determine how to doppler shift
    /// the sound. Sounds moving towards the listener will be
    /// perceived to have a higher pitch and sounds moving away
    /// from the listener will be perceived to have a lower pitch.
    ///
    /// # Parameters
    /// velocity - Velocity of the sound in the scene
    fn set_velocity<P: Into<Vector3f>>(&mut self, velocity: P);

    /// Set the doppler factor of the sound
    ///
    /// The doppler factor determines how strong the doppler
    /// shift will be.
    ///
    /// # Parameters
    /// factor - New doppler factor to apply to the sound
    fn set_doppler_factor(&mut self, factor: f32);

    /// Set the directional attenuation factor of the sound
    ///
    /// Depending on the virtual position of an output channel
    /// relative to the listener (such as in surround sound
    /// setups), sounds will be attenuated when emitting them
    /// from certain channels. This factor determines how strong
    /// the attenuation based on output channel position
    /// relative to the listener is.
    ///
    /// # Parameters
    /// factor - New directional attenuation factor to apply to the sound
    fn set_directional_attenuation_factor(&mut self, factor: f32);

    /// Make the sound's position relative to the listener or absolute.
    ///
    /// Making a sound relative to the listener will ensure that it will always be played the
    /// same way regardless of the position of the listener. This can be useful for
    /// non-spatialized sounds, sounds that are produced by the listener, or sounds attached to it.
    /// The default value is false (position is absolute).
    ///
    /// # Parameters
    /// relative - True to set the position relative, false to set it absolute
    fn set_relative_to_listener(&mut self, relative: bool);

    /// Set the minimum distance of the sound.
    ///
    /// The "minimum distance" of a sound is the maximum distance at which it is heard at its
    /// maximum volume. Further than the minimum distance, it will start to fade out according to
    /// its attenuation factor. A value of 0 ("inside the head of the listener") is an invalid
    /// value and is forbidden. The default value of the minimum distance is 1.
    ///
    /// # Parameters
    /// distance - New minimum distance of the sound
    fn set_min_distance(&mut self, distance: f32);

    /// Set the maximum distance of the sound
    ///
    /// The "maximum distance" of a sound is the minimum
    /// distance at which it is heard at its minimum volume. Closer
    /// than the maximum distance, it will start to fade in according
    /// to its attenuation factor.
    /// The default value of the maximum distance is the maximum
    /// value a float can represent.
    ///
    /// # Parameters
    /// distance - New maximum distance of the sound
    fn set_max_distance(&mut self, distance: f32);

    /// Set the minimum gain of the sound
    ///
    /// When the sound is further away from the listener than
    /// the "maximum distance" the attenuated gain is clamped
    /// so it cannot go below the minimum gain value.
    ///
    /// # Parameters
    /// gain - New minimum gain of the sound
    fn set_min_gain(&mut self, gain: f32);

    /// Set the maximum gain of the sound
    ///
    /// When the sound is closer from the listener than
    /// the "minimum distance" the attenuated gain is clamped
    /// so it cannot go above the maximum gain value.
    ///
    /// # Parameters
    /// gain - New maximum gain of the sound
    fn set_max_gain(&mut self, gain: f32);

    /// Set the attenuation factor of the sound.
    ///
    /// The attenuation is a multiplicative factor which makes the sound more or less loud
    /// according to its distance from the listener. An attenuation of 0 will produce a
    /// non-attenuated sound, i.e. its volume will always be the same whether it is heard from
    /// near or from far. On the other hand, an attenuation value such as 100 will make the sound
    /// fade out very quickly as it gets further from the listener.
    /// The default value of the attenuation is 1.
    ///
    /// # Parameters
    /// attenuation - New attenuation factor of the sound
    fn set_attenuation(&mut self, attenuation: f32);

    /// Set the effect processor to be applied to the sound
    ///
    /// The effect processor is a callable that will be called
    /// with sound data to be processed.
    ///
    /// # Parameters
    /// effectProcessor - The effect processor to attach to this sound, attach an empty processor to disable processing    
    fn set_effect_processor(&mut self, effect_processor: EffectProcessor);

    /// Get the pitch of the sound.
    #[must_use]
    fn pitch(&self) -> f32;

    /// Get the pan of the sound
    #[must_use]
    fn pan(&self) -> f32;

    /// Get the volume of the sound.
    ///
    /// Returns the volume of the sound, in the range [0, 100]
    #[must_use]
    fn volume(&self) -> f32;

    /// Returns whether spatialization of the sound is enabled
    #[must_use]
    fn is_spatialization_enabled(&self) -> bool;

    /// Get the 3D position of the sound in the audio scene.
    #[must_use]
    fn position(&self) -> Vector3f;

    /// Get the 3D direction of the sound in the audio scene
    #[must_use]
    fn direction(&self) -> Vector3f;

    /// Get the cone properties of the sound in the audio scene
    #[must_use]
    fn cone(&self) -> Cone;

    /// Get the 3D velocity of the sound in the audio scene
    #[must_use]
    fn velocity(&self) -> Vector3f;

    /// Get the doppler factor of the sound
    #[must_use]
    fn doppler_factor(&self) -> f32;

    /// Get the directional attenuation factor of the sound
    #[must_use]
    fn directional_attenuation_factor(&self) -> f32;

    /// Tell whether the sound's position is relative to the listener or is absolute.
    #[must_use]
    fn is_relative_to_listener(&self) -> bool;

    /// Get the minimum distance of the sound.
    #[must_use]
    fn min_distance(&self) -> f32;

    ///Get the maximum distance of the sound
    #[must_use]
    fn get_max_distance(&self) -> f32;

    ///Get the minimum gain of the sound
    #[must_use]
    fn get_min_gain(&self) -> f32;

    ///Get the maximum gain of the sound
    #[must_use]
    fn get_max_gain(&self) -> f32;

    /// Get the attenuation factor of the sound.
    #[must_use]
    fn attenuation(&self) -> f32;

    /// Get the current status of a sound (stopped, paused, playing)
    #[must_use]
    fn status(&self) -> Status;
}
