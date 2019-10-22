use crate::cpu::*;

pub fn merge_bytes(first: u8, second: u8) -> u16 {
    ((first as u16) << BYTE) | (second as u16)
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
