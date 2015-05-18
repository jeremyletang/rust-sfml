//! Example from SFML: Sound Capture

extern crate sfml;

use std::io::BufRead;

use sfml::audio::{SoundBufferRecorder, Sound, SoundStatus, PlayableSound};
use sfml::system::{sleep, Time};

fn main() {
    // Check that the device can capture audio
    if !SoundBufferRecorder::is_available() {
        panic!("Sorry, audio capture is not supported by your system");
    }

    // Choose the sample rate
    println!("Please choose the sample rate for sound capture (44100 is CD quality): ");
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    unsafe { line.as_mut_vec().pop(); }
    let sample_rate: u32 = line.trim().parse().ok().expect("Error, input is not valid.");

    // Wait for user input...
    println!("Press enter to start recording audio");
    reader.read_line(&mut String::new()).unwrap();

    // Here we'll use an integrated custom recorder, which saves the captured data into a SoundBuffer
    let mut recorder = SoundBufferRecorder::new().expect("Error, cannot initialize Sound buffer recorder.");

    // Audio capture is done in a separate thread, so we can block the main thread while it is capturing
    recorder.start(sample_rate);
    println!("Recording... press enter to stop");
    reader.read_line(&mut String::new()).unwrap();
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = recorder.get_buffer().expect("Error when retreiving buffer.");

    // Display captured sound informations
    println!("Sound informations :");
    println!(" {} seconds", buffer.get_duration().as_seconds());
    println!(" {} samples / sec", buffer.get_sample_rate());
    println!(" {} channels", buffer.get_channel_count());


    // Choose what to do with the recorded sound data
    println!("What do you want to do with captured sound (p = play, s = save) ? ");
    let mut resp = String::new();
    reader.read_line(&mut resp).unwrap();

    if resp.trim().chars().last().unwrap() == 's' {
        // Choose a filename
        println!("Choose the file to create: ");
        let mut filename = String::new();
        reader.read_line(&mut filename).unwrap();

        // Save the buffer
        buffer.save_to_file(filename.trim());
    }
    else {
        let mut sound = Sound::new_with_buffer(&buffer).expect("Error cannot create Sound");
        sound.play();

        loop {
            match sound.get_status() {
                SoundStatus::Playing => {
                    // Display the playing position
                    println!("\rPlaying...   {}", sound.get_playing_offset().as_seconds());
                    // Leave some CPU time for other processes
                    sleep(Time::with_milliseconds(100));
                },
                _ => break
            }
        }
    }

    // Finished
    println!("Done!");

    // Wait until the user presses 'enter' key
    println!("Press enter to exit...");
    let _ = reader.read_line(&mut String::new());
}