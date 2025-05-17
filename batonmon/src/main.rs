use ddc_hi::{Ddc, Display, FeatureCode, VcpValue};
use evdev::{Device, EventSummary, KeyCode};
use std::path::PathBuf;
use std::{thread, time};
extern crate libnotify;
extern crate single_instance;

const INPUT_SOURCE: FeatureCode = 0x60;
const DISPLAYPORT: u16 = 0x0f;
const THUNDERBOLT: u16 = 0x19;

fn main() {
    let target_name = String::from("Sofabaton03B03 Consumer Control");

    assert!(ensure_single_instance("batonmon"));

    libnotify::init("batonmon").unwrap();

    loop {
        let devices = evdev::enumerate();
        let mut device_found: bool = false;

        for mut dev in devices {
            match dev.1.name() {
                Some(d) => {
                    if d == target_name {
                        println!("Found device: {}", d);
                        device_found = true;
                        poll_device(&mut dev);
                    }
                }
                None => println!("Invalid device"),
            }
        }

        if !device_found {
            println!("Could not find device");
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

pub fn ensure_single_instance(uniq_id: &str) -> bool {
    let instance = Box::new(single_instance::SingleInstance::new(uniq_id).unwrap());
    if instance.is_single() {
        Box::leak(instance);
        true
    } else {
        false
    }
}

fn poll_device(dev: &mut (PathBuf, Device)) {
    loop {
        let events = dev.1.fetch_events();
        match events {
            Ok(e) => {
                for event in e {
                    match event.destructure() {
                        EventSummary::Key(key_event, KeyCode::KEY_SEARCH, 1) => {
                            let notification = libnotify::Notification::new(
                                "Input Detected",
                                "Switching input",
                                "ok",
                            );
                            notification.show().unwrap();
                            println!("Key pressed: {:?}", key_event);
                            match toggle_input(1) {
                                Err(e) => println!("Could not toggle input: {:?}", e),
                                _ => {}
                            }
                        }
                        _ => (),
                    }
                }
            }
            Err(e) => {
                println!("Failed to fetch events: {:?}", e);
                break;
            }
        }
    }
}

fn toggle_input(display_index: usize) -> Result<(), anyhow::Error> {
    let notification = libnotify::Notification::new("", None, "display");
    let mut displays = Display::enumerate();
    println! {"Detected display {:?}", displays[display_index].info.model_name.clone().unwrap()};
    let current_output: VcpValue = displays[display_index]
        .handle
        .get_vcp_feature(INPUT_SOURCE)?;
    let output = match current_output.sl as u16 {
        DISPLAYPORT => {
            notification
                .update("Switching output", Some("Switching to THUNDERBOLT"), None)
                .unwrap();
            notification.show()?;
            THUNDERBOLT
        }
        THUNDERBOLT => {
            notification
                .update("Switching output", Some("Switching to DISPLAYPORT"), None)
                .unwrap();
            notification.show()?;
            DISPLAYPORT
        }
        _ => current_output.value(),
    };
    displays[display_index]
        .handle
        .set_vcp_feature(INPUT_SOURCE, output)
}
