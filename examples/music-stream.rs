use sfml::{
    audio::{Music, SoundStatus},
    system::{sleep, InputStream, Time},
};
use std::{fs::File, io::Write};

include!("../example_common.rs");

fn main() {
    let mut file = File::open(example_res!("orchestral.ogg")).unwrap();
    let mut stream = InputStream::new(&mut file);
    let mut music = Music::from_stream(&mut stream).unwrap();

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} seconds", music.duration().as_seconds());
    println!(" {} samples / sec", music.sample_rate());
    println!(" {} channels", music.channel_count());

    music.play();

    while music.status() == SoundStatus::PLAYING {
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
