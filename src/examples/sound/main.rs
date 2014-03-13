/*!
* Example from SFML : play sound and music
*/

#[crate_id = "sound"];
#[desc = "Sound example for rsfml"];
#[crate_type = "bin"];

extern crate rsfml;

use std::rc::Rc;
use std::cell::RefCell;
use rsfml::audio::{SoundBuffer, rc, Music, Playing};
use rsfml::system::{sleep, Time};

/* Play a Sound */
fn play_sound() -> () {
    let buffer = match SoundBuffer::new("resources/canary.wav") {
        Some(buffer)    => Rc::new(RefCell::new(buffer)),
        None            => fail!("Error, cannot load sound buffer!")
    };
    
    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", (*buffer).with(|b| b.get_duration().as_seconds()));
    println!(" {} samples / sec", (*buffer).with(|b| b.get_sample_rate()));
    println!(" {} channels", (*buffer).with(|b| b.get_channel_count()));

    let mut sound : rc::Sound = match rc::Sound::new_with_buffer(buffer.clone()) {
        Some(sound)     => sound,
        None            => fail!("Error cannot create Sound")
    };

    sound.play();

    loop {
        match sound.get_status() {
            Playing     => {
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                // Display the playing position
                println!("\rPlaying...   {}", sound.get_playing_offset().as_seconds());
            },
            _           => break
            
        }
        
        println!("");
    }
}

/* Play a Music */
fn play_music() -> () {
    let mut music : Music = match Music::new_from_file("resources/orchestral.ogg") {
        Some(music)     => music,
        None            => fail!("Error, cannot load music")
    };
    
    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} format seconds", music.get_duration().as_seconds());
    println!(" {} samples / sec", music.get_sample_rate());
    println!(" {} channels", music.get_channel_count());

    music.play();
    
    loop {
        match music.get_status() {
            Playing     => {
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                // Display the playing position
                println!("\rPlaying...   {}", music.get_playing_offset().as_seconds());
            },
            _           => break
            
        }
        
        println!("");
    }    
}

fn main() -> ()
{
    // Play a sound
    play_sound();
    
    // Play a music
    play_music();
}
