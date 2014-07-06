/*!
* Example from SFML : Sound Capture
*/


#![crate_name = "sound_capture"]
#![desc = "Sound capture example for rsfml"]
#![crate_type = "bin"]
#![allow(unused_must_use)]

extern crate rsfml;

use std::rc::Rc;
use std::cell::RefCell;
use std::io::{BufferedReader, stdin};
use std::num::strconv;

use rsfml::audio::{rc, SoundBufferRecorder, Playing};
use rsfml::system::{sleep, Time};

fn main() -> () {
    // Check that the device can capture audio
    if !SoundBufferRecorder::is_available() {
        fail!("Sorry, audio capture is not supported by your system");
    }

    // Choose the sample rate
    println!("Please choose the sample rate for sound capture (44100 is CD quality) : ");
    let mut stdin = BufferedReader::new(stdin());
    let mut line = stdin.read_line().unwrap();
    unsafe { line.pop_byte(); }
    let sample_rate : uint = match strconv::from_str_common(line.as_slice(), 10, false, false, false, strconv::ExpNone, true, false) {
        Some(value)     => value,
        None            => fail!("Error, input is not valid")
    };

    // Wait for user input...
    println!("Press enter to start recording audio");
    stdin.read_line().unwrap();

    // Here we'll use an integrated custom recorder, which saves the captured data into a SoundBuffer
    let mut recorder : SoundBufferRecorder = match SoundBufferRecorder::new() {
        Some(rec)       => rec,
        None            => fail!("Error, cannot initialize Sound buffer recorder.")
    };

    // Audio capture is done in a separate thread, so we can block the main thread while it is capturing
    recorder.start(sample_rate);
    println!("Recording... press enter to stop");
    stdin.read_line().unwrap();
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = match recorder.get_buffer() {
        Some(buf)       => Rc::new(RefCell::new(buf)),
        None            => fail!("Error when retreiving buffer.")
    };

    // Display captured sound informations
    println!("Sound informations :");
    println!(" {} seconds", (*buffer).borrow().get_duration().as_seconds());
    println!(" {} samples / sec", (*buffer).borrow().get_sample_rate());
    println!(" {} channels", (*buffer).borrow().get_channel_count());


    // Choose what to do with the recorded sound data
    println!("What do you want to do with captured sound (p = play, s = save) ? ");
    let mut resp = stdin.read_line().unwrap();

    if unsafe { resp.pop_byte().unwrap() } == 's' as u8 {
        // Choose a filename
        println!("Choose the file to create : ");
        let filename = stdin.read_line().unwrap();

        // Save the buffer
        (*buffer).borrow().save_to_file(filename.as_slice());
    }
    else {
        let mut sound : rc::Sound = match rc::Sound::new_with_buffer(buffer.clone()) {
            Some(sound)     => sound,
            None            => fail!("Error cannot create Sound")
        };

         sound.play();

        loop {
            match sound.get_status() {
                Playing     => {
                // Display the playing position
                println!("\rPlaying...   {}", sound.get_playing_offset().as_seconds());
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                },
            _               => break

            }
        }
    }

    // Finished
    println!("Done!");

    // Wait until the user presses 'enter' key
    println!("Press enter to exit...");
    stdin.read_line();
}