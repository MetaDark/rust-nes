use mem::Mem;
use opcode::{AddressingMode, Instruction};

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

/*
 * References:
 *  - http://obelisk.me.uk/6502/instructions.html
 *  - http://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
 *
 * TODO:
 *  - Use the bitfield crate for flags
 */

const CARRY_FLAG:    u8 = 1 << 0;
const ZERO_FLAG:     u8 = 1 << 1;
const IRQ_FLAG:      u8 = 1 << 2;
const DECIMAL_FLAG:  u8 = 1 << 3;
const OVERFLOW_FLAG: u8 = 1 << 6;
const NEGATIVE_FLAG: u8 = 1 << 7;

#[derive(Debug)]
pub struct Cpu<M: Mem> {
    clock: usize,
    mem: M,
    pc: u16,
    sp: u8,
    a: u8, x: u8, y: u8,
    status: u8,
}

impl<M: Mem> Cpu<M> {
    pub fn new(mem: M) -> Cpu<M> {
        Cpu {
            mem: mem,
            clock: 0,
            pc: 0xc000,
            sp: 0xfd,
            a: 0, x: 0, y: 0,
            status: 0x34,
        }
    }

    pub fn reset(&mut self) {
        self.clock = 0;
        self.pc = 0xc000;
        self.sp = 0xfd;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = 0x34;
    }

    pub fn step(&mut self) {
        let opcode = self.next8();

        let addr = match AddressingMode::from(opcode) {
            AddressingMode::Implied => None,
            AddressingMode::Accumulator => None,
            AddressingMode::Immediate => Some(self.immediate()),
            AddressingMode::ZeroPage => Some(self.zero_page()),
            AddressingMode::ZeroPageX => Some(self.zero_page_x()),
            AddressingMode::ZeroPageY => Some(self.zero_page_y()),
            AddressingMode::Relative => Some(self.relative()),
            AddressingMode::Absolute => Some(self.absolute()),
            AddressingMode::AbsoluteX => Some(self.absolute_x()),
            AddressingMode::AbsoluteY => Some(self.absolute_y()),
            AddressingMode::Indirect => Some(self.indirect()),
            AddressingMode::IndexedIndirect => Some(self.indexed_indirect()),
            AddressingMode::IndirectIndexed => Some(self.indirect_indexed()),
        };

        match Instruction::from(opcode) {
            Instruction::LDA => self.lda(addr.unwrap()),
            Instruction::LDX => self.ldx(addr.unwrap()),
            Instruction::LDY => self.ldy(addr.unwrap()),
            Instruction::STA => self.sta(addr.unwrap()),
            Instruction::STX => self.stx(addr.unwrap()),
            Instruction::STY => self.sty(addr.unwrap()),
            Instruction::TAX => self.tax(),
            Instruction::TAY => self.tay(),
            Instruction::TXA => self.txa(),
            Instruction::TYA => self.tya(),
            Instruction::TSX => self.tsx(),
            Instruction::TXS => self.txs(),
            Instruction::PHA => self.pha(),
            Instruction::PHP => self.php(),
            Instruction::PLA => self.pla(),
            Instruction::PLP => self.plp(),
            Instruction::AND => self.and(addr.unwrap()),
            Instruction::EOR => self.eor(addr.unwrap()),
            Instruction::ORA => self.ora(addr.unwrap()),
            Instruction::BIT => self.bit(addr.unwrap()),
            Instruction::ADC => self.adc(addr.unwrap()),
            Instruction::SBC => self.sbc(addr.unwrap()),
            Instruction::CMP => self.cmp(addr.unwrap()),
            Instruction::CPX => self.cpx(addr.unwrap()),
            Instruction::CPY => self.cpy(addr.unwrap()),
            Instruction::INC => self.inc(addr.unwrap()),
            Instruction::INX => self.inx(),
            Instruction::INY => self.iny(),
            Instruction::DEC => self.dec(addr.unwrap()),
            Instruction::DEX => self.dex(),
            Instruction::DEY => self.dey(),
            Instruction::ASL => self.asl(addr),
            Instruction::LSR => self.lsr(addr),
            Instruction::ROL => self.rol(addr),
            Instruction::ROR => self.ror(addr),
            Instruction::JMP => self.jmp(addr.unwrap()),
            Instruction::JSR => self.jsr(addr.unwrap()),
            Instruction::RTS => self.rts(),
            Instruction::BCC => self.bcc(addr.unwrap()),
            Instruction::BCS => self.bcs(addr.unwrap()),
            Instruction::BEQ => self.beq(addr.unwrap()),
            Instruction::BMI => self.bmi(addr.unwrap()),
            Instruction::BNE => self.bne(addr.unwrap()),
            Instruction::BPL => self.bpl(addr.unwrap()),
            Instruction::BVC => self.bvc(addr.unwrap()),
            Instruction::BVS => self.bvs(addr.unwrap()),
            Instruction::CLC => self.clc(),
            Instruction::CLD => self.cld(),
            Instruction::CLI => self.cli(),
            Instruction::CLV => self.clv(),
            Instruction::SEC => self.sec(),
            Instruction::SED => self.sed(),
            Instruction::SEI => self.sei(),
            Instruction::BRK => self.brk(),
            Instruction::NOP => self.nop(),
            Instruction::RTI => self.rti(),

            /* Treat unofficial instructions as NOP */
            _ => self.nop(),
        };
    }

