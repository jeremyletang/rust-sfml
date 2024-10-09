use sfml::{
    audio::{SoundStatus, SoundStream, SoundStreamPlayer},
    system::Time,
};

// Melody by ryg - https://youtu.be/tCRPUv8V22o?t=176
struct BitMelody {
    buf: [i16; BUF_SIZE],
    t: i32,
    vol: i16,
}

const FULLVOL_DURATION: i32 = 1_048_576;
const INIT_VOL: i16 = 128;
const BUF_SIZE: usize = 2048;

impl SoundStream for BitMelody {
    fn get_data(&mut self) -> (&[i16], bool) {
        for buf_sample in self.buf.iter_mut() {
            self.t = self.t.wrapping_add(1);
            let t = self.t;
            let melody = b"36364689";
            let index = t >> 13 & 7;
            let note = t * (i32::from(melody[index as usize]) & 15);
            let sample = ((note / 12) & 128)
                + ((((((t >> 12) ^ ((t >> 12) - 2)) % 11 * t) / 4) | t >> 13) & 127);
            *buf_sample = sample as i16 * self.vol;
            // Fade out after a while
            if t > FULLVOL_DURATION && t % 4096 == 0 {
                self.vol -= 1;
            }
        }
        (&self.buf, self.vol > 0)
    }
    fn seek(&mut self, offset: Time) {
        // Not exactly correct, but meh.
        self.t = offset.as_milliseconds();
    }
    fn channel_count(&self) -> u32 {
        1
    }
    fn sample_rate(&self) -> u32 {
        44_100
    }
}

impl BitMelody {
    fn new() -> Self {
        BitMelody {
            buf: [0; BUF_SIZE],
            t: 0,
            vol: INIT_VOL,
        }
    }
    fn total_duration_samples(&self) -> usize {
        (FULLVOL_DURATION + INIT_VOL as i32 * 4096) as usize
    }
}

fn main() {
    let mut stream = BitMelody::new();
    let total_dur = stream.total_duration_samples() as f32 / stream.sample_rate() as f32;
    let mut player = SoundStreamPlayer::new(&mut stream);
    player.play();
    while player.status() == SoundStatus::PLAYING {
        let current = player.playing_offset().as_seconds();
        eprint!("Playing custom sound stream: {current:06.03}/{total_dur:06.03}\r");
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    eprintln!();
}
