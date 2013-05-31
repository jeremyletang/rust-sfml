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

    /* Modify this in the future, compiler bug when use 3 float*/
    pub struct sfVector3f {
        x : c_int,
        y : c_int,
        z : c_int
    }
    
    pub extern "C" {
        fn sfListener_setGlobalVolume(volume : f32) -> ();
        fn sfListener_getGlobalVolume() -> f32;
        fn sfListener_setPosition(position : sfVector3f) -> ();
        fn sfListener_getPosition() -> sfVector3f;
        fn sfListener_setDirection(orientation : sfVector3f) -> ();
        fn sfListener_getDirection() -> sfVector3f;
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
pub fn set_position(position : Vector3f) -> () {
    unsafe {
        csfml::sfListener_setPosition(csfml::sfVector3f{x : position.x as c_int, y : position.y as c_int, z : position.z as c_int})
    }
}

/**
* Get the current position of the listener in the scene (Doesn't work)
*/
pub fn get_position() -> Vector3f {
    unsafe {
        let tmp : csfml::sfVector3f = csfml::sfListener_getPosition();
        Vector3f {x : tmp.x as float, y : tmp.y as float, z : tmp.z as float}
    }
}

/**
* Set the orientation of the listener in the scene (Doesn't work)
*/
pub fn set_direction(position : Vector3f) -> () {
    unsafe {
        csfml::sfListener_setDirection(csfml::sfVector3f{x : position.x as c_int, y : position.y as c_int, z : position.z as c_int})
    }
}

/**
* Get the current orientation of the listener in the scene
*/
pub fn get_direction() -> Vector3f {
    unsafe {
        let tmp : csfml::sfVector3f = csfml::sfListener_getDirection();
        Vector3f {x : tmp.x as float, y : tmp.y as float, z : tmp.z as float}
    }
}