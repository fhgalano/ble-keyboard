use keyboard::Matrix;

pub struct PhantomMatrix;

impl PhantomMatrix {
    pub fn new() -> Self {
        Self
    }
}

impl Matrix for PhantomMatrix {
    fn poll(&mut self) -> Vec<keyboard::MatrixLoc> {
        Vec::new()
    }
}

