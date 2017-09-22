use super::instruction::Instruction;

pub struct Instructions {

}

impl Instructions {
    pub fn new() -> Instructions {
        Instructions {

        }
    }

    pub fn parse(&self, opcode: u16) -> Instruction {
        match opcode {
            0x1000 => Instruction::JP,
            0xA000 => Instruction::LdI,
            0x6000 => Instruction::LdV,
            _ => Instruction::Unknown
        }
    }

    pub fn parse_address(&self, opcode: u16) -> u16 {
        opcode & 0x0FFF
    }

    pub fn parse_last(&self, opcode: u16) -> u8 {
        opcode & 0x00FF
    }

    pub fn parse_nibble(&self, nibble: u8, opcode: u16) -> u8 {
        let mut mult = 0x0000; // todo: rename this
        if nibble == 0 {
            mult = 0xF000;
        } else if nibble == 1 {
            mult = 0x0F00;
        } else if nibble == 2 {
            mult = 0x00F0;
        } else if nibble == 3{
            mult = 0x000F;
        } else {
            panic!("Nibble out of range.");
        }

        opcode & mult
    }
}