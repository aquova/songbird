use crate::cpu::*;

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
