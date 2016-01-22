#![allow(unused_variables)]
use mem::Mem;

/* TODO: Use a bitfield macro for simplication */
const CARRY_FLAG:    u8 = 1 << 0;
const ZERO_FLAG:     u8 = 1 << 1;
const IRQ_FLAG:      u8 = 1 << 2;
const DECIMAL_FLAG:  u8 = 1 << 3;
const BREAK_FLAG:    u8 = 1 << 4;
const OVERFLOW_FLAG: u8 = 1 << 6;
const NEGATIVE_FLAG: u8 = 1 << 7;

/*
 * Reference: http://obelisk.me.uk/6502/instructions.html
 */
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
            status: 0x24,
        }
    }

    pub fn reset(&mut self) {
        unimplemented!();
    }

    pub fn step(&mut self) {
        self.trace();

        let opcode = self.next8();
        match opcode {

            /* Loads */
            0xa9 => { let addr = self.immediate(); self.lda(addr) },
            0xa5 => { let addr = self.zero_page(); self.lda(addr) },
            0xb5 => { let addr = self.zero_page_x(); self.lda(addr) },
            0xad => { let addr = self.absolute(); self.lda(addr) },
            0xbd => { let addr = self.absolute_x(); self.lda(addr) },
            0xb9 => { let addr = self.absolute_y(); self.lda(addr) },
            0xa1 => { let addr = self.indexed_indirect(); self.lda(addr) },
            0xb1 => { let addr = self.indirect_indexed(); self.lda(addr) },

            0xa2 => { let addr = self.immediate(); self.ldx(addr) },
            0xa6 => { let addr = self.zero_page(); self.ldx(addr) },
            0xb6 => { let addr = self.zero_page_y(); self.ldx(addr) },
            0xae => { let addr = self.absolute(); self.ldx(addr) },
            0xbe => { let addr = self.absolute_y(); self.ldx(addr) },

            0xa0 => { let addr = self.immediate(); self.ldy(addr) },
            0xa4 => { let addr = self.zero_page(); self.ldy(addr) },
            0xb4 => { let addr = self.zero_page_x(); self.ldy(addr) },
            0xac => { let addr = self.absolute(); self.ldy(addr) },
            0xbc => { let addr = self.absolute_x(); self.ldy(addr) },

            /* Stores */
            0x85 => { let addr = self.zero_page(); self.sta(addr) },
            0x95 => { let addr = self.zero_page_x(); self.sta(addr) },
            0x8d => { let addr = self.absolute(); self.sta(addr) },
            0x9d => { let addr = self.absolute_x(); self.sta(addr) },
            0x99 => { let addr = self.absolute_y(); self.sta(addr) },
            0x81 => { let addr = self.indexed_indirect(); self.sta(addr) },
            0x91 => { let addr = self.indirect_indexed(); self.sta(addr) },

            0x86 => { let addr = self.zero_page(); self.stx(addr) },
            0x96 => { let addr = self.zero_page_y(); self.stx(addr) },
            0x8e => { let addr = self.absolute(); self.stx(addr) },

            0x84 => { let addr = self.zero_page(); self.sty(addr) },
            0x94 => { let addr = self.zero_page_x(); self.sty(addr) },
            0x8c => { let addr = self.absolute(); self.sty(addr) },

            /* Register Transfers */
            0xaa => self.tax(),
            0xa8 => self.tay(),
            0x8a => self.txa(),
            0x98 => self.tya(),

            /* Stack Operations */
            0xba => self.tsx(),
            0x9a => self.txs(),
            0x48 => self.pha(),
            0x08 => self.php(),
            0x68 => self.pla(),
            0x28 => self.plp(),

            /* Logical */
            0x29 => { let addr = self.immediate(); self.and(addr) },
            0x25 => { let addr = self.zero_page(); self.and(addr) },
            0x35 => { let addr = self.zero_page_x(); self.and(addr) },
            0x2d => { let addr = self.absolute(); self.and(addr) },
            0x3d => { let addr = self.absolute_x(); self.and(addr) },
            0x39 => { let addr = self.absolute_y(); self.and(addr) },
            0x21 => { let addr = self.indexed_indirect(); self.and(addr) },
            0x31 => { let addr = self.indirect_indexed(); self.and(addr) },

            0x49 => { let addr = self.immediate(); self.eor(addr) },
            0x45 => { let addr = self.zero_page(); self.eor(addr) },
            0x55 => { let addr = self.zero_page_x(); self.eor(addr) },
            0x4d => { let addr = self.absolute(); self.eor(addr) },
            0x5d => { let addr = self.absolute_x(); self.eor(addr) },
            0x59 => { let addr = self.absolute_y(); self.eor(addr) },
            0x41 => { let addr = self.indexed_indirect(); self.eor(addr) },
            0x51 => { let addr = self.indirect_indexed(); self.eor(addr) },

            0x09 => { let addr = self.immediate(); self.ora(addr) },
            0x05 => { let addr = self.zero_page(); self.ora(addr) },
            0x15 => { let addr = self.zero_page_x(); self.ora(addr) },
            0x0d => { let addr = self.absolute(); self.ora(addr) },
            0x1d => { let addr = self.absolute_x(); self.ora(addr) },
            0x19 => { let addr = self.absolute_y(); self.ora(addr) },
            0x01 => { let addr = self.indexed_indirect(); self.ora(addr) },
            0x11 => { let addr = self.indirect_indexed(); self.ora(addr) },

            0x24 => { let addr = self.zero_page(); self.bit(addr) },
            0x2C => { let addr = self.absolute(); self.bit(addr) },

            /* Arithmetic */
            0x69 => { let addr = self.immediate(); self.adc(addr) },
            0x65 => { let addr = self.zero_page(); self.adc(addr) },
            0x75 => { let addr = self.zero_page_x(); self.adc(addr) },
            0x6d => { let addr = self.absolute(); self.adc(addr) },
            0x7d => { let addr = self.absolute_x(); self.adc(addr) },
            0x79 => { let addr = self.absolute_y(); self.adc(addr) },
            0x61 => { let addr = self.indexed_indirect(); self.adc(addr) },
            0x71 => { let addr = self.indirect_indexed(); self.adc(addr) },

            0xe9 => { let addr = self.immediate(); self.sbc(addr) },
            0xe5 => { let addr = self.zero_page(); self.sbc(addr) },
            0xf5 => { let addr = self.zero_page_x(); self.sbc(addr) },
            0xed => { let addr = self.absolute(); self.sbc(addr) },
            0xfd => { let addr = self.absolute_x(); self.sbc(addr) },
            0xf9 => { let addr = self.absolute_y(); self.sbc(addr) },
            0xe1 => { let addr = self.indexed_indirect(); self.sbc(addr) },
            0xf1 => { let addr = self.indirect_indexed(); self.sbc(addr) },

            0xc9 => { let addr = self.immediate(); self.cmp(addr) },
            0xc5 => { let addr = self.zero_page(); self.cmp(addr) },
            0xd5 => { let addr = self.zero_page_x(); self.cmp(addr) },
            0xcd => { let addr = self.absolute(); self.cmp(addr) },
            0xdd => { let addr = self.absolute_x(); self.cmp(addr) },
            0xd9 => { let addr = self.absolute_y(); self.cmp(addr) },
            0xc1 => { let addr = self.indexed_indirect(); self.cmp(addr) },
            0xd1 => { let addr = self.indirect_indexed(); self.cmp(addr) },

            0xe0 => { let addr = self.immediate(); self.cpx(addr) },
            0xe4 => { let addr = self.zero_page(); self.cpx(addr) },
            0xec => { let addr = self.absolute(); self.cpx(addr) },

            0xc0 => { let addr = self.immediate(); self.cpy(addr) },
            0xc4 => { let addr = self.zero_page(); self.cpy(addr) },
            0xcc => { let addr = self.absolute(); self.cpy(addr) },

            /* Increments */
            0xe6 => { let addr = self.zero_page(); self.inc(addr) },
            0xf6 => { let addr = self.zero_page_x(); self.inc(addr) },
            0xee => { let addr = self.absolute(); self.inc(addr) },
            0xfe => { let addr = self.absolute_x(); self.inc(addr) },

            0xe8 => self.inx(),
            0xc8 => self.iny(),

            /* Decrements */
            0xc6 => { let addr = self.zero_page(); self.dec(addr) },
            0xd6 => { let addr = self.zero_page_x(); self.dec(addr) },
            0xce => { let addr = self.absolute(); self.dec(addr) },
            0xde => { let addr = self.absolute_x(); self.dec(addr) },

            0xca => self.dex(),
            0x88 => self.dey(),

            /* Shifts */
            0x0a => self.asl(None),
            0x06 => { let addr = self.zero_page(); self.asl(Some(addr)) },
            0x16 => { let addr = self.zero_page_x(); self.asl(Some(addr)) },
            0x0e => { let addr = self.absolute(); self.asl(Some(addr)) },
            0x1e => { let addr = self.absolute_x(); self.asl(Some(addr)) },

            0x4a => self.lsr(None),
            0x46 => { let addr = self.zero_page(); self.lsr(Some(addr)) },
            0x56 => { let addr = self.zero_page_x(); self.lsr(Some(addr)) },
            0x4e => { let addr = self.absolute(); self.lsr(Some(addr)) },
            0x5e => { let addr = self.absolute_x(); self.lsr(Some(addr)) },

            0x2a => self.rol(None),
            0x26 => { let addr = self.zero_page(); self.rol(Some(addr)) },
            0x36 => { let addr = self.zero_page_x(); self.rol(Some(addr)) },
            0x2e => { let addr = self.absolute(); self.rol(Some(addr)) },
            0x3e => { let addr = self.absolute_x(); self.rol(Some(addr)) },

            0x6a => self.ror(None),
            0x66 => { let addr = self.zero_page(); self.ror(Some(addr)) },
            0x76 => { let addr = self.zero_page_x(); self.ror(Some(addr)) },
            0x6e => { let addr = self.absolute(); self.ror(Some(addr)) },
            0x7e => { let addr = self.absolute_x(); self.ror(Some(addr)) },

            /* Jumps & Calls */
            0x4c => { let addr = self.absolute(); self.jmp(addr) },
            0x6c => { let addr = self.indirect(); self.jmp(addr) },

            0x20 => { let addr = self.absolute(); self.jsr(addr) },
            0x60 => self.rts(),

            /* Branches */
            0x90 => { let addr = self.relative(); self.bcc(addr) },
            0xb0 => { let addr = self.relative(); self.bcs(addr) },
            0xf0 => { let addr = self.relative(); self.beq(addr) },
            0x30 => { let addr = self.relative(); self.bmi(addr) },
            0xd0 => { let addr = self.relative(); self.bne(addr) },
            0x10 => { let addr = self.relative(); self.bpl(addr) },
            0x50 => { let addr = self.relative(); self.bvc(addr) },
            0x70 => { let addr = self.relative(); self.bvs(addr) },

            // Status Flag Changes
            0x18 => self.clc(),
            0xd8 => self.cld(),
            0x58 => self.cli(),
            0xb8 => self.clv(),
            0x38 => self.sec(),
            0xf8 => self.sed(),
            0x78 => self.sei(),

            // System Functions
            0x00 => self.brk(),
            0xea => self.nop(),
            0x40 => self.rti(),

            _ => panic!("not yet implemented opcode 0x{:02X}", opcode),
        }
    }

    pub fn trace(&mut self) {
        println!(
            "{:04X}  {} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.pc,
            "TODO: DISASSEMBLE",
            self.a,
            self.x,
            self.y,
            self.status,
            self.sp
        );
    }

    /* Memory Access */
    fn next8(&mut self) -> u8 {
        let val = self.read8(self.pc);
        self.pc += 1;
        val
    }

    fn next16(&mut self) -> u16 {
        let val = self.read16(self.pc);
        self.pc += 2;
        val
    }

    /* Addressing Modes */
    fn immediate(&mut self) -> u16 {
        let val = self.pc;
        self.pc += 1;
        val
    }

    fn zero_page(&mut self) -> u16 {
        self.next8() as u16
    }

    fn zero_page_x(&mut self) -> u16 {
        (self.next8() + self.x) as u16
    }

    fn zero_page_y(&mut self) -> u16 {
        (self.next8() + self.y) as u16
    }

    fn relative(&mut self) -> u16 {
        self.next8() as u16 + self.pc
    }

    fn absolute(&mut self) -> u16 {
        self.next16()
    }

    fn absolute_x(&mut self) -> u16 {
        self.next16() + self.x as u16
    }

    fn absolute_y(&mut self) -> u16 {
        self.next16() + self.y as u16
    }

    /* I think like these reads get fucked up */
    fn indirect(&mut self) -> u16 {
        let addr = self.next16();
        self.read16(addr)
    }

    fn indexed_indirect(&mut self) -> u16 {
        let addr = self.next8() as u16;
        self.read16(addr) + self.y as u16
    }

    fn indirect_indexed(&mut self) -> u16 {
        let addr = self.next8() as u16;
        self.read16(addr + self.x as u16)
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
        self.set_zn(val);
    }

    fn pha(&mut self) {
        let val = self.a;
        self.push8(val);
    }

    fn php(&mut self) {
        let status = self.read_status();
        self.push8(status);
    }

    fn pla(&mut self) {
        let val = self.pull8();
        self.a = val;
        self.set_zn(val)
    }

    fn plp(&mut self) {
        let status = self.pull8();
        self.write_status(status)
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
        let val = self.a;
        let mask = self.read8(addr);
        self.set_status(ZERO_FLAG, (mask & val) == 0);
        self.set_status(OVERFLOW_FLAG, (mask >> 6) != 0);
        self.set_status(NEGATIVE_FLAG, (mask >> 7) != 0);
    }


    /* Arithmetic */
    fn adc(&mut self, addr: u16) {
        let a = self.a as u16;
        let b = self.read8(addr) as u16;
        let result = a + b + self.get_status(CARRY_FLAG) as u16;

        self.a = result as u8;
        self.set_status(CARRY_FLAG, (result & 0x0100) != 0);
        self.set_status(OVERFLOW_FLAG, unimplemented!());
        self.set_zn(result as u8);
    }

    fn sbc(&mut self, addr: u16) {
        let a = self.a as u16;
        let b = self.read8(addr) as u16;
        let result = a - b - !self.get_status(CARRY_FLAG) as u16;

        self.a = result as u8;
        self.set_status(CARRY_FLAG, (result & 0x0100) == 0);
        self.set_status(OVERFLOW_FLAG, unimplemented!());
        self.set_zn(result as u8);
    }

    fn cmp_base(&mut self, a: u8, b: u8) {
        let result = a - b;
        self.set_status(CARRY_FLAG, b <= a);
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
        let result = self.read8(addr) + 1;
        self.write8(addr, result);
        self.set_zn(result);
    }

    fn inx(&mut self) {
        let result = self.x + 1;
        self.x = result;
        self.set_zn(result);
    }

    fn iny(&mut self) {
        let result = self.y + 1;
        self.y = result;
        self.set_zn(result);
    }

    fn dec(&mut self, addr: u16) {
        let result = self.read8(addr) - 1;
        self.write8(addr, result);
        self.set_zn(result);
    }

    fn dex(&mut self) {
        let result = self.x - 1;
        self.x = result;
        self.set_zn(result);
    }

    fn dey(&mut self) {
        let result = self.y - 1;
        self.y = result;
        self.set_zn(result);
    }


    /* Shifts */
    fn shl_base(&mut self, addr: u16, msb: bool) {
        unimplemented!();
    }

    fn shr_base(&mut self, addr: u16, msb: bool) {
        unimplemented!();
    }

    fn asl(&mut self, addr: Option<u16>) {
        unimplemented!();
    }

    fn lsr(&mut self, addr: Option<u16>) {
        unimplemented!();
    }

    fn rol(&mut self, addr: Option<u16>) {
        unimplemented!();
    }

    fn ror(&mut self, addr: Option<u16>) {
        unimplemented!();
    }


    /* Jumps & Calls */
    fn jmp(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn jsr(&mut self, addr: u16) {
        let pc = self.pc - 1;
        self.push16(pc);
        self.pc = addr;
    }

    fn rts(&mut self) {
        let pc = self.pull16() + 1;
        self.pc = pc;
    }


    /* Branches */
    fn bcc(&mut self, addr: u16) {
        if !self.get_status(CARRY_FLAG) {
            self.pc = addr;
        }
    }

    fn bcs(&mut self, addr: u16) {
        if self.get_status(CARRY_FLAG) {
            self.pc = addr;
        }
    }

    fn beq(&mut self, addr: u16) {
        if self.get_status(ZERO_FLAG) {
            self.pc = addr;
        }
    }

    fn bmi(&mut self, addr: u16) {
        if self.get_status(NEGATIVE_FLAG) {
            self.pc = addr;
        }
    }

    fn bne(&mut self, addr: u16) {
        if !self.get_status(ZERO_FLAG) {
            self.pc = addr;
        }
    }

    fn bpl(&mut self, addr: u16) {
        if !self.get_status(NEGATIVE_FLAG) {
            self.pc = addr;
        }
    }

    fn bvc(&mut self, addr: u16) {
        if !self.get_status(OVERFLOW_FLAG) {
            self.pc = addr;
        }
    }

    fn bvs(&mut self, addr: u16) {
        if self.get_status(OVERFLOW_FLAG) {
            self.pc = addr;
        }
    }


    /* Status Flag Changes */
    fn clc(&mut self) {
        self.set_status(CARRY_FLAG, false);
    }

    fn cld(&mut self) {
        self.set_status(DECIMAL_FLAG, false);
    }

    fn cli(&mut self) {
        self.set_status(IRQ_FLAG, false);
    }

    fn clv(&mut self) {
        self.set_status(OVERFLOW_FLAG, false);
    }

    fn sec(&mut self) {
        self.set_status(CARRY_FLAG, true);
    }

    fn sed(&mut self) {
        self.set_status(DECIMAL_FLAG, true);
    }

    fn sei(&mut self) {
        self.set_status(IRQ_FLAG, true);
    }


    /* System Functions */
    fn brk(&mut self) {
        self.jsr(0xFFFE);
        self.php();
        self.sei();
    }

    fn nop(&mut self) {}

    fn rti(&mut self) {
        self.plp();
        self.pc = self.pull16();
    }


    /* Flag helpers */
    fn get_status(&mut self, status: u8) -> bool {
        (self.status & status) != 0
    }

    fn set_status(&mut self, status: u8, on: bool) {
        if on {
            self.status |= status;
        } else {
            self.status &= !status;
        }
    }

    fn read_status(&mut self) -> u8 {
        self.status | 0x30
    }

    fn write_status(&mut self, val: u8) {
        self.status |= val & !0x30;
    }

    fn set_zn(&mut self, val: u8) {
        self.set_status(ZERO_FLAG, val == 0);
        self.set_status(NEGATIVE_FLAG, (val >> 7) != 0);
    }


    /* Stack helpers */
    fn pull8(&mut self) -> u8 {
        let sp = self.sp as u16;
        let val = self.read8(0x0100 + sp + 1);
        self.sp += 1;
        val
    }

    fn push8(&mut self, val: u8) {
        let sp = self.sp as u16;
        self.write8(0x0100 + sp, val);
        self.sp -= 1;
    }

    fn pull16(&mut self) -> u16 {
        let sp = self.sp as u16;
        let val = self.read16(0x0100 + sp + 1);
        self.sp += 2;
        val
    }

    fn push16(&mut self, val: u16) {
        let sp = self.sp as u16;
        self.write16(0x0100 + (sp - 1), val);
        self.sp -= 2;
    }
}

impl<M: Mem> Mem for Cpu<M> {
    fn read8(&self, addr: u16) -> u8 {
        self.mem.read8(addr)
    }

    fn write8(&mut self, addr: u16, val: u8) {
        self.mem.write8(addr, val)
    }
}
