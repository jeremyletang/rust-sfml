/*
* Rust-SFML - Copyright (c) 2013 Letang Jeremy.
*
* The original software, SFML library, is provided by Laurent Gomila.
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/

/*!
* Play sounds.
*
* Regular sound that can be played in the audio environment.
*
*/

use std::libc::{c_float};
use std::ptr;

use system::time;
use audio::sound_status;
use audio::sound_buffer::SoundBuffer;
use system::vector3::Vector3f;

#[doc(hidden)]
pub mod csfml {
    
    use std::libc::{c_float, c_void};

    use audio::sound_status;
    use audio::sound_buffer;
    use system::time;
    use rsfml::sfTypes::{sfBool};
    use system::vector3::Vector3f;

    pub struct sfSound {
        This : *c_void,
        This2 : *c_void
    }
    
    pub extern "C" {
        fn sfSound_create() -> *sfSound;
        fn sfSound_copy(sound : *sfSound) -> *sfSound;
        fn sfSound_destroy(sound : *sfSound) -> ();
        fn sfSound_play(sound : *sfSound) -> ();
        fn sfSound_pause(sound : *sfSound) -> ();
        fn sfSound_stop(sound : *sfSound) -> ();
        fn sfSound_setBuffer(sound : *sfSound, buffer : *sound_buffer::csfml::sfSoundBuffer) -> (); // a faire
        fn sfSound_getBuffer(sound : *sfSound) -> *sound_buffer::csfml::sfSoundBuffer; // a faire
        fn sfSound_setLoop(sound : *sfSound, lloop : sfBool) -> ();
        fn sfSound_getLoop(sound : *sfSound) -> sfBool;
        fn sfSound_getStatus(sound : *sfSound) -> sound_status::csfml::sfSoundStatus;
        fn sfSound_setPitch(sound : *sfSound, pitch : c_float) -> ();
        fn sfSound_setVolume(sound : *sfSound, volume : c_float) -> ();
        fn sfSound_setPosition(sound : *sfSound, position : Vector3f) -> ();
        fn sfSound_setRelativeToListener(sound : *sfSound, relative : sfBool) -> ();
        fn sfSound_setMinDistance(sound : *sfSound, distance : c_float) -> ();
        fn sfSound_setAttenuation(sound : *sfSound, attenuation : c_float) -> ();
        fn sfSound_setPlayingOffset(sound : *sfSound, timeOffset : time::csfml::sfTime) -> ();
        fn sfSound_getPitch(sound : *sfSound) -> c_float;
        fn sfSound_getVolume(sound : *sfSound) -> c_float;
        fn sfSound_getPosition(sound : *sfSound) -> Vector3f;
        fn sfSound_isRelativeToListener(sound : *sfSound) -> sfBool;
        fn sfSound_getMinDistance(sound : *sfSound) -> c_float;
        fn sfSound_getAttenuation(sound : *sfSound) -> c_float;
        fn sfSound_getPlayingOffset(sound : *sfSound) -> time::csfml::sfTime;
    }
}

#[doc(hidden)]
pub struct Sound {
    priv sound : *csfml::sfSound
}

impl Sound {
    
    /**
    * Create a new Sound
    *
    * Return a new option to Sound object or None
    */
    pub fn new() -> Option<Sound> {
        let s = unsafe {csfml::sfSound_create()};
        if s == ptr::null() {
            None
        }
        else {
            Some(Sound { sound : s})
        }
    }

    /**
    * Tell whether or not a sound is in loop mode
    *
    * Return true if the sound is looping, false otherwise
    */
    pub fn set_loop(&mut self, lloop : bool) -> () {
        unsafe {
            if lloop == true {
                csfml::sfSound_setLoop(self.sound, 1)
            }
            else {
                csfml::sfSound_setLoop(self.sound, 0)
            }
        }
    }

