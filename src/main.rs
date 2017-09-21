mod core;

use core::cpu::Cpu;
use core::ram::Ram;
use core::keyboard::Keyboard;
use core::registers::Registers;
use core::instructions::Instructions;
use core::rom::Rom;

fn main() {
    let rom = Rom::new("roms/BLITZ".to_owned());
    let cpu = Cpu::new();
    let ram = Ram::new();

    loop {
        cpu.tick();
    }
}