    /* Addressing Modes */
    fn immediate(&mut self) -> u16 {
        let val = self.pc;
        self.next8();
        val
    }

    fn zero_page(&mut self) -> u16 {
        self.next8() as u16
    }

    fn zero_page_x(&mut self) -> u16 {
        (self.next8().wrapping_add(self.x)) as u16
    }

    fn zero_page_y(&mut self) -> u16 {
        (self.next8().wrapping_add(self.y)) as u16
    }

    fn relative(&mut self) -> u16 {
        (self.next8() as u16).wrapping_add(self.pc)
    }

    fn absolute(&mut self) -> u16 {
        self.next16()
    }

    fn absolute_x(&mut self) -> u16 {
        self.next16().wrapping_add(self.x as u16)
    }

    fn absolute_y(&mut self) -> u16 {
        self.next16().wrapping_add(self.y as u16)
    }

    fn indirect(&mut self) -> u16 {
        let addr = self.next16();
        self.read16_zero_page(addr)
    }

    fn indexed_indirect(&mut self) -> u16 {
        let addr = self.next8();
        self.read16_zero_page(addr.wrapping_add(self.x) as u16)
    }

    fn indirect_indexed(&mut self) -> u16 {
        let addr = self.next8();
        self.read16_zero_page(addr as u16).wrapping_add(self.y as u16)
    }

    /* Load / Store */
    fn lda(&mut self, addr: u16) {
        let val = self.read8(addr);
        self.a = val;
        self.set_zn(val);
    }

    fn ldx(&mut self, addr: u16) {
        let val = self.read8(addr);
        self.x = val;
        self.set_zn(val);
    }

    fn ldy(&mut self, addr: u16) {
        let val = self.read8(addr);
        self.y = val;
        self.set_zn(val);
    }

    fn sta(&mut self, addr: u16) {
        let val = self.a;
        self.write8(addr, val);
    }

    fn stx(&mut self, addr: u16) {
        let val = self.x;
        self.write8(addr, val);
    }

    fn sty(&mut self, addr: u16) {
        let val = self.y;
        self.write8(addr, val);
    }


    /* Register Transfers */
    fn tax(&mut self) {
        let val = self.a;
        self.x = val;
        self.set_zn(val);
    }

    fn tay(&mut self) {
        let val = self.a;
        self.y = val;
        self.set_zn(val);
    }

    fn txa(&mut self) {
        let val = self.x;
        self.a = val;
        self.set_zn(val);
    }

    fn tya(&mut self) {
        let val = self.y;
        self.a = val;
        self.set_zn(val);
    }


    /* Stack Operations */
    fn tsx(&mut self) {
        let val = self.sp;
        self.x = val;
        self.set_zn(val);
    }

    fn txs(&mut self) {
        let val = self.x;
        self.sp = val;
    }

    fn pha(&mut self) {
        let val = self.a;
        self.push8(val);
    }

