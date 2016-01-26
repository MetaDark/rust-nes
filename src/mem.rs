use cartridge::Cartridge;

pub trait Mem {
    fn read8(&self, addr: u16) -> u8;
    fn write8(&mut self, addr: u16, val: u8);
}

pub struct MemMap<'a> {
    ram: [u8; 0x0800],
    cartridge: &'a Cartridge,
}

impl<'a> MemMap<'a> {
    pub fn new(cartridge: &'a Cartridge) -> MemMap<'a> {
        MemMap {
            ram: [0; 0x0800],
            cartridge: cartridge,
        }
    }
}

impl<'a> Mem for MemMap<'a> {
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
