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
    instructions: Instructions
}

impl Cpu {
    pub fn new(rom: String) -> Cpu {
        Cpu {
            ram: Ram::new(),
            rom: Rom::new(rom),
            keyboard: Keyboard::new(),
            registers: Registers::new(),
            instructions: Instructions::new()
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
        let instr = self.ram.read(self.registers.pc as usize);
        let opcode = instr & 0xF000;
        let instruction = self.instructions.parse(opcode);
        match instruction {
            Instruction::JP => {
                let addr = self.instructions.parse_address(instr);
                self.registers.jump(addr as u16);
            },
            Instruction::LdI => {
                let addr = self.instructions.parse_address(instr);
                self.registers.i = addr;
                self.registers.step();
            },
            Instruction::LdV => {
                let x = self.instructions.parse_nibble(1, instr) as usize;
                let value = self.instructions.parse_last(instr);
                self.registers.v[x] = value;
                self.registers.step();
            },
            _ => panic!("Unknown instruction: 0x{:X}", instr)
        }
    }
}