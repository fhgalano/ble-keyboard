// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]

// std
use std::collections::HashMap;
use std::ops::Mul;
use std::sync::{Arc, Mutex as StdMutex};

// external crates
use esp_idf_svc::hal::peripherals::*;
use esp_idf_svc::hal::task::block_on;
use esp32_nimble::{
  enums::*, hid::*, utilities::mutex::Mutex, BLEAdvertisementData, BLECharacteristic, BLEDevice,
  BLEHIDDevice, BLEScan, BLEServer, NimbleProperties, uuid128,
};
use zerocopy::{FromBytes, IntoBytes, transmute};

// local dependencies
use keyboard::{ Key, Keyboard, KeyCombo, Layer, layer, Matrix, MatrixLoc2};

// local
use ble_kdb_rs::{GpioMatrix, PhantomMatrix};
use ble_kdb_rs::WirelessKeyboard;
use ble_kdb_rs::WirelessMatrix;
use ble_kdb_rs::MultiMatrix;


fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    
    let mut args: Vec<String> = std::env::args().collect();
    let keymap_variant = args.pop().unwrap_or("right".to_string());

    let peripherals = Peripherals::take()?;

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
    
    let service_uuid = uuid128!("9410d4c1-bf2e-490c-821f-56d4f1a0e2d4");

    block_on(async {
        let gpio_matrix = GpioMatrix::new(rows, columns);
        let phantom_matrix = PhantomMatrix::new();
        ::log::info!("created gpio matrix...");
        let internal_keyboard = Keyboard::new(Box::new(phantom_matrix), key_map("big_mode"), mod_map());
        ::log::info!("created internal keyboard");
        let mut wireless_keyboard = WirelessKeyboard::new(internal_keyboard, Some(keymap_variant))?;
        if let Ok(mut wireless_matrix) = WirelessMatrix::new().await {
            let wm = Arc::new(StdMutex::new(wireless_matrix));
            let matrix = MultiMatrix::new_from_two(gpio_matrix, wm.clone());

            let overwrite_internal_keyboard = Keyboard::new(
                Box::new(matrix), key_map("big_mode"), mod_map()
            );

            wireless_keyboard.keyboard = overwrite_internal_keyboard;

            loop {
                if wireless_keyboard.connected() {
                    //::log::info!("wireless pos: {:?}", wireless_matrix.poll());
                    wireless_keyboard.poll_and_send();
                    //::log::info!("virtual_pos: {:?}", virtual_pos);
                }
                //let virtual_pos = matrix_loc_characteristic.read_value().await?;
                //::log::info!("virtual_pos: {:?}", virtual_pos);
                esp_idf_svc::hal::delay::FreeRtos::delay_ms(30);
            }

        };

        loop {
            if wireless_keyboard.connected() {
                wireless_keyboard.poll_and_send();
            }
            ::log::info!("In fallback loop");
            esp_idf_svc::hal::delay::FreeRtos::delay_ms(30);
        }

    })

}

fn key_map(variant: &str) -> HashMap<KeyCombo, Layer> {
    match variant {
        "right" => {
            let base_map = layer!(
                [Key::SIX_HAT, Key::SEVEN_AMPERSAND, Key::EIGHT_STAR, Key::NINE_LPARENTHESIS, Key::ZERO_RPARENTHESIS, Key::BACKSPACE],
                [Key::Yy, Key::Uu, Key::Ii, Key::Oo, Key::Pp, Key::DASH_UNDERSCORE],
                [Key::Hh, Key::Jj, Key::Kk, Key::Ll, Key::SEMICOLON_COLON, Key::SQUOTE_DQUOTE],
                [Key::Nn, Key::Mm, Key::COMMA_LANGLE, Key::PERIOD_RANGLE, Key::FSLASH_QUESTION, Key::ENTER],
                [Key::BACKSPACE, Key::BSLASH_BAR, Key::NOKEY, Key::SPACE, Key::NOKEY, Key::EQUALS_PLUS],
            );
            let paw_map = layer!(
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::PRINT_SCREEN],
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::LSQUARE_LCURLY, Key::RSQUARE_RCURYLY, Key::NOKEY],
                [Key::LEFT,  Key::DOWN, Key::RIGHT, Key::UP, Key::NOKEY, Key::NOKEY],
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY],
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY],
            );

            let mut big_map = HashMap::new();
            big_map.insert(KeyCombo::default(), base_map);
            big_map.insert(KeyCombo::new(&[Key::PAW]), paw_map);

            big_map
        },
        "big_mode" => {
            let base_map = layer!(
                [
                    Key::SIX_HAT, Key::SEVEN_AMPERSAND, Key::EIGHT_STAR, Key::NINE_LPARENTHESIS, Key::ZERO_RPARENTHESIS, Key::BACKSPACE,
                    Key::FIVE_PERCENT, Key::FOUR_DOLLAR, Key::THREE_HASH, Key::TWO_AT, Key::ONE_EXCLAMATION, Key::GRAVE_TILDE
                ],
                [
                    Key::Yy, Key::Uu, Key::Ii, Key::Oo, Key::Pp, Key::DASH_UNDERSCORE,
                    Key::Tt, Key::Rr, Key::Ee, Key::Ww, Key::Qq, Key::TAB
                ],
                [
                    Key::Hh, Key::Jj, Key::Kk, Key::Ll, Key::SEMICOLON_COLON, Key::SQUOTE_DQUOTE,
                    Key::Gg, Key::Ff, Key::Dd, Key::Ss, Key::Aa, Key::NOKEY
                ],
                [
                    Key::Nn, Key::Mm, Key::COMMA_LANGLE, Key::PERIOD_RANGLE, Key::FSLASH_QUESTION, Key::ENTER,
                    Key::Bb, Key::Vv, Key::Cc, Key::Xx, Key::Zz, Key::NOKEY
                ],
                [
                    Key::BACKSPACE, Key::BSLASH_BAR, Key::NOKEY, Key::SPACE, Key::NOKEY, Key::EQUALS_PLUS,
                    Key::NOKEY, Key::NOKEY, Key::ESCAPE, Key::SPACE, Key::NOKEY, Key::NOKEY
                ],
            );
            let paw_map = layer!(
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::PRINT_SCREEN],
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::LSQUARE_LCURLY, Key::RSQUARE_RCURYLY, Key::NOKEY],
                [Key::LEFT,  Key::DOWN, Key::RIGHT, Key::UP, Key::NOKEY, Key::NOKEY],
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY],
                [Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY, Key::NOKEY],
            );

            let mut big_map = HashMap::new();
            big_map.insert(KeyCombo::default(), base_map);

            big_map
        }
        _ => panic!("unknown keymap variant {}", variant),
    }
}

fn mod_map() -> Layer {
    let mut map = HashMap::new();
    // right side
    map.insert((4,2), Key::PAW);
    map.insert((4, 4), Key::RSHIFT);

    // left side
    map.insert((4,9), Key::LALT);
    map.insert((4,10), Key::LSUPER);
    map.insert((2,11), Key::LCTRL);
    map.insert((3,11), Key::LSHIFT);

    map
}
