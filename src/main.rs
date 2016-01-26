mod cartridge;
mod mem;
mod cpu;
mod opcode;

use cartridge::Cartridge;
use mem::MemMap;
use cpu::Cpu;

use std::fs::File;

fn main() {
    let mut file = File::open("rom/nestest.nes").unwrap();
    let cartridge = Cartridge::new(&mut file).unwrap();

    let mem = MemMap::new(cartridge.as_ref());
    let mut cpu = Cpu::new(mem);
    cpu.interactive();
}
