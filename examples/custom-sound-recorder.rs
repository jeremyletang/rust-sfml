extern crate sfml;

use std::{thread, time};
use sfml::audio::SoundRecorder;

fn main(){

    let sr = SoundRecorder::new(
        || {
            println!("Recording started!");
            true
        },
        |_| {
            println!("Samples received!");
            true
        },
        || {
            println!("Recording stopped!");
        }
    );

    sr.start(44100);
    thread::sleep(time::Duration::from_secs(3));
    sr.stop();

}