    fn php(&mut self) {
        let status = self.get_status();
        self.push8(status);
    }

    fn pla(&mut self) {
        let val = self.pull8();
        self.a = val;
        self.set_zn(val);
    }

    fn plp(&mut self) {
        let status = self.pull8();
        self.set_status(status);
    }


    /* Logical */
    fn and(&mut self, addr: u16) {
        let result = self.a & self.read8(addr);
        self.a = result;
        self.set_zn(result);
    }

    fn eor(&mut self, addr: u16) {
        let result = self.a ^ self.read8(addr);
        self.a = result;
        self.set_zn(result);
    }

    fn ora(&mut self, addr: u16) {
        let result = self.a | self.read8(addr);
        self.a = result;
        self.set_zn(result);
    }

    fn bit(&mut self, addr: u16) {
        let mask = self.a;
        let val = self.read8(addr);
        self.set_flag(ZERO_FLAG, (val & mask) == 0);
        self.set_flag(OVERFLOW_FLAG, (val & 0x40) != 0);
        self.set_flag(NEGATIVE_FLAG, (val & 0x80) != 0);
    }


    /* Arithmetic */
    fn adc(&mut self, addr: u16) {
        let a = self.a as u16;
        let b = self.read8(addr) as u16;
        let c = self.get_flag(CARRY_FLAG) as u16;
        let result = a.wrapping_add(b).wrapping_add(c);

        self.a = result as u8;
        self.set_flag(CARRY_FLAG, (result & 0x0100) != 0);
        self.set_flag(OVERFLOW_FLAG,
                        (a ^ b) & 0x80 == 0 &&
                        (a ^ result) & 0x80 != 0);

        self.set_zn(result as u8);
    }

    fn sbc(&mut self, addr: u16) {
        let a = self.a as u16;
        let b = self.read8(addr) as u16;
        let c = !self.get_flag(CARRY_FLAG) as u16;
        let result = a.wrapping_sub(b).wrapping_sub(c);

        self.a = result as u8;
        self.set_flag(CARRY_FLAG, (result & 0x0100) == 0);
        self.set_flag(OVERFLOW_FLAG,
                        (a ^ b) & 0x80 != 0 &&
                        (a ^ result) & 0x80 != 0);

        self.set_zn(result as u8);
    }

    fn cmp_base(&mut self, a: u8, b: u8) {
        let result = a.wrapping_sub(b);
        self.set_flag(CARRY_FLAG, a >= b);
        self.set_zn(result);
    }

    fn cmp(&mut self, addr: u16) {
        let a = self.a;
        let b = self.read8(addr);
        self.cmp_base(a, b);
    }

    fn cpx(&mut self, addr: u16) {
        let a = self.x;
        let b = self.read8(addr);
        self.cmp_base(a, b);
    }

    fn cpy(&mut self, addr: u16) {
        let a = self.y;
        let b = self.read8(addr);
        self.cmp_base(a, b);
    }


    /* Increments & Decrements */
    fn inc(&mut self, addr: u16) {
        let result = self.read8(addr).wrapping_add(1);
        self.write8(addr, result);
        self.set_zn(result);
    }

    fn inx(&mut self) {
        let result = self.x.wrapping_add(1);
        self.x = result;
        self.set_zn(result);
    }

    fn iny(&mut self) {
        let result = self.y.wrapping_add(1);
        self.y = result;
        self.set_zn(result);
    }

    fn dec(&mut self, addr: u16) {
        let result = self.read8(addr).wrapping_sub(1);
        self.write8(addr, result);
        self.set_zn(result);
    }

    fn dex(&mut self) {
        let result = self.x.wrapping_sub(1);
        self.x = result;
        self.set_zn(result);
    }

    fn dey(&mut self) {
        let result = self.y.wrapping_sub(1);
        self.y = result;
        self.set_zn(result);
    }


    /* Shifts */
    fn shl_base(&mut self, addr: Option<u16>, c: bool) {
        let val = match addr {
            Some(addr) => self.read8(addr),
            None => self.a,
        };

        let result = val << 1 | c as u8;
        self.set_flag(CARRY_FLAG, val & 0x80 != 0);
        self.set_zn(result);

        match addr {
            Some(addr) => self.write8(addr, result),
            None => self.a = result,
        };
    }

