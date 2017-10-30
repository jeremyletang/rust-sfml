extern crate sfml;

use sfml::audio::{Music, SoundStatus};
use sfml::system::{sleep, Time};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::open("resources/orchestral.ogg").unwrap();
    let mut music = Music::from_stream(&mut file).unwrap();

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} format seconds", music.duration().as_seconds());
    println!(" {} samples / sec", music.sample_rate());
    println!(" {} channels", music.channel_count());

    music.play();

    while music.status() == SoundStatus::Playing {
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
        // Display the playing position
        print!(
            "\rPlaying... {:.2} sec",
            music.playing_offset().as_seconds()
        );
        let _ = std::io::stdout().flush();
    }
    println!();
}
