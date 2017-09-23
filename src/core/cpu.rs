use std::io::{self, Read, Write, BufRead};
use std::process;

use super::ram::Ram;
use super::rom::Rom;
use super::keyboard::Keyboard;
use super::registers::Registers;
use super::instruction::Instruction;
use super::instructions::Instructions;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80, //F
];

pub struct Cpu {
    ram: Ram,
    rom: Rom,
    keyboard: Keyboard,
    registers: Registers,
    instructions: Instructions,
    debug: bool,
    interactive: bool,
    test: bool,
    test_pc: u16
}

impl Cpu {
    pub fn new(rom: String, debug: bool, interactive: bool, test: bool, test_pc: u16) -> Cpu {
        Cpu {
            ram: Ram::new(),
            rom: Rom::new(rom),
            keyboard: Keyboard::new(),
            registers: Registers::new(),
            instructions: Instructions::new(),
            debug,
            interactive,
            test,
            test_pc
        }
    }

    pub fn load_font(&mut self) {
        for i in 0..FONT_SET.len() {
            self.ram.write(i, FONT_SET[i]);
        }
    }

    pub fn load_rom(&mut self) {
        let pc = self.registers.pc as usize;
        for i in 0..self.rom.rom.len() {
            self.ram.write(pc + i, self.rom.rom[i])
        }
    }

    pub fn tick(&mut self) {
        self.process_debugger();
        let instr = self.ram.read(self.registers.pc as usize);
        let mut opcode = instr & 0xF000;
        if opcode == 0xF000 {
            opcode = instr & 0xF0FF; // CHIP8 has a series of opcodes which start with F, hence preserving the last byte make them identifiable.
        }
        let instruction = self.instructions.parse(opcode);
        match instruction {
            Instruction::JP => {
                let addr = self.instructions.parse_address(instr);
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, addr, 0, 0);
                    println!("{}", debug_info);
                }

                self.registers.jump(addr as u16);
            },
            Instruction::LdI => {
                let addr = self.instructions.parse_address(instr);
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, addr, 0, 0);
                    println!("{}", debug_info);
                }

                self.registers.i = addr;
                self.registers.step();
            },
            Instruction::LdV => {
                let x = self.instructions.parse_nibble(1, instr) as usize;
                let value = self.instructions.parse_last(instr);
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, x as u16, value as u16, 0);
                    println!("{}", debug_info);
                }

                self.registers.v[x] = value;
                self.registers.step();
            },
            Instruction::DRW => {
                let x = self.instructions.parse_nibble(1, instr) as usize;
                let y = self.instructions.parse_nibble(2, instr) as usize;
                let n = self.instructions.parse_nibble(3, instr) as usize;
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, x as u16, y as u16, n as u16);
                    println!("{}", debug_info);
                }
                // todo: implement drawing and storing
                self.registers.step();
            },
            Instruction::AddI => {
                let x = self.instructions.parse_nibble(1, instr) as u16;
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, x, 0, 0);
                    println!("{}", debug_info);
                }

                self.registers.i += x;
                self.registers.step();
            },
            Instruction::AddX => {
                let x = self.instructions.parse_nibble(1, instr);
                let byte = self.instructions.parse_last(instr);
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, x as u16, byte as u16, 0);
                    println!("{}", debug_info);
                }

                self.registers.v[x as usize] += byte;
                self.registers.step();
            },
            Instruction::SeX => {
                let x = self.instructions.parse_nibble(1, instr);
                let byte = self.instructions.parse_last(instr);
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, x as u16, byte as u16, 0);
                    println!("{}", debug_info);
                }

                let vx = self.registers.v[x as usize];

                if vx == byte {
                    self.registers.step();
                }
                self.registers.step();
            },
            Instruction::SeXY => {
                let x = self.instructions.parse_nibble(1, instr);
                let y = self.instructions.parse_nibble(2, instr);
                if self.debug {
                    let debug_info = self.instructions.get_debug_info(instruction, self.registers.pc, x as u16, y as u16, 0);
                    println!("{}", debug_info);
                }

                let vx = self.registers.v[x as usize];
                let vy = self.registers.v[y as usize];

                if vx == vy {
                    self.registers.step();
                }
                self.registers.step();
            },
            _ => panic!("Unknown instruction: 0x{:X}", instr)
        }
    }

    fn process_debugger(&self) {
        if self.interactive {
            io::stdout().write("$ ".as_bytes());
            io::stdout().flush();
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.lock().read_line(&mut buffer).expect("Could not read line.");
            buffer = buffer.trim_right_matches("\r\n").to_string();
            if buffer == "dump" {
                println!("{:#?}", self.registers);
                return;
            } else if buffer == "help" {
                println!("{}", "dump: dump registers");
                println!("{}", "help: this message");
                println!("{}", "anything else: step into");
            }
        } else if self.test {
            if self.registers.pc == self.test_pc {
                println!("{:#?}", self.registers);
                process::exit(1337);
            }
        }
    }
}