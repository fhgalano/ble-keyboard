// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]

// std
use std::sync::Arc;
use std::vec::Vec;
use std::collections::HashMap;

// external crates
use esp_idf_svc::hal::peripherals::*;
use esp_idf_svc::hal::gpio::*;
use esp32_nimble::{
  enums::*, hid::*, utilities::mutex::Mutex, BLEAdvertisementData, BLECharacteristic, BLEDevice,
  BLEHIDDevice, BLEServer,
};

// local dependencies
use keyboard::{ Key, Keyboard, KeyCombo, Layer };

// local
use ble_kdb_rs::GpioMatrix;
use ble_kdb_rs::WirelessKeyboard;


fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let rows = vec![
      peripherals.pins.gpio6.into(),
      peripherals.pins.gpio7.into(),
    ];

    let columns = vec![
      peripherals.pins.gpio4.into(),
      peripherals.pins.gpio5.into(),
    ];

    let matrix = GpioMatrix::new(rows, columns);
    let internal_keyboard = Keyboard::new(Box::new(matrix), key_map(), mod_map());

    let mut wireless_keyboard = WirelessKeyboard::new(internal_keyboard)?;

    loop {
        if wireless_keyboard.connected() {
            wireless_keyboard.test_message();
            ::log::info!("Sending 'Hello world'...");
        }
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(5000);
    }
}

fn key_map() -> HashMap<KeyCombo, Layer> {
    let mut map = HashMap::new();
    map.insert((0, 1), Key::Dd);
    map.insert((1, 0), Key::Ee);
    map.insert((1, 1), Key::Zz);

    let mut big_map = HashMap::new();
    big_map.insert(KeyCombo::default(), map);

    big_map
}

fn mod_map() -> Layer {
    let mut map = HashMap::new();
    map.insert((0, 0), Key::LSHIFT);

    map
}
