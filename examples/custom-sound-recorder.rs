extern crate sfml;

use std::{thread, time};
use sfml::audio::SoundRecorder;

fn main(){

    let sr = SoundRecorder::new(
        || {
            println!("Recording started!");
            true
        },
        |samples| {
            println!("Samples received! {}", samples.len());
            true
        },
        || {
            println!("Recording stopped!");
        }
    );

    sr.set_processing_interval(time::Duration::from_millis(10));
    sr.start(44100);
    thread::sleep(time::Duration::from_secs(3));
    sr.stop();

}