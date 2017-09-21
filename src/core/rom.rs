extern crate byteorder;

use self::byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::fs::File;
use std::io::Read;

pub struct Rom {
    pub rom_path: String,
    pub rom: Vec<u8>
}

impl Rom {
    pub fn new(rom_path: String) -> Rom {
        let mut rom = Vec::<u8>::new();
        let mut f = File::open(&rom_path).unwrap();
        let _ = f.read_to_end(&mut rom);
        Rom {
            rom_path,
            rom
        }
    }
}