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
* Play Music
*
* Streamed music played from an audio file.
* Musics are sounds that are streamed rather than completely loaded in memory.
*
*/

use std::libc::{c_float};
use std::ptr;
use std::str;

use system::time::Time;
use audio::sound_status;
use system::vector3::Vector3f;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void, c_char, c_uint, c_float};
    use rsfml::sfTypes::{sfBool};
    use system::time;
    use audio::sound_status;
    use system::vector3::Vector3f;

    pub struct sfMusic {
        This : *c_void,
        This1 : *c_void
    }

    pub extern "C" {
        fn sfMusic_createFromFile(filename : *c_char) -> *sfMusic;
        // sfMusic* sfMusic_createFromMemory(const void* data, size_t sizeInBytes);
        // sfMusic* sfMusic_createFromStream(sfInputStream* stream);
        fn sfMusic_destroy(music : *sfMusic) -> ();
        fn sfMusic_setLoop(music : *sfMusic, lloop : sfBool) -> ();
        fn sfMusic_getLoop(music : *sfMusic) -> sfBool;
        fn sfMusic_getDuration(music : *sfMusic) -> time::csfml::sfTime;
        fn sfMusic_play(music : *sfMusic) -> ();
        fn sfMusic_pause(music : *sfMusic) -> ();
        fn sfMusic_stop(music : *sfMusic) -> ();
        fn sfMusic_getChannelCount(music : *sfMusic) -> c_uint;
        fn sfMusic_getSampleRate(music : *sfMusic) -> c_uint;
        fn sfMusic_getStatus(music : *sfMusic) -> sound_status::csfml::sfSoundStatus;
        fn sfMusic_getPlayingOffset(music : *sfMusic) -> time::csfml::sfTime;
        fn sfMusic_setPitch(music : *sfMusic, pitch : c_float) -> ();
        fn sfMusic_setVolume(music : *sfMusic, volume : c_float) -> ();
        fn sfMusic_setPosition(music : *sfMusic, position : Vector3f) -> ();
        fn sfMusic_setRelativeToListener(music : *sfMusic, relative : sfBool) -> ();
        fn sfMusic_setMinDistance(music : *sfMusic, distance : c_float) -> ();
        fn sfMusic_setAttenuation(music : *sfMusic, attenuation : c_float) -> ();
        fn sfMusic_setPlayingOffset(music : *sfMusic, timeOffset : time::csfml::sfTime) -> ();
        fn sfMusic_getPitch(music : *sfMusic) -> c_float;
        fn sfMusic_getVolume(music : *sfMusic) -> c_float;
        fn sfMusic_getPosition(music : *sfMusic) -> Vector3f;
        fn sfMusic_isRelativeToListener(music : *sfMusic) -> sfBool;
        fn sfMusic_getMinDistance(music : *sfMusic) -> c_float;
        fn sfMusic_getAttenuation(music : *sfMusic) -> c_float;
    }
}

#[doc(hidden)]
pub struct Music {
    priv music : *csfml::sfMusic
}

impl Music {

    /**
    * Create a new music and load it from a file
    *
    * This function doesn't start playing the music (call
    * sfMusic_play to do so).
    * Here is a complete list of all the supported audio formats:
    * ogg, wav, flac, aiff, au, raw, paf, svx, nist, voc, ircam,
    * w64, mat4, mat5 pvf, htk, sds, avr, sd2, caf, wve, mpc2k, rf64.
    *
    * # Arguments
    * * filename - Path of the music file to open
    *
    * Return a new option to Music object or none 
    */
    pub fn new_from_file(filename : ~str) -> Option<Music> {
        let mut music_tmp : *csfml::sfMusic = ptr::null();
        do str::as_c_str(filename) |filename_buf| {
            unsafe { music_tmp = csfml::sfMusic_createFromFile(filename_buf); }
        };
        if music_tmp == ptr::null() {
            return None;
        }
        Some(Music{music : music_tmp})

    }

    /**
    * Set whether or not a music should loop after reaching the end
    *
    * If set, the music will restart from beginning after
    * reaching the end and so on, until it is stopped or
    * sfMusic_setLoop(music, sfFalse) is called.
    * The default looping state for musics is false.
    *
    * # Arguments
    * * loop - sfTrue to play in loop, sfFalse to play once
    */
    pub fn set_loop(&mut self, lloop : bool) -> () {
        unsafe {
            if lloop == true {
                csfml::sfMusic_setLoop(self.music, 1)
            }
            else {
                csfml::sfMusic_setLoop(self.music, 0)
            }
        }
    } 

