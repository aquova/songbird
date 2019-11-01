use std::fs::File;
use std::io::Read;

pub const BYTE: u8 = 8;
const RAM_SIZE: usize = 0xFFFF;
const ROM_SIZE: usize = 0xFFFFFFFF;

pub enum Flags {
    Z,
    N,
    H,
    C
}

#[derive(Copy, Clone)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

#[derive(Copy, Clone)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL
}

#[derive(Copy, Clone)]
pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub ram: [u8; RAM_SIZE],
    pub rom: [u8; ROM_SIZE]
}

pub trait ModifyBits {
    fn get_bit(&self, digit: u8) -> bool;
    fn set_bit(&mut self, digit: u8);
    fn clear_bit(&mut self, digit: u8);
    fn write_bit(&mut self, digit: u8, val: bool);
}

impl ModifyBits for u8 {
    // Bits are organized as 0b7654_3210
    fn get_bit(&self, digit: u8) -> bool {
        let mut mask = 0b1;
        mask <<= digit;
        self & mask != 0
    }

    fn set_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self |= mask;
    }

    fn clear_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self &= !mask;
    }

    fn write_bit(&mut self, digit: u8, val: bool) {
        if val {
            self.set_bit(digit);
        } else {
            self.clear_bit(digit);
        }
    }
}

// TODO: See if u8 and u16 can be merged. Maybe with generics?
impl ModifyBits for u16 {
    fn get_bit(&self, digit: u8) -> bool {
        let mut mask = 0b1;
        mask <<= digit;
        self & mask != 0
    }

    fn set_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self |= mask;
    }

    fn clear_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self &= !mask;
    }

    fn write_bit(&mut self, digit: u8, val: bool) {
        if val {
            self.set_bit(digit);
        } else {
            self.clear_bit(digit);
        }
    }
}

pub trait ModifyBytes {
    fn get_high_byte(&self) -> u8;
    fn get_low_byte(&self) -> u8;
}

impl ModifyBytes for u16 {
    fn get_high_byte(&self) -> u8 {
        (self >> 8) as u8
    }

