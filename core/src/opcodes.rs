// Borrowed some of the implementation from here: https://github.com/blackxparade/Rust-Boy/blob/master/Emulator/src/cpu/opcode.rs
use crate::cpu::*;

pub struct Opcode {
    pub op: [fn(&mut Cpu) -> u8; 256]
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            op: [Opcode::invalid; 256]
        }
    }

    pub fn execute(self, cpu: &mut Cpu, opcode: u8) -> u8 {
        cpu.pc += 1;
        self.op[opcode as usize](cpu)
    }

    // Set up opcode lookup table
    // May god have mercy on our souls
    pub fn init(&mut self) {
        self.op[0x00] = Opcode::nop;
        self.op[0x01] = Opcode::ld_01;
        self.op[0x02] = Opcode::ld_02;
        self.op[0x03] = Opcode::inc_03;
        self.op[0x04] = Opcode::inc_04;
        self.op[0x05] = Opcode::dec_05;
        self.op[0x06] = Opcode::ld_06;
        self.op[0x07] = Opcode::rlca_07;
        self.op[0x08] = Opcode::ld_08;
        self.op[0x09] = Opcode::add_09;
        self.op[0x0A] = Opcode::ld_0a;
        self.op[0x0B] = Opcode::dec_0b;
        self.op[0x0C] = Opcode::inc_0c;
        self.op[0x0D] = Opcode::dec_0d;
        self.op[0x0E] = Opcode::ld_0e;
        self.op[0x0F] = Opcode::rrca_0f;
        self.op[0x10] = Opcode::stop_10;
        self.op[0x11] = Opcode::ld_11;
        self.op[0x12] = Opcode::ld_12;
        self.op[0x13] = Opcode::inc_13;
        self.op[0x14] = Opcode::inc_14;
        self.op[0x15] = Opcode::dec_15;
        self.op[0x16] = Opcode::ld_16;
        self.op[0x17] = Opcode::rla_17;
        self.op[0x18] = Opcode::jr_18;
        self.op[0x19] = Opcode::add_19;
        self.op[0x1A] = Opcode::ld_1a;
        self.op[0x1B] = Opcode::dec_1b;
        self.op[0x1C] = Opcode::inc_1c;
        self.op[0x1D] = Opcode::dec_1d;
        self.op[0x1E] = Opcode::ld_1e;
        self.op[0x1F] = Opcode::rra_1f;
        self.op[0x20] = Opcode::jr_20;
        self.op[0x21] = Opcode::ld_21;
        self.op[0x22] = Opcode::ld_22;
        self.op[0x23] = Opcode::inc_23;
        self.op[0x24] = Opcode::inc_24;
        self.op[0x25] = Opcode::dec_25;
        self.op[0x26] = Opcode::ld_26;
        self.op[0x27] = Opcode::daa_27;
        self.op[0x28] = Opcode::jr_28;
        self.op[0x29] = Opcode::add_29;
        self.op[0x2A] = Opcode::ld_2a;
        self.op[0x2B] = Opcode::dec_2b;
        self.op[0x2C] = Opcode::inc_2c;
        self.op[0x2D] = Opcode::dec_2d;
        self.op[0x2E] = Opcode::ld_2e;
        self.op[0x2F] = Opcode::cpl_2f;
        self.op[0x30] = Opcode::jr_30;
        self.op[0x31] = Opcode::ld_31;
        self.op[0x32] = Opcode::ld_32;
        self.op[0x33] = Opcode::inc_33;
        self.op[0x34] = Opcode::inc_34;
        self.op[0x35] = Opcode::dec_35;
        self.op[0x36] = Opcode::ld_36;
        self.op[0x37] = Opcode::scf_37;
        self.op[0x38] = Opcode::jr_38;
        self.op[0x39] = Opcode::add_39;
        self.op[0x3A] = Opcode::ld_3a;
        self.op[0x3B] = Opcode::dec_3b;
        self.op[0x3C] = Opcode::inc_3c;
        self.op[0x3D] = Opcode::dec_3d;
        self.op[0x3E] = Opcode::ld_3e;
        self.op[0x3F] = Opcode::ccf_3f;
        self.op[0x40] = Opcode::ld_40;
        self.op[0x41] = Opcode::ld_41;
        self.op[0x42] = Opcode::ld_42;
        self.op[0x43] = Opcode::ld_43;
        self.op[0x44] = Opcode::ld_44;
        self.op[0x45] = Opcode::ld_45;
        self.op[0x46] = Opcode::ld_46;
        self.op[0x47] = Opcode::ld_47;
        self.op[0x48] = Opcode::ld_48;
        self.op[0x49] = Opcode::ld_49;
        self.op[0x4A] = Opcode::ld_4a;
        self.op[0x4B] = Opcode::ld_4b;
        self.op[0x4C] = Opcode::ld_4c;
        self.op[0x4D] = Opcode::ld_4d;
        self.op[0x4E] = Opcode::ld_4e;
        self.op[0x4F] = Opcode::ld_4f;
        self.op[0x50] = Opcode::ld_50;
        self.op[0x51] = Opcode::ld_51;
        self.op[0x52] = Opcode::ld_52;
        self.op[0x53] = Opcode::ld_53;
        self.op[0x54] = Opcode::ld_54;
        self.op[0x55] = Opcode::ld_55;
        self.op[0x56] = Opcode::ld_56;
        self.op[0x57] = Opcode::ld_57;
        self.op[0x58] = Opcode::ld_58;
        self.op[0x59] = Opcode::ld_59;
        self.op[0x5A] = Opcode::ld_5a;
        self.op[0x5B] = Opcode::ld_5b;
        self.op[0x5C] = Opcode::ld_5c;
        self.op[0x5D] = Opcode::ld_5d;
        self.op[0x5E] = Opcode::ld_5e;
        self.op[0x5F] = Opcode::ld_5f;
        self.op[0x60] = Opcode::ld_60;
        self.op[0x61] = Opcode::ld_61;
        self.op[0x62] = Opcode::ld_62;
        self.op[0x63] = Opcode::ld_63;
        self.op[0x64] = Opcode::ld_64;
        self.op[0x65] = Opcode::ld_65;
        self.op[0x66] = Opcode::ld_66;
        self.op[0x67] = Opcode::ld_67;
        self.op[0x68] = Opcode::ld_68;
        self.op[0x69] = Opcode::ld_69;
        self.op[0x6A] = Opcode::ld_6a;
        self.op[0x6B] = Opcode::ld_6b;
        self.op[0x6C] = Opcode::ld_6c;
        self.op[0x6D] = Opcode::ld_6d;
        self.op[0x6E] = Opcode::ld_6e;
        self.op[0x6F] = Opcode::ld_6f;
        self.op[0x70] = Opcode::ld_70;
        self.op[0x71] = Opcode::ld_71;
        self.op[0x72] = Opcode::ld_72;
        self.op[0x73] = Opcode::ld_73;
        self.op[0x74] = Opcode::ld_74;
        self.op[0x75] = Opcode::ld_75;
        self.op[0x76] = Opcode::halt_76;
        self.op[0x77] = Opcode::ld_77;
        self.op[0x78] = Opcode::ld_78;
        self.op[0x79] = Opcode::ld_79;
        self.op[0x7A] = Opcode::ld_7a;
        self.op[0x7B] = Opcode::ld_7b;
        self.op[0x7C] = Opcode::ld_7c;
        self.op[0x7D] = Opcode::ld_7d;
        self.op[0x7E] = Opcode::ld_7e;
        self.op[0x7F] = Opcode::ld_7f;
        self.op[0x80] = Opcode::add_80;
        self.op[0x81] = Opcode::add_81;
        self.op[0x82] = Opcode::add_82;
        self.op[0x83] = Opcode::add_83;
        self.op[0x84] = Opcode::add_84;
        self.op[0x85] = Opcode::add_85;
        self.op[0x86] = Opcode::add_86;
        self.op[0x87] = Opcode::add_87;
        self.op[0x88] = Opcode::adc_88;
        self.op[0x89] = Opcode::adc_89;
        self.op[0x8A] = Opcode::adc_8a;
        self.op[0x8B] = Opcode::adc_8b;
        self.op[0x8C] = Opcode::adc_8c;
        self.op[0x8D] = Opcode::adc_8d;
        self.op[0x8E] = Opcode::adc_8e;
        self.op[0x8F] = Opcode::adc_8f;
        self.op[0x90] = Opcode::sub_90;
        self.op[0x91] = Opcode::sub_91;
        self.op[0x92] = Opcode::sub_92;
        self.op[0x93] = Opcode::sub_93;
        self.op[0x94] = Opcode::sub_94;
        self.op[0x95] = Opcode::sub_95;
        self.op[0x96] = Opcode::sub_96;
        self.op[0x97] = Opcode::sub_97;
        self.op[0x98] = Opcode::sbc_98;
        self.op[0x99] = Opcode::sbc_99;
        self.op[0x9A] = Opcode::sbc_9a;
        self.op[0x9B] = Opcode::sbc_9b;
        self.op[0x9C] = Opcode::sbc_9c;
        self.op[0x9D] = Opcode::sbc_9d;
        self.op[0x9E] = Opcode::sbc_9e;
        self.op[0x9F] = Opcode::sbc_9f;
        self.op[0xA0] = Opcode::and_a0;
        self.op[0xA1] = Opcode::and_a1;
        self.op[0xA2] = Opcode::and_a2;
        self.op[0xA3] = Opcode::and_a3;
        self.op[0xA4] = Opcode::and_a4;
        self.op[0xA5] = Opcode::and_a5;
        self.op[0xA6] = Opcode::and_a6;
        self.op[0xA7] = Opcode::and_a7;
        self.op[0xA8] = Opcode::xor_a8;
        self.op[0xA9] = Opcode::xor_a9;
        self.op[0xAA] = Opcode::xor_aa;
        self.op[0xAB] = Opcode::xor_ab;
        self.op[0xAC] = Opcode::xor_ac;
        self.op[0xAD] = Opcode::xor_ad;
        self.op[0xAE] = Opcode::xor_ae;
        self.op[0xAF] = Opcode::xor_af;
        self.op[0xB0] = Opcode::or_b0;
        self.op[0xB1] = Opcode::or_b1;
        self.op[0xB2] = Opcode::or_b2;
        self.op[0xB3] = Opcode::or_b3;
        self.op[0xB4] = Opcode::or_b4;
        self.op[0xB5] = Opcode::or_b5;
        self.op[0xB6] = Opcode::or_b6;
        self.op[0xB7] = Opcode::or_b7;
        self.op[0xB8] = Opcode::cp_b8;
        self.op[0xB9] = Opcode::cp_b9;
        self.op[0xBA] = Opcode::cp_ba;
        self.op[0xBB] = Opcode::cp_bb;
        self.op[0xBC] = Opcode::cp_bc;
        self.op[0xBD] = Opcode::cp_bd;
        self.op[0xBE] = Opcode::cp_be;
        self.op[0xBF] = Opcode::cp_bf;
        self.op[0xC0] = Opcode::ret_c0;
        self.op[0xC1] = Opcode::pop_c1;
        self.op[0xC2] = Opcode::jp_c2;
        self.op[0xC3] = Opcode::jp_c3;
        self.op[0xC4] = Opcode::call_c4;
        self.op[0xC5] = Opcode::push_c5;
        self.op[0xC6] = Opcode::add_c6;
        self.op[0xC7] = Opcode::rst_c7;
        self.op[0xC8] = Opcode::ret_c8;
        self.op[0xC9] = Opcode::ret_c9;
        self.op[0xCA] = Opcode::jp_ca;
        self.op[0xCB] = Opcode::prefix_cb;
        self.op[0xCC] = Opcode::call_cc;
        self.op[0xCD] = Opcode::call_cd;
        self.op[0xCE] = Opcode::adc_ce;
        self.op[0xCF] = Opcode::rst_cf;
        self.op[0xD0] = Opcode::ret_d0;
        self.op[0xD1] = Opcode::pop_d1;
        self.op[0xD2] = Opcode::jp_d2;
        self.op[0xD3] = Opcode::invalid;
        self.op[0xD4] = Opcode::call_d4;
        self.op[0xD5] = Opcode::push_d5;
        self.op[0xD6] = Opcode::sub_d6;
        self.op[0xD7] = Opcode::rst_d7;
        self.op[0xD8] = Opcode::ret_d8;
        self.op[0xD9] = Opcode::reti_d9;
        self.op[0xDA] = Opcode::jp_da;
        self.op[0xDB] = Opcode::invalid;
        self.op[0xDC] = Opcode::call_dc;
        self.op[0xDD] = Opcode::invalid;
        self.op[0xDE] = Opcode::sbc_de;
        self.op[0xDF] = Opcode::rst_df;
        self.op[0xE0] = Opcode::ldh_e0;
        self.op[0xE1] = Opcode::pop_e1;
        self.op[0xE2] = Opcode::ld_e2;
        self.op[0xE3] = Opcode::invalid;
        self.op[0xE4] = Opcode::invalid;
        self.op[0xE5] = Opcode::push_e5;
        self.op[0xE6] = Opcode::and_e6;
        self.op[0xE7] = Opcode::rst_e7;
        self.op[0xE8] = Opcode::add_e8;
        self.op[0xE9] = Opcode::jp_e9;
        self.op[0xEA] = Opcode::ld_ea;
        self.op[0xEB] = Opcode::invalid;
        self.op[0xEC] = Opcode::invalid;
        self.op[0xED] = Opcode::invalid;
        self.op[0xEE] = Opcode::xor_ee;
        self.op[0xEF] = Opcode::rst_ef;
        self.op[0xF0] = Opcode::ldh_f0;
        self.op[0xF1] = Opcode::pop_f1;
        self.op[0xF2] = Opcode::ld_f2;
        self.op[0xF3] = Opcode::di_f3;
        self.op[0xF4] = Opcode::invalid;
        self.op[0xF5] = Opcode::push_f5;
        self.op[0xF6] = Opcode::or_f6;
        self.op[0xF7] = Opcode::rst_f7;
        self.op[0xF8] = Opcode::ld_f8;
        self.op[0xF9] = Opcode::ld_f9;
        self.op[0xFA] = Opcode::ld_fa;
        self.op[0xFB] = Opcode::ei_fb;
        self.op[0xFC] = Opcode::invalid;
        self.op[0xFD] = Opcode::invalid;
        self.op[0xFE] = Opcode::cp_fe;
        self.op[0xFF] = Opcode::rst_ff;
    }

    fn invalid(_cpu: &mut Cpu) -> u8 {
        panic!("Invalid opcode");
    }

    // NOP
    fn nop(_cpu: &mut Cpu) -> u8 {
        4
    }

    // LD BC, d16
    fn ld_01(cpu: &mut Cpu) -> u8 {
        let byte1 = cpu.fetch();
        let byte2 = cpu.fetch();
        cpu.ld_nn_d16(Regs::B, Regs::C, byte1, byte2);
        12
    }

    // LD (BC), A
    fn ld_02(cpu: &mut Cpu) -> u8 {
        let b = cpu.get_reg(Regs::B);
        let c = cpu.get_reg(Regs::C);
        let bc = merge_bytes(b, c);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(bc, val);
        8
    }

    // INC BC
    fn inc_03(cpu: &mut Cpu) -> u8 {
        cpu.inc_16(Regs::B, Regs::C);
        8
    }

    // INC B
    fn inc_04(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::B);
        4
    }

    // DEC B
    fn dec_05(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::B);
        4
    }

    // LD B, d8
    fn ld_06(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::B, byte);
        8
    }

    // RLCA
    fn rlca_07(cpu: &mut Cpu) -> u8 {
        cpu.rlca();
        4
    }

    // LD (a16), SP
    fn ld_08(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let addr = merge_bytes(high, low);
        cpu.write_ram(addr, cpu.sp.get_low_byte());
        cpu.write_ram(addr + 1, cpu.sp.get_high_byte());
        20
    }

    // ADD HL, BC
    fn add_09(cpu: &mut Cpu) -> u8 {
        let high_byte = cpu.get_reg(Regs::B);
        let low_byte = cpu.get_reg(Regs::C);
        cpu.add_nn_d16(Regs::H, Regs::L, high_byte, low_byte);
        8
    }

    // LD A, (BC)
    fn ld_0a(cpu: &mut Cpu) -> u8 {
        let b = cpu.get_reg(Regs::B);
        let c = cpu.get_reg(Regs::C);
        let bc = merge_bytes(b, c);
        let val = cpu.read_ram(bc);
        cpu.ld_n_d8(Regs::A, val);
        8
    }

    // DEC BC
    fn dec_0b(cpu: &mut Cpu) -> u8 {
        cpu.dec_16(Regs::B, Regs::C);
        8
    }

    // INC C
    fn inc_0c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::C);
        4
    }

    // DEC C
    fn dec_0d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::C);
        4
    }

    // LD C, d8
    fn ld_0e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::C, byte);
        8
    }

    // RRCA
    fn rrca_0f(cpu: &mut Cpu) -> u8 {
        cpu.rrca();
        4
    }

    // STOP 0
    fn stop_10(_cpu: &mut Cpu) -> u8 {
        // I'm not sure how to implement this
        4
    }

    // LD DE, d16
    fn ld_11(cpu: &mut Cpu) -> u8 {
        let byte1 = cpu.fetch();
        let byte2 = cpu.fetch();
        cpu.ld_nn_d16(Regs::D, Regs::E, byte1, byte2);
        12
    }

    // LD (DE), A
    fn ld_12(cpu: &mut Cpu) -> u8 {
        let d = cpu.get_reg(Regs::D);
        let e = cpu.get_reg(Regs::E);
        let de = merge_bytes(d, e);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(de, val);
        8
    }

    // INC DE
    fn inc_13(cpu: &mut Cpu) -> u8 {
        cpu.inc_16(Regs::D, Regs::E);
        8
    }

    // INC D
    fn inc_14(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::D);
        4
    }

    // DEC D
    fn dec_15(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::D);
        4
    }

    // LD D, d8
    fn ld_16(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::D, byte);
        8
    }

    // RLA
    fn rla_17(cpu: &mut Cpu) -> u8 {
        cpu.rla();
        4
    }

    // JR r8
    fn jr_18(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        cpu.pc += offset as u16;
        12
    }

    // ADD HL, DE
    fn add_19(cpu: &mut Cpu) -> u8 {
        let high_byte = cpu.get_reg(Regs::D);
        let low_byte = cpu.get_reg(Regs::E);
        cpu.add_nn_d16(Regs::H, Regs::L, high_byte, low_byte);
        8
    }

    // LD A, (DE)
    fn ld_1a(cpu: &mut Cpu) -> u8 {
        let d = cpu.get_reg(Regs::D);
        let e = cpu.get_reg(Regs::E);
        let de = merge_bytes(d, e);
        let val = cpu.read_ram(de);
        cpu.set_reg(Regs::A, val);
        8
    }

    // DEC DE
    fn dec_1b(cpu: &mut Cpu) -> u8 {
        cpu.dec_16(Regs::D, Regs::E);
        8
    }

    // INC E
    fn inc_1c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::E);
        4
    }

    // DEC E
    fn dec_1d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::E);
        4
    }

    // LD E, d8
    fn ld_1e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::E, byte);
        8
    }

    // RRA
    fn rra_1f(cpu: &mut Cpu) -> u8 {
        cpu.rra();
        4
    }

    // JR NZ, r8
    fn jr_20(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if !cpu.get_flag(Flags::Z) {
           cpu.pc += offset as u16;
        }
        12
    }

    // LD HL, d16
    fn ld_21(cpu: &mut Cpu) -> u8 {
        let byte1 = cpu.fetch();
        let byte2 = cpu.fetch();
        cpu.ld_nn_d16(Regs::H, Regs::L, byte1, byte2);
        12
    }

    // LD (HL+), A
    fn ld_22(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let mut hl = merge_bytes(h, l);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(hl, val);
        hl += 1; // TODO: Add trait for u16
        cpu.set_reg(Regs::H, hl.get_high_byte());
        cpu.set_reg(Regs::L, hl.get_low_byte());
        8
    }

    // INC HL
    fn inc_23(cpu: &mut Cpu) -> u8 {
        cpu.inc_16(Regs::H, Regs::L);
        8
    }

    // INC H
    fn inc_24(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::H);
        4
    }

    // DEC H
    fn dec_25(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::H);
        4
    }

    // LD H, d8
    fn ld_26(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::H, byte);
        8
    }

    // DAA
    fn daa_27(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 4
    }

    // JR Z, r8
    fn jr_28(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if cpu.get_flag(Flags::Z) {
           cpu.pc += offset as u16;
           12
        } else {
            8
        }
    }

    // ADD HL, HL
    fn add_29(cpu: &mut Cpu) -> u8 {
        let high_byte = cpu.get_reg(Regs::H);
        let low_byte = cpu.get_reg(Regs::L);
        cpu.add_nn_d16(Regs::H, Regs::L, high_byte, low_byte);
        8
    }

    // LD A, (HL+)
    fn ld_2a(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let mut hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::A, val);
        hl += 1;
        cpu.set_reg(Regs::H, hl.get_high_byte());
        cpu.set_reg(Regs::L, hl.get_low_byte());
        8
    }

    // DEC HL
    fn dec_2b(cpu: &mut Cpu) -> u8 {
        cpu.dec_16(Regs::H, Regs::L);
        8
    }

    // INC L
    fn inc_2c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::L);
        4
    }

    // DEC L
    fn dec_2d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::L);
        4
    }

    // LD L, d8
    fn ld_2e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::L, byte);
        8
    }

    // CPL
    fn cpl_2f(cpu: &mut Cpu) -> u8 {
        cpu.a = !cpu.a;
        cpu.set_flag(Flags::N);
        cpu.set_flag(Flags::H);
        4
    }

    // JR NC, r8
    fn jr_30(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if !cpu.get_flag(Flags::C) {
           cpu.pc += offset as u16;
           12
        } else {
            8
        }
    }

    // LD SP, d16
    fn ld_31(cpu: &mut Cpu) -> u8 {
        let byte1 = cpu.fetch();
        let byte2 = cpu.fetch();
        cpu.sp = merge_bytes(byte1, byte2);
        12
    }

    // LD (HL-), A
    fn ld_32(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let mut hl = merge_bytes(h, l);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(hl, val);
        hl -= 1;
        cpu.set_reg(Regs::H, hl.get_high_byte());
        cpu.set_reg(Regs::L, hl.get_low_byte());
        8
    }

    // INC SP
    fn inc_33(cpu: &mut Cpu) -> u8 {
        // May need to check for flags
        cpu.sp += 1;
        8
    }

    // INC (HL)
    fn inc_34(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let mut val = cpu.read_ram(hl);
        val += 1;
        cpu.write_ram(hl, val);
        12
    }

    // DEC (HL)
    fn dec_35(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let mut val = cpu.read_ram(hl);
        val -= 1;
        cpu.write_ram(hl, val);
        12
    }

    // LD (HL), d8
    fn ld_36(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.fetch();
        cpu.write_ram(hl, val);
        12
    }

    // SCF
    fn scf_37(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(Flags::C);
        cpu.clear_flag(Flags::H);
        cpu.clear_flag(Flags::N);
        4
    }

    // JR C, r8
    fn jr_38(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if cpu.get_flag(Flags::C) {
           cpu.pc += offset as u16;
           12
        } else {
            8
        }
    }

    // ADD HL, SP
    fn add_39(cpu: &mut Cpu) -> u8 {
        let high_byte = cpu.sp.get_high_byte();
        let low_byte = cpu.sp.get_low_byte();
        cpu.add_nn_d16(Regs::H, Regs::L, high_byte, low_byte);
        8
    }

    // LD A, (HL-)
    fn ld_3a(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let mut hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::A, val);
        hl -= 1;
        cpu.set_reg(Regs::H, hl.get_high_byte());
        cpu.set_reg(Regs::L, hl.get_low_byte());
        8
    }

    // DEC SP
    fn dec_3b(cpu: &mut Cpu) -> u8 {
        cpu.sp -= 1;
        8
    }

    // INC A
    fn inc_3c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::A);
        4
    }

    // DEC A
    fn dec_3d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::A);
        4
    }

    // LD A, d8
    fn ld_3e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::A, byte);
        8
    }

    // CCF
    fn ccf_3f(cpu: &mut Cpu) -> u8 {
        cpu.clear_flag(Flags::N);
        cpu.clear_flag(Flags::H);
        let cf = cpu.get_flag(Flags::C);
        cpu.write_flag(Flags::C, !cf);
        4
    }

    // LD B, B
    fn ld_40(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD B, C
    fn ld_41(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD B, D
    fn ld_42(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD B, E
    fn ld_43(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD B, H
    fn ld_44(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD B, L
    fn ld_45(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD B, (HL)
    fn ld_46(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::B, val);
        8
    }

    // LD B, A
    fn ld_47(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    // LD C, B
    fn ld_48(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD C, C
    fn ld_49(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD C, D
    fn ld_4a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD C, E
    fn ld_4b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD C, H
    fn ld_4c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD C, L
    fn ld_4d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD C, (HL)
    fn ld_4e(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::C, val);
        8
    }

    // LD C, A
    fn ld_4f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    // LD D, B
    fn ld_50(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD D, C
    fn ld_51(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD D, D
    fn ld_52(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD D, E
    fn ld_53(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD D, H
    fn ld_54(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD D, L
    fn ld_55(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD D, (HL)
    fn ld_56(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::D, val);
        8
    }

    // LD D, A
    fn ld_57(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    // LD E, B
    fn ld_58(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD E, C
    fn ld_59(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD E, D
    fn ld_5a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD E, E
    fn ld_5b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD E, H
    fn ld_5c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD E, L
    fn ld_5d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD E, (HL)
    fn ld_5e(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::E, val);
        8
    }

    // LD E, A
    fn ld_5f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    // LD H, B
    fn ld_60(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD H, C
    fn ld_61(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD H, D
    fn ld_62(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD H, E
    fn ld_63(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD H, H
    fn ld_64(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD H, L
    fn ld_65(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD H, (HL)
    fn ld_66(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::H, val);
        8
    }

    // LD H, A
    fn ld_67(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    // LD L, B
    fn ld_68(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD L, C
    fn ld_69(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD L, D
    fn ld_6a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD L, E
    fn ld_6b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD L, H
    fn ld_6c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD L, L
    fn ld_6d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD L, (HL)
    fn ld_6e(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::L, val);
        8
    }

    // LD L, A
    fn ld_6f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    // LD (HL), B
    fn ld_70(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // LD (HL), C
    fn ld_71(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // LD (HL), D
    fn ld_72(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // LD (HL), E
    fn ld_73(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // LD (HL), H
    fn ld_74(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // LD (HL), L
    fn ld_75(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // HALT
    fn halt_76(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 8
    }

    // LD (HL), A
    fn ld_77(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        cpu.write_ram(hl, val);
        8
    }

    // LD A, B
    fn ld_78(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // LD A, C
    fn ld_79(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // LD A, D
    fn ld_7a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // LD A, E
    fn ld_7b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // LD A, H
    fn ld_7c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // LD A, L
    fn ld_7d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // LD A, (HL)
    fn ld_7e(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::A, val);
        8
    }

    // LD A, A
    fn ld_7f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    // ADD A, B
    fn add_80(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.add_a_d8(val, false);
        4
    }

    // ADD A, C
    fn add_81(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.add_a_d8(val, false);
        4
    }

    // ADD A, D
    fn add_82(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.add_a_d8(val, false);
        4
    }

    // ADD A, E
    fn add_83(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.add_a_d8(val, false);
        4
    }

    // ADD A, H
    fn add_84(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.add_a_d8(val, false);
        4
    }

    // ADD A, L
    fn add_85(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.add_a_d8(val, false);
        4
    }

    // ADD A, (HL)
    fn add_86(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.add_a_d8(val, false);
        8
    }

    // ADD A, A
    fn add_87(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.add_a_d8(val, false);
        4
    }

    // ADC A, B
    fn adc_88(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.add_a_d8(val, true);
        4
    }

    // ADC A, C
    fn adc_89(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.add_a_d8(val, true);
        4
    }

    // ADC A, D
    fn adc_8a(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.add_a_d8(val, true);
        4
    }

    // ADC A, E
    fn adc_8b(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.add_a_d8(val, true);
        4
    }

    // ADC A, H
    fn adc_8c(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.add_a_d8(val, true);
        4
    }

    // ADC A, L
    fn adc_8d(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.add_a_d8(val, true);
        4
    }

    // ADC A, (HL)
    fn adc_8e(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.add_a_d8(val, true);
        8
    }

    // ADC A, A
    fn adc_8f(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.add_a_d8(val, true);
        4
    }

    // SUB B
    fn sub_90(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.sub_a_d8(val, false);
        4
    }

    // SUB C
    fn sub_91(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.sub_a_d8(val, false);
        4
    }

    // SUB D
    fn sub_92(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.sub_a_d8(val, false);
        4
    }

    // SUB E
    fn sub_93(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.sub_a_d8(val, false);
        4
    }

    // SUB H
    fn sub_94(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.sub_a_d8(val, false);
        4
    }

    // SUB L
    fn sub_95(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.sub_a_d8(val, false);
        4
    }

    // SUB (HL)
    fn sub_96(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.sub_a_d8(val, false);
        8
    }

    // SUB A
    fn sub_97(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.sub_a_d8(val, false);
        4
    }

    // SBC A, B
    fn sbc_98(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.sub_a_d8(val, true);
        4
    }

    // SBC A, C
    fn sbc_99(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.sub_a_d8(val, true);
        4
    }

    // SBC A, D
    fn sbc_9a(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.sub_a_d8(val, true);
        4
    }

    // SBC A, E
    fn sbc_9b(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.sub_a_d8(val, true);
        4
    }

    // SBC A, H
    fn sbc_9c(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.sub_a_d8(val, true);
        4
    }

    // SBC A, L
    fn sbc_9d(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.sub_a_d8(val, true);
        4
    }

    // SBC A, (HL)
    fn sbc_9e(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.sub_a_d8(val, true);
        8
    }

    // SBC A, A
    fn sbc_9f(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.sub_a_d8(val, true);
        4
    }

    // AND B
    fn and_a0(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.and_a_d8(val);
        4
    }

    // AND C
    fn and_a1(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.and_a_d8(val);
        4
    }

    // AND D
    fn and_a2(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.and_a_d8(val);
        4
    }

    // AND E
    fn and_a3(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.and_a_d8(val);
        4
    }

    // AND H
    fn and_a4(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.and_a_d8(val);
        4
    }

    // AND L
    fn and_a5(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.and_a_d8(val);
        4
    }

    // AND (HL)
    fn and_a6(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.and_a_d8(val);
        8
    }

    // AND A
    fn and_a7(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.and_a_d8(val);
        4
    }

    // XOR B
    fn xor_a8(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.xor_a_d8(val);
        4
    }

    // XOR C
    fn xor_a9(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.xor_a_d8(val);
        4
    }

    // XOR D
    fn xor_aa(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.xor_a_d8(val);
        4
    }

    // XOR E
    fn xor_ab(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.xor_a_d8(val);
        4
    }

    // XOR H
    fn xor_ac(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.xor_a_d8(val);
        4
    }

    // XOR L
    fn xor_ad(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.xor_a_d8(val);
        4
    }

    // XOR (HL)
    fn xor_ae(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.xor_a_d8(val);
        8
    }

    // XOR A
    fn xor_af(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.xor_a_d8(val);
        4
    }

    // OR B
    fn or_b0(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.or_a_d8(val);
        4
    }

    // OR C
    fn or_b1(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.or_a_d8(val);
        4
    }

    // OR D
    fn or_b2(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.or_a_d8(val);
        4
    }

    // OR E
    fn or_b3(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.or_a_d8(val);
        4
    }

    // OR H
    fn or_b4(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.or_a_d8(val);
        4
    }

    // OR L
    fn or_b5(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.or_a_d8(val);
        4
    }

    // OR (HL)
    fn or_b6(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.or_a_d8(val);
        8
    }

    // OR A
    fn or_b7(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.or_a_d8(val);
        4
    }

    // CP B
    fn cp_b8(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.cp_a_d8(val);
        4
    }

    // CP C
    fn cp_b9(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.cp_a_d8(val);
        4
    }

    // CP D
    fn cp_ba(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.cp_a_d8(val);
        4
    }

    // CP E
    fn cp_bb(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.cp_a_d8(val);
        4
    }

    // CP H
    fn cp_bc(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.cp_a_d8(val);
        4
    }

    // CP L
    fn cp_bd(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.cp_a_d8(val);
        4
    }

    // CP (HL)
    fn cp_be(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.cp_a_d8(val);
        8
    }

    // CP A
    fn cp_bf(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.cp_a_d8(val);
        4
    }

    // RET NZ
    fn ret_c0(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 20 // Or 8?
    }

    // POP BC
    fn pop_c1(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // JP NZ, a16
    fn jp_c2(cpu: &mut Cpu) -> u8 {
        // Is this the right order?
        let low = cpu.fetch();
        let high = cpu.fetch();
        let offset = merge_bytes(high, low);
        if !cpu.get_flag(Flags::Z) {
            cpu.pc = offset;
            16
        } else {
            12
        }
    }

    // JP a16
    fn jp_c3(cpu: &mut Cpu) -> u8 {
        let low = cpu.fetch();
        let high = cpu.fetch();
        let offset = merge_bytes(high, low);
        cpu.pc = offset;
        16
    }

    // CALL NZ, a16
    fn call_c4(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 24 // Or 12?
    }

    // PUSH BC
    fn push_c5(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // ADD A, d8
    fn add_c6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.add_a_d8(val, false);
        8
    }

    // RST 00H
    fn rst_c7(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // RET Z
    fn ret_c8(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 20 // Or 8?
    }

    // RET
    fn ret_c9(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // JP Z, a16
    fn jp_ca(cpu: &mut Cpu) -> u8 {
        let low = cpu.fetch();
        let high = cpu.fetch();
        let offset = merge_bytes(high, low);
        if cpu.get_flag(Flags::Z) {
            cpu.pc = offset;
            16
        } else {
            12
        }
    }

    // PREFIX CB
    fn prefix_cb(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 4
    }

    // CALL Z, a16
    fn call_cc(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 24 // Or 12?
    }

    // CALL a16
    fn call_cd(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 24
    }

    // ADC A, d8
    fn adc_ce(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.add_a_d8(val, true);
        8
    }

    // RST 08H
    fn rst_cf(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // RET NC
    fn ret_d0(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 20 // Or 8?
    }

    // POP DE
    fn pop_d1(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // JP NC, a16
    fn jp_d2(cpu: &mut Cpu) -> u8 {
        let low = cpu.fetch();
        let high = cpu.fetch();
        let offset = merge_bytes(high, low);
        if !cpu.get_flag(Flags::C) {
            cpu.pc = offset;
            16
        } else {
            12
        }
    }

    // CALL NC, a16
    fn call_d4(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 24 // Or 12?
    }

    // PUSH DE
    fn push_d5(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // SUB d8
    fn sub_d6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.sub_a_d8(val, false);
        8
    }

    // RST 10H
    fn rst_d7(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // RET C
    fn ret_d8(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 20 // Or 8?
    }

    // RETI
    fn reti_d9(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // JP C, a16
    fn jp_da(cpu: &mut Cpu) -> u8 {
        let low = cpu.fetch();
        let high = cpu.fetch();
        let offset = merge_bytes(high, low);
        if cpu.get_flag(Flags::C) {
            cpu.pc = offset;
            16
        } else {
            12
        }
    }

    // CALL C, a16
    fn call_dc(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 24 // Or 12?
    }

    // SBC A, d8
    fn sbc_de(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.sub_a_d8(val, true);
        8
    }

    // RST 18H
    fn rst_df(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // LDH (a8), A
    fn ldh_e0(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // POP HL
    fn pop_e1(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // LD (C), A
    fn ld_e2(cpu: &mut Cpu) -> u8 {
        let c = cpu.get_reg(Regs::C);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(c as u16, val);
        8
    }

    // PUSH HL
    fn push_e5(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // AND d8
    fn and_e6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.and_a_d8(val);
        8
    }

    // RST 20H
    fn rst_e7(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // ADD SP, r8
    fn add_e8(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // JP (HL)
    fn jp_e9(cpu: &mut Cpu) -> u8 {
        let h = cpu.get_reg(Regs::H);
        let l = cpu.get_reg(Regs::L);
        let hl = merge_bytes(h, l);
        let val = cpu.read_ram(hl);
        cpu.pc = val as u16;
        4
    }

    // LD (a16), A
    fn ld_ea(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // XOR d8
    fn xor_ee(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.xor_a_d8(val);
        8
    }

    // RST 28H
    fn rst_ef(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // LDH A, (a8)
    fn ldh_f0(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // POP AF
    fn pop_f1(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // LD A, (C)
    fn ld_f2(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 8
    }

    // DI
    fn di_f3(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 4
    }

    // PUSH AF
    fn push_f5(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // OR d8
    fn or_f6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.or_a_d8(val);
        8
    }

    // RST 30H
    fn rst_f7(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // LD HL, SP+r8
    fn ld_f8(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 12
    }

    // LD SP, HL
    fn ld_f9(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 8
    }

    // LD A, (a16)
    fn ld_fa(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }

    // EI
    fn ei_fb(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 4
    }

    // CP d8
    fn cp_fe(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.cp_a_d8(val);
        8
    }

    // RST 38H
    fn rst_ff(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 16
    }
}
