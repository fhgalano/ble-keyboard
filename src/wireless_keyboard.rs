use esp32_nimble::{
  enums::*, hid::*, utilities::mutex::Mutex, BLEAdvertisementData, BLECharacteristic, BLEDevice,
  BLEHIDDevice, BLEServer,
};
use keyboard::*;
use std::sync::Arc;
use zerocopy::IntoBytes;

const KEYBOARD_ID: u8 = 0x01;
const MEDIA_KEYS_ID: u8 = 0x02;

const MANUFACTURER: &str = "DaVeggies";
const DEVICE_NAME: &str = "DeezKeys";

const HID_REPORT_DISCRIPTOR: &[u8] = hid!(
  (USAGE_PAGE, 0x01), // USAGE_PAGE (Generic Desktop Ctrls)
  (USAGE, 0x06),      // USAGE (Keyboard)
  (COLLECTION, 0x01), // COLLECTION (Application)
  // ------------------------------------------------- Keyboard
  // Modifiers
  (REPORT_ID, KEYBOARD_ID), //   REPORT_ID (1)
  (USAGE_PAGE, 0x07),       //   USAGE_PAGE (Kbrd/Keypad)
  (USAGE_MINIMUM, 0xE0),    //   USAGE_MINIMUM (0xE0)
  (USAGE_MAXIMUM, 0xE7),    //   USAGE_MAXIMUM (0xE7)
  (LOGICAL_MINIMUM, 0x00),  //   LOGICAL_MINIMUM (0)
  (LOGICAL_MAXIMUM, 0x01),  //   Logical Maximum (1)
  (REPORT_SIZE, 0x01),      //   REPORT_SIZE (1)
  (REPORT_COUNT, 0x08),     //   REPORT_COUNT (8)
  (HIDINPUT, 0x02), //   INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
                    //
  // Reserved  
  (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 1 byte (Reserved)
  (REPORT_SIZE, 0x08), //   REPORT_SIZE (8)
  (HIDINPUT, 0x01), //   INPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
                    //
  // LEDs
  (REPORT_COUNT, 0x05), //   REPORT_COUNT (5) ; 5 bits (Num lock, Caps lock, Scroll lock, Compose, Kana)
  (REPORT_SIZE, 0x01),  //   REPORT_SIZE (1)
  (USAGE_PAGE, 0x08),   //   USAGE_PAGE (LEDs)
  (USAGE_MINIMUM, 0x01), //   USAGE_MINIMUM (0x01) ; Num Lock
  (USAGE_MAXIMUM, 0x05), //   USAGE_MAXIMUM (0x05) ; Kana
  (HIDOUTPUT, 0x02), //   OUTPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
  // Padding
  (REPORT_COUNT, 0x01), //   REPORT_COUNT (1) ; 3 bits (Padding)
  (REPORT_SIZE, 0x03), //   REPORT_SIZE (3)
  (HIDOUTPUT, 0x01), //   OUTPUT (Const,Array,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
                        
  // Keys
  (REPORT_COUNT, 0x06), //   REPORT_COUNT (6) ; 6 bytes (Keys)
  (REPORT_SIZE, 0x08), //   REPORT_SIZE(8)
  (LOGICAL_MINIMUM, 0x00), //   LOGICAL_MINIMUM(0)
  (LOGICAL_MAXIMUM, 0x65), //   LOGICAL_MAXIMUM(0x65) ; 101 keys
  (USAGE_PAGE, 0x07), //   USAGE_PAGE (Kbrd/Keypad)
  (USAGE_MINIMUM, 0x00), //   USAGE_MINIMUM (0)
  (USAGE_MAXIMUM, 0x65), //   USAGE_MAXIMUM (0x65)
  (HIDINPUT, 0x00),  //   INPUT (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
  (END_COLLECTION),  // END_COLLECTION
  // ------------------------------------------------- Media Keys
  (USAGE_PAGE, 0x0C),         // USAGE_PAGE (Consumer)
  (USAGE, 0x01),              // USAGE (Consumer Control)
  (COLLECTION, 0x01),         // COLLECTION (Application)
  (REPORT_ID, MEDIA_KEYS_ID), //   REPORT_ID (3)
  (USAGE_PAGE, 0x0C),         //   USAGE_PAGE (Consumer)
  (LOGICAL_MINIMUM, 0x00),    //   LOGICAL_MINIMUM (0)
  (LOGICAL_MAXIMUM, 0x01),    //   LOGICAL_MAXIMUM (1)
  (REPORT_SIZE, 0x01),        //   REPORT_SIZE (1)
  (REPORT_COUNT, 0x10),       //   REPORT_COUNT (16)
  (USAGE, 0xB5),              //   USAGE (Scan Next Track)     ; bit 0: 1
  (USAGE, 0xB6),              //   USAGE (Scan Previous Track) ; bit 1: 2
  (USAGE, 0xB7),              //   USAGE (Stop)                ; bit 2: 4
  (USAGE, 0xCD),              //   USAGE (Play/Pause)          ; bit 3: 8
  (USAGE, 0xE2),              //   USAGE (Mute)                ; bit 4: 16
  (USAGE, 0xE9),              //   USAGE (Volume Increment)    ; bit 5: 32
  (USAGE, 0xEA),              //   USAGE (Volume Decrement)    ; bit 6: 64
  (USAGE, 0x23, 0x02),        //   Usage (WWW Home)            ; bit 7: 128
  (USAGE, 0x94, 0x01),        //   Usage (My Computer) ; bit 0: 1
  (USAGE, 0x92, 0x01),        //   Usage (Calculator)  ; bit 1: 2
  (USAGE, 0x2A, 0x02),        //   Usage (WWW fav)     ; bit 2: 4
  (USAGE, 0x21, 0x02),        //   Usage (WWW search)  ; bit 3: 8
  (USAGE, 0x26, 0x02),        //   Usage (WWW stop)    ; bit 4: 16
  (USAGE, 0x24, 0x02),        //   Usage (WWW back)    ; bit 5: 32
  (USAGE, 0x83, 0x01),        //   Usage (Media sel)   ; bit 6: 64
  (USAGE, 0x8A, 0x01),        //   Usage (Mail)        ; bit 7: 128
  (HIDINPUT, 0x02), // INPUT (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
  (END_COLLECTION), // END_COLLECTION
);

pub struct WirelessKeyboard {
  server: &'static mut BLEServer,
  input_keyboard: Arc<Mutex<BLECharacteristic>>,
  #[allow(unused)]
  output_keyboard: Arc<Mutex<BLECharacteristic>>,
  input_media_keys: Arc<Mutex<BLECharacteristic>>,
  pub keyboard: Keyboard,
}

impl WirelessKeyboard {
  pub fn new(
      keyboard: Keyboard,
      display_name: Option<String>,
  ) -> anyhow::Result<Self> {
    let mut device = BLEDevice::take();
    device
      .security()
      .set_auth(AuthReq::Bond)
      .set_passkey(69420)
      .set_io_cap(SecurityIOCap::NoInputNoOutput)
      .resolve_rpa();

    match display_name {
        Some(n) => BLEDevice::set_device_name(&format!("{}_{}", DEVICE_NAME, n)),
        None => BLEDevice::set_device_name(DEVICE_NAME),
    };

    let server = device.get_server();
    let mut hid = BLEHIDDevice::new(server);

    let input_keyboard = hid.input_report(KEYBOARD_ID);
    let output_keyboard = hid.output_report(KEYBOARD_ID);
    let input_media_keys = hid.input_report(MEDIA_KEYS_ID);

    hid.manufacturer(MANUFACTURER);
    hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
    hid.hid_info(0x00, 0x01);

    hid.report_map(HID_REPORT_DISCRIPTOR);

    hid.set_battery_level(100);

    let ble_advertising = device.get_advertising();
    ble_advertising.lock().scan_response(false).set_data(
      BLEAdvertisementData::new()
        .name(DEVICE_NAME)
        .appearance(0x03C1)
        .add_service_uuid(hid.hid_service().lock().uuid()),
    )?;
    ble_advertising.lock().start()?;

    Ok(Self {
      server,
      input_keyboard,
      output_keyboard,
      input_media_keys,
      keyboard
    })
  }

  pub fn connected(&self) -> bool {
    self.server.connected_count() > 0
  }

  fn send_report(&self, keys: &KeyReport) {
    self
      .input_keyboard
      .lock()
      .set_value(keys.as_bytes())
      .notify();
    esp_idf_svc::hal::delay::Ets::delay_ms(7);
  }

  pub fn poll_and_send(&mut self) {
      self.keyboard.poll();

      if let Some(rpt) = self.keyboard.give_report() {
          ::log::info!("Sending Report: {:?}", &rpt);
          self.send_report(&rpt);
      }
  }

  pub fn test_message(&self) {
      let test_report = KeyReport::new(0x00, [0x07, 0x00, 0x00, 0x00, 0x00, 0x00]);

      ::log::info!("{:?}", &test_report);
      self.send_report(&test_report);
      
      esp_idf_svc::hal::delay::Ets::delay_ms(100);
      self.send_report(&KeyReport::default());
  }
}

