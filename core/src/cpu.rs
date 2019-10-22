pub const BYTE: u8 = 8;
const RAM_SIZE: usize = 65535;

pub enum Flags {
    Z,
    N,
    H,
    C
}

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
            Flags::Z => {self.f |= 0b0111_1111},
            Flags::N => {self.f |= 0b1011_1111},
            Flags::H => {self.f |= 0b1101_1111},
            Flags::C => {self.f |= 0b1110_1111},
        }
    }

    pub fn get_flag(self, f: Flags) -> bool {
        match f {
            Flags::Z => { return (self.f & 0b1000_0000) == 1},
            Flags::N => { return (self.f & 0b0100_0000) == 1},
            Flags::H => { return (self.f & 0b0010_0000) == 1},
            Flags::C => { return (self.f & 0b0001_0000) == 1},
        }
    }
}
