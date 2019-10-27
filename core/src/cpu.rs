use std::fs::File;
use std::io::Read;

pub const BYTE: u8 = 8;
const RAM_SIZE: usize = 65535;

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

pub trait ModifyBytes {
    fn get_high_byte(&self) -> u8;
    fn get_low_byte(&self) -> u8;
    fn set_high_byte(&mut self, val: u8);
    fn set_low_byte(&mut self, val: u8);
}

impl ModifyBytes for u16 {
    fn get_high_byte(&self) -> u8 {
        (self >> 8) as u8
    }

    fn get_low_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }

    fn set_high_byte(&mut self, val: u8) {
        *self |= (val as u16) << 8;
    }

    fn set_low_byte(&mut self, val: u8) {
        *self |= val as u16;
    }
}

pub fn merge_bytes(first: u8, second: u8) -> u16 {
    ((first as u16) << BYTE) | (second as u16)
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
    pub ram: [u8; RAM_SIZE]
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            ram: [0; RAM_SIZE]
        }
    }

    pub fn load_game(&mut self, path: &str) {
        let mut buffer: Vec<u8> = Vec::new();
        let mut f = File::open(path).expect("Error opening ROM");

        f.read_to_end(&mut buffer).expect("Error reading ROM to buffer");

        // for i in 0x100..buffer.len() {
        //     self.ram[i] = buffer[i];
        // }
    }

    pub fn tick(&mut self) {

    }

    pub fn fetch(&mut self) -> u8 {
        let val = self.ram[self.pc as usize];
        self.pc += 1;
        val
    }

    pub fn set_byte(&mut self, data: u8, index: usize) {
        self.ram[index] = data;
    }

    pub fn get_reg(self, r: Regs) -> u8 {
        match r {
            Regs::A => { self.a },
            Regs::B => { self.b },
            Regs::C => { self.c },
            Regs::D => { self.d },
            Regs::E => { self.e },
            Regs::H => { self.h },
            Regs::L => { self.l },
            _ => { panic!("Invalid reg"); }
        }
    }

    pub fn set_reg(&mut self, r: Regs, val: u8) {
        match r {
            Regs::A => { self.a = val },
            Regs::B => { self.b = val },
            Regs::C => { self.c = val },
            Regs::D => { self.d = val },
            Regs::E => { self.e = val },
            Regs::H => { self.h = val },
            Regs::L => { self.l = val },
            _ => { panic!("Invalid reg"); }
        }
    }

    pub fn set_flag(&mut self, f: Flags) {
        match f {
            Flags::Z => {self.f |= 0b1000_0000},
            Flags::N => {self.f |= 0b0100_0000},
            Flags::H => {self.f |= 0b0010_0000},
            Flags::C => {self.f |= 0b0001_0000},
        }
    }

    pub fn clear_flag(&mut self, f: Flags) {
        match f {
            Flags::Z => {self.f &= 0b0111_1111},
            Flags::N => {self.f &= 0b1011_1111},
            Flags::H => {self.f &= 0b1101_1111},
            Flags::C => {self.f &= 0b1110_1111},
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

    pub fn ld_n_d8(&mut self, reg: Regs, byte: u8) {
        self.set_reg(reg, byte);
    }

    pub fn ld_nn_d16(&mut self, high_reg: Regs, low_reg: Regs, high_byte: u8, low_byte: u8) {
        self.set_reg(high_reg, high_byte);
        self.set_reg(low_reg, low_byte);
    }

    pub fn inc_8(&mut self, reg: Regs) {
        let mut val = self.get_reg(reg);
        val += 1;
        self.set_reg(reg, val);
        self.clear_flag(Flags::N);
        if val == 0 {
            self.clear_flag(Flags::Z);
            self.clear_flag(Flags::H);
        } else {
            self.set_flag(Flags::Z);
            self.set_flag(Flags::H);
        }
    }

    // TODO: Can probably infer high reg from low
    pub fn inc_16(&mut self, high_reg: Regs, low_reg: Regs) {
        let mut low = self.get_reg(low_reg);
        low += 1;
        self.set_reg(low_reg, low);
        // If overflow, increase higher byte
        if low == 0 {
            let high = self.get_reg(high_reg);
            self.set_reg(high_reg, high + 1);
        }
    }

    pub fn dec_8(&mut self, reg: Regs) {
        let mut val = self.get_reg(reg);
        val -= 1;
        self.set_reg(reg, val);
        self.set_flag(Flags::N);
        if val == 0xFF {
            self.set_flag(Flags::H);
        } else {
            self.clear_flag(Flags::H);
        }

        if val == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }
    }

    pub fn dec_16(&mut self, high_reg: Regs, low_reg: Regs) {
        let low = self.get_reg(low_reg);
        let high = self.get_reg(high_reg);
        let mut data = merge_bytes(high, low);
        data -= 1;
        self.set_reg(high_reg, data.get_high_byte());
        self.set_reg(low_reg, data.get_low_byte());
    }

    pub fn add_a_d8(&mut self, val: u8, adc: bool) {
        let carry = 0;
        if adc && self.get_flag(Flags::C) {
            let carry = 1;
        }

        let sum = (self.get_reg(Regs::A) as u16) + (val as u16) + carry;
        self.clear_flag(Flags::N);
        if sum > 0xFF {
            self.set_flag(Flags::C);
            self.set_flag(Flags::H);
        } else {
            self.clear_flag(Flags::C);
            self.clear_flag(Flags::H);
        }

        if sum == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }

        self.set_reg(Regs::A, sum as u8);
    }

    pub fn add_nn_d16(&mut self, high_target: Regs, low_target: Regs, high_val: u8, low_val: u8) {
        self.clear_flag(Flags::N);
        let high = self.get_reg(high_target);
        let low = self.get_reg(low_target);

        let lower_sum = (low as u16) + (low_val as u16);
        let upper_sum = (high as u16) + (high_val as u16);
        let carry = if lower_sum > 0xFF { 1 } else { 0 };

        if carry == 1 {
            self.set_flag(Flags::H);
        } else {
            self.clear_flag(Flags::H);
        }

        if upper_sum + carry > 0xFF {
            self.set_flag(Flags::C);
        } else {
            self.clear_flag(Flags::C);
        }

        self.set_reg(low_target, lower_sum.get_low_byte());
        self.set_reg(high_target, ((upper_sum << BYTE) + lower_sum).get_high_byte());
    }

    pub fn sub_a_d8(&mut self, val: u8, sbc: bool) {
        let carry = 0;
        if sbc && self.get_flag(Flags::C) {
            let carry = 1;
        }

        let diff: i16 = (self.get_reg(Regs::A) as i16) - (val as i16) - carry;
        self.set_flag(Flags::N);

        if diff == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }

        if diff < 0 {
            self.set_flag(Flags::H);
            self.set_flag(Flags::C);
        } else {
            self.clear_flag(Flags::H);
            self.clear_flag(Flags::C);
        }

        self.set_reg(Regs::A, diff as u8);
    }

    pub fn and_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a &= val;
        self.clear_flag(Flags::N);
        self.set_flag(Flags::H);
        self.clear_flag(Flags::C);
        if a == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }

        self.set_reg(Regs::A, a);
    }

    pub fn or_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a |= val;
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
        if a == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }

        self.set_reg(Regs::A, a);
    }

    pub fn xor_a_d8(&mut self, val: u8) {
        let mut A = self.get_reg(Regs::A);
        A ^= val;
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
        if A == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }

        self.set_reg(Regs::A, A);
    }

    pub fn cp_a_d8(&mut self, val: u8) {
        let diff = (self.get_reg(Regs::A) as i16) - (val as i16);
        self.set_flag(Flags::N);
        if diff == 0 {
            self.set_flag(Flags::Z);
        } else {
            self.clear_flag(Flags::Z);
        }
        // TODO: H and C flags
    }

    pub fn rlca(&mut self) {
        self.clear_flag(Flags::Z);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
    }
}
