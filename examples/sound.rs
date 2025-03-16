use {
    sfml::{
        SfResult,
        audio::{
            Music, Sound, SoundBuffer,
            sound_source::{self, SoundSource},
        },
        system::{Time, sleep},
    },
    std::io::Write,
};

include!("../example_common.rs");

// Play a Sound
fn play_sound() -> SfResult<()> {
    let buffer = SoundBuffer::from_file("canary.wav")?;

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer.duration().as_seconds());
    println!(" {} samples / sec", buffer.sample_rate());
    println!(" {} channels", buffer.channel_count());

    let mut sound = Sound::with_buffer(&buffer);
    sound.play();

    while sound.status() == sound_source::Status::Playing {
        // Display the playing position
        print!("\rPlaying... {:.2}", sound.playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }
    println!();
    Ok(())
}

// Play a Music
fn play_music() -> SfResult<()> {
    let mut music = Music::from_file("orchestral.ogg")?;

    // Display Music informations
    println!("orchestral.ogg :");
    println!(" {} seconds", music.duration().as_seconds());
    println!(" {} samples / sec", music.sample_rate());
    println!(" {} channels", music.channel_count());

    music.play();

    while music.status() == sound_source::Status::Playing {
        // Display the playing position
        print!("\rPlaying... {:.2}", music.playing_offset().as_seconds());
        let _ = std::io::stdout().flush();
        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }

    println!();
    Ok(())
}

fn main() -> SfResult<()> {
    example_ensure_right_working_dir();

    play_sound()?;
    play_music()?;
    Ok(())
}
