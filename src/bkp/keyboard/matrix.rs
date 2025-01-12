use std::vec::Vec;

pub type MatrixLoc = (u8, u8);

pub trait Matrix {
    fn poll(&self) -> Vec<MatrixLoc>;
}
