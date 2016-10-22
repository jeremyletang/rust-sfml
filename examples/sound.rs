extern crate sfml;

use sfml::audio::{Music, Sound, SoundBuffer, SoundStatus};
use sfml::system::{Time, sleep};
use std::io::Write;

// Play a Sound
fn play_sound() {
    let buffer = SoundBuffer::new("resources/canary.wav").unwrap();
    let buffer_ref = buffer.get_ref();

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer_ref.get_duration().as_seconds());
    println!(" {} samples / sec", buffer_ref.get_sample_rate());
    println!(" {} channels", buffer_ref.get_channel_count());

    let mut sound = Sound::with_buffer(buffer_ref);
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
