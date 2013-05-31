/*!
* Base module of SFML, defining various utilities.
*
* It provides vector classes, unicode strings and conversion functions, threads and mutexes, timing classes.
*
*/

#[cfg(target_os="linux")]
#[cfg(target_os="win32")]
mod others {
    #[link_args="-lcsfml-system"]
    extern {}
}

pub mod vector2;
pub mod vector3;
pub mod time;
pub mod clock;
pub mod sleep;
pub mod mutex;
//pub mod thread;