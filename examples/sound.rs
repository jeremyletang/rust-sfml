use {
    sfml::{
        SfResult,
        audio::{
            Music, Sound, SoundBuffer,
            playback_device::{self, Notification},
            sound_source::{self, SoundSource},
        },
        system::{Time, sleep},
    },
    std::{
        io::Write,
        sync::mpsc,
        time::{Duration, Instant},
    },
};

include!("../example_common.rs");

// Notices when playback has stopped advancing, e.g. because no audio device could be opened.
// Without it, the playback loops would wait forever.
struct StallGuard {
    last_offset: f32,
    last_progress: Instant,
}

impl StallGuard {
    fn new() -> Self {
        Self {
            last_offset: -1.0,
            last_progress: Instant::now(),
        }
    }

    // Returns true once the playing offset has not moved for two seconds
    fn stalled(&mut self, offset: f32) -> bool {
        if offset > self.last_offset {
            self.last_offset = offset;
            self.last_progress = Instant::now();
        }
        self.last_progress.elapsed() > Duration::from_secs(2)
    }
}

// Display the playback devices and the one currently in use
fn print_playback_devices() {
    let default = playback_device::default_device();
    let devices = playback_device::available_devices();
    println!("{} playback devices available:", devices.len());
    for (i, device) in devices.iter().enumerate() {
        let def_str = if default.as_ref() == Some(device) {
            " (default)"
        } else {
            ""
        };
        println!("Device {i}: {device}{def_str}");
    }
    print_current_device();
}

// Display the current playback device and its sample rate
fn print_current_device() {
    match playback_device::device() {
        Some(device) => println!("Current device: {device}"),
        None => println!("Current device: none"),
    }
    println!(
        "Current device is the default: {}",
        playback_device::is_default_device()
    );
    match playback_device::device_sample_rate() {
        Some(rate) => println!("Current device sample rate: {rate} Hz"),
        None => println!("Current device sample rate: unknown"),
    }
}

fn print_notifications(rx: &mpsc::Receiver<Notification>) {
    for notification in rx.try_iter() {
        println!("\nNotification: {notification:?}");
    }
}

// Play a Sound, changing the playback device while it plays
fn play_sound() -> SfResult<()> {
    let buffer = SoundBuffer::from_file("canary.wav")?;

    // Display sound informations
    println!("canary.wav :");
    println!(" {} seconds", buffer.duration().as_seconds());
    println!(" {} samples / sec", buffer.sample_rate());
    println!(" {} channels", buffer.channel_count());

    print_playback_devices();
    let devices = playback_device::available_devices();
    let first_device = (!devices.is_empty()).then(|| devices[0].to_string());

    // SFML calls the notification callback from its own thread, so forward the notifications
    let (tx, rx) = mpsc::channel();
    playback_device::set_notification_callback(Some(move |notification: Notification| {
        let _ = tx.send(notification);
    }));

    let mut sound = Sound::with_buffer(&buffer);
    sound.play();

    // Change the playback device at evenly spaced points of the sound.
    // The sound keeps playing across each change.
    let interval = buffer.duration().as_seconds() / 5.0;
    let mut next_change: usize = 0;
    let mut stall = StallGuard::new();
    while sound.status() == sound_source::Status::Playing {
        // Display the playing position
        let offset = sound.playing_offset().as_seconds();
        print!("\rPlaying... {offset:.2}");
        let _ = std::io::stdout().flush();

        if stall.stalled(offset) {
            println!("\nPlayback stopped advancing, giving up");
            break;
        }

        print_notifications(&rx);

        if next_change < 4 && offset >= interval * (next_change + 1) as f32 {
            println!();
            match next_change {
                0 => println!(
                    "set_device_to_null: {:?}",
                    playback_device::set_device_to_null()
                ),
                1 => println!(
                    "set_device_to_default: {:?}",
                    playback_device::set_device_to_default()
                ),
                2 => match &first_device {
                    Some(name) => println!(
                        "set_device({name:?}): {:?}",
                        playback_device::set_device(name)
                    ),
                    None => println!("No playback device to select by name"),
                },
                _ => println!(
                    "set_device(\"no such device\"): {:?}",
                    playback_device::set_device("no such device")
                ),
            }
            print_current_device();
            next_change += 1;
        }

        // Leave some CPU time for other processes
        sleep(Time::milliseconds(100));
    }
    println!();
    print_notifications(&rx);
    playback_device::set_notification_callback(None::<fn(Notification)>);
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

    let mut stall = StallGuard::new();
    while music.status() == sound_source::Status::Playing {
        // Display the playing position
        let offset = music.playing_offset().as_seconds();
        print!("\rPlaying... {offset:.2}");
        let _ = std::io::stdout().flush();
        if stall.stalled(offset) {
            println!("\nPlayback stopped advancing, giving up");
            break;
        }
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
