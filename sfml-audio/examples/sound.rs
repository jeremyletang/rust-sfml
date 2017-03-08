extern crate sfml_system;
extern crate sfml_audio;

use sfml_audio::{Music, Sound, SoundBuffer, SoundStatus};
use sfml_system::{Time, sleep};
use std::io::Write;

// Play a Sound
fn play_sound() {
    let buffer = SoundBuffer::from_file("resources/canary.wav").unwrap();

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer.get_duration().as_seconds());
    println!(" {} samples / sec", buffer.get_sample_rate());
    println!(" {} channels", buffer.get_channel_count());

    let mut sound = Sound::with_buffer(&buffer);
    sound.play();

    while sound.get_status() == SoundStatus::Playing {
        // Display the playing position
        print!("\rPlaying... {:.2}",
               sound.get_playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }
    println!("");
}

// Play a Music
fn play_music() {
    let mut music = Music::from_file("resources/orchestral.ogg").unwrap();

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} format seconds", music.get_duration().as_seconds());
    println!(" {} samples / sec", music.get_sample_rate());
    println!(" {} channels", music.get_channel_count());

    music.play();

    while music.get_status() == SoundStatus::Playing {
        // Display the playing position
        print!("\rPlaying... {:.2}",
               music.get_playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }

    println!("");
}

fn main() {
    play_sound();
    play_music();
}
