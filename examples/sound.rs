use sfml::{
    audio::{Music, Sound, SoundBuffer, SoundStatus},
    system::{sleep, Time},
};
use std::io::Write;

// Play a Sound
fn play_sound() {
    let buffer = SoundBuffer::from_file("resources/canary.wav").unwrap();

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer.duration().as_seconds());
    println!(" {} samples / sec", buffer.sample_rate());
    println!(" {} channels", buffer.channel_count());

    let mut sound = Sound::with_buffer(&buffer);
    sound.play();

    while sound.status() == SoundStatus::PLAYING {
        // Display the playing position
        print!("\rPlaying... {:.2}", sound.playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }
    println!();
}

// Play a Music
fn play_music() {
    let mut music = Music::from_file("resources/orchestral.ogg").unwrap();

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} seconds", music.duration().as_seconds());
    println!(" {} samples / sec", music.sample_rate());
    println!(" {} channels", music.channel_count());

    music.play();

    while music.status() == SoundStatus::PLAYING {
        // Display the playing position
        print!("\rPlaying... {:.2}", music.playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }

    println!();
}

fn main() {
    play_sound();
    play_music();
}
