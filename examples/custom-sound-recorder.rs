use {
    sfml::audio::{capture, SoundRecorder, SoundRecorderDriver},
    std::{error::Error, fs::File, io::Write},
};

struct FileRecorder {
    file: File,
}

impl SoundRecorder for FileRecorder {
    fn on_process_samples(&mut self, data: &[i16]) -> bool {
        match self.file.write_all(unsafe {
            std::slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 2)
        }) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("Error writing to file: {e}");
                false
            }
        }
    }
}

impl FileRecorder {
    fn create(path: &str) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self { file })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
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
    print!("Enter device and channel count: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let mut tokens = input.trim().split_whitespace();
    let device_idx = match tokens.next() {
        Some(tok) => tok.parse::<usize>()?,
        None => 0,
    };
    let channel_count = match tokens.next() {
        Some(tok) => tok.parse::<u32>()?,
        None => 1,
    };
    let mut fr = FileRecorder::create("hello.pcm")?;
    let mut recorder = SoundRecorderDriver::new(&mut fr);
    match devices.get(device_idx) {
        Some(dev) => recorder.set_device(dev.to_str()?)?,
        None => eprintln!("Invalid device index: {device_idx}"),
    }
    recorder.set_channel_count(channel_count);
    recorder.start(44_100)?;
    println!(
        "Recorder properties:\n\
               sample rate: {}\n\
               channel count: {}\n\
               device: {}",
        recorder.sample_rate(),
        recorder.channel_count(),
        recorder.device()
    );
    let mut left = 5000;
    while left > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        left -= 100;
        print!("You have {left} left to record\r");
        let _ = std::io::stdout().flush();
    }
    Ok(())
}
