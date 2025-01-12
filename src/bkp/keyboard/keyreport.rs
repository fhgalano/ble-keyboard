use super::keys::Key;
use anyhow::{ anyhow, Result };

// structure for keyboard reports as defined by HID
pub struct KeyReport {
    modifiers: u8,
    reserved: u8,
    keys: [u8; 6],
}

impl KeyReport {
    pub fn new_from_key(key: Key, modifiers: &[Key]) -> Result<Self> {
        Ok(Self {
            modifiers: Self::modifier_byte_from_keys(modifiers)?,
            reserved: 0x00,
            keys: [key.scancode(), 0x00, 0x00, 0x00, 0x00, 0x00],
        })
    }

    pub fn new_from_keys(keys: &[Key], modifiers: &[Key]) -> Result<Self> {
        match keys.len() > 6 {
            true => Err(anyhow!("this dude be mashing (pressing more than 6 keys at once)")),
            false => Ok(Self {
                modifiers: Self::modifier_byte_from_keys(modifiers)?,
                reserved: 0x00, 
                keys: [0, 1, 2, 3, 4, 5]
                    .map(|idx| keys.get(idx).unwrap_or(&Key::NOKEY).scancode())
            })
        }
    }

    fn modifier_byte_from_keys(modifiers: &[Key]) -> Result<u8> {
        if modifiers.iter().any(|k| !Key::modifiers().contains(&k.scancode())) {
            return Err(anyhow!("cannot use non-modifier keys"))
        }

        let mut modifier_byte: u8 = 0;
        for key in modifiers {
            match key {
                Key::LCTRL => modifier_byte |= 0b1000_0000,
                Key::LSHIFT => modifier_byte |= 0b0100_0000,
                Key::LALT => modifier_byte |= 0b0010_0000,
                Key::LSUPER => modifier_byte |= 0b0001_0000,
                Key::RCTRL => modifier_byte |= 0b0000_1000,
                Key::RSHIFT => modifier_byte |= 0b0000_0100,
                Key::RALT => modifier_byte |= 0b0000_0010,
                Key::RSUPER => modifier_byte |= 0b0000_0001,
                _ => (),
            }
        }
        
        Ok(modifier_byte)
    }
}

impl Default for KeyReport {
    fn default() -> Self {
        Self {
            modifiers: 0x00,
            reserved: 0x00,
            keys: [0; 6],
        }
    }
}
