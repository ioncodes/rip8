const MEMORY_SIZE: usize = 4096;

pub struct Ram {
    pub ram: [u8; MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            ram: [0; MEMORY_SIZE],
        }
    }

    pub fn write(&mut self, position: usize, byte: u8) {
        self.ram[position] = byte;
    }

    // Returns the next instruction which is 2 bytes long
    pub fn read(&mut self, position: usize) -> u16 {
        let instruction: [u16; 2] = [self.ram[position] as u16, self.ram[position + 1] as u16];
        instruction[0] << 8 | instruction[1]
    }

    pub fn read_byte(&mut self, position: usize) -> u8 {
        self.ram[position]
    }
}
