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
            0xD000 => Instruction::DRW,
            0xF01E => Instruction::AddI,
            0x7000 => Instruction::AddX,
            0x3000 => Instruction::SeX,
            0x5000 => Instruction::SeXY,
            0xF00A => Instruction::LdXK,
            0xE0 => Instruction::CLS,
            0xEE => Instruction::RET,
            0x2000 => Instruction::CALL,
            0x8000 => Instruction::LdXY,
            0x8006 => Instruction::SHR,
            _ => Instruction::Unknown
        }
    }

    pub fn parse_address(&self, opcode: u16) -> u16 {
        opcode & 0x0FFF
    }

    pub fn parse_last(&self, opcode: u16) -> u8 {
        (opcode & 0x00FF) as u8
    }

    pub fn parse_nibble(&self, nibble: u8, opcode: u16) -> u8 {
        if nibble == 0 {
            (opcode & 0xF000) as u8
        } else if nibble == 1 {
            ((opcode & 0x0F00) >> 8) as u8
        } else if nibble == 2 {
            ((opcode & 0x00F0) >> 4) as u8
        } else if nibble == 3{
            (opcode & 0x000F) as u8
        } else {
            panic!("Nibble out of range.");
        }
    }

    // Get instruction details
    pub fn get_debug_info(&self, instruction: Instruction, pc: u16, v1: u16, v2: u16, v3: u16) -> String {
        match instruction {
            Instruction::JP  => format!("0x{:x}: jp #{:x}", pc, v1),
            Instruction::LdI => format!("0x{:x}: ld I, #{:x}", pc, v1),
            Instruction::LdV => format!("0x{:x}: ld V{:x}, #{:x}", pc, v1, v2),
            Instruction::DRW => format!("0x{:x}: drw V{:x}, V{:x}, #{:x}", pc, v1, v2, v3),
            Instruction::AddI => format!("0x{:x}: add I, V{:x}", pc, v1),
            Instruction::AddX => format!("0x{:x}: add V{:x}, #{:x}", pc, v1, v2),
            Instruction::SeX => format!("0x{:x}: se V{:x}, #{:x}", pc, v1, v2),
            Instruction::SeXY => format!("0x{:x}: se V{:x}, V{:x}", pc, v1, v2),
            Instruction::LdXK => format!("0x{:x}: ld V{:x}, K", pc, v1),
            Instruction::CLS => format!("0x{:x}: cls", pc),
            Instruction::RET => format!("0x{:x}: ret", pc),
            Instruction::CALL => format!("0x{:x}: call #{:x}", pc, v1),
            Instruction::LdXY => format!("0x{:x}: ld V{:x}, V{:x}", pc, v1, v2),
            Instruction::SHR => format!("0x{:x}: shr V{:x}", pc, v1),
            _ => format!("0x{:x}: Unknown", pc)
        }
    }
}