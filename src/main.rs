use std::{thread::sleep, time::Duration};

use hidapi::HidApi;
use psutil::{cpu, sensors};

enum DisplayType {
    Temp,
    Load,
}

// CPU Sensor - Tctl
// Vendor ID - 13875
// Product ID - 2
fn main() {
    let hidapi = match HidApi::new() {
        Ok(api) => api,
        Err(_e) => panic!("Could not initialize HID api"),
    };

    let hid = match HidApi::open(&hidapi, 13875, 2) {
        Ok(hid) => hid,
        Err(e) => {
            eprintln!("{:?}", e);
            panic!("Could not connect to device");
        }
    };

    let mut collector = cpu::CpuPercentCollector::new().unwrap();

    loop {
        let load = collector.cpu_percent().unwrap();

        let temperature_list = sensors::temperatures();

        let cpu_temp = temperature_list.iter().find(|&temp| {
            return match temp {
                Ok(t) => match t.label() {
                    Some(label) => label == "Tctl",
                    None => false,
                },
                Err(_e) => false,
            };
        });

        let temperature = match cpu_temp {
            Some(temp) => temp.as_ref().unwrap().current().celsius() as u32,
            None => 0,
        };

        if false {
            let _ = hid.write(&prep_value(DisplayType::Load, load.round() as u32));
        } else {
            let _ = hid.write(&prep_value(DisplayType::Temp, temperature));
        }

        sleep(Duration::from_secs(2))
    }
}

fn prep_value(display: DisplayType, value: u32) -> [u8; 64] {
    let mut b_array = [
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    b_array[1] = match display {
        DisplayType::Load => 76,
        DisplayType::Temp => 19,
    };

    let array: Vec<u8> = value
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    let len = array.len();

    for (key, value) in array.iter().enumerate() {
        b_array[6 - len + key] = *value;
    }

    return b_array;
}
