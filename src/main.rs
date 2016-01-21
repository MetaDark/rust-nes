#![allow(dead_code)]

mod cartridge;
mod mem;
mod cpu;

use std::fs::File;
use cartridge::Cartridge;
use mem::MemMap;
use cpu::Cpu;

fn main() {
    let mut file = File::open("rom/nestest.nes").unwrap();
    let cartridge = Cartridge::new(&mut file).unwrap();

    let mem = MemMap::new(cartridge);
    let mut cpu = Cpu::new(mem);

    cpu.step();
}