//! Example from SFML: Sound Capture

extern crate sfml;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::{BufRead, Write};

use sfml::audio::{rc, SoundBufferRecorder, SoundStatus};
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
    let sample_rate: u32 = match line.trim_right().parse() {
        Ok(value)     => value,
        Err(e)        => panic!("Error, input is not valid: {}", e)
    };

    // Wait for user input...
    println!("Press enter to start recording audio");
    reader.read_line(&mut String::new()).unwrap();

    // Here we'll use an integrated custom recorder, which saves the captured data into a SoundBuffer
    let mut recorder: SoundBufferRecorder = match SoundBufferRecorder::new() {
        Some(rec)       => rec,
        None            => panic!("Error, cannot initialize Sound buffer recorder.")
    };

    // Audio capture is done in a separate thread, so we can block the main thread while it is capturing
    recorder.start(sample_rate);
    println!("Recording... press enter to stop");
    reader.read_line(&mut String::new()).unwrap();
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = match recorder.get_buffer() {
        Some(buf)       => Rc::new(RefCell::new(buf)),
        None            => panic!("Error when retreiving buffer.")
    };

    // Display captured sound informations
    println!("Sound informations :");
    println!(" {} seconds", (*buffer).borrow().get_duration().as_seconds());
    println!(" {} samples / sec", (*buffer).borrow().get_sample_rate());
    println!(" {} channels", (*buffer).borrow().get_channel_count());


    // Choose what to do with the recorded sound data
    print!("What do you want to do with captured sound (p = play, s = save) ? ");
    let _ = std::io::stdout().flush();
    let mut resp = String::new();
    reader.read_line(&mut resp).unwrap();

    if resp.trim().chars().last().unwrap() == 's' {
        // Choose a filename
        println!("Choose the file to create: ");
        let mut filename = String::new();
        reader.read_line(&mut filename).unwrap();

        // Save the buffer
        (*buffer).borrow().save_to_file(filename.trim());
    }
    else {
        let mut sound: rc::Sound = match rc::Sound::new_with_buffer(buffer.clone()) {
            Some(sound)     => sound,
            None            => panic!("Error cannot create Sound")
        };

         sound.play();

        loop {
            match sound.get_status() {
                SoundStatus::Playing     => {
                // Display the playing position
                print!("\rPlaying... {:.2} sec", sound.get_playing_offset().as_seconds());
                let _ = std::io::stdout().flush();
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                },
            _               => break

            }
        }
    }

    // Finished
    println!("\nDone!");

    // Wait until the user presses 'enter' key
    println!("Press enter to exit...");
    let _ = reader.read_line(&mut String::new());
}
