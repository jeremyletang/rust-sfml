use {
    sfml::{
        audio::{
            Sound, SoundBufferRecorder, capture,
            sound_source::{self, SoundSource},
        },
        system::{Time, sleep},
    },
    std::{
        error::Error,
        io::{BufRead, Write},
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    // Check that the device can capture audio
    assert!(
        capture::is_available(),
        "Sorry, audio capture is not supported by your system"
    );
    let default_device = capture::default_device();
    let devices = capture::available_devices();
    println!("{} recording devices available:", devices.len());
    for (i, device) in devices.iter().enumerate() {
        let def_str = if device == &*default_device {
            " (default)"
        } else {
            ""
        };
        println!("Device {i}: {device}{def_str}");
    }

    // Choose the sample rate
    println!(
        "Please input device and sample rate, then press enter\n\
              Example:\n\
              1 22050 # record at 22050 Hz on device 1\n\
              Default (on empty input) is device 0 and 44100 Hz (CD quality)"
    );
    let stdin = std::io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let input = line.trim_end();
    let mut tokens = input.split_whitespace();
    let device_idx = if let Some(token) = tokens.next() {
        match token.parse::<usize>() {
            Ok(value) => value,
            Err(e) => return Err(format!("Input is not valid: {e}").into()),
        }
    } else {
        0
    };
    let sample_rate = if let Some(token) = tokens.next() {
        match token.parse::<u32>() {
            Ok(value) => value,
            Err(e) => return Err(format!("Input is not valid: {e}").into()),
        }
    } else {
        44_100
    };

    // Here we'll use an integrated custom recorder,
    // which saves the captured data into a SoundBuffer
    let mut recorder = SoundBufferRecorder::new();
    match devices.get(device_idx) {
        Some(device) => {
            recorder.set_device(device.to_str()?)?;
        }
        None => {
            eprintln!("No device with index {device_idx}");
        }
    }

    // Audio capture is done in a separate thread,
    // so we can block the main thread while it is capturing
    recorder.start(sample_rate)?;
    println!(
        "Recording on device {} @ {} Hz...\nPress enter to stop",
        recorder.device().to_str().unwrap_or("<invalid utf-8>"),
        recorder.sample_rate()
    );
    reader.read_line(&mut String::new())?;
    recorder.stop();

    // Get the buffer containing the captured data
    let buffer = recorder.buffer();

    // Display captured sound information
    println!("Sound information :");
    println!(" {} seconds", buffer.duration().as_seconds());
    println!(" {} samples / sec", buffer.sample_rate());
    println!(" {} channels", buffer.channel_count());

    // Choose what to do with the recorded sound data
    print!("What do you want to do with captured sound (p = play, s = save) ? ");
    let _ = std::io::stdout().flush();
    let mut resp = String::new();
    reader.read_line(&mut resp)?;

    if resp.trim() == "s" {
        // Choose a filename
        println!("Choose the file to create: ");
        let mut filename = String::new();
        reader.read_line(&mut filename)?;

        // Save the buffer
        if buffer.save_to_file(filename.trim()).is_err() {
            eprintln!("Error saving buffer to {}!", filename.trim());
        }
    } else {
        let mut sound = Sound::with_buffer(buffer);

        sound.play();

        while sound.status() == sound_source::Status::Playing {
            // Display the playing position
            print!(
                "\rPlaying... {:.2} sec",
                sound.playing_offset().as_seconds()
            );
            let _ = std::io::stdout().flush();
            // Leave some CPU time for other processes
            sleep(Time::milliseconds(100));
        }
    }

    // Finished
    println!("\nDone!");
    Ok(())
}
