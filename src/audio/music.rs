/*
* Rust-SFML - Copyright (c) Letang Jeremy.
*
* The Original software, SFML library, is provided by Laurent Gomila.
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

use system::time;
use audio::sound_status;
use system::vector3;

#[doc(hidden)]
pub mod csfml {

    use std::libc::{c_void, c_char, c_uint, c_float};
    use rsfml::sfTypes::{sfBool};
    use system::time;
    use audio::sound_status;
    use system::vector3;

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
        fn sfMusic_setPosition(music : *sfMusic, position : vector3::Vector3f) -> ();
        fn sfMusic_setRelativeToListener(music : *sfMusic, relative : sfBool) -> ();
        fn sfMusic_setMinDistance(music : *sfMusic, distance : c_float) -> ();
        fn sfMusic_setAttenuation(music : *sfMusic, attenuation : c_float) -> ();
        fn sfMusic_setPlayingOffset(music : *sfMusic, timeOffset : time::csfml::sfTime) -> ();
        fn sfMusic_getPitch(music : *sfMusic) -> c_float;
        fn sfMusic_getVolume(music : *sfMusic) -> c_float;
        fn sfMusic_getPosition(music : *sfMusic) -> vector3::Vector3f;
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
    */
    pub fn set_loop(&self, lloop : bool) -> () {
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
    */
    pub fn get_loop(&self) -> bool {
        match unsafe {csfml::sfMusic_getLoop(self.music)} {
            0 => false,
            _ => true
        }
    }
    
    /**
    * Get the total duration of a music
    */
    pub fn get_duration(&self) -> time::Time {
        time::Time::wrap( unsafe {csfml::sfMusic_getDuration(self.music)})
    }

    /**
    * Start or resume playing a music
    */
    pub fn play(&self) -> () {
        unsafe {csfml::sfMusic_play(self.music)}
    }

    /**
    * Pause a music
    */
    pub fn pause(&self) -> () {
        unsafe {csfml::sfMusic_pause(self.music)}
    }

    /**
    * Stop playing a music
    */
    pub fn stop(&self) -> () {
        unsafe {csfml::sfMusic_stop(self.music)}
    }
    
    /**
    * Return the number of channels of a music
    */
    pub fn get_channel_count(&self) -> uint {
        unsafe {csfml::sfMusic_getChannelCount(self.music) as uint}
    }
    
    /**
    * Get the sample rate of a music
    */
    pub fn get_sample_rate(&self) -> uint {
        unsafe {csfml::sfMusic_getSampleRate(self.music) as uint}
    }
    
    /**
    * Get the current status of a music (stopped, paused, playing)
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
    */
    pub fn get_playing_offset(&self) -> time::Time {
        time::Time::wrap( unsafe {csfml::sfMusic_getPlayingOffset(self.music)})
    }
    
    /**
    * Set the pitch of a music
    */
    pub fn set_pitch(&self, pitch : float) -> () {
        unsafe {csfml::sfMusic_setPitch(self.music, pitch as c_float)}
    }
    
    /**
    * Set the volume of a music
    */
    pub fn set_volume(&self, volume : float) -> () {
        unsafe {csfml::sfMusic_setVolume(self.music, volume as c_float)}
    }

    /**
    * Make a musics's position relative to the listener or absolute
    */
    pub fn set_relative_to_listener(&self, relative : bool) -> () {
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
    */
    pub fn set_min_distance(&self, distance : float) -> () {
        unsafe {csfml::sfMusic_setMinDistance(self.music, distance as c_float)}
    }
    
    /**
    *  Set the attenuation factor of a music
    */
    pub fn set_attenuation(&self, attenuation : float) -> () {
        unsafe {csfml::sfMusic_setAttenuation(self.music, attenuation as c_float)}
    }
    
    /**
    * Change the current playing position of a music
    */
    pub fn set_playing_offset(&self, timeOffset : time::Time) -> () {
        unsafe {
            csfml::sfMusic_setPlayingOffset(self.music, timeOffset.unwrap())
        }
    }
    
    /**
    * Get the pitch of a music
    */
    pub fn get_pitch(&self) -> float {
        unsafe {
            csfml::sfMusic_getPitch(self.music) as float
        }
    }

    /**
    * Get the volume of a music
    */
    pub fn get_volume(&self) -> float {
        unsafe {
            csfml::sfMusic_getVolume(self.music) as float
        }
    }

    /**
    * Tell whether a music's position is relative to the listener or is absolute
    */
    pub fn is_relative_to_listener(&self) -> bool {
        match unsafe {csfml::sfMusic_isRelativeToListener(self.music)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Get the minimum distance of a music
    */
    pub fn get_min_distance(&self) -> float {
        unsafe {
           csfml::sfMusic_getMinDistance(self.music) as float
       }        
    }

    /**
    * Get the attenuation factor of a music
    */
    pub fn get_attenuation(&self) -> float {
        unsafe {
            csfml::sfMusic_getAttenuation(self.music) as float
        }        
    }

    fn set_position(&self, position : &vector3::Vector3f) -> () {
        unsafe {
            csfml::sfMusic_setPosition(self.music, *position)
        }
    }

    fn get_position(&self) -> vector3::Vector3f {
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
