use cartridge::Cartridge;

pub trait Mem {
    fn read8(&self, addr: u16) -> u8;
    fn write8(&mut self, addr: u16, val: u8);

    fn read16(&self, addr: u16) -> u16 {
        (self.read8(addr) as u16) << 8 | (self.read8(addr + 1) as u16)
    }

    fn write16(&mut self, addr: u16, val: u16) {
        self.write8(addr, (val >> 8) as u8);
        self.write8(addr + 1, (val & 0x0f) as u8)
    }
}

pub struct MemMap {
    ram: [u8; 0x0800],
    cartridge: Cartridge,
}

impl MemMap {
    pub fn new(cartridge: Cartridge) -> MemMap {
        MemMap {
            ram: [0u8; 0x0800],
            cartridge: cartridge,
        }
    }
}

impl Mem for MemMap {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x1fff => self.ram[(addr % 0x0800) as usize],
            0x4020 ... 0xffff => self.cartridge.read8(addr),
            _ => panic!("not yet implemented MemMap::read8(0x{:04X})", addr),
        }
    }

    fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000 ... 0x1fff => self.ram[(addr % 0x0800) as usize] = val,
            0x4020 ... 0xffff => self.write8(addr, val),
            _ => panic!("not yet implemented MemMap::write8(0x{:04X},0x{:02X})", addr, val),
        }
    }
}