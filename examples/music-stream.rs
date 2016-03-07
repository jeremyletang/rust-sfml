extern crate sfml;

use sfml::audio::{Music, SoundStatus};
use sfml::system::{Time, sleep};
use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::open("resources/orchestral.ogg").unwrap();
    let mut music = Music::new_from_stream(&mut file).unwrap();

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} format seconds", music.get_duration().as_seconds());
    println!(" {} samples / sec", music.get_sample_rate());
    println!(" {} channels", music.get_channel_count());

    music.play();

    while music.get_status() == SoundStatus::Playing {
        // Leave some CPU time for other processes
        sleep(Time::with_milliseconds(100));
        // Display the playing position
        print!("\rPlaying... {:.2} sec",
               music.get_playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
    }
    print!("\n");
}
