#![allow(dead_code)]

use mem::Mem;
use std::io::{self, Read};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    BadFileFormat,
    UnsupportedMapper(u8),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

pub type Cartridge = Mem;
impl Cartridge {
    pub fn new(stream: &mut Read) -> Result<Box<Cartridge>, Error> {
        let mut header = [0; 16];
        try!(stream.read(&mut header));

        let magic = &header[0..4];
        if magic != b"NES\x1a" {
            return Err(Error::BadFileFormat);
        }

        let trainer = (header[6] & 0x04) != 0;
        if trainer {
            try!(stream.read(&mut [0; 512]));
        }

        let prg_rom_size = (header[4] as usize) << 14;
        let mut prg_rom = vec![0; prg_rom_size];
        try!(stream.read(&mut prg_rom));
        let prg_rom = prg_rom.into_boxed_slice();

        let chr_rom_size = (header[5] as usize) << 13;
        let mut chr_rom = vec![0; chr_rom_size];
        try!(stream.read(&mut chr_rom));
        let chr_rom = chr_rom.into_boxed_slice();

        let mapper = (header[7] & 0xf0) | (header[6] >> 4);
        match mapper {
            0 => Ok(Box::new(Mapper0 {
                prg_rom: prg_rom,
                chr_rom: chr_rom,
            })),
            _ => Err(Error::UnsupportedMapper(mapper)),
        }
    }
}

pub struct Mapper0 {
    prg_rom: Box<[u8]>,
    chr_rom: Box<[u8]>,
}

impl Mem for Mapper0 {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x8000 ... 0xFFFF =>
                self.prg_rom[(addr - 0x8000) as usize % self.prg_rom.len()],
            _ => panic!("not yet implemented Cartridge::read8(0x{:04X})", addr),
        }
    }

    fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            _ => panic!("not yet implemented Cartridge::write8(0x{:04X},0x{:02X})", addr, val),
        }
    }
}
