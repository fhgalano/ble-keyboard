// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]

// std
use std::collections::HashMap;

// external crates
use esp_idf_svc::hal::peripherals::*;
use esp32_nimble::{
  enums::*, hid::*, utilities::mutex::Mutex, BLEAdvertisementData, BLECharacteristic, BLEDevice,
  BLEHIDDevice, BLEServer, NimbleProperties, uuid128,
};
use zerocopy::IntoBytes;

// local dependencies
use keyboard::{ Key, Keyboard, KeyCombo, Layer, layer, Matrix, MatrixLoc, MatrixLoc2 };

// local
use ble_kdb_rs::GpioMatrix;
use ble_kdb_rs::WirelessKeyboard;


fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let mut args: Vec<String> = std::env::args().collect();
    let keymap_variant = args.pop().unwrap_or("right".to_string());

    let peripherals = Peripherals::take()?;

    // Virual Matrix Setup
    let rows = vec![
	  peripherals.pins.gpio2.into(),
	  peripherals.pins.gpio3.into(),
	  peripherals.pins.gpio0.into(),
	  peripherals.pins.gpio1.into(),
	  peripherals.pins.gpio10.into(),
    ];

    let columns = vec![
	  peripherals.pins.gpio19.into(),
	  peripherals.pins.gpio18.into(),
	  peripherals.pins.gpio4.into(),
	  peripherals.pins.gpio5.into(),
	  peripherals.pins.gpio6.into(),
	  peripherals.pins.gpio7.into(),
    ];

    let mut matrix = GpioMatrix::new(rows, columns);

    // Server Setup
    let ble_device = BLEDevice::take();
    BLEDevice::set_device_name("Secondary");
    let ble_advertising = ble_device.get_advertising();

    let server = ble_device.get_server();
    server.on_connect(|server, desc| {
        ::log::info!("Client connected: {:?}", desc);

        server
            .update_conn_params(desc.conn_handle(), 24, 48, 0, 300)
            .unwrap();
    });

    server.on_disconnect(|_desc, reason| {
        ::log::info!("Client disconnected ({:?})", reason);
    });

    // Setup Services and Characteristics
    let matrix_loc_service = server.create_service(uuid128!("9410d4c1-bf2e-490c-821f-56d4f1a0e2d4"));

    let matrix_loc_characteristic = matrix_loc_service.lock().create_characteristic(
        uuid128!("05f09692-ac23-4e1b-8ff1-410d16cd07c2"), 
        NimbleProperties::READ | NimbleProperties::NOTIFY
    );

    matrix_loc_characteristic.lock().on_read(|c, d| {
        ::log::info!("read request from: {}", d.address());
    });

    // Advertise
    ble_advertising.lock().set_data(
    BLEAdvertisementData::new()
        .name("Secondary")
        .add_service_uuid(uuid128!("9410d4c1-bf2e-490c-821f-56d4f1a0e2d4")),
    )?;
    ble_advertising.lock().start()?;

    server.ble_gatts_show_local();

    let mut counter: u8 = 1;
    let mut prev_state: Vec<MatrixLoc2> = Vec::new();

    loop {
        if server.connected_count() > 0 {
            let keys: Vec<MatrixLoc2> = matrix.poll().into_iter().map(|x| {x.into()}).collect();

            if prev_state != keys || !keys.is_empty() {
                matrix_loc_characteristic.lock().set_value(keys.as_bytes()).notify();
                prev_state = keys;
            }
        }
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(30);
    }

}

