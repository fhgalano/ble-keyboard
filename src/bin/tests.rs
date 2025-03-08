// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
#![allow(dead_code)]

// std
use std::collections::HashMap;

// external crates
use esp_idf_svc::hal::peripherals::*;

// local dependencies
use keyboard::{ Key, Keyboard, KeyCombo, Layer };
use keyboard::layer;

// local
use ble_kdb_rs::GpioMatrix;
use ble_kdb_rs::WirelessKeyboard;


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

    let matrix = GpioMatrix::new(rows, columns);
    let internal_keyboard = Keyboard::new(Box::new(matrix), key_map(&keymap_variant), mod_map());

    let mut wireless_keyboard = WirelessKeyboard::new(internal_keyboard, Some(keymap_variant))?;

    loop {
        if wireless_keyboard.connected() {
            wireless_keyboard.poll_and_send();
        }
        esp_idf_svc::hal::delay::FreeRtos::delay_ms(30);
    }
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
        _ => panic!("unknown keymap variant {}", variant),
    }
}

fn mod_map() -> Layer {
    let mut map = HashMap::new();
    map.insert((4,2), Key::PAW);
    map.insert((4, 4), Key::LSHIFT);

    map
}
