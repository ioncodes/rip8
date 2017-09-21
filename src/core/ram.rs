const MEMORY_SIZE: usize = 4096;

pub struct Ram {
    pub ram: [u16; MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            ram: [0; MEMORY_SIZE],
        }
    }
}
