/**
* Example from SFML : play sound and music
*/

extern mod rsfml;

use std::io;

use rsfml::audio::sound_buffer::SoundBuffer;
use rsfml::audio::sound::*;
use rsfml::audio::music::*;
use rsfml::system::sleep::sleep;
use rsfml::system::time::Time;
use rsfml::audio::sound_status;

/* Play a Sound */
fn play_sound() -> () {
    let buffer : @SoundBuffer = match SoundBuffer::new(~"resources/canary.wav") {
        Some(buffer)    => @buffer,
        None            => fail!("Error, cannot load sound buffer!")
    };
    
    // Display sound informations
    io::println("canary.wav :");
    io::println(fmt!(" %f seconds", buffer.get_duration().as_seconds() as float));
    io::println(fmt!(" %d samples / sec", buffer.get_sample_rate() as int));
    io::println(fmt!(" %d channels", buffer.get_channel_count() as int));

    let mut sound : Sound = match Sound::new(buffer) {
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
                io::println(fmt!("\rPlaying...   %f", sound.get_playing_offset().as_seconds() as float));
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
    io::println(fmt!(" %f seconds", music.get_duration().as_seconds() as float));
    io::println(fmt!(" %d samples / sec", music.get_sample_rate() as int));
    io::println(fmt!(" %d channels", music.get_channel_count() as int));

    music.play();
    
    loop {
        match music.get_status() {
            sound_status::Playing       => {
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                // Display the playing position
                io::println(fmt!("\rPlaying...   %f", music.get_playing_offset().as_seconds() as float));
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