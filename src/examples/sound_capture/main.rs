/**
* Example from SFML : Sound Capture
*/

extern mod rsfml;

use std::rt::io;

use std::num::strconv;

use rsfml::audio::{Sound, SoundRecorder, SoundBufferRecorder, sound_status};
use rsfml::system::{sleep, Time};

fn main() -> () {
    
    // Check that the device can capture audio
    if SoundRecorder::is_available() == false {
        fail!("Sorry, audio capture is not supported by your system");
    }

    // Choose the sample rate
    print!("Please choose the sample rate for sound capture (44100 is CD quality) : ");
    let stdin = io::stdin();
    let line = stdin.read_line();
    let sampleRate : uint = match strconv::from_str_common(line, 10, false, false, false, strconv::ExpNone, true, false) {
        Some(value)     => value,
        None            => fail!("Error, input is not valid")
    };

    // Wait for user input...
    println!("Press enter to start recording audio");
    stdin.read_line();

    // Here we'll use an integrated custom recorder, which saves the captured data into a SoundBuffer
    let mut recorder : SoundBufferRecorder = match SoundBufferRecorder::new() {
        Some(rec)       => rec,
        None            => fail!("Error, cannot initialize Sound buffer recorder.")
    };

    // Audio capture is done in a separate thread, so we can block the main thread while it is capturing
    recorder.start(sampleRate);
    print!("Recording... press enter to stop");
    stdin.read_line();
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = match recorder.get_buffer() {
        Some(buf)       => @buf,
        None            => fail!("Error when retreiving buffer.")
    };

    // Display captured sound informations
    println("Sound informations :");
    println!(" {} seconds", buffer.get_duration().as_seconds());
    println!(" {} samples / sec", buffer.get_sample_rate());
    println!(" {} channels", buffer.get_channel_count());
    

    // Choose what to do with the recorded sound data
    print!("What do you want to do with captured sound (p = play, s = save) ? ");
    let mut resp = stdin.read_line();
    
    if resp.pop_char() == 's' {
        // Choose a filename
        print!("Choose the file to create : ");
        let filename = stdin.read_line();
        
        // Save the buffer
        buffer.save_to_file(filename);
    }
    else {
        let mut sound : Sound = match Sound::new_with_buffer(buffer) {
            Some(sound)     => sound,
            None            => fail!("Error cannot create Sound")
        };
        
         sound.play();
        
        loop {
            match sound.get_status() {
                sound_status::Playing       => {
                // Display the playing position
                println!("\rPlaying...   {}", sound.get_playing_offset().as_seconds());
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                },
            _                           => break

            }
        }
    }
    
    // Finished
    println!("Done!");
    
    // Wait until the user presses 'enter' key
    println!("Press enter to exit...");
    stdin.read_line();
}