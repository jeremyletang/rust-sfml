use {
    sfml::{
        audio::{
            Music,
            sound_source::{self, SoundSource},
        },
        system::{InputStream, Time, sleep},
    },
    std::{error::Error, fs::File, io::Write},
};

include!("../example_common.rs");

fn get_music() -> Result<Music<'static, File>, Box<dyn Error>> {
    let file = File::open("orchestral.ogg")?;
    let stream = InputStream::new_owned(file);
    let music = Music::from_stream_owned(stream)?;

    Ok(music)
}

fn main() -> Result<(), Box<dyn Error>> {
    example_ensure_right_working_dir();

    let mut music = get_music()?;

    // Display Music information
    println!("orchestral.ogg :");
    println!(" {} seconds", music.duration().as_seconds());
    println!(" {} samples / sec", music.sample_rate());
    println!(" {} channels", music.channel_count());

    music.play();

    while music.status() == sound_source::Status::Playing {
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
    Ok(())
}
