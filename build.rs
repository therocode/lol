use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::{thread, time};

fn main() {
    let device = rodio::default_output_device().unwrap();

    let file = File::open("lol.ogg").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());

    let ten_millis = time::Duration::from_millis(10000);
    let _ = time::Instant::now();

    thread::sleep(ten_millis);
}
