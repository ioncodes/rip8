mod core;

use core::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new("roms/BLITZ".to_owned());
    cpu.load_font();
    cpu.load_rom();

    loop {
        cpu.tick();
    }
}