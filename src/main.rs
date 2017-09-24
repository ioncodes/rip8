extern crate minifb;

mod core;

use std::env;
use core::cpu::Cpu;
use minifb::{Key, WindowOptions, Window, Scale};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {
    let rom_path = env::args().nth(1).unwrap();
    let mut debug = false;
    let mut interactive = false;
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

    let mut cpu = Cpu::new(rom_path, debug, interactive);
    cpu.load_font();
    cpu.load_rom();
    cpu.init();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = match Window::new("rip8", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: true,
                                           scale: Scale::X8,
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
                cpu.keyboard.set(0x01);
            } else {
                cpu.keyboard.unset(0x01);
            }
            if window.is_key_down(Key::Key2) {
                cpu.keyboard.set(0x02);
            } else {
                cpu.keyboard.unset(0x02);
            }
            if window.is_key_down(Key::Key3) {
                cpu.keyboard.set(0x03);
            } else {
                cpu.keyboard.unset(0x03);
            }
            if window.is_key_down(Key::Key4) {
                cpu.keyboard.set(0x0C);
            } else {
                cpu.keyboard.unset(0x0C);
            }
            if window.is_key_down(Key::Q) {
                cpu.keyboard.set(0x04);
            } else {
                cpu.keyboard.unset(0x04);
            }
            if window.is_key_down(Key::W) {
                cpu.keyboard.set(0x05);
            } else {
                cpu.keyboard.unset(0x05);
            }
            if window.is_key_down(Key::E) {
                cpu.keyboard.set(0x06);
            } else {
                cpu.keyboard.unset(0x06);
            }
            if window.is_key_down(Key::R) {
                cpu.keyboard.set(0x0D);
            } else {
                cpu.keyboard.unset(0x0D);
            }
            if window.is_key_down(Key::A) {
                cpu.keyboard.set(0x07);
            } else {
                cpu.keyboard.unset(0x07);
            }
            if window.is_key_down(Key::S) {
                cpu.keyboard.set(0x08);
            } else {
                cpu.keyboard.unset(0x08);
            }
            if window.is_key_down(Key::D) {
                cpu.keyboard.set(0x09);
            } else {
                cpu.keyboard.unset(0x09);
            }
            if window.is_key_down(Key::F) {
                cpu.keyboard.set(0x0E);
            } else {
                cpu.keyboard.unset(0x0E);
            }
            if window.is_key_down(Key::Y) || window.is_key_down(Key::Z) {
                cpu.keyboard.set(0x0A);
            } else {
                cpu.keyboard.unset(0x0A);
            }
            if window.is_key_down(Key::X) {
                cpu.keyboard.set(0x00);
            } else {
                cpu.keyboard.unset(0x00);
            }
            if window.is_key_down(Key::C) {
                cpu.keyboard.set(0x0B);
            } else {
                cpu.keyboard.unset(0x0B);
            }
            if window.is_key_down(Key::V) {
                cpu.keyboard.set(0x0F);
            } else {
                cpu.keyboard.unset(0x0F);
            }
        }

        cpu.tick();
        let screen = cpu.screen.screen;

        for y in 0..32 {
            for x in 0..64 {
                // println!("{}: {},", y, screen[x][y]);
                buffer[y * 64 + x] = (screen[x][y] as u32) * 255;
            }
        }

        window.update_with_buffer(&buffer).unwrap();

        std::thread::sleep(std::time::Duration::new(0,1000));
    }
}