    fn get_low_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

pub fn merge_bytes(first: u8, second: u8) -> u16 {
    ((first as u16) << BYTE) | (second as u16)
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0xFFFE,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            ram: [0; RAM_SIZE],
            rom: [0; ROM_SIZE]
        }
    }

    pub fn load_game(&mut self, path: &str) {
        let mut buffer: Vec<u8> = Vec::new();
        let mut f = File::open(path).expect("Error opening ROM");
        f.read_to_end(&mut buffer).expect("Error reading ROM to buffer");

        for i in 0..buffer.len() {
            self.rom[i] = buffer[i];
        }
    }

    pub fn tick(&mut self) {

    }

    pub fn fetch(&mut self) -> u8 {
        let val = self.ram[self.pc as usize];
        self.pc += 1;
        val
    }

    pub fn read_ram(self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_ram(&mut self, address: u16, val: u8) {
        self.ram[address as usize] = val;
    }

    pub fn get_reg(self, r: Regs) -> u8 {
        match r {
            Regs::A => { self.a },
            Regs::B => { self.b },
            Regs::C => { self.c },
            Regs::D => { self.d },
            Regs::E => { self.e },
            Regs::F => { self.f },
            Regs::H => { self.h },
            Regs::L => { self.l },
        }
    }

    pub fn set_reg(&mut self, r: Regs, val: u8) {
        match r {
            Regs::A => { self.a = val },
            Regs::B => { self.b = val },
            Regs::C => { self.c = val },
            Regs::D => { self.d = val },
            Regs::E => { self.e = val },
            Regs::F => { self.f = val },
            Regs::H => { self.h = val },
            Regs::L => { self.l = val },
        }
    }

    pub fn set_reg_16(&mut self, r: Regs16, val: u16) {
        let high = val.get_high_byte();
        let low = val.get_low_byte();
        match r {
            Regs16::AF => {
                self.set_reg(Regs::A, high);
                self.set_reg(Regs::F, low);
            },
            Regs16::BC => {
                self.set_reg(Regs::B, high);
                self.set_reg(Regs::C, low);
            },
            Regs16::DE => {
                self.set_reg(Regs::D, high);
                self.set_reg(Regs::E, low);
            },
            Regs16::HL => {
                self.set_reg(Regs::H, high);
                self.set_reg(Regs::L, low);
            }
        }
    }

    pub fn get_reg_16(self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => {
                let high = self.get_reg(Regs::A);
                let low = self.get_reg(Regs::F);
                merge_bytes(high, low)
            },
            Regs16::BC => {
                let high = self.get_reg(Regs::B);
                let low = self.get_reg(Regs::C);
                merge_bytes(high, low)
            },
            Regs16::DE => {
                let high = self.get_reg(Regs::D);
                let low = self.get_reg(Regs::E);
                merge_bytes(high, low)
            },
            Regs16::HL => {
                let high = self.get_reg(Regs::H);
                let low = self.get_reg(Regs::L);
                merge_bytes(high, low)
            }
        }
    }

    pub fn set_flag(&mut self, f: Flags) {
        match f {
            Flags::Z => { self.f |= 0b1000_0000 },
            Flags::N => { self.f |= 0b0100_0000 },
            Flags::H => { self.f |= 0b0010_0000 },
            Flags::C => { self.f |= 0b0001_0000 },
        }
    }

    pub fn clear_flag(&mut self, f: Flags) {
        match f {
            Flags::Z => { self.f &= 0b0111_1111 },
            Flags::N => { self.f &= 0b1011_1111 },
            Flags::H => { self.f &= 0b1101_1111 },
            Flags::C => { self.f &= 0b1110_1111 },
        }
    }

    pub fn get_flag(self, f: Flags) -> bool {
        match f {
            Flags::Z => { return (self.f & 0b1000_0000) != 0 },
            Flags::N => { return (self.f & 0b0100_0000) != 0 },
            Flags::H => { return (self.f & 0b0010_0000) != 0 },
            Flags::C => { return (self.f & 0b0001_0000) != 0 },
        }
    }

    pub fn write_flag(&mut self, f: Flags, val: bool) {
        if val {
            self.set_flag(f);
        } else {
            self.clear_flag(f);
        }
    }

    pub fn ld_n_d8(&mut self, reg: Regs, byte: u8) {
        self.set_reg(reg, byte);
    }

    pub fn ld_nn_d16(&mut self, reg: Regs16, val: u16) {
        self.set_reg_16(reg, val);
    }

    pub fn inc_8(&mut self, reg: Regs) {
        let val = self.get_reg(reg);
        let old_h = val.get_bit(3);
        let result = val.wrapping_add(1);
        self.set_reg(reg, result);

        self.clear_flag(Flags::N);
        self.write_flag(Flags::Z, result == 0);
        // Set H flag if overflow 3rd bit
        let set_h = old_h && !val.get_bit(3);
        self.write_flag(Flags::H, set_h);
    }

    pub fn inc_16(&mut self, reg: Regs16) {
        let val = self.get_reg_16(reg);
        let result = val.wrapping_add(1);
        self.set_reg_16(reg, result);
    }

    pub fn dec_8(&mut self, reg: Regs) {
        let val = self.get_reg(reg);
        let old_h = val.get_bit(4);
        let result = val.wrapping_sub(1);
        self.set_reg(reg, result);

        self.set_flag(Flags::N);
        self.write_flag(Flags::Z, result == 0);
        // Set H flag if borrow from 4th bit
        let set_h = old_h && !val.get_bit(4);
        self.write_flag(Flags::H, set_h);
    }

    pub fn dec_16(&mut self, reg: Regs16) {
        let val = self.get_reg_16(reg);
        let result = val.wrapping_sub(1);
        self.set_reg_16(reg, result);
    }

    pub fn add_a_d8(&mut self, val: u8, adc: bool) {
        let mut carry = 0;
        if adc && self.get_flag(Flags::C) {
            carry = 1;
        }
        let a = self.get_reg(Regs::A);
        let old_h = a.get_bit(3);
        let result1 = a.overflowing_add(val);
        let result2 = result1.0.overflowing_add(carry);
        let set_h = old_h && !result2.0.get_bit(3);

        self.clear_flag(Flags::N);
        self.write_flag(Flags::C, result1.1 || result2.1);
        self.write_flag(Flags::H, set_h);
        self.write_flag(Flags::Z, result2.0 == 0);
        self.set_reg(Regs::A, result2.0);
    }

    pub fn add_nn_d16(&mut self, reg: Regs16, source: u16) {
        let target = self.get_reg_16(reg);
        let old_h = target.get_bit(11);
        let result = target.overflowing_add(source);
        let set_h = old_h && !result.0.get_bit(11);

        self.set_reg_16(reg, result.0);
        self.clear_flag(Flags::N);
        self.write_flag(Flags::C, result.1);
        self.write_flag(Flags::H, set_h);
    }

    pub fn sub_a_d8(&mut self, val: u8, sbc: bool) {
        let carry = 0;
        if sbc && self.get_flag(Flags::C) {
            let carry = 1;
        }

        let a = self.get_reg(Regs::A);
        let old_h = a.get_bit(3);
        let result1 = a.overflowing_sub(val);
        let result2 = result1.0.overflowing_sub(carry);

        // TODO: Need to do H flags properly
        self.set_flag(Flags::N);
        self.write_flag(Flags::Z, result2.0 == 0);
        self.write_flag(Flags::H, diff < 0);
        self.write_flag(Flags::C, result2.1);
        self.set_reg(Regs::A, result2.0);
    }

    pub fn and_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a &= val;
        self.clear_flag(Flags::N);
        self.set_flag(Flags::H);
        self.clear_flag(Flags::C);
        self.write_flag(Flags::Z, a == 0);
        self.set_reg(Regs::A, a);
    }

    pub fn or_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a |= val;
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
        self.write_flag(Flags::Z, a == 0);
        self.set_reg(Regs::A, a);
    }

    pub fn xor_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a ^= val;
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
        self.write_flag(Flags::Z, a == 0);
        self.set_reg(Regs::A, a);
    }

    pub fn cp_a_d8(&mut self, val: u8) {
        let diff = (self.get_reg(Regs::A) as i16) - (val as i16);
        self.set_flag(Flags::N);
        self.write_flag(Flags::Z, diff == 0);
        // TODO: H and C flags
    }

    // Stack starts at 0xFFFE, goes down as stack increases
    pub fn pop(&mut self) -> u16 {
        let byte1 = self.read_ram(self.sp);
        let byte2 = self.read_ram(self.sp + 1);
        let byte = merge_bytes(byte1, byte2);
        self.pc += 2;
        byte
    }

    pub fn push(&mut self, val: u16) {
        let byte1 = val.get_high_byte();
        let byte2 = val.get_low_byte();
        self.write_ram(self.pc - 1, byte1);
        self.write_ram(self.pc, byte2);
        self.pc -= 2;
    }

    // TODO: This might not be right. Not sure if C flag gets swapped in, or just logical/arithmetic
    pub fn rot_right(&mut self, reg: Regs, carry: bool) {
        let mut byte = self.get_reg(reg);
        let lsb = byte.get_bit(0);
        byte >>= 1;
        if carry {
            let old_c = self.get_flag(Flags::C);
            byte.write_bit(7, old_c);
        }
        self.set_reg(reg, byte);
        self.write_flag(Flags::C, lsb);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.write_flag(Flags::Z, byte == 0);
    }

    pub fn rot_left(&mut self, reg: Regs, carry: bool) {
        let mut byte = self.get_reg(reg);
        let msb = byte.get_bit(7);
        byte <<= 1;
        if carry {
            let old_c = self.get_flag(Flags::C);
            byte.write_bit(0, old_c);
        }
        self.set_reg(reg, byte);
        self.write_flag(Flags::C, msb);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.write_flag(Flags::Z, byte == 0);
    }
}
