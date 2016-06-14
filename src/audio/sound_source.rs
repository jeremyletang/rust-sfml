// Rust-SFML - Copyright (c) 2013 Letang Jeremy.
//
// The original software, SFML library, is provided by Laurent Gomila.
//
// This software is provided 'as-is', without any express or implied warranty.
// In no event will the authors be held liable for any damages arising from
// the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not claim
//    that you wrote the original software. If you use this software in a product,
//    an acknowledgment in the product documentation would be appreciated but is
//    not required.
//
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
//
// 3. This notice may not be removed or altered from any source distribution.
//

//! Base trait for sounds source (music and sounds)

use system::Vector3f;

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

    /// Set the volume of the sound.
    ///
    /// The volume is a value between 0 (mute) and 100 (full volume).
    /// The default value for the volume is 100.
    ///
    /// # Parameters
    /// volume - Volume of the sound
    fn set_volume(&mut self, volume: f32);

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be spatialized.
    /// The default position of a sound is (0, 0, 0).
    ///
    /// # Parameters
    /// x - X coordinate of the position of the sound in the scene
    /// y - Y coordinate of the position of the sound in the scene
    /// z - Z coordinate of the position of the sound in the scene
    fn set_position3f(&mut self, x: f32, y: f32, z: f32);

    /// Set the 3D position of the sound in the audio scene.
    ///
    /// Only sounds with one channel (mono sounds) can be spatialized.
    /// The default position of a sound is (0, 0, 0).
    ///
    /// # Parameters
    /// position - Position of the sound in the scene
    fn set_position(&mut self, position: &Vector3f);

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

    /// Get the pitch of the sound.
    fn get_pitch(&self) -> f32;

    /// Get the volume of the sound.
    ///
    /// Returns the volume of the sound, in the range [0, 100]
    fn get_volume(&self) -> f32;

    /// Get the 3D position of the sound in the audio scene.
    fn get_position(&self) -> Vector3f;

    /// Tell whether the sound's position is relative to the listener or is absolute.
    fn is_relative_to_listener(&self) -> bool;

    /// Get the minimum distance of the sound.
    fn get_min_distance(&self) -> f32;

    /// Get the attenuation factor of the sound.
    fn get_attenuation(&self) -> f32;
}
