/**
* Example from SFML : play sound and music
*/

extern mod rsfml;

use std::io;

use rsfml::audio::{SoundBuffer, Sound, Music, sound_status};
use rsfml::system::{sleep, Time};

/* Play a Sound */
fn play_sound() -> () {
    let buffer : @SoundBuffer = match SoundBuffer::new(~"resources/canary.wav") {
        Some(buffer)    => @buffer,
        None            => fail!("Error, cannot load sound buffer!")
    };
    
    // Display sound informations
    io::println("canary.wav :");
    io::println(format!(" {} seconds", buffer.get_duration().as_seconds()));
    io::println(format!(" {} samples / sec", buffer.get_sample_rate()));
    io::println(format!(" {} channels", buffer.get_channel_count()));

    let mut sound : Sound = match Sound::new_with_buffer(buffer) {
        Some(sound)     => sound,
        None            => fail!("Error cannot create Sound")
    };

    sound.play();
    
    loop {
        match sound.get_status() {
            sound_status::Playing       => {
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                // Display the playing position
                io::println(format!("\rPlaying...   {}", sound.get_playing_offset().as_seconds()));
            },
            _                           => break
            
        }
        
        io::println("");
    }
}

/* Play a Music */
fn play_music() -> () {
    let mut music : Music = match Music::new_from_file(~"resources/orchestral.ogg") {
        Some(music)     => music,
        None            => fail!("Error, cannot load music")
    };
    
    // Display Music informations
    io::println("orchestral.ogg :");
    io::println(format!(" {} format seconds", music.get_duration().as_seconds()));
    io::println(format!(" {} samples / sec", music.get_sample_rate()));
    io::println(format!(" {} channels", music.get_channel_count()));

    music.play();
    
    loop {
        match music.get_status() {
            sound_status::Playing       => {
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                // Display the playing position
                io::println(format!("\rPlaying...   {}", music.get_playing_offset().as_seconds()));
            },
            _                           => break
            
        }
        
        io::println("");
    }    
}

fn main() -> ()
{
    // Play a sound
    play_sound();
    
    // Play a music
    play_music();
}