mod core;

use std::env;
use core::cpu::Cpu;

fn main() {
    let rom_path = env::args().nth(1).unwrap();
    let mut debug = false;
    let mut step = false;
    if let Some(_debug) = env::args().nth(2) {
        if _debug == "-d" {
            debug = true;
        }
    }
    if let Some(_step) = env::args().nth(3) {
        if _step == "-s" {
            step = true;
        }
    }
    let mut cpu = Cpu::new(rom_path, debug, step);
    cpu.load_font();
    cpu.load_rom();

    loop {
        cpu.tick();
    }
}