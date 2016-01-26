mod cartridge;
mod mem;
mod cpu;
mod opcode;

use cartridge::Cartridge;
use mem::Mem;
use cpu::Cpu;

use std::fs::File;

fn main() {
    let mut file = File::open("rom/nestest.nes").unwrap();
    let cartridge = Cartridge::new(&mut file).unwrap();

    let mem = Mem::new(cartridge.as_ref());
    let mut cpu = Cpu::new(mem);
    cpu.interactive();
}
