/*!
* Audio listener
*
* The audio listener is the point in the scene from where all the sounds are heard.
*
*/

use system::vector3::*;
pub use core::libc::{c_int};

#[doc(hidden)]
pub mod csfml {
    
    pub use core::libc::{c_int};    
    pub use system::vector3;

    pub extern "C" {
        fn sfListener_setGlobalVolume(volume : f32) -> ();
        fn sfListener_getGlobalVolume() -> f32;
        fn sfListener_setPosition(position : vector3::Vector3f) -> ();
        fn sfListener_getPosition() -> vector3::Vector3f;
        fn sfListener_setDirection(orientation : vector3::Vector3f) -> ();
        fn sfListener_getDirection() -> vector3::Vector3f;
    }
}
/**
* Change the global volume of all the sounds and musics
*
* The volume is a number between 0 and 100.
*/
pub fn set_global_volume(volume : float) -> () {
    unsafe {
        csfml::sfListener_setGlobalVolume(volume as f32)
    }
}

/**
* Get the current value of the global volume
*/
pub fn get_global_volume() -> float {
    unsafe {
        csfml::sfListener_getGlobalVolume() as float
    }
}

/*
* Set the position of the listener in the scene (Doesnt't work)
*/
pub fn set_position(position : &Vector3f) -> () {
    unsafe {
        csfml::sfListener_setPosition(*position)
    }
}

/**
* Get the current position of the listener in the scene (Doesn't work)
*/
pub fn get_position() -> Vector3f {
    unsafe {
        csfml::sfListener_getPosition()
    }
}

/**
* Set the orientation of the listener in the scene (Doesn't work)
*/
pub fn set_direction(position : &Vector3f) -> () {
    unsafe {
        csfml::sfListener_setDirection(*position)
    }
}

/**
* Get the current orientation of the listener in the scene
*/
pub fn get_direction() -> Vector3f {
    unsafe {
        csfml::sfListener_getDirection()
    }
}