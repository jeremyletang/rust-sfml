//! Example which produces a brief buzz using SoundStream.

extern crate sfml;

use sfml::audio::{SoundStream, SoundStreamImpl, PlayableSound, SoundStatus};
use sfml::system::{sleep, Time};
use std::io::{stdout, Write};

struct AudioSource {
	buf: [i16; 44100],
	count: u32
}

impl SoundStreamImpl for AudioSource {
	fn get_data(&mut self) -> &[i16] {
		self.count += 1;
		if self.count > 50 {
			return &[]
		}
		for i in 0..20 {
			self.buf[i * 2] = 30000;
		}
		&self.buf[0..400]
	}

	fn seek(&mut self, _time: Time) {
	}
}

fn main() {
	let mut source = AudioSource { buf: [0; 44100], count: 0 };
	let mut sound = SoundStream::new(&mut source, 1, 22050).expect("Failed to create SoundStream");

    // Display sound informations
    println!("generated sound :");
    println!(" {} samples / sec", sound.get_sample_rate());
    println!(" {} channels", sound.get_channel_count());

	sound.play();

	while sound.get_status() == SoundStatus::Playing {
		// Display the playing position
		print!("\rPlaying...   {}  ", sound.get_playing_offset().as_seconds());
		stdout().flush().unwrap();
		// Leave some CPU time for other processes
		sleep(Time::with_milliseconds(100));
	}
	println!("");
}
