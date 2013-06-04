/*!
* Play sounds.
*
* Regular sound that can be played in the audio environment.
*
*/

use core::libc::{c_float};
use system::time;
use audio::sound_status;
use audio::sound_buffer;

#[doc(hidden)]
pub mod csfml {
    
    use core::libc::{c_float, c_void};
    use audio::sound_status;
    use audio::sound_buffer;
    use system::time;
    use rsfml::sfTypes::{sfBool};

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
        // fn sfSound_setPosition(sound : *sfSound, position : vector3::csfml::sfVector3f) -> ();
        fn sfSound_setRelativeToListener(sound : *sfSound, relative : sfBool) -> ();
        fn sfSound_setMinDistance(sound : *sfSound, distance : c_float) -> ();
        fn sfSound_setAttenuation(sound : *sfSound, attenuation : c_float) -> ();
        fn sfSound_setPlayingOffset(sound : *sfSound, timeOffset : time::csfml::sfTime) -> ();
        fn sfSound_getPitch(sound : *sfSound) -> c_float;
        fn sfSound_getVolume(sound : *sfSound) -> c_float;
        // fn sfSound_getPosition(sound : *sfSound) -> vector3::csfml::Vector3f;
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

pub impl Sound {
    
    /**
    * Constructor for class Sound.
    *
    * The constructor create a new instance of the sound object.
    */
    pub fn new() -> Sound {
        Sound { sound : unsafe {csfml::sfSound_create()}}
    }
    
    /**
    * Create a new sound by copying an existing one
    */
    pub fn new_copy(sound : Sound) -> Sound {
        Sound {sound : unsafe {csfml::sfSound_copy(sound.unwrap())}}
    }

    /**
    * Set whether or not a sound should loop after reaching the end
    */
    pub fn set_loop(&self, lloop : bool) -> () {
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
    */
    pub fn get_loop(&self) -> bool {
        match unsafe {csfml::sfSound_getLoop(self.sound)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Start or resume playing a sound
    */
    pub fn play(&self) -> () {
        unsafe {csfml::sfSound_play(self.sound)}
    }

    /**
    *  Start or resume playing a sound
    */
    pub fn pause(&self) -> () {
        unsafe {csfml::sfSound_pause(self.sound)}
    }

    /**
    * Stop playing a sound
    */
    pub fn stop(&self) -> () {
        unsafe {csfml::sfSound_stop(self.sound)}
    }

    /**
    * Get the current status of a sound (stopped, paused, playing)
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
    */
    pub fn get_playing_offset(&self) -> time::Time {
        time::Time::wrap( unsafe {csfml::sfSound_getPlayingOffset(self.sound)})
    }

    /**
    * Set the pitch of a sound 
    */
    pub fn set_pitch(&self, pitch : float) -> () {
        unsafe {csfml::sfSound_setPitch(self.sound, pitch as c_float)}
    }

    /**
    * Set the volume of a sound
    */
    pub fn set_volume(&self, volume : float) -> () {
        unsafe {csfml::sfSound_setVolume(self.sound, volume as c_float)}
    }

    /**
    * Make the sound's position relative to the listener or absolute
    */
    pub fn set_relative_to_listener(&self, relative : bool) -> () {
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
    */
    pub fn set_min_distance(&self, distance : float) -> () {
        unsafe {csfml::sfSound_setMinDistance(self.sound, distance as c_float)}
    }

    /**
    * Set the attenuation factor of a sound 
    */
    pub fn set_attenuation(&self, attenuation : float) -> () {
        unsafe {csfml::sfSound_setAttenuation(self.sound, attenuation as c_float)}
    }

    /**
    * Change the current playing position of a sound
    */
    pub fn set_playing_offset(&self, timeOffset : time::Time) -> () {
        unsafe {
            csfml::sfSound_setPlayingOffset(self.sound, timeOffset.unwrap())
        }
    }

    /**
    * Get the pitch of a sound
    */
    pub fn get_pitch(&self) -> float {
        unsafe {
            csfml::sfSound_getPitch(self.sound) as float
        }
    }

    /**
    * Get the volume of a sound
    */
    pub fn get_volume(&self) -> float {
        unsafe {
            csfml::sfSound_getVolume(self.sound) as float
        }
    }

    /**
    * Tell whether a sound's position is relative to the listener or is absolute
    */
    pub fn is_relative_to_listener(&self) -> bool {
        match unsafe {csfml::sfSound_isRelativeToListener(self.sound)} {
            0 => false,
            _ => true
        }
    }

    /**
    * Get the minimum distance of a sound
    */
    pub fn get_min_distance(&self) -> float {
        unsafe {
           csfml::sfSound_getMinDistance(self.sound) as float
        }
    }

    /**
    * Get the attenuation factor of a sound 
    */
    pub fn get_attenuation(&self) -> float {
        unsafe {
            csfml::sfSound_getAttenuation(self.sound) as float
        }
    }
    
    /**
    * 
    */
    fn set_buffer(&self, buffer : sound_buffer::SoundBuffer) -> () {
        unsafe {
            csfml::sfSound_setBuffer(self.sound, buffer.unwrap())
        }
    }

    /**
    * 
    */
    fn get_buffer(&self) -> sound_buffer::SoundBuffer {
        sound_buffer::SoundBuffer::wrap(unsafe {
            csfml::sfSound_getBuffer(self.sound)
        })
    }

    /**
    * 
    */
    #[doc(hidden)]
    fn unwrap(&self) -> *csfml::sfSound {
        self.sound
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
