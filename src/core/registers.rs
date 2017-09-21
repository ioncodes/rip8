const START_ADDRESS: u16 = 0x200; // todo: might also be 0x600

pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub i: u16,
    pub v: [u8; 16], // V0 - VF
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: [u16; 16]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: START_ADDRESS,
            sp: 0,
            i: 0,
            v: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16]
        }
    }
}