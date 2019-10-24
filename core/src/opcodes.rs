// Borrowed some of the implementation from here: https://github.com/blackxparade/Rust-Boy/blob/master/Emulator/src/cpu/opcode.rs
use crate::alu::*;
use crate::cpu::*;

pub struct Opcode {
    pub op: [fn(&mut Cpu) -> u8; 256]
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            op: [Opcode::default; 256]
        }
    }

    pub fn execute(self, cpu: &mut Cpu, opcode: u8) -> u8 {
        cpu.pc += 1;
        self.op[opcode as usize](&mut cpu)
    }

    pub fn fetch(cpu: &mut Cpu) -> u8 {
        let val = cpu.ram[cpu.pc as usize];
        cpu.pc += 1;
        val
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
        self.op[0x0F] = Opcode::rrca;
        self.op[0x10] = Opcode::stop;
        self.op[0x11] = Opcode::ld_11;
        self.op[0x12] = Opcode::ld_12;
        self.op[0x13] = Opcode::inc_13;
        self.op[0x14] = Opcode::inc_14;
        self.op[0x15] = Opcode::dec_15;
        self.op[0x16] = Opcode::ld_16;
        self.op[0x17] = Opcode::rla;
        self.op[0x18] = Opcode::jr_18;
        self.op[0x19] = Opcode::add_19;
        self.op[0x1A] = Opcode::ld_1a;
        self.op[0x1B] = Opcode::dec_1b;
        self.op[0x1C] = Opcode::inc_1c;
        self.op[0x1D] = Opcode::dec_1d;
        self.op[0x1E] = Opcode::ld_1e;
        self.op[0x1F] = Opcode::rra;
        self.op[0x20] = Opcode::jr_20;
        self.op[0x21] = Opcode::ld_21;
        self.op[0x22] = Opcode::ld_22;
        self.op[0x23] = Opcode::inc_23;
        self.op[0x24] = Opcode::inc_24;
        self.op[0x25] = Opcode::dec_25;
        self.op[0x26] = Opcode::ld_26;
        self.op[0x27] = Opcode::daa;
        self.op[0x28] = Opcode::jr_28;
        self.op[0x29] = Opcode::add_29;
        self.op[0x2A] = Opcode::ld_2a;
        self.op[0x2B] = Opcode::dec_2b;
        self.op[0x2C] = Opcode::inc_2c;
        self.op[0x2D] = Opcode::dec_2d;
        self.op[0x2E] = Opcode::ld_2e;
        self.op[0x2F] = Opcode::cpl;
        self.op[0x30] = Opcode::jr_30;
        self.op[0x31] = Opcode::ld_31;
        self.op[0x32] = Opcode::ld_32;
        self.op[0x33] = Opcode::inc_33;
        self.op[0x34] = Opcode::inc_34;
        self.op[0x35] = Opcode::dec_35;
        self.op[0x36] = Opcode::ld_36;
        self.op[0x37] = Opcode::scf;
        self.op[0x38] = Opcode::jr_38;
        self.op[0x39] = Opcode::add_39;
        self.op[0x3A] = Opcode::ld_3a;
        self.op[0x3B] = Opcode::dec_3b;
        self.op[0x3C] = Opcode::inc_3c;
        self.op[0x3D] = Opcode::dec_3d;
        self.op[0x3E] = Opcode::ld_3e;
        self.op[0x3F] = Opcode::ccf;
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
        self.op[0x76] = Opcode::halt;
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
        self.op[0xB8] = Opcode::or_b8;
        self.op[0xB9] = Opcode::or_b9;
        self.op[0xBA] = Opcode::or_ba;
        self.op[0xBB] = Opcode::or_bb;
        self.op[0xBC] = Opcode::or_bc;
        self.op[0xBD] = Opcode::or_bd;
        self.op[0xBE] = Opcode::or_be;
        self.op[0xBF] = Opcode::or_bf;
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
        self.op[0xCB] = Opcode::prefix;
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
        self.op[0xF3] = Opcode::di;
        self.op[0xF4] = Opcode::invalid;
        self.op[0xF5] = Opcode::push_f5;
        self.op[0xF6] = Opcode::or_f6;
        self.op[0xF7] = Opcode::rst_f7;
        self.op[0xF8] = Opcode::ld_f8;
        self.op[0xF9] = Opcode::ld_f9;
        self.op[0xFA] = Opcode::ld_fa;
        self.op[0xFB] = Opcode::ei;
        self.op[0xFC] = Opcode::invalid;
        self.op[0xFD] = Opcode::invalid;
        self.op[0xFE] = Opcode::cp_fe;
        self.op[0xFF] = Opcode::rst_ff;
    }

    fn default(cpu: &mut Cpu) -> u8 {
        0
    }

    // NOP
    fn nop(cpu: &mut Cpu) -> u8 {
        4
    }

    // LD BC, d16
    fn ld_01(cpu: &mut Cpu) -> u8 {
        let byte1 = Opcode::fetch(&mut cpu);
        let byte2 = Opcode::fetch(&mut cpu);
        ld_nn_d16(&mut cpu.b, &mut cpu.c, byte1, byte2);
        12
    }

    // LD (BC), A
    fn ld_02(cpu: &mut Cpu) -> u8 {

    }

    // INC BC
    fn inc_03(cpu: &mut Cpu) -> u8 {
        inc_16(&mut cpu.b, &mut cpu.c);
        8
    }

    // INC B
    fn inc_04(cpu: &mut Cpu) -> u8 {
        inc_8(&mut cpu, &mut cpu.b);
        4
    }

    // DEC B
    fn dec_05(cpu: &mut Cpu) -> u8 {
        dec_8(&mut cpu, &mut cpu.b);
        4
    }

    // LD B, d8
    fn ld_06(cpu: &mut Cpu) -> u8 {
        let byte = Opcode::fetch(&mut cpu);
        ld_n_d8(&mut cpu.b, byte);
        8
    }

    // RLCA
    fn rlca_07(cpu: &mut Cpu) -> u8 {
        rlca(&mut cpu);
        4
    }

    // LD (a16), SP
    fn ld_08(cpu: &mut Cpu) -> u8 {

        20
    }

    // ADD HL, BC
    fn add_09(cpu: &mut Cpu) -> u8 {
        add_16(&mut cpu, &mut cpu.h, &mut cpu.l, cpu.b, cpu.c);
        8
    }

    // DEC BC
    fn dec_0b(cpu: &mut Cpu) -> u8 {
        dec_16(&mut cpu.b, &mut cpu.c);
        8
    }

    // INC C
    fn inc_0c(cpu: &mut Cpu) -> u8 {
        inc_8(&mut cpu, &mut cpu.c);
        4
    }

    // DEC C
    fn dec_0d(cpu: &mut Cpu) -> u8 {
        dec_8(&mut cpu, &mut cpu.c);
        4
    }

    // LD DE, d16
    fn ld_11(cpu: &mut Cpu) -> u8 {
        let byte1 = Opcode::fetch(&mut cpu);
        let byte2 = Opcode::fetch(&mut cpu);
        ld_nn_d16(&mut cpu.d, &mut cpu.e, byte1, byte2);
        12
    }

    // INC DE
    fn inc_13(cpu: &mut Cpu) -> u8 {
        inc_16(&mut cpu.d, &mut cpu.e);
        8
    }

    // ADD HL, DE
    fn add_19(cpu: &mut Cpu) -> u8 {
        add_16(&mut cpu, &mut cpu.h, &mut cpu.l, cpu.d, cpu.e);
        8
    }

    // DEC DE
    fn dec_1b(cpu: &mut Cpu) -> u8 {
        dec_16(&mut cpu.d, &mut cpu.e);
        8
    }

    // INC E
    fn inc_1c(cpu: &mut Cpu) -> u8 {
        inc_8(&mut cpu, &mut cpu.e);
        4
    }

    // DEC E
    fn dec_1d(cpu: &mut Cpu) -> u8 {
        dec_8(&mut cpu, &mut cpu.e);
        4
    }

    // LD HL, d16
    fn ld_21(cpu: &mut Cpu) -> u8 {
        let byte1 = Opcode::fetch(&mut cpu);
        let byte2 = Opcode::fetch(&mut cpu);
        ld_nn_d16(&mut cpu.h, &mut cpu.l, byte1, byte2);
        12
    }

    // ADD HL, HL
    fn add_29(cpu: &mut Cpu) -> u8 {
        add_16(&mut cpu, &mut cpu.h, &mut cpu.l, cpu.h, cpu.l);
        8
    }

    // DEC HL
    fn dec_2b(cpu: &mut Cpu) -> u8 {
        dec_16(&mut cpu.h, &mut cpu.l);
        8
    }

    // INC L
    fn inc_2c(cpu: &mut Cpu) -> u8 {
        inc_8(&mut cpu, &mut cpu.l);
        4
    }

    // DEC L
    fn dec_2d(cpu: &mut Cpu) -> u8 {
        dec_8(&mut cpu, &mut cpu.l);
        4
    }

    // LD SP, d16
    fn ld_31(cpu: &mut Cpu) -> u8 {
        let byte1 = Opcode::fetch(&mut cpu);
        let byte2 = Opcode::fetch(&mut cpu);
        cpu.sp = merge_bytes(byte1, byte2);
        12
    }

    // ADD HL, SP
    fn add_39(cpu: &mut Cpu) -> u8 {
        add_16(&mut cpu, &mut cpu.h, &mut cpu.l, cpu.sp.get_high_byte(), cpu.sp.get_low_byte());
        8
    }

    // DEC SP
    fn dec_3b(cpu: &mut Cpu) -> u8 {
        cpu.sp -= 1;
        8
    }

    // INC A
    fn inc_3c(cpu: &mut Cpu) -> u8 {
        inc_8(&mut cpu, &mut cpu.a);
        4
    }

    // DEC A
    fn dec_3d(cpu: &mut Cpu) -> u8 {
        dec_8(&mut cpu, &mut cpu.a);
        4
    }

    // LD B, B
    fn ld_40(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.b);
        4
    }

    // LD B, C
    fn ld_41(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.c);
        4
    }

    // LD B, D
    fn ld_42(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.d);
        4
    }

    // LD B, E
    fn ld_43(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.e);
        4
    }

    // LD B, H
    fn ld_44(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.h);
        4
    }

    // LD B, L
    fn ld_45(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.l);
        4
    }

    // LD B, A
    fn ld_47(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.b, cpu.a);
        4
    }

    // LD C, B
    fn ld_48(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.b);
        4
    }

    // LD C, C
    fn ld_49(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.c);
        4
    }

    // LD C, D
    fn ld_4a(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.d);
        4
    }

    // LD C, E
    fn ld_4b(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.e);
        4
    }

    // LD C, H
    fn ld_4c(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.h);
        4
    }

    // LD C, L
    fn ld_4d(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.l);
        4
    }

    // LD C, A
    fn ld_4f(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.c, cpu.a);
        4
    }

    // LD D, B
    fn ld_50(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.b);
        4
    }

    // LD D, C
    fn ld_51(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.c);
        4
    }

    // LD D, D
    fn ld_52(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.d);
        4
    }

    // LD D, E
    fn ld_53(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.e);
        4
    }

    // LD D, H
    fn ld_54(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.h);
        4
    }

    // LD D, L
    fn ld_55(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.l);
        4
    }

    // LD D, A
    fn ld_57(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.d, cpu.a);
        4
    }

    // LD E, B
    fn ld_58(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.b);
        4
    }

    // LD E, C
    fn ld_59(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.c);
        4
    }

    // LD E, D
    fn ld_5a(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.d);
        4
    }

    // LD E, E
    fn ld_5b(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.e);
        4
    }

    // LD E, H
    fn ld_5c(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.h);
        4
    }

    // LD E, L
    fn ld_5d(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.l);
        4
    }

    // LD E, A
    fn ld_5f(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.e, cpu.a);
        4
    }

    // LD H, B
    fn ld_60(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.b);
        4
    }

    // LD H, C
    fn ld_61(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.c);
        4
    }

    // LD H, D
    fn ld_62(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.d);
        4
    }

    // LD H, E
    fn ld_63(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.e);
        4
    }

    // LD H, H
    fn ld_64(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.h);
        4
    }

    // LD H, L
    fn ld_65(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.l);
        4
    }

    // LD H, A
    fn ld_67(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.h, cpu.a);
        4
    }

    // LD L, B
    fn ld_68(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.b);
        4
    }

    // LD L, C
    fn ld_69(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.c);
        4
    }

    // LD L, D
    fn ld_6a(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.d);
        4
    }

    // LD L, E
    fn ld_6b(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.e);
        4
    }

    // LD L, H
    fn ld_6c(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.h);
        4
    }

    // LD L, L
    fn ld_6d(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.l);
        4
    }

    // LD L, A
    fn ld_6f(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.l, cpu.a);
        4
    }

    // LD A, B
    fn ld_78(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.b);
        4
    }

    // LD A, C
    fn ld_79(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.c);
        4
    }

    // LD A, D
    fn ld_7a(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.d);
        4
    }

    // LD A, E
    fn ld_7b(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.e);
        4
    }

    // LD A, H
    fn ld_7c(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.h);
        4
    }

    // LD A, L
    fn ld_7d(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.l);
        4
    }

    // LD A, A
    fn ld_7f(cpu: &mut Cpu) -> u8 {
        ld_n_d8(&mut cpu.a, cpu.a);
        4
    }

    // ADD A, B
    fn add_80(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.b, false);
        4
    }

    // ADD A, C
    fn add_81(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.c, false);
        4
    }

    // ADD A, D
    fn add_82(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.d, false);
        4
    }

    // ADD A, E
    fn add_83(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.e, false);
        4
    }

    // ADD A, H
    fn add_84(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.h, false);
        4
    }

    // ADD A, L
    fn add_85(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.l, false);
        4
    }

    // ADD A, A
    fn add_87(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.a, false);
        4
    }

    // ADC A, B
    fn add_88(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.b, true);
        4
    }

    // ADC A, C
    fn add_89(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.c, true);
        4
    }

    // ADC A, D
    fn add_8a(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.d, true);
        4
    }

    // ADC A, E
    fn add_8b(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.e, true);
        4
    }

    // ADC A, H
    fn add_8c(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.h, true);
        4
    }

    // ADC A, L
    fn add_8d(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.l, true);
        4
    }

    // ADC A, A
    fn add_8f(cpu: &mut Cpu) -> u8 {
        add_8(&mut cpu, &mut cpu.a, cpu.a, true);
        4
    }
}
