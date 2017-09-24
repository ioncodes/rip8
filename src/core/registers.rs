use std::thread;

const START_ADDRESS: u16 = 0x200; // todo: might also be 0x600
pub static mut DELAY_TIMER: u8 = 0;
pub static mut SOUND_TIMER: u8 = 0;

#[derive(Debug, Clone)]
pub struct Registers {
    pub pc: u16,
    pub sp: u8,
    pub i: u16,
    pub v: [u8; 16], // V0 - VF
    pub stack: Vec<u16>
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: START_ADDRESS,
            sp: 0,
            i: 0,
            v: [0; 16],
            stack: Vec::new()
        }
    }

    pub fn step(&mut self) {
        self.pc += 2; // each instruction has 2 bytes
    }

    pub fn jump(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn start_delay_timer(&self) {
        // todo: 60Hz
        thread::spawn(move || {
            unsafe {
                loop {
                    if DELAY_TIMER > 0 {
                        DELAY_TIMER -= 1;
                    }
                }
            }
        });
    }

    pub fn start_sound_timer(&self) {
        // todo: 60Hz
        thread::spawn(move || {
            unsafe {
                loop {
                    if SOUND_TIMER > 0 {
                        SOUND_TIMER -= 1;
                    }
                }
            }
        });
    }
}