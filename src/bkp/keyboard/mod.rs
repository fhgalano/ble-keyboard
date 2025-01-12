mod keycombo;
mod keyreport;
mod keys;
mod matrix;

use std::boxed::Box;
use std::collections::HashMap;
pub use keycombo::KeyCombo;
pub use keyreport::KeyReport;
pub use keys::Key;
pub use matrix::{ Matrix, MatrixLoc };

type Layer = HashMap<MatrixLoc, Key>;

pub struct Keyboard {
    matrix: Box<dyn matrix::Matrix>,
    key_map: HashMap<KeyCombo, Layer>,
    modifier_map: Layer, // specific layer for holding modifier keys
    report_queue: Vec<KeyReport>,
    state: Vec<MatrixLoc>,
}

impl Keyboard {
    pub fn new() -> Self {
        todo!();
    }

    pub fn poll(&mut self) {
        let new_state = self.matrix.poll();

        if new_state != self.state || new_state.len() > 0 {
            self.state = new_state;
            self.eval_state();
        }
    }

    fn eval_state(&mut self) {
        // check for modifier keys
        let pressed_modifiers: Vec<_> = self.state
            .iter()
            .filter_map(|key_loc| self.modifier_map.get(key_loc))
            .copied()
            .collect();

        // determine which key layer should be referenced
        let active_combo = self.key_map
            .keys()
            .filter(|kc| kc.detect(&pressed_modifiers))
            .max_by(|kc1, kc2| kc1.size().cmp(&kc2.size()))
            .copied()
            .unwrap_or_default();
        let active_layer = self.key_map.get(&active_combo).unwrap();

        // get keys from the correct map
        let keys = self.state
            .iter()
            .filter_map(|key_loc| active_layer.get(key_loc));

        // add report to the queue if valid
    }
}
