use std::sync::{Arc, Mutex};
use keyboard::{Matrix, MatrixLoc2};
use crate::wireless_matrix::WirelessMatrix;
use crate::GpioMatrix;

pub struct MultiMatrix <'a> {
    local_matrix: GpioMatrix<'a>,
    remote_matrix: Arc<Mutex<WirelessMatrix>>,
}

impl <'a> MultiMatrix <'a> {
    pub fn new_from_two(local_matrix: GpioMatrix<'a>, remote_matrix: Arc<Mutex<WirelessMatrix>>) -> Self {
        Self {
            local_matrix,
            remote_matrix,
        }
    }
}

impl Matrix for MultiMatrix<'_> {
    fn poll(&mut self) -> Vec<keyboard::MatrixLoc> {
        let mut locs: Vec<keyboard::MatrixLoc> = Vec::new();
        let mut counter: u8 = 0;
        for m in [self.local_matrix.poll(), self.remote_matrix.lock().unwrap().poll()] {
            let mut temp = m
                .iter()
                .map(|(r, c)| {
                    (*r, *c + (counter * 6))
                })
                .collect();
            locs.append(&mut temp);
            counter += 1;
        }

        ::log::info!("polled - {:?}", &locs);
        locs
    }
}
