extern crate sfml_system;
extern crate sfml_audio;

use sfml_audio::{SoundStatus, SoundStream, SoundStreamPlayer};
use sfml_system::Time;

// Melody by ryg - https://youtu.be/tCRPUv8V22o?t=176
struct BitMelody {
    buf: [i16; 2048],
    t: i32,
    amp: i16,
}

impl SoundStream for BitMelody {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        for buf_sample in self.buf.iter_mut() {
            self.t = self.t.wrapping_add(1);
            let t = self.t;
            let melody = b"36364689";
            let index = t >> 13 & 7;
            let note = t * (melody[index as usize] as i32 & 15);
            let sample = ((note / 12) & 128) +
                         ((((((t >> 12) ^ ((t >> 12) - 2)) % 11 * t) / 4) | t >> 13) & 127);
            *buf_sample = sample as i16 * self.amp;
            // Fade out after a while
            if t > 1_048_576 && t % 4096 == 0 {
                self.amp -= 1;
            }
        }
        (&mut self.buf[..], self.amp > 0)
    }
    fn seek(&mut self, offset: Time) {
        // Not exactly correct, but meh.
        self.t = offset.as_milliseconds();
    }
    fn channel_count(&self) -> u32 {
        1
    }
    fn sample_rate(&self) -> u32 {
        44100
    }
}

impl BitMelody {
    fn new() -> Self {
        BitMelody {
            buf: [0; 2048],
            t: 0,
            amp: 128,
        }
    }
}

fn main() {
    let mut stream = BitMelody::new();
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();
    while player.get_status() == SoundStatus::Playing {
        ::std::thread::sleep(::std::time::Duration::from_millis(100));
    }
}
