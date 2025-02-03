mod wireless_keyboard;

use esp_idf_svc::hal::gpio::{ AnyIOPin, PinDriver, Output, Input, Pull};

use keyboard::Matrix;

pub use wireless_keyboard::WirelessKeyboard;

// INFO: Read on rows, output on columns
pub struct GpioMatrix<'a> {
    row_pins: Vec<PinDriver<'a, AnyIOPin, Input>>,
    col_pins: Vec<PinDriver<'a, AnyIOPin, Output>>,
}

impl <'a> GpioMatrix<'a> {
    pub fn new(row_pins: Vec<AnyIOPin>, col_pins: Vec<AnyIOPin>) -> Self {
        let mut matrix = Self {
            row_pins: row_pins.into_iter().map(|rp| PinDriver::input(rp).unwrap()).collect(),
            col_pins: col_pins.into_iter().map(|cp| PinDriver::output(cp).unwrap()).collect(),
        };

        for cp in matrix.col_pins.iter_mut() {
            cp.set_low().unwrap();
        }

        for rp in matrix.row_pins.iter_mut() {
            rp.set_pull(Pull::Down).unwrap();
        }
        matrix
    }
}

impl Matrix for GpioMatrix<'_> {
    fn poll(&mut self) -> Vec<keyboard::MatrixLoc> {
        let mut positions: Vec<keyboard::MatrixLoc> = Vec::new();
        for (col_num, col) in self.col_pins.iter_mut().enumerate() {
            if let Ok(_) = col.set_high() {
                for (row_num, row) in self.row_pins.iter_mut().enumerate() {
                    if row.is_high() {
                        positions.push((row_num as u8, col_num as u8));
                    }
                }
            }
            col.set_low().unwrap();
        }
        ::log::info!("active pos: {:?}", &positions);
        positions
    }
}

