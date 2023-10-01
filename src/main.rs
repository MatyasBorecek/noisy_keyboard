extern crate device_query;
extern crate rodio;

use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::{source::SineWave, OutputStream};
use std::time::Duration;
use std::thread;

const SLOPE: f64 = 25795.0 / 29.0;

fn main() {
    // Get the default output device.
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let device_state = DeviceState::new();

    let mut sink = rodio::Sink::try_new(&stream_handle).unwrap();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if !keys.is_empty() {
            for key in keys {
                let char_value = keycode_to_char(key);
                let source =
                    SineWave::new(
                        map_ascii_to_value(
                            char_to_ascii(char_value.unwrap())
                        ) as f32
                    );

                sink = rodio::Sink::try_new(&stream_handle).unwrap();
                sink.append(source.clone());
            }
        } else {
            // Stop playing sound.
            sink.stop();
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn keycode_to_char(key: Keycode) -> Option<char> {
    let key_str = format!("{:?}", key);
    if key_str.len() == 1 {
        return key_str.chars().next();
    }

    return None;
}

fn char_to_ascii(c: char) -> u8 {
    return c as u8;
}

fn map_ascii_to_value(x: u8) -> i32 {
    (50.0 + SLOPE * (x as f64 - 97.0)).round() as i32
}