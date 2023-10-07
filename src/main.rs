extern crate device_query;
extern crate rodio;

use std::collections::HashSet;
use device_query::{DeviceQuery, DeviceState, Keycode};
use rodio::{source::SineWave, OutputStream};
use std::time::Duration;
use std::thread;

const SLOPE: f64 = (25790.0 / 170.0) - 170.0;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let device_state = DeviceState::new();

    let mut active_sinks: Vec<rodio::Sink> = Vec::new();
    let mut previous_keys = HashSet::new();

    loop {
        let keys: HashSet<Keycode> = device_state.get_keys().into_iter().collect();

        for &key in &keys {
            if !previous_keys.contains(&key) {
                let source = SineWave::new(map_ascii_to_value(key_to_ascii(key)) as f32);
                let sink = rodio::Sink::try_new(&stream_handle).unwrap();
                sink.append(source.clone());
                active_sinks.push(sink);
            }
        }

        active_sinks.retain(|sink| {
            for &key in &previous_keys {
                if !keys.contains(&key) {
                    sink.stop();
                    return false;
                }
            }
            true
        });

        previous_keys = keys;
        thread::sleep(Duration::from_millis(10));
    }
}

fn key_to_ascii(key: Keycode) -> u8 {
    return match key {
        Keycode::Space => 32,
        Keycode::Key0 => 48,
        Keycode::Key1 => 49,
        Keycode::Key2 => 50,
        Keycode::Key3 => 51,
        Keycode::Key4 => 52,
        Keycode::Key5 => 53,
        Keycode::Key6 => 54,
        Keycode::Key7 => 55,
        Keycode::Key8 => 56,
        Keycode::Key9 => 57,
        Keycode::Escape => 16,
        Keycode::LControl => 17,
        Keycode::RControl => 18,
        Keycode::LShift => 14,
        Keycode::RShift => 15,
        Keycode::LAlt => 19,
        Keycode::RAlt => 20,
        Keycode::Backspace => 8,
        Keycode::Tab => 9,
        Keycode::End => 3,
        Keycode::Numpad0 => 48,
        Keycode::Numpad1 => 49,
        Keycode::Numpad2 => 50,
        Keycode::Numpad3 => 51,
        Keycode::Numpad4 => 52,
        Keycode::Numpad5 => 53,
        Keycode::Numpad6 => 54,
        Keycode::Numpad7 => 55,
        Keycode::Numpad8 => 56,
        Keycode::Numpad9 => 57,
        Keycode::NumpadSubtract => 45,
        Keycode::NumpadAdd => 43,
        Keycode::NumpadDivide => 47,
        Keycode::NumpadMultiply => 42,
        _ => {
            let key_str = format!("{:?}", key);
            if key_str.len() == 1 {
                return key_str.chars().next().unwrap() as u8;
            }

            //Fallback for other special keys
            return 100;
        }
    };
}

fn map_ascii_to_value(x: u8) -> i32 {
    (SLOPE * x as f64).round() as i32
}