    /**
    * Tell whether or not a music is in loop mode
    *
    * Return true if the music is looping, false otherwise
    */
    pub fn get_loop(&self) -> bool {
        match unsafe {csfml::sfMusic_getLoop(self.music)} {
            0 => false,
            _ => true
        }
    }
    
    /**
    * Get the total duration of a music
    *
    * Return Music duration
    */
    pub fn get_duration(&self) -> Time {
        Time::wrap( unsafe {csfml::sfMusic_getDuration(self.music)})
    }

    /**
    * Start or resume playing a music
    *
    * This function starts the music if it was stopped, resumes
    * it if it was paused, and restarts it from beginning if it
    * was it already playing.
    * This function uses its own thread so that it doesn't block
    * the rest of the program while the music is played.
    */
    pub fn play(&mut self) -> () {
        unsafe {csfml::sfMusic_play(self.music)}
    }

    /**
    * Pause a music
    *
    * This function pauses the music if it was playing,
    * otherwise (music already paused or stopped) it has no effect.
    */
    pub fn pause(&mut self) -> () {
        unsafe {csfml::sfMusic_pause(self.music)}
    }

    /**
    * Stop playing a music
    *
    * This function stops the music if it was playing or paused,
    * and does nothing if it was already stopped.
    * It also resets the playing position (unlike pause).
    */
    pub fn stop(&mut self) -> () {
        unsafe {csfml::sfMusic_stop(self.music)}
    }
    
    /**
    * Return the number of channels of a music
    *
    * 1 channel means a mono sound, 2 means stereo, etc.
    *
    * Return the number of channels
    */
    pub fn get_channel_count(&self) -> uint {
        unsafe {csfml::sfMusic_getChannelCount(self.music) as uint}
    }
    
    /**
    * Get the sample rate of a music
    *
    * The sample rate is the number of audio samples played per
    * second. The higher, the better the quality.
    *
    * Return the sample rate, in number of samples per second
    */
    pub fn get_sample_rate(&self) -> uint {
        unsafe {csfml::sfMusic_getSampleRate(self.music) as uint}
    }
    
    /**
    * Get the current status of a music (stopped, paused, playing)
    *
    * Return current status
    */
    pub fn get_status(&self) -> sound_status::Status {
        match unsafe {csfml::sfMusic_getStatus(self.music)} {
            sound_status::csfml::sfStopped => sound_status::Stopped,
            sound_status::csfml::sfPaused => sound_status::Paused,
            sound_status::csfml::sfPlaying => sound_status::Playing,
        }
    }

    /**
    * Get the current playing position of a music
    *
    * Return the current playing position
    */
    pub fn get_playing_offset(&self) -> Time {
        Time::wrap( unsafe {csfml::sfMusic_getPlayingOffset(self.music)})
    }
    
    /**
    * Set the pitch of a music
    *
    * The pitch represents the perceived fundamental frequency
    * of a sound; thus you can make a music more acute or grave
    * by changing its pitch. A side effect of changing the pitch
    * is to modify the playing speed of the music as well.
    * The default value for the pitch is 1.
    *
    * # Arguments
    * * pitch - new pitch to apply to the music
    */
    pub fn set_pitch(&mut self, pitch : float) -> () {
        unsafe {csfml::sfMusic_setPitch(self.music, pitch as c_float)}
    }
    
    /**
    * Set the volume of a music
    *
    * he volume is a value between 0 (mute) and 100 (full volume).
    * The default value for the volume is 100.
    *
    * # Arguments
    * * volume - Volume of the music
    */
    pub fn set_volume(&mut self, volume : float) -> () {
        unsafe {csfml::sfMusic_setVolume(self.music, volume as c_float)}
    }

    /**
    * Make a musics's position relative to the listener or absolute
    *
    * Making a music relative to the listener will ensure that it will always
    * be played the same way regardless the position of the listener.
    * This can be useful for non-spatialized musics, musics that are
    * produced by the listener, or musics attached to it.
    * The default value is false (position is absolute).
    *
    * # Arguments
    * * relative - true to set the position relative, false to set it absolute
    */
    pub fn set_relative_to_listener(&mut self, relative : bool) -> () {
        unsafe {
            if relative == true {
                csfml::sfMusic_setRelativeToListener(self.music, 1);
            }
            else {
                csfml::sfMusic_setRelativeToListener(self.music, 0);
            }
        }  
    }

