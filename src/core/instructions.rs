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
            // 0x0 => Instruction::CLS,
            _ => Instruction::Unknown
        }
    }

    pub fn parse_address(&self, opcode: u16) -> u16 {
        opcode & 0x0FFF
    }
}