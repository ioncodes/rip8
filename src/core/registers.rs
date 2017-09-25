extern crate time;

use std::thread;
use std::sync::Mutex;

const START_ADDRESS: u16 = 0x200; // todo: might also be 0x600
const REFRESH_RATE: i32 = 60; // DT & ST have a 60Hz refresh rate
lazy_static! {
    pub static ref DELAY_TIMER: Mutex<u8> = Mutex::new(0);
    pub static ref SOUND_TIMER: Mutex<u8> = Mutex::new(0);
}

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
        thread::spawn(move || {
            let mut task_time = 0;
            let sleep_time: i32 = 1000/REFRESH_RATE;
            unsafe {
                loop {
                    let mut dt = DELAY_TIMER.lock().unwrap();
                    if *dt > 0 {
                        task_time = time::now().tm_sec * 1000;
                        *dt -= 1;
                        drop(dt); // As long as the Mutex is locked, other code accessing it gets blocked. So let's just release it manually before the sleep.
                        task_time = (time::now().tm_sec * 1000) - task_time;
                        if sleep_time - task_time > 0 {
                            thread::sleep_ms((sleep_time - task_time) as u32);
                        }
                    }
                }
            }
        });
    }

    pub fn start_sound_timer(&self) {
        thread::spawn(move || {
            let mut task_time = 0;
            let sleep_time: i32 = 1000/REFRESH_RATE;
            unsafe {
                loop {
                    let mut st = SOUND_TIMER.lock().unwrap();
                    if *st > 0 {
                        task_time = time::now().tm_sec * 1000;
                        *st -= 1;
                        drop(st); // As long as the Mutex is locked, other code accessing it gets blocked. So let's just release it manually before the sleep.
                        task_time = (time::now().tm_sec * 1000) - task_time;
                        if sleep_time - task_time > 0 {
                            thread::sleep_ms((sleep_time - task_time) as u32);
                        }
                    }
                }
            }
        });
    }
}