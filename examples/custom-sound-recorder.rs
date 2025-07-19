use {
    sfml::{
        audio::{Sound, SoundBuffer, SoundChannel, SoundRecorder, SoundRecorderDriver, capture},
        graphics::{Color, Font, RectangleShape, RenderTarget, RenderWindow, Text, Transformable},
        window::{Event, Key, Style},
    },
    std::{
        error::Error,
        sync::mpsc::{Receiver, Sender},
    },
};

include!("../example_common.rs");

struct MyRecorder {
    sender: Sender<Vec<i16>>,
}

impl MyRecorder {
    pub fn new() -> (Self, Receiver<Vec<i16>>) {
        let (send, recv) = std::sync::mpsc::channel();
        (Self { sender: send }, recv)
    }
}

impl SoundRecorder for MyRecorder {
    fn on_process_samples(&mut self, samples: &[i16]) -> bool {
        self.sender.send(samples.to_vec()).unwrap();
        true
    }
}

struct TextWriter<'font> {
    text: Text<'font>,
    x: f32,
    y_cursor: f32,
    font_size: u32,
}

impl<'font> TextWriter<'font> {
    fn new(font: &'font Font, font_size: u32, x: f32, init_y: f32) -> Self {
        Self {
            text: Text::new("", font, font_size),
            x,
            y_cursor: init_y,
            font_size,
        }
    }
    fn write(&mut self, text: &str, rw: &mut RenderWindow) {
        self.text.set_string(text);
        self.text.set_position((self.x, self.y_cursor));
        rw.draw(&self.text);
        self.y_cursor += self.font_size as f32 + 4.0;
    }
}

enum Mode {
    Main,
    SetDevice,
    SetSampleRate,
    SetChannelCount,
    Export,
}