    fn shr_base(&mut self, addr: Option<u16>, c: bool) {
        let val = match addr {
            Some(addr) => self.read8(addr),
            None => self.a,
        };

        let result = val >> 1 | (c as u8) << 7;
        self.set_flag(CARRY_FLAG, val & 0x01 != 0);
        self.set_zn(result);

        match addr {
            Some(addr) => self.write8(addr, result),
            None => self.a = result,
        };
    }

    fn asl(&mut self, addr: Option<u16>) {
        self.shl_base(addr, false);
    }

    fn lsr(&mut self, addr: Option<u16>) {
        self.shr_base(addr, false);
    }

    fn rol(&mut self, addr: Option<u16>) {
        let c = self.get_flag(CARRY_FLAG);
        self.shl_base(addr, c);
    }

    fn ror(&mut self, addr: Option<u16>) {
        let c = self.get_flag(CARRY_FLAG);
        self.shr_base(addr, c);
    }


    /* Jumps & Calls */
    fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn jsr(&mut self, addr: u16) {
        let pc = self.pc.wrapping_sub(1);
        self.push16(pc);
        self.pc = addr;
    }

    fn rts(&mut self) {
        let pc = self.pull16().wrapping_add(1);
        self.pc = pc;
    }


    /* Branches */
    fn bcc(&mut self, addr: u16) {
        if !self.get_flag(CARRY_FLAG) {
            self.pc = addr;
        }
    }

    fn bcs(&mut self, addr: u16) {
        if self.get_flag(CARRY_FLAG) {
            self.pc = addr;
        }
    }

    fn beq(&mut self, addr: u16) {
        if self.get_flag(ZERO_FLAG) {
            self.pc = addr;
        }
    }

    fn bmi(&mut self, addr: u16) {
        if self.get_flag(NEGATIVE_FLAG) {
            self.pc = addr;
        }
    }

    fn bne(&mut self, addr: u16) {
        if !self.get_flag(ZERO_FLAG) {
            self.pc = addr;
        }
    }

    fn bpl(&mut self, addr: u16) {
        if !self.get_flag(NEGATIVE_FLAG) {
            self.pc = addr;
        }
    }

    fn bvc(&mut self, addr: u16) {
        if !self.get_flag(OVERFLOW_FLAG) {
            self.pc = addr;
        }
    }

    fn bvs(&mut self, addr: u16) {
        if self.get_flag(OVERFLOW_FLAG) {
            self.pc = addr;
        }
    }


    /* Status Flag Changes */
    fn clc(&mut self) {
        self.set_flag(CARRY_FLAG, false);
    }

    fn cld(&mut self) {
        self.set_flag(DECIMAL_FLAG, false);
    }

    fn cli(&mut self) {
        self.set_flag(IRQ_FLAG, false);
    }

    fn clv(&mut self) {
        self.set_flag(OVERFLOW_FLAG, false);
    }

    fn sec(&mut self) {
        self.set_flag(CARRY_FLAG, true);
    }

    fn sed(&mut self) {
        self.set_flag(DECIMAL_FLAG, true);
    }

    fn sei(&mut self) {
        self.set_flag(IRQ_FLAG, true);
    }


    /* System Functions */
    fn brk(&mut self) {
        self.jsr(0xFFFE);
        self.php();
        self.sei();
    }

    fn nop(&self) {}

    fn rti(&mut self) {
        self.plp();
        self.pc = self.pull16();
    }

    /* Memory helpers */
    fn read16(&self, addr: u16) -> u16 {
        let low = self.read8(addr) as u16;
        let high = self.read8(addr.wrapping_add(1)) as u16;
        high << 8 | low
    }

    /*
     * Only the lowest nibble of addr is
     * incremented on a zero page read
     *
     * This is the cause of the indirect JMP bug on the 6502
     */
    fn read16_zero_page(&self, addr: u16) -> u16 {
        let low = self.read8(addr) as u16;
        let high = self.read8(addr & 0xFF00 | (addr as u8).wrapping_add(1) as u16) as u16;
        high << 8 | low
    }

