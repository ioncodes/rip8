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
    pub keyboard: Keyboard,
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
        if instr == 0xE0 || instr == 0xEE {
            opcode = instr; // CHIP8 has 2 instructions starting with 00 which does not get parsed, so let's check for them manually.
        } else if opcode == 0xF000 {
            opcode = instr & 0xF0FF; // CHIP8 has a series of opcodes which start with F, hence preserving the last byte make them identifiable.
        }
        let instruction = self.instructions.parse(opcode);
        match instruction {
            Instruction::JP => {
                // Jump to address
                let addr = self.instructions.parse_address(instr);
                self.print_debug_info(instruction, self.registers.pc, addr, 0, 0);

                self.registers.jump(addr as u16);
            },
            Instruction::LdI => {
                // set index register to address
                let addr = self.instructions.parse_address(instr);
                self.print_debug_info(instruction, self.registers.pc, addr, 0, 0);

                self.registers.i = addr;
                self.registers.step();
            },
            Instruction::LdV => {
                // set Vx to value
                let x = self.instructions.parse_nibble(1, instr) as usize;
                let value = self.instructions.parse_last(instr);
                self.print_debug_info(instruction, self.registers.pc, x as u16, value as u16, 0);

                self.registers.v[x] = value;
                self.registers.step();
            },
            Instruction::DRW => {
                // set pixels
                let x = self.instructions.parse_nibble(1, instr) as usize;
                let y = self.instructions.parse_nibble(2, instr) as usize;
                let n = self.instructions.parse_nibble(3, instr) as usize;
                self.print_debug_info(instruction, self.registers.pc, x as u16, y as u16, n as u16);
                // todo: implement drawing and storing

                self.registers.step();
            },
            Instruction::AddI => {
                // add x to I
                let x = self.instructions.parse_nibble(1, instr) as u16;
                self.print_debug_info(instruction, self.registers.pc, x, 0, 0);

                self.registers.i += x;
                self.registers.step();
            },
            Instruction::AddX => {
                // add byte to Vx
                let x = self.instructions.parse_nibble(1, instr);
                let byte = self.instructions.parse_last(instr);
                self.print_debug_info(instruction, self.registers.pc, x as u16, byte as u16, 0);

                self.registers.v[x as usize] += byte;
                self.registers.step();
            },
            Instruction::SeX => {
                // skip if Vx equals byte
                let x = self.instructions.parse_nibble(1, instr);
                let byte = self.instructions.parse_last(instr);
                self.print_debug_info(instruction, self.registers.pc, x as u16, byte as u16, 0);

                let vx = self.registers.v[x as usize];

                if vx == byte {
                    self.registers.step();
                }
                self.registers.step();
            },
            Instruction::SeXY => {
                // skip if Vx equals Vy
                let x = self.instructions.parse_nibble(1, instr);
                let y = self.instructions.parse_nibble(2, instr);
                self.print_debug_info(instruction, self.registers.pc, x as u16, y as u16, 0);

                let vx = self.registers.v[x as usize];
                let vy = self.registers.v[y as usize];

                if vx == vy {
                    self.registers.step();
                }
                self.registers.step();
            },
            Instruction::LdXK => {
                // wait for keypress, store in Vx
                for i in 0..self.keyboard.keyboard.len() {
                    if self.keyboard.pressed(i as u8) {
                        let x = self.instructions.parse_nibble(1, instr);
                        self.print_debug_info(instruction, self.registers.pc, x as u16, 0, 0); // todo: move this out of the if

                        self.registers.v[x as usize] = i as u8;
                        self.registers.step();
                    }
                }
            },
            Instruction::CLS => {
                // todo: clear the screen
                self.print_debug_info(instruction, self.registers.pc, 0, 0, 0);

                self.registers.step();
            },
            Instruction::RET => {
                let addr = *self.registers.stack.first().unwrap();
                self.print_debug_info(instruction, self.registers.pc, 0, 0, 0);

                self.registers.jump(addr);
                self.registers.sp -= 1;
            },
            _ => panic!("Unknown instruction: 0x{:X}", instr)
        }
    }

    fn print_debug_info(&self, instruction: Instruction, pc: u16, v1: u16, v2: u16, v3: u16) {
        if self.debug {
            let debug_info = self.instructions.get_debug_info(instruction, pc, v1, v2, v3);
            println!("{}", debug_info);
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