/**
* Example from SFML : Sound Capture
*/

extern mod rsfml;

use std::io;
use std::num::strconv;

use rsfml::audio::sound::*;
use rsfml::audio::sound_recorder::SoundRecorder;
use rsfml::audio::sound_buffer_recorder::SoundBufferRecorder;
use rsfml::audio::sound_status;
use rsfml::system::sleep::sleep;
use rsfml::system::time::Time;

fn main() -> () {
    
    // Check that the device can capture audio
    if SoundRecorder::is_available() == false {
        fail!("Sorry, audio capture is not supported by your system");
    }

    // Choose the sample rate
    io::print("Please choose the sample rate for sound capture (44100 is CD quality) : ");
    let stdin = io::stdin();
    let line = stdin.read_line();
    let sampleRate : uint = match strconv::from_str_common(line, 10, false, false, false, strconv::ExpNone, true, false) {
        Some(value)     => value,
        None            => fail!("Error, input is not valid")
    };

    // Wait for user input...
    io::println("Press enter to start recording audio");
    stdin.read_line();

    // Here we'll use an integrated custom recorder, which saves the captured data into a SoundBuffer
    let mut recorder : SoundBufferRecorder = match SoundBufferRecorder::new() {
        Some(rec)       => rec,
        None            => fail!("Error, cannot initialize Sound buffer recorder.")
    };

    // Audio capture is done in a separate thread, so we can block the main thread while it is capturing
    recorder.start(sampleRate);
    io::print("Recording... press enter to stop");
    stdin.read_line();
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = match recorder.get_buffer() {
        Some(buf)       => buf,
        None            => fail!("Error when retreiving buffer.")
    };

    // Display captured sound informations
    io::println("Sound informations :");
    io::println(fmt!(" %f seconds", buffer.get_duration().as_seconds() as float));
    io::println(fmt!(" %d samples / sec", buffer.get_sample_rate() as int));
    io::println(fmt!(" %d channels", buffer.get_channel_count() as int));
    

    // Choose what to do with the recorded sound data
    io::print("What do you want to do with captured sound (p = play, s = save) ? ");
    let mut resp = stdin.read_line();
    
    if resp.pop_char() == 's' {
        // Choose a filename
        io::print("Choose the file to create : ");
        let filename = stdin.read_line();
        
        // Save the buffer
        buffer.save_to_file(filename);
    }
    else {
        let mut sound : Sound = match Sound::new() {
            Some(sound)     => sound,
            None            => fail!("Error cannot create Sound")
        };
        
        sound.set_buffer(&buffer);
        sound.play();
        
        loop {
            match sound.get_status() {
                sound_status::Playing       => {
                // Display the playing position
                io::println(fmt!("\rPlaying...   %f", sound.get_playing_offset().as_seconds() as float));
                // Leave some CPU time for other processes
                sleep(Time::with_milliseconds(100));
                },
            _                           => break

            }
        }
    }
    
    // Finished
    io::println("Done!");
    
    // Wait until the user presses 'enter' key
    io::println("Press enter to exit...");
    stdin.read_line();
}