    fn write16(&mut self, addr: u16, val: u16) {
        let low = val as u8;
        let high = (val >> 8) as u8;
        self.write8(addr, low);
        self.write8(addr.wrapping_add(1), high);
    }

    fn next8(&mut self) -> u8 {
        let val = self.read8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        val
    }

    fn next16(&mut self) -> u16 {
        let val = self.read16(self.pc);
        self.pc = self.pc.wrapping_add(2);
        val
    }

    /* Stack helpers */
    fn pull8(&mut self) -> u8 {
        let sp = self.sp.wrapping_add(1);
        let val = self.read8(0x0100 + sp as u16);
        self.sp = self.sp.wrapping_add(1);
        val
    }

    fn push8(&mut self, val: u8) {
        let sp = self.sp;
        self.write8(0x0100 + sp as u16, val);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull16(&mut self) -> u16 {
        let sp = self.sp.wrapping_add(1);
        let val = self.read16(0x0100 + sp as u16);
        self.sp = self.sp.wrapping_add(2);
        val
    }

    fn push16(&mut self, val: u16) {
        let sp = self.sp.wrapping_sub(1);
        self.write16(0x0100 + sp as u16, val);
        self.sp = self.sp.wrapping_sub(2);
    }

    /* Flag helpers */
    fn get_flag(&self, status: u8) -> bool {
        (self.status & status) != 0
    }

    fn set_flag(&mut self, status: u8, on: bool) {
        if on {
            self.status |= status;
        } else {
            self.status &= !status;
        }
    }

    fn get_status(&self) -> u8 {
        self.status | 0x30
    }

    fn set_status(&mut self, val: u8) {
        self.status = val & !0x30;
    }

    fn set_zn(&mut self, val: u8) {
        self.set_flag(ZERO_FLAG, val == 0);
        self.set_flag(NEGATIVE_FLAG, val & 0x80 != 0);
    }

    /* Debug helpers */
    pub fn interactive(&mut self) {
        fn prompt() {
            print!("> ");
            io::stdout().flush().unwrap();
        }

        prompt();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();

            let mut input = line.split(' ');
            let command = input.next();

            match command {
                Some("reset") | Some("r") => self.reset(),
                Some("step") | Some("") => {println!("{}", self.trace()); self.step();},
                Some("test") | Some("t") => self.nestest(),
                Some("trace") => println!("{}", self.trace()),
                Some("quit") | Some("q") => break,
                _ => println!("Invalid command"),
            }

            prompt();
        }
    }

    fn nestest(&mut self) {
        let file = File::open("test/nestest-mod.log").unwrap();
        let reader = BufReader::new(file);

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            let trace = self.trace();
            let expected = line.trim_right();

            if trace != expected {
                println!("Test Failed (line {})", i + 1);
                println!("Expected: {}", expected);
                println!("Obtained: {}", trace);
                break;
            }

            println!("{}", trace);
            self.step();
        }
    }

    fn trace(&self) -> String {
        let addr = self.pc;
        let opcode = self.read8(addr);

        let mode = AddressingMode::from(opcode);
        let instruction = Instruction::from(opcode);

        let mut hex_repr = Vec::new();
        for i in 0..mode.bytes() + 1 {
            write!(&mut hex_repr, "{:02X} ", self.read8(addr + i as u16)).unwrap();
        }

        let display_repr = format!("{:?}", instruction);

        format!(
            "{:04X}  {:<10}{:<31} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            addr,
            String::from_utf8(hex_repr).unwrap(),
            display_repr,
            self.a,
            self.x,
            self.y,
            self.get_status() & !0x10,
            self.sp,
        )
    }
}

/*
 * TODO(Future):
 * Method delegation: https://github.com/rust-lang/rfcs/pull/1406
 */
impl<M: Mem> Mem for Cpu<M> {
    fn read8(&self, addr: u16) -> u8 {
        self.mem.read8(addr)
    }

    fn write8(&mut self, addr: u16, val: u8) {
        self.mem.write8(addr, val)
    }
}
