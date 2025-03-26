use std::sync::{Arc, Mutex};
use esp32_nimble::{
    enums::*, 
    utilities::BleUuid, 
    BLEDevice, 
    BLEScan,
    BLEClient,
    BLERemoteCharacteristic,
    BLERemoteService,
};
use esp_idf_svc::hal::task::block_on;
use log::*;

use keyboard::{Matrix, MatrixLoc2};

pub struct WirelessMatrix {
    client: BLEClient,
    notify_buffer: Arc<Mutex<Vec<u8>>>,
    //poll_service: Arc<Mutex<BLERemoteService>>,
    //poll_characteristic: Arc<Mutex<BLERemoteCharacteristic>>,
    pub poll_state: Vec<MatrixLoc2>,
}

impl WirelessMatrix {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let ble_device = BLEDevice::take();
        let service_uuid = esp32_nimble::uuid128!("9410d4c1-bf2e-490c-821f-56d4f1a0e2d4");

        // scan
        let mut scan = BLEScan::new();
        let device = scan
            .active_scan(true)
            .interval(100)
            .window(99)
            .start(ble_device, 10000, |device, data| {
                if data.is_advertising_service(&service_uuid) {
                    return Some(*device)
                }
                None
            })
            .await?;

        let matrix_notify_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        let matrix_notify_buffer2 = matrix_notify_buffer.clone();

        if let Some(device) = device {
            let mut client = ble_device.new_client();
            let mut wm = Self {
                client,
                notify_buffer: matrix_notify_buffer,
                //poll_service: Arc::new(Mutex::new(matrix_loc_service.to_owned())),
                //poll_characteristic: Arc::new(Mutex::new(matrix_loc_characteristic.to_owned())),
                poll_state: Vec::new(),
            };


            wm.client.on_connect(|client| {
                ::log::info!("Starting client connection");
            });

            wm.client.connect(&device.addr()).await?;
            ::log::info!("Client connection complete");
            esp_idf_svc::hal::delay::FreeRtos::delay_ms(100);

            wm.client.on_disconnect(|_| {
                ::log::info!("Disconnected from wireless matrix");
            });

            let matrix_loc_service = wm.client
                .get_service(service_uuid)
                .await?;
            esp_idf_svc::hal::delay::FreeRtos::delay_ms(100);

            let cuuid = esp32_nimble::uuid128!("05f09692-ac23-4e1b-8ff1-410d16cd07c2");
            let matrix_loc_characteristic = matrix_loc_service
                .get_characteristic(cuuid)
                .await?;

            matrix_loc_characteristic.on_notify(move |data| {
                ::log::info!("notified: {:?}", data);
                *matrix_notify_buffer2.lock().unwrap() = Vec::from(data);
            }).subscribe_notify(true).await?;

            return Ok(wm)
        }
        Err(anyhow::anyhow!("Failed to setup ble client for matrix"))
    }
}

impl Matrix for WirelessMatrix {
    fn poll(&mut self) -> Vec<keyboard::MatrixLoc> {
        let recent_data = self.notify_buffer.lock().unwrap();
        let mut rditer = recent_data.iter();
        let mut locs = Vec::new();

        while let (Some(row), Some(col)) = (rditer.next(), rditer.next()) {
            locs.push((row.clone(), col.clone()));
        }

        locs
    }
}
