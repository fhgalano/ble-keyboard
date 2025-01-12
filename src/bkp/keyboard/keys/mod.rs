#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Key {
    // None
    NOKEY,
    // modifiers 
    LCTRL,
    RCTRL,
    LSHIFT,
    RSHIFT,
    LALT,
    RALT,
    LSUPER,
    RSUPER,
    // normal keys
    Dd,
    Ee,
}

impl Key {
    pub const fn scancode(&self) -> u8 {
        match *self {
            Self::NOKEY => 0x00,

            Self::LCTRL => 0xE0,
            Self::RCTRL => 0xE4,
            Self::LSHIFT => 0xE1,
            Self::RSHIFT => 0xE5,
            Self::LALT => 0xE2,
            Self::RALT => 0xE6,
            Self::LSUPER => 0xE3,
            Self::RSUPER => 0xE7,

            Self::Dd => 0x07,
            Self::Ee => 0x08,
        }
    }

    pub fn modifiers() -> [u8; 8] {
        [
            Self::LCTRL.scancode(),
            Self::LSHIFT.scancode(), 
            Self::LALT.scancode(),
            Self::LSUPER.scancode(),
            Self::RCTRL.scancode(),
            Self::RSHIFT.scancode(),
            Self::RALT.scancode(),
            Self::RSUPER.scancode(),
        ]
    }
}
