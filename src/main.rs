extern crate minifb;

mod core;

use std::env;
use core::cpu::Cpu;
use minifb::{Key, WindowOptions, Window, Scale};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

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

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new("rip8", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: true,
                                           scale: Scale::X2,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        {
            // todo: redo this.
            if window.is_key_down(Key::Key1) {
                cpu.keyboard.set(0);
            } else {
                cpu.keyboard.unset(0);
            }
            if window.is_key_down(Key::Key2) {
                cpu.keyboard.set(1);
            } else {
                cpu.keyboard.unset(1);
            }
            if window.is_key_down(Key::Key3) {
                cpu.keyboard.set(2);
            } else {
                cpu.keyboard.unset(2);
            }
            if window.is_key_down(Key::C) {
                cpu.keyboard.set(3);
            } else {
                cpu.keyboard.unset(3);
            }
            if window.is_key_down(Key::Key4) {
                cpu.keyboard.set(4);
            } else {
                cpu.keyboard.unset(4);
            }
            if window.is_key_down(Key::Key5) {
                cpu.keyboard.set(5);
            } else {
                cpu.keyboard.unset(5);
            }
            if window.is_key_down(Key::Key6) {
                cpu.keyboard.set(6);
            } else {
                cpu.keyboard.unset(6);
            }
            if window.is_key_down(Key::D) {
                cpu.keyboard.set(7);
            } else {
                cpu.keyboard.unset(7);
            }
            if window.is_key_down(Key::Key7) {
                cpu.keyboard.set(8);
            } else {
                cpu.keyboard.unset(8);
            }
            if window.is_key_down(Key::Key8) {
                cpu.keyboard.set(9);
            } else {
                cpu.keyboard.unset(9);
            }
            if window.is_key_down(Key::Key9) {
                cpu.keyboard.set(10);
            } else {
                cpu.keyboard.unset(10);
            }
            if window.is_key_down(Key::E) {
                cpu.keyboard.set(11);
            } else {
                cpu.keyboard.unset(11);
            }
            if window.is_key_down(Key::A) {
                cpu.keyboard.set(12);
            } else {
                cpu.keyboard.unset(12);
            }
            if window.is_key_down(Key::Key0) {
                cpu.keyboard.set(13);
            } else {
                cpu.keyboard.unset(13);
            }
            if window.is_key_down(Key::B) {
                cpu.keyboard.set(14);
            } else {
                cpu.keyboard.unset(14);
            }
            if window.is_key_down(Key::F) {
                cpu.keyboard.set(15);
            } else {
                cpu.keyboard.unset(15);
            }
        }

        cpu.tick();

        for i in buffer.iter_mut() {
            *i = 0;
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}