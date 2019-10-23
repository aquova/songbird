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

pub fn ld_nn_d16(reg1: &mut u8, reg2: &mut u8, byte1: u8, byte2: u8) {
    *reg1 = byte1;
    *reg2 = byte2;
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

pub fn inc_16(reg1: &mut u8, reg2: &mut u8) {
    *reg2 += 1;
    // If overflow, increase higher byte
    if *reg2 == 0 {
        *reg1 += 1;
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

pub fn add_16(cpu: &mut Cpu, target1: &mut u8, target2: &mut u8, source1: u8, source2: u8) {
    cpu.clear_flag(Flags::N);

    let lower = (*target2 as u16) + (source2 as u16);
    let upper = (*target1 as u16) + (source1 as u16);
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

    *target2 = lower.get_low_byte();
    *target1 = ((upper << BYTE) + lower).get_high_byte();
}

pub fn rlca(cpu: &mut Cpu) {
    cpu.clear_flag(Flags::Z);
    cpu.clear_flag(Flags::N);
    cpu.clear_flag(Flags::H);
}
