extern crate sfml;

use sfml::audio::{SoundStream, SoundStreamPlayer};
use sfml::system::Time;

// Melody by ryg - https://youtu.be/tCRPUv8V22o?t=176
struct BitMelody {
    buf: [i16; 1024],
    t: i32,
}

impl SoundStream for BitMelody {
    fn get_data(&mut self) -> (&mut [i16], bool) {
        for buf_sample in self.buf.iter_mut() {
            self.t = self.t.wrapping_add(1);
            let t = self.t;
            let melody = b"36364689";
            let index = t >> 13 & 7;
            let note = t * (melody[index as usize] as i32 & 15);
            let sample = (note / 12 & 128) +
                         (((((t >> 12) ^ (t >> 12) - 2) % 11 * t) / 4 | t >> 13) & 127);
            *buf_sample = sample as i16 * 128;
        }
        (&mut self.buf[..], true)
    }
    fn seek(&mut self, offset: Time) {
        // Not exactly correct, but meh.
        self.t = offset.as_milliseconds();
    }
}

impl BitMelody {
    fn new() -> Self {
        BitMelody {
            buf: [0; 1024],
            t: 0,
        }
    }
}

fn main() {
    let stream = BitMelody::new();
    let mut player = SoundStreamPlayer::new(stream, 1, 44100);
    player.play();
    loop {
        ::std::thread::sleep(::std::time::Duration::from_millis(100));
    }
}
