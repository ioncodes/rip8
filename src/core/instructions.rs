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
            _ => Instruction::Unknown
        }
    }
}