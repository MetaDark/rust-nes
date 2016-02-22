use std::io::{self, Read};
use mem::Mem;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    BadFileFormat,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

pub struct Cartridge {
    mapper: u8,
    prg_rom: Box<[u8]>,
    chr_rom: Box<[u8]>,
    // save_ram: Box<[u8]>,
    // prg_ram: Box<[u8]>,
}

impl Cartridge {
    pub fn new(stream: &mut Read) -> Result<Cartridge, Error> {
        let mut header = [0u8; 16];
        try!(stream.read(&mut header));

        let magic = &header[0..4];
        if magic != b"NES\x1a" {
            return Err(Error::BadFileFormat)
        }

        // let trainer = (header[6] & 0x04) != 0;
        // if trainer {
        //     let mut trainer = [0u8; 512];
        //     try!(stream.read(&mut trainer));
        // }

        let prg_rom_size = (header[4] as usize) << 14;
        let mut prg_rom = vec![0u8; prg_rom_size];
        try!(stream.read(&mut prg_rom));

        let chr_rom_size = (header[5] as usize) << 13;
        let mut chr_rom = vec![0u8; chr_rom_size];
        try!(stream.read(&mut chr_rom));

        let mapper = (header[7] & 0xf0) | (header[6] >> 4);

        Ok(Cartridge {
            mapper: mapper,
            prg_rom: prg_rom.into_boxed_slice(),
            chr_rom: chr_rom.into_boxed_slice(),
        })
    }
}

impl Mem for Cartridge {
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