    /**
    * Tell whether or not a sound is in loop mode
    *
    * Return true if the sound is looping, false otherwise
    */
    pub fn get_loop(&self) -> bool {
        match unsafe {csfml::sfSound_getLoop(self.sound)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Start or resume playing a sound
    *
    * This function starts the sound if it was stopped, resumes
    * it if it was paused, and restarts it from beginning if it
    * was it already playing.
    * This function uses its own thread so that it doesn't block
    * the rest of the program while the sound is played.
    */
    pub fn play(&mut self) -> () {
        unsafe {csfml::sfSound_play(self.sound)}
    }

    /**
    * Pause a sound
    *
    * This function pauses the sound if it was playing,
    * otherwise (sound already paused or stopped) it has no effect.
    */
    pub fn pause(&mut self) -> () {
        unsafe {csfml::sfSound_pause(self.sound)}
    }

    /**
    * Stop playing a sound
    *
    * This function stops the sound if it was playing or paused,
    * and does nothing if it was already stopped.
    * It also resets the playing position (unlike pause).
    */
    pub fn stop(&mut self) -> () {
        unsafe {csfml::sfSound_stop(self.sound)}
    }

    /**
    * Get the current status of a sound (stopped, paused, playing)
    *
    * Return current status
    */
    pub fn get_status(&self) -> sound_status::Status {
        match unsafe {csfml::sfSound_getStatus(self.sound)} {
            sound_status::csfml::sfStopped => sound_status::Stopped,
            sound_status::csfml::sfPaused => sound_status::Paused,
            sound_status::csfml::sfPlaying => sound_status::Playing,
        }
    }

    /**
    * Get the current playing position of a sound
    *
    * Return the current playing position
    */
    pub fn get_playing_offset(&self) -> time::Time {
        time::Time::wrap( unsafe {csfml::sfSound_getPlayingOffset(self.sound)})
    }

    /**
    * Set the pitch of a sound
    *
    * The pitch represents the perceived fundamental frequency
    * of a sound; thus you can make a sound more acute or grave
    * by changing its pitch. A side effect of changing the pitch
    * is to modify the playing speed of the sound as well.
    * The default value for the pitch is 1.
    *
    * # Arguments
    * * pitch - new pitch to apply to the sound
    */
    pub fn set_pitch(&mut self, pitch : float) -> () {
        unsafe {csfml::sfSound_setPitch(self.sound, pitch as c_float)}
    }

    /**
    * Set the volume of a sound
    *
    * he volume is a value between 0 (mute) and 100 (full volume).
    * The default value for the volume is 100.
    *
    * # Arguments
    * * volume - Volume of the sound
    */
    pub fn set_volume(&mut self, volume : float) -> () {
        unsafe {csfml::sfSound_setVolume(self.sound, volume as c_float)}
    }

    /**
    * Make a sounds's position relative to the listener or absolute
    *
    * Making a sound relative to the listener will ensure that it will always
    * be played the same way regardless the position of the listener.
    * This can be useful for non-spatialized sounds, sounds that are
    * produced by the listener, or sounds attached to it.
    * The default value is false (position is absolute).
    *
    * # Arguments
    * * relative - true to set the position relative, false to set it absolute
    */
    pub fn set_relative_to_listener(&mut self, relative : bool) -> () {
        unsafe {
            if relative == true {
                csfml::sfSound_setRelativeToListener(self.sound, 1);
            }
            else {
                csfml::sfSound_setRelativeToListener(self.sound, 0);
            }
        }
    }

    /**
    * Set the minimum distance of a sound
    *
    * The "minimum distance" of a sound is the maximum
    * distance at which it is heard at its maximum volume. Further
    * than the minimum distance, it will start to fade out according
    * to its attenuation factor. A value of 0 ("inside the head
    * of the listener") is an invalid value and is forbidden.
    * The default value of the minimum distance is 1.
    *
    * # Arguments
    * * distance - New minimum distance of the sound
    */
    pub fn set_min_distance(&mut self, distance : float) -> () {
        unsafe {csfml::sfSound_setMinDistance(self.sound, distance as c_float)}
    }

    /**
    *  Set the attenuation factor of a sound
    *
    * The attenuation is a multiplicative factor which makes
    * the sound more or less loud according to its distance
    * from the listener. An attenuation of 0 will produce a
    * non-attenuated sound, i.e. its volume will always be the same
    * whether it is heard from near or from far. On the other hand,
    * an attenuation value such as 100 will make the sound fade out
    * very quickly as it gets further from the listener.
    * The default value of the attenuation is 1.
    *
    * # Arguments
    * * attenuation - New attenuation factor of the sound
    */
    pub fn set_attenuation(&mut self, attenuation : float) -> () {
        unsafe {csfml::sfSound_setAttenuation(self.sound, attenuation as c_float)}
    }

    /**
    * Change the current playing position of a sound
    *
    * The playing position can be changed when the sound is
    * either paused or playing.
    *
    * # Arguments
    * * timeOffset - New playing position
    */
    pub fn set_playing_offset(&mut self, timeOffset : time::Time) -> () {
        unsafe {
            csfml::sfSound_setPlayingOffset(self.sound, timeOffset.unwrap())
        }
    }

    /**
    * Get the pitch of a sound
    *
    * Return the pitch of the sound
    */
    pub fn get_pitch(&self) -> float {
        unsafe {
            csfml::sfSound_getPitch(self.sound) as float
        }
    }

    /**
    * Get the volume of a sound
    *
    * Return the volume of the sound, in the range [0, 100]
    */
    pub fn get_volume(&self) -> float {
        unsafe {
            csfml::sfSound_getVolume(self.sound) as float
        }
    }

    /**
    * Tell whether a sound's position is relative to the listener or is absolute
    *
    * Return true if the position is relative, false if it's absolute
    */
    pub fn is_relative_to_listener(&self) -> bool {
        match unsafe {csfml::sfSound_isRelativeToListener(self.sound)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Get the minimum distance of a sound
    *
    * Return the minimum distance of the sound
    */
    pub fn get_min_distance(&self) -> float {
        unsafe {
           csfml::sfSound_getMinDistance(self.sound) as float
        }
    }

    /**
    * Get the attenuation factor of a sound
    *
    * Return the attenuation factor of the sound
    */
    pub fn get_attenuation(&self) -> float {
        unsafe {
            csfml::sfSound_getAttenuation(self.sound) as float
        }
    }
    
    /**
    * Set the source buffer containing the audio data to play
    *
    * It is important to note that the sound buffer is not copied,
    * thus the sfSoundBuffer object must remain alive as long
    * as it is attached to the sound.
    *
    * # Arguments
    * * buffer - Sound buffer to attach to the sound
    */
    pub fn set_buffer(&mut self, buffer : &SoundBuffer) -> () {
        unsafe {
            csfml::sfSound_setBuffer(self.sound, buffer.unwrap())
        }
    }

    /**
    *  Get the audio buffer attached to a sound
    *
    * Return an option to Sound buffer attached to the sound or None
    */
    pub fn get_buffer(&self) -> SoundBuffer {
        SoundBuffer::wrap(unsafe {
            csfml::sfSound_getBuffer(self.sound)
        })
    }

    /**
    * Get the 3D position of a sound in the audio scene
    *
    * Return the position of the sound in the world
    */
    pub fn get_position(&self) -> Vector3f {
        unsafe {
            csfml::sfSound_getPosition(self.sound)
        }
    }

    /**
    * Set the 3D position of a sound in the audio scene
    *
    * Only sounds with one channel (mono sounds) can be
    * spatialized.
    * The default position of a sound is (0, 0, 0).
    *
    * # Arguments
    * * position - Position of the sound in the scene
    */
    pub fn set_position(&mut self, position : &Vector3f) -> () {
        unsafe {
            csfml::sfSound_setPosition(self.sound, *position)
        }
    }

    /**
    * Set the 3D position of a sound in the audio scene
    *
    * Only sounds with one channel (mono sounds) can be
    * spatialized.
    * The default position of a sound is (0, 0, 0).
    *
    * # Arguments
    * * x - X coordinate of the position of the sound in the scene
    * * y - Y coordinate of the position of the sound in the scene
    * * z - Z coordinate of the position of the sound in the scene
    */
    pub fn set_position3f(&self, x : f32, y : f32, z: f32) -> () {
        unsafe {
            csfml::sfSound_setPosition(self.sound, Vector3f::new(x, y, z))
        }
    }

    /**
    * 
    */
    #[doc(hidden)]
    pub fn unwrap(&self) -> *csfml::sfSound {
        self.sound
    }
}

impl Clone for Sound {
    fn clone(&self) -> Sound {
        unsafe {
            Sound {
                sound: csfml::sfSound_copy(self.sound)
            }
        }
    }
}

impl Drop for Sound {
    /* Destructor for class Sound. Destroy all the ressource.
    *
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfSound_destroy(self.sound);
        }
    }
}
