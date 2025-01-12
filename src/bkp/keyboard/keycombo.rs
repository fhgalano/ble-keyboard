use super::keys::Key;
use std::vec::Vec;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct KeyCombo {
    key_0: Option<Key>,
    key_1: Option<Key>,
    key_2: Option<Key>
}

impl KeyCombo {
    pub fn new(keys: &[Key]) -> Self {
        Self {
            key_0: keys.get(0).copied(),
            key_1: keys.get(1).copied(),
            key_2: keys.get(2).copied(),
        }
    }

    pub fn detect(&self, keys: &[Key]) -> bool {
        self.keys().iter().all(|k| keys.contains(k))
    }

    fn keys(&self) -> Vec<Key> {
        let real_keys: Vec<_> = [self.key_0, self.key_1, self.key_2].iter().filter_map(|x| *x).collect();

        real_keys
    }

    pub fn size(&self) -> usize {
        self.keys().len()
    }
}

impl Default for KeyCombo {
    fn default() -> Self {
        Self {
            key_0: None,
            key_1: None,
            key_2: None,
        }
    }
}