fn main() -> Result<(), Box<dyn Error>> {
    example_ensure_right_working_dir();

    assert!(
        capture::is_available(),
        "Sorry, audio capture is not supported by your system"
    );
    let default_device = capture::default_device();
    let devices = capture::available_devices();
    let mut rw = RenderWindow::new(
        (800, 600),
        "Custom sound recorder",
        Style::CLOSE,
        Default::default(),
        &Default::default(),
    )?;
    rw.set_vertical_sync_enabled(true);
    let font = Font::from_file("sansation.ttf")?;
    let (mut rec, recv) = MyRecorder::new();
    let mut samp_buf = Vec::new();
    let mut driver = SoundRecorderDriver::new(&mut rec);
    let mut started = false;
    let mut snd_buf = SoundBuffer::new()?;
    let mut samp_accum = Vec::new();
    let mut sound = Some(Sound::new(&snd_buf));
    let mut selected_dev_idx = 0;
    let mut mode = Mode::Main;
    let mut input_buf = String::new();
    let mut desired_sample_rate = 44_100;
    let mut desired_channel_count = 2;

    while rw.is_open() {
        while let Some(ev) = rw.poll_event() {
            match ev {
                Event::Closed => rw.close(),
                Event::KeyPressed { code, .. } => match mode {
                    Mode::Main => match code {
                        Key::R => {
                            if started {
                                driver.stop();
                                sound = None;
                                snd_buf.load_from_samples(
                                    &samp_accum[..],
                                    desired_channel_count,
                                    desired_sample_rate,
                                    &[SoundChannel::Mono],
                                )?;
                                samp_accum.clear();
                                started = false;
                            } else {
                                driver.set_device(devices[selected_dev_idx].to_str()?)?;
                                driver.set_channel_count(desired_channel_count);
                                driver.start(desired_sample_rate)?;
                                started = true;
                            }
                        }
                        Key::P => {
                            if !started {
                                let sound = sound.insert(Sound::with_buffer(&snd_buf));
                                sound.play();
                            }
                        }
                        Key::D => mode = Mode::SetDevice,
                        Key::S => {
                            input_buf = desired_sample_rate.to_string();
                            mode = Mode::SetSampleRate;
                        }
                        Key::C => {
                            input_buf = desired_channel_count.to_string();
                            mode = Mode::SetChannelCount;
                        }
                        Key::E => {
                            input_buf = "export.wav".to_string();
                            mode = Mode::Export;
                        }
                        _ => {}
                    },
                    Mode::SetDevice => match code {
                        Key::Up => {
                            selected_dev_idx -= selected_dev_idx.saturating_sub(1);
                        }
                        Key::Down => {
                            if selected_dev_idx + 1 < devices.len() {
                                selected_dev_idx += 1;
                            }
                        }
                        Key::Enter | Key::Escape => {
                            mode = Mode::Main;
                        }
                        _ => {}
                    },
                    Mode::SetSampleRate => {
                        if code == Key::Enter {
                            desired_sample_rate = input_buf.parse()?;
                            mode = Mode::Main;
                        }
                    }
                    Mode::SetChannelCount => {
                        if code == Key::Enter {
                            desired_channel_count = input_buf.parse()?;
                            mode = Mode::Main;
                        }
                    }
                    Mode::Export => {
                        if code == Key::Enter {
                            snd_buf.save_to_file(&input_buf)?;
                            mode = Mode::Main;
                        }
                    }
                },
                Event::TextEntered { unicode } => match mode {
                    Mode::SetSampleRate | Mode::SetChannelCount => {
                        if unicode.is_ascii_digit() {
                            input_buf.push(unicode);
                        } else if unicode == 0x8 as char {
                            input_buf.pop();
                        }
                    }
                    Mode::Export => {
                        if !unicode.is_ascii_control() {
                            input_buf.push(unicode);
                        } else if unicode == 0x8 as char {
                            input_buf.pop();
                        }
                    }
                    Mode::Main | Mode::SetDevice => {}
                },
                _ => {}
            }
        }
        if let Ok(samples) = recv.try_recv() {
            samp_accum.extend_from_slice(&samples);
            samp_buf = samples;
        }
        rw.clear(Color::rgb(10, 60, 40));
        let mut writer = TextWriter::new(&font, 20, 0.0, 0.0);
        macro_rules! w {
            ($($arg:tt)*) => {
                writer.write(&format!($($arg)*), &mut rw);
            }
        }
        match mode {
            Mode::Main => {
                w!("D - set device, S - set sample rate, C - set channel count, E - export");
                let s = if started {
                    "Press R to stop recording"
                } else {
                    "Press R to start recording. Press P to play the recording."
                };
                w!("{s}");
                w!(
                    "{} @ {} Hz\n{} samples, {} channels, {} bytes recorded",
                    driver.device(),
                    driver.sample_rate(),
                    samp_buf.len(),
                    driver.channel_count(),
                    samp_accum.len() * 2,
                );
                let mut rect = RectangleShape::new();
                for (i, &sample) in samp_buf.iter().enumerate() {
                    let ratio = samp_buf.len() as f32 / rw.size().x as f32;
                    rect.set_position((i as f32 / ratio, 300.0));
                    rect.set_size((2.0, sample as f32 / 48.0));
                    rw.draw(&rect);
                }
            }
            Mode::SetDevice => {
                for (i, dev) in devices.iter().enumerate() {
                    let default_str = if dev == &*default_device {
                        " (default)"
                    } else {
                        ""
                    };
                    let color = if selected_dev_idx == i {
                        Color::YELLOW
                    } else {
                        Color::WHITE
                    };
                    writer.text.set_fill_color(color);
                    w!("{}: {}{default_str}", i + 1, dev.to_str()?);
                }
            }
            Mode::SetSampleRate => {
                w!("Enter desired sample rate");
                w!("{input_buf}");
            }
            Mode::SetChannelCount => {
                w!("Enter desired channel count");
                w!("{input_buf}");
            }
            Mode::Export => {
                w!("Enter filename to export as");
                w!("{input_buf}");
            }
        }
        rw.display();
    }
    Ok(())
}
