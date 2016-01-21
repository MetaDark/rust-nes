#![allow(unused_variables)]
use mem::Mem;

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
            status: 0,
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

            _ => unimplemented!(),
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
        let addr = self.read8(self.pc);
        self.pc += 1;
        addr
    }

    fn next16(&mut self) -> u16 {
        let addr = self.read16(self.pc);
        self.pc += 2;
        addr
    }

    /* Addressing Modes */
    fn immediate(&mut self) -> u16 {
        let addr = self.pc;
        self.pc += 1;
        addr
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
        let val = self.next16();
        self.read16(val)
    }

    fn indexed_indirect(&mut self) -> u16 {
        let val = self.next8() as u16;
        self.read16(val) + self.y as u16
    }

    fn indirect_indexed(&mut self) -> u16 {
        let val = self.next8() as u16;
        self.read16(val + self.x as u16)
    }

    /* Load / Store */
    fn lda(&mut self, addr: u16) {
        unimplemented!();
    }

    fn ldx(&mut self, addr: u16) {
        unimplemented!();
    }

    fn ldy(&mut self, addr: u16) {
        unimplemented!();
    }

    fn sta(&mut self, addr: u16) {
        unimplemented!();
    }

    fn stx(&mut self, addr: u16) {
        unimplemented!();
    }

    fn sty(&mut self, addr: u16) {
        unimplemented!();
    }


    /* Register Transfers */
    fn tax(&mut self) {
        unimplemented!();
    }

    fn tay(&mut self) {
        unimplemented!();
    }

    fn txa(&mut self) {
        unimplemented!();
    }

    fn tya(&mut self) {
        unimplemented!();
    }


    /* Stack Operations */
    fn tsx(&mut self) {
        unimplemented!();
    }

    fn txs(&mut self) {
        unimplemented!();
    }

    fn pha(&mut self) {
        unimplemented!();
    }

    fn php(&mut self) {
        unimplemented!();
    }

    fn pla(&mut self) {
        unimplemented!();
    }

    fn plp(&mut self) {
        unimplemented!();
    }


    /* Logical */
    fn and(&mut self, addr: u16) {
        unimplemented!();
    }

    fn eor(&mut self, addr: u16) {
        unimplemented!();
    }

    fn ora(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bit(&mut self, addr: u16) {
        unimplemented!();
    }


    /* Arithmetic */
    fn adc(&mut self, addr: u16) {
        unimplemented!();
    }

    fn sbc(&mut self, addr: u16) {
        unimplemented!();
    }

    fn cmp(&mut self, addr: u16) {
        unimplemented!();
    }

    fn cpx(&mut self, addr: u16) {
        unimplemented!();
    }

    fn cpy(&mut self, addr: u16) {
        unimplemented!();
    }


    /* Increments & Decrements */
    fn inc(&mut self, addr: u16) {
        unimplemented!();
    }

    fn inx(&mut self) {
        unimplemented!();
    }

    fn iny(&mut self) {
        unimplemented!();
    }

    fn dec(&mut self, addr: u16) {
        unimplemented!();
    }

    fn dex(&mut self) {
        unimplemented!();
    }

    fn dey(&mut self) {
        unimplemented!();
    }


    /* Shifts */
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
        unimplemented!();
    }

    fn jsr(&mut self, addr: u16) {
        unimplemented!();
    }

    fn rts(&mut self) {
        unimplemented!();
    }


    /* Branches */
    fn bcc(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bcs(&mut self, addr: u16) {
        unimplemented!();
    }

    fn beq(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bmi(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bne(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bpl(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bvc(&mut self, addr: u16) {
        unimplemented!();
    }

    fn bvs(&mut self, addr: u16) {
        unimplemented!();
    }


    /* Status Flag Changes */
    fn clc(&mut self) {
        unimplemented!();
    }

    fn cld(&mut self) {
        unimplemented!();
    }

    fn cli(&mut self) {
        unimplemented!();
    }

    fn clv(&mut self) {
        unimplemented!();
    }

    fn sec(&mut self) {
        unimplemented!();
    }

    fn sed(&mut self) {
        unimplemented!();
    }

    fn sei(&mut self) {
        unimplemented!();
    }


    /* System Functions */
    fn brk(&mut self) {
        unimplemented!();
    }

    fn nop(&mut self) {
        unimplemented!();
    }

    fn rti(&mut self) {
        unimplemented!();
    }

}

impl<M: Mem> Mem for Cpu<M> {
    fn read8(&self, index: u16) -> u8 {
        self.mem.read8(index)
    }

    fn write8(&mut self, index: u16, val: u8) {
        self.mem.write8(index, val)
    }
}
