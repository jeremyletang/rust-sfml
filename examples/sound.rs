//! Example from SFML: play sound and music

extern crate sfml;

use sfml::audio::{SoundBuffer, Sound, Music, Status};
use sfml::system::{sleep, Time};

/* Play a Sound */
fn play_sound() {
    let buffer = match SoundBuffer::new("resources/canary.wav") {
        Some(buffer)    => buffer,
        None            => panic!("Error, cannot load sound buffer!")
    };

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer.get_duration().as_seconds());
    println!(" {} samples / sec", buffer.get_sample_rate());
    println!(" {} channels", buffer.get_channel_count());

    let mut sound = match Sound::new_with_buffer(&buffer) {
        Some(sound)     => sound,
        None            => panic!("Error cannot create Sound")
    };

    sound.play();

    loop {
        match sound.get_status() {
            Status::Playing     => {
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
fn play_music() {
    let mut music: Music = match Music::new_from_file("resources/orchestral.ogg") {
        Some(music)     => music,
        None            => panic!("Error, cannot load music")
    };

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} format seconds", music.get_duration().as_seconds());
    println!(" {} samples / sec", music.get_sample_rate());
    println!(" {} channels", music.get_channel_count());

    music.play();

    loop {
        match music.get_status() {
            Status::Playing     => {
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

fn main() {
    // Play a sound
    play_sound();

    // Play a music
    play_music();
}
