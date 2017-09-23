mod core;

use std::env;
use core::cpu::Cpu;

fn main() {
    let rom_path = env::args().nth(1).unwrap();
    let mut debug = false;
    let mut interactive = false;
    let mut test = false;
    let mut test_pc: u16 = 0;
    if let Some(_debug) = env::args().nth(2) {
        if _debug == "-d" {
            debug = true;
        }
    }
    if let Some(_interactive) = env::args().nth(3) {
        if _interactive == "-i" {
            interactive = true;
        }
    }
    if let Some(_test) = env::args().nth(2) {
        if _test == "-t" {
            if let Some(_test_pc) = env::args().nth(3) {
                test = true;
                test_pc = u16::from_str_radix(&_test_pc, 16).unwrap();
            }
        }
    }
    let mut cpu = Cpu::new(rom_path, debug, interactive, test, test_pc);
    cpu.load_font();
    cpu.load_rom();

    loop {
        cpu.tick();
    }
}