/*!
* Sound and musics statues
* 
*/

#[doc(hidden)]
pub mod csfml {
    
    pub enum sfSoundStatus {
        sfStopped = 0,
        sfPaused = 1,
        sfPlaying = 2
    }
}

/**
* Enumeration of statuses for sounds and musics
*/
pub enum Status {
    Stopped,
    Paused,
    Playing
}