//! Example from SFML: play sound and music

extern crate sfml;

use sfml::audio::{SoundBuffer, Sound, Music, SoundStatus, PlayableSound};
use sfml::system::{sleep, Time};
use std::io::{stdout, Write};

/* Play a Sound */
fn play_sound() {
    let buffer = SoundBuffer::new("resources/canary.wav").expect("Error, cannot load sound buffer!");

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer.get_duration().as_seconds());
    println!(" {} samples / sec", buffer.get_sample_rate());
    println!(" {} channels", buffer.get_channel_count());

    let mut sound = Sound::new_with_buffer(&buffer).expect("Error cannot create Sound");
    sound.play();

    loop {
        match sound.get_status() {
            SoundStatus::Playing => {
                // Display the playing position
                print!("\rPlaying...   {}  ", sound.get_playing_offset().as_seconds());
                stdout().flush().unwrap();
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
            },
            _ => break
        }
    }
    println!("\rPlaying...   {}  ", buffer.get_duration().as_seconds());
}

/* Play a Music */
fn play_music() {
    let mut music = Music::new_from_file("resources/orchestral.ogg").expect("Error, cannot load music");

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} seconds", music.get_duration().as_seconds());
    println!(" {} samples / sec", music.get_sample_rate());
    println!(" {} channels", music.get_channel_count());

    music.play();

    loop {
        match music.get_status() {
            SoundStatus::Playing => {
                // Display the playing position
                print!("\rPlaying...   {}  ", music.get_playing_offset().as_seconds());
                stdout().flush().unwrap();
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
            },
            _ => break
        }
    }
    println!("\rPlaying...   {}  ", music.get_duration().as_seconds());
}

fn main() {
    // Play a sound
    play_sound();

    // Play a music
    play_music();
}
