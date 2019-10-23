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

    fn set_high_byte(&mut self, val) {
        *self |= (val as u16) << 8;
    }

    fn set_low_byte(&mut self, val) {
        *self |= val as u16;
    }
}

pub fn merge_bytes(first: u8, second: u8) -> u16 {
    ((first as u16) << BYTE) | (second as u16)
}

pub fn ld_n_d8(reg: &mut u8, byte: u8) {
    *reg = byte;
}

pub fn ld_nn_d16(high_reg: &mut u8, low_reg: &mut u8, high_byte: u8, low_byte: u8) {
    *high_reg = high_byte;
    *low_reg = low_byte;
}

pub fn inc_8(cpu: &mut Cpu, reg: &mut u8) {
    *reg += 1;
    cpu.clear_flag(Flags::N);
    if *reg == 0 {
        cpu.clear_flag(Flags::Z);
        cpu.clear_flag(Flags::H);
    } else {
        cpu.set_flag(Flags::Z);
        cpu.set_flag(Flags::H);
    }
}

pub fn inc_16(high_reg: &mut u8, low_reg: &mut u8) {
    *low_reg += 1;
    // If overflow, increase higher byte
    if *low_reg == 0 {
        *high_reg += 1;
    }
}

pub fn dec_8(cpu: &mut Cpu, reg: &mut u8) {
    *reg -= 1;
    cpu.set_flag(Flags::N);
    if *reg == 0xFF {
        cpu.set_flag(Flags::H);
    } else {
        cpu.clear_flag(Flags::H);
    }

    if *reg == 0 {
        cpu.set_flag(Flags::Z);
    } else {
        cpu.clear_flag(Flags::Z);
    }
}

pub fn dec_16(high_reg: &mut u8, low_reg: &mut u8) {
    let mut data = merge_bytes(*high_reg, *low_reg);
    data -= 1;
    *high_reg = data.get_high_byte();
    *low_reg = data.get_low_byte();
}

pub fn add_16(cpu: &mut Cpu, high_target: &mut u8, low_target: &mut u8, high_source: u8, low_source: u8) {
    cpu.clear_flag(Flags::N);

    let lower = (*low_target as u16) + (low_source as u16);
    let upper = (*high_target as u16) + (high_source as u16);
    let carry = if lower > 0xFF { 1 } else { 0 };

    if carry == 1 {
        cpu.set_flag(Flags::H);
    } else {
        cpu.clear_flag(Flags::H);
    }

    if upper + carry > 0xFF {
        cpu.set_flag(Flags::C);
    } else {
        cpu.clear_flag(Flags::C);
    }

    *low_target = lower.get_low_byte();
    *high_target = ((upper << BYTE) + lower).get_high_byte();
}

pub fn rlca(cpu: &mut Cpu) {
    cpu.clear_flag(Flags::Z);
    cpu.clear_flag(Flags::N);
    cpu.clear_flag(Flags::H);
}