    /**
    * Set the minimum distance of a music
    *
    * The "minimum distance" of a music is the maximum
    * distance at which it is heard at its maximum volume. Further
    * than the minimum distance, it will start to fade out according
    * to its attenuation factor. A value of 0 ("inside the head
    * of the listener") is an invalid value and is forbidden.
    * The default value of the minimum distance is 1.
    *
    * # Arguments
    * * distance - New minimum distance of the music
    */
    pub fn set_min_distance(&mut self, distance : float) -> () {
        unsafe {csfml::sfMusic_setMinDistance(self.music, distance as c_float)}
    }
    
    /**
    *  Set the attenuation factor of a music
    *
    * The attenuation is a multiplicative factor which makes
    * the music more or less loud according to its distance
    * from the listener. An attenuation of 0 will produce a
    * non-attenuated music, i.e. its volume will always be the same
    * whether it is heard from near or from far. On the other hand,
    * an attenuation value such as 100 will make the music fade out
    * very quickly as it gets further from the listener.
    * The default value of the attenuation is 1.
    *
    * # Arguments
    * * attenuation - New attenuation factor of the music
    */
    pub fn set_attenuation(&mut self, attenuation : float) -> () {
        unsafe {csfml::sfMusic_setAttenuation(self.music, attenuation as c_float)}
    }
    
    /**
    * Change the current playing position of a music
    *
    * The playing position can be changed when the music is
    * either paused or playing.
    *
    * # Arguments
    * * timeOffset - New playing position
    */
    pub fn set_playing_offset(&mut self, timeOffset : Time) -> () {
        unsafe {
            csfml::sfMusic_setPlayingOffset(self.music, timeOffset.unwrap())
        }
    }
    
    /**
    * Get the pitch of a music
    *
    * Return the pitch of the music
    */
    pub fn get_pitch(&self) -> float {
        unsafe {
            csfml::sfMusic_getPitch(self.music) as float
        }
    }

    /**
    * Get the volume of a music
    *
    * Return the volume of the music, in the range [0, 100]
    */
    pub fn get_volume(&self) -> float {
        unsafe {
            csfml::sfMusic_getVolume(self.music) as float
        }
    }

    /**
    * Tell whether a music's position is relative to the listener or is absolute
    *
    * Return true if the position is relative, false if it's absolute
    */
    pub fn is_relative_to_listener(&self) -> bool {
        match unsafe {csfml::sfMusic_isRelativeToListener(self.music)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Get the minimum distance of a music
    *
    * Return the minimum distance of the music
    */
    pub fn get_min_distance(&self) -> float {
        unsafe {
           csfml::sfMusic_getMinDistance(self.music) as float
       }        
    }

    /**
    * Get the attenuation factor of a music
    *
    * Return the attenuation factor of the music
    */
    pub fn get_attenuation(&self) -> float {
        unsafe {
            csfml::sfMusic_getAttenuation(self.music) as float
        }        
    }


    /**
    * Set the 3D position of a music in the audio scene
    *
    * Only musics with one channel (mono musics) can be
    * spatialized.
    * The default position of a music is (0, 0, 0).
    *
    * # Arguments
    * * position - Position of the music in the scene
    */
    fn set_position(&mut self, position : &Vector3f) -> () {
        unsafe {
            csfml::sfMusic_setPosition(self.music, *position)
        }
    }

    /**
    * Set the 3D position of a music in the audio scene
    *
    * Only musics with one channel (mono musics) can be
    * spatialized.
    * The default position of a music is (0, 0, 0).
    *
    * # Arguments
    * * x - X coordinate of the position of the sound in the scene
    * * y - Y coordinate of the position of the sound in the scene
    * * z - Z coordinate of the position of the sound in the scene
    */
    fn set_position3f(&self, x : f32, y : f32, z : f32) -> () {
        unsafe {
            csfml::sfMusic_setPosition(self.music, Vector3f::new(x, y, z))
        }
    }

    /**
    * Get the 3D position of a music in the audio scene
    *
    * Return the position of the music in the world
    */
    fn get_position(&self) -> Vector3f {
        unsafe {
            csfml::sfMusic_getPosition(self.music)
        }
    }    
}

impl Drop for Music {
    /**
    *   Destructor for class Music. Destroy all the ressource.
    */
    fn finalize(&self) {
        unsafe {
            csfml::sfMusic_destroy(self.music);
        }
    }
}
