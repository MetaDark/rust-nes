use cpu;
use cartridge::Cartridge;

pub struct Mem<'a> {
    ram: [u8; 0x0800],
    cartridge: &'a Cartridge,
}

impl<'a> Mem<'a> {
    pub fn new(cartridge: &'a Cartridge) -> Mem<'a> {
        Mem {
            ram: [0; 0x0800],
            cartridge: cartridge,
        }
    }
}

impl<'a> cpu::Mem for Mem<'a> {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000 ... 0x1fff => self.ram[(addr % 0x0800) as usize],
            0x4020 ... 0xffff => self.cartridge.read8(addr),
            _ => panic!("not yet implemented Mem::read8(0x{:04X})", addr),
        }
    }

    fn write8(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000 ... 0x1fff => self.ram[(addr % 0x0800) as usize] = val,
            0x4020 ... 0xffff => self.write8(addr, val),
            _ => panic!("not yet implemented Mem::write8(0x{:04X},0x{:02X})", addr, val),
        }
    }
}
