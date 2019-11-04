use crate::cpu::*;
use crate::utils::*;

pub struct Opcode {
    op: [fn(&mut Cpu) -> u8; 256],
    cbop: [fn(&mut Cpu) -> u8; 256]
}

impl Opcode {
    pub fn new() -> Opcode {
        Opcode {
            op: [Opcode::invalid; 256],
            cbop: [Opcode::invalid; 256]
        }
    }

    pub fn execute(self, cpu: &mut Cpu) -> u8 {
        let opcode = cpu.fetch();
        // If opcode is $CB, then use other opcode table
        if opcode == 0xcb {
            self.cbop[opcode as usize](cpu)
        } else {
            self.op[opcode as usize](cpu)
        }
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

        self.cbop[0x00] = Opcode::rlc_00;
        self.cbop[0x01] = Opcode::rlc_01;
        self.cbop[0x02] = Opcode::rlc_02;
        self.cbop[0x03] = Opcode::rlc_03;
        self.cbop[0x04] = Opcode::rlc_04;
        self.cbop[0x05] = Opcode::rlc_05;
        self.cbop[0x06] = Opcode::rlc_06;
        self.cbop[0x07] = Opcode::rlc_07;
        self.cbop[0x08] = Opcode::rrc_08;
        self.cbop[0x09] = Opcode::rrc_09;
        self.cbop[0x0a] = Opcode::rrc_0a;
        self.cbop[0x0b] = Opcode::rrc_0b;
        self.cbop[0x0c] = Opcode::rrc_0c;
        self.cbop[0x0d] = Opcode::rrc_0d;
        self.cbop[0x0e] = Opcode::rrc_0e;
        self.cbop[0x0f] = Opcode::rrc_0f;
        self.cbop[0x10] = Opcode::rl_10;
        self.cbop[0x11] = Opcode::rl_11;
        self.cbop[0x12] = Opcode::rl_12;
        self.cbop[0x13] = Opcode::rl_13;
        self.cbop[0x14] = Opcode::rl_14;
        self.cbop[0x15] = Opcode::rl_15;
        self.cbop[0x16] = Opcode::rl_16;
        self.cbop[0x17] = Opcode::rl_17;
        self.cbop[0x18] = Opcode::rr_18;
        self.cbop[0x19] = Opcode::rr_19;
        self.cbop[0x1a] = Opcode::rr_1a;
        self.cbop[0x1b] = Opcode::rr_1b;
        self.cbop[0x1c] = Opcode::rr_1c;
        self.cbop[0x1d] = Opcode::rr_1d;
        self.cbop[0x1e] = Opcode::rr_1e;
        self.cbop[0x1f] = Opcode::rr_1f;
        self.cbop[0x20] = Opcode::sla_20;
        self.cbop[0x21] = Opcode::sla_21;
        self.cbop[0x22] = Opcode::sla_22;
        self.cbop[0x23] = Opcode::sla_23;
        self.cbop[0x24] = Opcode::sla_24;
        self.cbop[0x25] = Opcode::sla_25;
        self.cbop[0x26] = Opcode::sla_26;
        self.cbop[0x27] = Opcode::sla_27;
        self.cbop[0x28] = Opcode::sra_28;
        self.cbop[0x29] = Opcode::sra_29;
        self.cbop[0x2a] = Opcode::sra_2a;
        self.cbop[0x2b] = Opcode::sra_2b;
        self.cbop[0x2c] = Opcode::sra_2c;
        self.cbop[0x2d] = Opcode::sra_2d;
        self.cbop[0x2e] = Opcode::sra_2e;
        self.cbop[0x2f] = Opcode::sra_2f;
        self.cbop[0x30] = Opcode::swap_30;
        self.cbop[0x31] = Opcode::swap_31;
        self.cbop[0x32] = Opcode::swap_32;
        self.cbop[0x33] = Opcode::swap_33;
        self.cbop[0x34] = Opcode::swap_34;
        self.cbop[0x35] = Opcode::swap_35;
        self.cbop[0x36] = Opcode::swap_36;
        self.cbop[0x37] = Opcode::swap_37;
        self.cbop[0x38] = Opcode::srl_38;
        self.cbop[0x39] = Opcode::srl_39;
        self.cbop[0x3a] = Opcode::srl_3a;
        self.cbop[0x3b] = Opcode::srl_3b;
        self.cbop[0x3c] = Opcode::srl_3c;
        self.cbop[0x3d] = Opcode::srl_3d;
        self.cbop[0x3e] = Opcode::srl_3e;
        self.cbop[0x3f] = Opcode::srl_3f;
        self.cbop[0x40] = Opcode::bit_40;
        self.cbop[0x41] = Opcode::bit_41;
        self.cbop[0x42] = Opcode::bit_42;
        self.cbop[0x43] = Opcode::bit_43;
        self.cbop[0x44] = Opcode::bit_44;
        self.cbop[0x45] = Opcode::bit_45;
        self.cbop[0x46] = Opcode::bit_46;
        self.cbop[0x47] = Opcode::bit_47;
        self.cbop[0x48] = Opcode::bit_48;
        self.cbop[0x49] = Opcode::bit_49;
        self.cbop[0x4a] = Opcode::bit_4a;
        self.cbop[0x4b] = Opcode::bit_4b;
        self.cbop[0x4c] = Opcode::bit_4c;
        self.cbop[0x4d] = Opcode::bit_4d;
        self.cbop[0x4e] = Opcode::bit_4e;
        self.cbop[0x4f] = Opcode::bit_4f;
        self.cbop[0x50] = Opcode::bit_50;
        self.cbop[0x51] = Opcode::bit_51;
        self.cbop[0x52] = Opcode::bit_52;
        self.cbop[0x53] = Opcode::bit_53;
        self.cbop[0x54] = Opcode::bit_54;
        self.cbop[0x55] = Opcode::bit_55;
        self.cbop[0x56] = Opcode::bit_56;
        self.cbop[0x57] = Opcode::bit_57;
        self.cbop[0x58] = Opcode::bit_58;
        self.cbop[0x59] = Opcode::bit_59;
        self.cbop[0x5a] = Opcode::bit_5a;
        self.cbop[0x5b] = Opcode::bit_5b;
        self.cbop[0x5c] = Opcode::bit_5c;
        self.cbop[0x5d] = Opcode::bit_5d;
        self.cbop[0x5e] = Opcode::bit_5e;
        self.cbop[0x5f] = Opcode::bit_5f;
        self.cbop[0x60] = Opcode::bit_60;
        self.cbop[0x61] = Opcode::bit_61;
        self.cbop[0x62] = Opcode::bit_62;
        self.cbop[0x63] = Opcode::bit_63;
        self.cbop[0x64] = Opcode::bit_64;
        self.cbop[0x65] = Opcode::bit_65;
        self.cbop[0x66] = Opcode::bit_66;
        self.cbop[0x67] = Opcode::bit_67;
        self.cbop[0x68] = Opcode::bit_68;
        self.cbop[0x69] = Opcode::bit_69;
        self.cbop[0x6a] = Opcode::bit_6a;
        self.cbop[0x6b] = Opcode::bit_6b;
        self.cbop[0x6c] = Opcode::bit_6c;
        self.cbop[0x6d] = Opcode::bit_6d;
        self.cbop[0x6e] = Opcode::bit_6e;
        self.cbop[0x6f] = Opcode::bit_6f;
        self.cbop[0x70] = Opcode::bit_70;
        self.cbop[0x71] = Opcode::bit_71;
        self.cbop[0x72] = Opcode::bit_72;
        self.cbop[0x73] = Opcode::bit_73;
        self.cbop[0x74] = Opcode::bit_74;
        self.cbop[0x75] = Opcode::bit_75;
        self.cbop[0x76] = Opcode::bit_76;
        self.cbop[0x77] = Opcode::bit_77;
        self.cbop[0x78] = Opcode::bit_78;
        self.cbop[0x79] = Opcode::bit_79;
        self.cbop[0x7a] = Opcode::bit_7a;
        self.cbop[0x7b] = Opcode::bit_7b;
        self.cbop[0x7c] = Opcode::bit_7c;
        self.cbop[0x7d] = Opcode::bit_7d;
        self.cbop[0x7e] = Opcode::bit_7e;
        self.cbop[0x7f] = Opcode::bit_7f;
        self.cbop[0x80] = Opcode::res_80;
        self.cbop[0x81] = Opcode::res_81;
        self.cbop[0x82] = Opcode::res_82;
        self.cbop[0x83] = Opcode::res_83;
        self.cbop[0x84] = Opcode::res_84;
        self.cbop[0x85] = Opcode::res_85;
        self.cbop[0x86] = Opcode::res_86;
        self.cbop[0x87] = Opcode::res_87;
        self.cbop[0x88] = Opcode::res_88;
        self.cbop[0x89] = Opcode::res_89;
        self.cbop[0x8a] = Opcode::res_8a;
        self.cbop[0x8b] = Opcode::res_8b;
        self.cbop[0x8c] = Opcode::res_8c;
        self.cbop[0x8d] = Opcode::res_8d;
        self.cbop[0x8e] = Opcode::res_8e;
        self.cbop[0x8f] = Opcode::res_8f;
        self.cbop[0x90] = Opcode::res_90;
        self.cbop[0x91] = Opcode::res_91;
        self.cbop[0x92] = Opcode::res_92;
        self.cbop[0x93] = Opcode::res_93;
        self.cbop[0x94] = Opcode::res_94;
        self.cbop[0x95] = Opcode::res_95;
        self.cbop[0x96] = Opcode::res_96;
        self.cbop[0x97] = Opcode::res_97;
        self.cbop[0x98] = Opcode::res_98;
        self.cbop[0x99] = Opcode::res_99;
        self.cbop[0x9a] = Opcode::res_9a;
        self.cbop[0x9b] = Opcode::res_9b;
        self.cbop[0x9c] = Opcode::res_9c;
        self.cbop[0x9d] = Opcode::res_9d;
        self.cbop[0x9e] = Opcode::res_9e;
        self.cbop[0x9f] = Opcode::res_9f;
        self.cbop[0xa0] = Opcode::res_a0;
        self.cbop[0xa1] = Opcode::res_a1;
        self.cbop[0xa2] = Opcode::res_a2;
        self.cbop[0xa3] = Opcode::res_a3;
        self.cbop[0xa4] = Opcode::res_a4;
        self.cbop[0xa5] = Opcode::res_a5;
        self.cbop[0xa6] = Opcode::res_a6;
        self.cbop[0xa7] = Opcode::res_a7;
        self.cbop[0xa8] = Opcode::res_a8;
        self.cbop[0xa9] = Opcode::res_a9;
        self.cbop[0xaa] = Opcode::res_aa;
        self.cbop[0xab] = Opcode::res_ab;
        self.cbop[0xac] = Opcode::res_ac;
        self.cbop[0xad] = Opcode::res_ad;
        self.cbop[0xae] = Opcode::res_ae;
        self.cbop[0xaf] = Opcode::res_af;
        self.cbop[0xb0] = Opcode::res_b0;
        self.cbop[0xb1] = Opcode::res_b1;
        self.cbop[0xb2] = Opcode::res_b2;
        self.cbop[0xb3] = Opcode::res_b3;
        self.cbop[0xb4] = Opcode::res_b4;
        self.cbop[0xb5] = Opcode::res_b5;
        self.cbop[0xb6] = Opcode::res_b6;
        self.cbop[0xb7] = Opcode::res_b7;
        self.cbop[0xb8] = Opcode::res_b8;
        self.cbop[0xb9] = Opcode::res_b9;
        self.cbop[0xba] = Opcode::res_ba;
        self.cbop[0xbb] = Opcode::res_bb;
        self.cbop[0xbc] = Opcode::res_bc;
        self.cbop[0xbd] = Opcode::res_bd;
        self.cbop[0xbe] = Opcode::res_be;
        self.cbop[0xbf] = Opcode::res_bf;
        self.cbop[0xc0] = Opcode::set_c0;
        self.cbop[0xc1] = Opcode::set_c1;
        self.cbop[0xc2] = Opcode::set_c2;
        self.cbop[0xc3] = Opcode::set_c3;
        self.cbop[0xc4] = Opcode::set_c4;
        self.cbop[0xc5] = Opcode::set_c5;
        self.cbop[0xc6] = Opcode::set_c6;
        self.cbop[0xc7] = Opcode::set_c7;
        self.cbop[0xc8] = Opcode::set_c8;
        self.cbop[0xc9] = Opcode::set_c9;
        self.cbop[0xca] = Opcode::set_ca;
        self.cbop[0xcb] = Opcode::set_cb;
        self.cbop[0xcc] = Opcode::set_cc;
        self.cbop[0xcd] = Opcode::set_cd;
        self.cbop[0xce] = Opcode::set_ce;
        self.cbop[0xcf] = Opcode::set_cf;
        self.cbop[0xd0] = Opcode::set_d0;
        self.cbop[0xd1] = Opcode::set_d1;
        self.cbop[0xd2] = Opcode::set_d2;
        self.cbop[0xd3] = Opcode::set_d3;
        self.cbop[0xd4] = Opcode::set_d4;
        self.cbop[0xd5] = Opcode::set_d5;
        self.cbop[0xd6] = Opcode::set_d6;
        self.cbop[0xd7] = Opcode::set_d7;
        self.cbop[0xd8] = Opcode::set_d8;
        self.cbop[0xd9] = Opcode::set_d9;
        self.cbop[0xda] = Opcode::set_da;
        self.cbop[0xdb] = Opcode::set_db;
        self.cbop[0xdc] = Opcode::set_dc;
        self.cbop[0xdd] = Opcode::set_dd;
        self.cbop[0xde] = Opcode::set_de;
        self.cbop[0xdf] = Opcode::set_df;
        self.cbop[0xe0] = Opcode::set_e0;
        self.cbop[0xe1] = Opcode::set_e1;
        self.cbop[0xe2] = Opcode::set_e2;
        self.cbop[0xe3] = Opcode::set_e3;
        self.cbop[0xe4] = Opcode::set_e4;
        self.cbop[0xe5] = Opcode::set_e5;
        self.cbop[0xe6] = Opcode::set_e6;
        self.cbop[0xe7] = Opcode::set_e7;
        self.cbop[0xe8] = Opcode::set_e8;
        self.cbop[0xe9] = Opcode::set_e9;
        self.cbop[0xea] = Opcode::set_ea;
        self.cbop[0xeb] = Opcode::set_eb;
        self.cbop[0xec] = Opcode::set_ec;
        self.cbop[0xed] = Opcode::set_ed;
        self.cbop[0xee] = Opcode::set_ee;
        self.cbop[0xff] = Opcode::set_ef;
        self.cbop[0xf0] = Opcode::set_f0;
        self.cbop[0xf1] = Opcode::set_f1;
        self.cbop[0xf2] = Opcode::set_f2;
        self.cbop[0xf3] = Opcode::set_f3;
        self.cbop[0xf4] = Opcode::set_f4;
        self.cbop[0xf5] = Opcode::set_f5;
        self.cbop[0xf6] = Opcode::set_f6;
        self.cbop[0xf7] = Opcode::set_f7;
        self.cbop[0xf8] = Opcode::set_f8;
        self.cbop[0xf9] = Opcode::set_f9;
        self.cbop[0xfa] = Opcode::set_fa;
        self.cbop[0xfb] = Opcode::set_fb;
        self.cbop[0xfc] = Opcode::set_fc;
        self.cbop[0xfd] = Opcode::set_fd;
        self.cbop[0xfe] = Opcode::set_fe;
        self.cbop[0xff] = Opcode::set_ff;
    }

    fn invalid(_cpu: &mut Cpu) -> u8 {
        panic!("Invalid opcode");
    }

    /// NOP
    fn nop(_cpu: &mut Cpu) -> u8 {
        4
    }

    /// LD BC, d16
    fn ld_01(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let val = merge_bytes(high, low);
        cpu.ld_nn_d16(Regs16::BC, val);
        12
    }

    /// LD (BC), A
    fn ld_02(cpu: &mut Cpu) -> u8 {
        let bc = cpu.get_reg_16(Regs16::BC);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(bc, val);
        8
    }

    /// INC BC
    fn inc_03(cpu: &mut Cpu) -> u8 {
        cpu.inc_16(Regs16::BC);
        8
    }

    /// INC B
    fn inc_04(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::B);
        4
    }

    /// DEC B
    fn dec_05(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::B);
        4
    }

    /// LD B, d8
    fn ld_06(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::B, byte);
        8
    }

    /// RLCA
    fn rlca_07(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::A, false);
        4
    }

    /// LD (a16), SP
    fn ld_08(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let addr = merge_bytes(high, low);
        cpu.write_ram(addr, cpu.sp.get_low_byte());
        cpu.write_ram(addr + 1, cpu.sp.get_high_byte());
        20
    }

    /// ADD HL, BC
    fn add_09(cpu: &mut Cpu) -> u8 {
        let bc = cpu.get_reg_16(Regs16::BC);
        cpu.add_nn_d16(Regs16::HL, bc);
        8
    }

    /// LD A, (BC)
    fn ld_0a(cpu: &mut Cpu) -> u8 {
        let bc = cpu.get_reg_16(Regs16::BC);
        let val = cpu.read_ram(bc);
        cpu.ld_n_d8(Regs::A, val);
        8
    }

    /// DEC BC
    fn dec_0b(cpu: &mut Cpu) -> u8 {
        cpu.dec_16(Regs16::BC);
        8
    }

    /// INC C
    fn inc_0c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::C);
        4
    }

    /// DEC C
    fn dec_0d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::C);
        4
    }

    /// LD C, d8
    fn ld_0e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::C, byte);
        8
    }

    /// RRCA
    fn rrca_0f(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::A, false);
        4
    }

    /// STOP 0
    fn stop_10(_cpu: &mut Cpu) -> u8 {
        // I'm not sure how to implement this
        4
    }

    /// LD DE, d16
    fn ld_11(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let val = merge_bytes(high, low);
        cpu.ld_nn_d16(Regs16::DE, val);
        12
    }

    /// LD (DE), A
    fn ld_12(cpu: &mut Cpu) -> u8 {
        let de = cpu.get_reg_16(Regs16::DE);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(de, val);
        8
    }

    /// INC DE
    fn inc_13(cpu: &mut Cpu) -> u8 {
        cpu.inc_16(Regs16::DE);
        8
    }

    /// INC D
    fn inc_14(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::D);
        4
    }

    /// DEC D
    fn dec_15(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::D);
        4
    }

    /// LD D, d8
    fn ld_16(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::D, byte);
        8
    }

    /// RLA
    fn rla_17(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::A, true);
        4
    }

    /// JR r8
    fn jr_18(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        cpu.pc += offset as u16;
        12
    }

    /// ADD HL, DE
    fn add_19(cpu: &mut Cpu) -> u8 {
        let de = cpu.get_reg_16(Regs16::DE);
        cpu.add_nn_d16(Regs16::HL, de);
        8
    }

    /// LD A, (DE)
    fn ld_1a(cpu: &mut Cpu) -> u8 {
        let de = cpu.get_reg_16(Regs16::DE);
        let val = cpu.read_ram(de);
        cpu.set_reg(Regs::A, val);
        8
    }

    /// DEC DE
    fn dec_1b(cpu: &mut Cpu) -> u8 {
        cpu.dec_16(Regs16::DE);
        8
    }

    /// INC E
    fn inc_1c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::E);
        4
    }

    /// DEC E
    fn dec_1d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::E);
        4
    }

    /// LD E, d8
    fn ld_1e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::E, byte);
        8
    }

    /// RRA
    fn rra_1f(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::A, true);
        4
    }

    /// JR NZ, r8
    fn jr_20(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if !cpu.get_flag(Flags::Z) {
           cpu.pc += offset as u16;
        }
        12
    }

    /// LD HL, d16
    fn ld_21(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let val = merge_bytes(high, low);
        cpu.ld_nn_d16(Regs16::HL, val);
        12
    }

    /// LD (HL+), A
    fn ld_22(cpu: &mut Cpu) -> u8 {
        let mut hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(hl, val);
        hl += 1;
        cpu.set_reg_16(Regs16::HL, hl);
        8
    }

    /// INC HL
    fn inc_23(cpu: &mut Cpu) -> u8 {
        cpu.inc_16(Regs16::HL);
        8
    }

    /// INC H
    fn inc_24(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::H);
        4
    }

    /// DEC H
    fn dec_25(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::H);
        4
    }

    /// LD H, d8
    fn ld_26(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::H, byte);
        8
    }

    /// DAA
    fn daa_27(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 4
    }

    /// JR Z, r8
    fn jr_28(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if cpu.get_flag(Flags::Z) {
           cpu.pc += offset as u16;
           12
        } else {
            8
        }
    }

    /// ADD HL, HL
    fn add_29(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.add_nn_d16(Regs16::HL, hl);
        8
    }

    /// LD A, (HL+)
    fn ld_2a(cpu: &mut Cpu) -> u8 {
        let mut hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::A, val);
        hl += 1;
        cpu.set_reg_16(Regs16::HL, hl);
        8
    }

    /// DEC HL
    fn dec_2b(cpu: &mut Cpu) -> u8 {
        cpu.dec_16(Regs16::HL);
        8
    }

    /// INC L
    fn inc_2c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::L);
        4
    }

    /// DEC L
    fn dec_2d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::L);
        4
    }

    /// LD L, d8
    fn ld_2e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::L, byte);
        8
    }

    /// CPL
    fn cpl_2f(cpu: &mut Cpu) -> u8 {
        cpu.a = !cpu.a;
        cpu.set_flag(Flags::N);
        cpu.set_flag(Flags::H);
        4
    }

    /// JR NC, r8
    fn jr_30(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if !cpu.get_flag(Flags::C) {
           cpu.pc += offset as u16;
           12
        } else {
            8
        }
    }

    /// LD SP, d16
    fn ld_31(cpu: &mut Cpu) -> u8 {
        let byte1 = cpu.fetch();
        let byte2 = cpu.fetch();
        cpu.sp = merge_bytes(byte1, byte2);
        12
    }

    /// LD (HL-), A
    fn ld_32(cpu: &mut Cpu) -> u8 {
        let mut hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(hl, val);
        hl -= 1;
        cpu.set_reg_16(Regs16::HL, hl);
        8
    }

    /// INC SP
    fn inc_33(cpu: &mut Cpu) -> u8 {
        // May need to check for flags
        cpu.sp += 1;
        8
    }

    /// INC (HL)
    fn inc_34(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let mut val = cpu.read_ram(hl);
        val += 1;
        cpu.write_ram(hl, val);
        12
    }

    /// DEC (HL)
    fn dec_35(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let mut val = cpu.read_ram(hl);
        val -= 1;
        cpu.write_ram(hl, val);
        12
    }

    /// LD (HL), d8
    fn ld_36(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.fetch();
        cpu.write_ram(hl, val);
        12
    }

    /// SCF
    fn scf_37(cpu: &mut Cpu) -> u8 {
        cpu.set_flag(Flags::C);
        cpu.clear_flag(Flags::H);
        cpu.clear_flag(Flags::N);
        4
    }

    /// JR C, r8
    fn jr_38(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch();
        if cpu.get_flag(Flags::C) {
           cpu.pc += offset as u16;
           12
        } else {
            8
        }
    }

    /// ADD HL, SP
    fn add_39(cpu: &mut Cpu) -> u8 {
        cpu.add_nn_d16(Regs16::HL, cpu.sp);
        8
    }

    /// LD A, (HL-)
    fn ld_3a(cpu: &mut Cpu) -> u8 {
        let mut hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::A, val);
        hl -= 1;
        cpu.set_reg_16(Regs16::HL, hl);
        8
    }

    /// DEC SP
    fn dec_3b(cpu: &mut Cpu) -> u8 {
        cpu.sp -= 1;
        8
    }

    /// INC A
    fn inc_3c(cpu: &mut Cpu) -> u8 {
        cpu.inc_8(Regs::A);
        4
    }

    /// DEC A
    fn dec_3d(cpu: &mut Cpu) -> u8 {
        cpu.dec_8(Regs::A);
        4
    }

    /// LD A, d8
    fn ld_3e(cpu: &mut Cpu) -> u8 {
        let byte = cpu.fetch();
        cpu.ld_n_d8(Regs::A, byte);
        8
    }

    /// CCF
    fn ccf_3f(cpu: &mut Cpu) -> u8 {
        cpu.clear_flag(Flags::N);
        cpu.clear_flag(Flags::H);
        let cf = cpu.get_flag(Flags::C);
        cpu.write_flag(Flags::C, !cf);
        4
    }

    /// LD B, B
    fn ld_40(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD B, C
    fn ld_41(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD B, D
    fn ld_42(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD B, E
    fn ld_43(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD B, H
    fn ld_44(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD B, L
    fn ld_45(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD B, (HL)
    fn ld_46(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::B, val);
        8
    }

    /// LD B, A
    fn ld_47(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::B, byte);
        4
    }

    /// LD C, B
    fn ld_48(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD C, C
    fn ld_49(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD C, D
    fn ld_4a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD C, E
    fn ld_4b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD C, H
    fn ld_4c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD C, L
    fn ld_4d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD C, (HL)
    fn ld_4e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::C, val);
        8
    }

    /// LD C, A
    fn ld_4f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::C, byte);
        4
    }

    /// LD D, B
    fn ld_50(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD D, C
    fn ld_51(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD D, D
    fn ld_52(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD D, E
    fn ld_53(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD D, H
    fn ld_54(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD D, L
    fn ld_55(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD D, (HL)
    fn ld_56(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::D, val);
        8
    }

    /// LD D, A
    fn ld_57(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::D, byte);
        4
    }

    /// LD E, B
    fn ld_58(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD E, C
    fn ld_59(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD E, D
    fn ld_5a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD E, E
    fn ld_5b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD E, H
    fn ld_5c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD E, L
    fn ld_5d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD E, (HL)
    fn ld_5e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::E, val);
        8
    }

    /// LD E, A
    fn ld_5f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::E, byte);
        4
    }

    /// LD H, B
    fn ld_60(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD H, C
    fn ld_61(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD H, D
    fn ld_62(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD H, E
    fn ld_63(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD H, H
    fn ld_64(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD H, L
    fn ld_65(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD H, (HL)
    fn ld_66(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::H, val);
        8
    }

    /// LD H, A
    fn ld_67(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::H, byte);
        4
    }

    /// LD L, B
    fn ld_68(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD L, C
    fn ld_69(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD L, D
    fn ld_6a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD L, E
    fn ld_6b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD L, H
    fn ld_6c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD L, L
    fn ld_6d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD L, (HL)
    fn ld_6e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::L, val);
        8
    }

    /// LD L, A
    fn ld_6f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::L, byte);
        4
    }

    /// LD (HL), B
    fn ld_70(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// LD (HL), C
    fn ld_71(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// LD (HL), D
    fn ld_72(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// LD (HL), E
    fn ld_73(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// LD (HL), H
    fn ld_74(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// LD (HL), L
    fn ld_75(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// HALT
    fn halt_76(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode");
        // 8
    }

    /// LD (HL), A
    fn ld_77(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_ram(hl, val);
        8
    }

    /// LD A, B
    fn ld_78(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::B);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// LD A, C
    fn ld_79(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::C);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// LD A, D
    fn ld_7a(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::D);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// LD A, E
    fn ld_7b(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::E);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// LD A, H
    fn ld_7c(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::H);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// LD A, L
    fn ld_7d(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::L);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// LD A, (HL)
    fn ld_7e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.set_reg(Regs::A, val);
        8
    }

    /// LD A, A
    fn ld_7f(cpu: &mut Cpu) -> u8 {
        let byte = cpu.get_reg(Regs::A);
        cpu.ld_n_d8(Regs::A, byte);
        4
    }

    /// ADD A, B
    fn add_80(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADD A, C
    fn add_81(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADD A, D
    fn add_82(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADD A, E
    fn add_83(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADD A, H
    fn add_84(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADD A, L
    fn add_85(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADD A, (HL)
    fn add_86(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.add_a_d8(val, false);
        8
    }

    /// ADD A, A
    fn add_87(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.add_a_d8(val, false);
        4
    }

    /// ADC A, B
    fn adc_88(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.add_a_d8(val, true);
        4
    }

    /// ADC A, C
    fn adc_89(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.add_a_d8(val, true);
        4
    }

    /// ADC A, D
    fn adc_8a(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.add_a_d8(val, true);
        4
    }

    /// ADC A, E
    fn adc_8b(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.add_a_d8(val, true);
        4
    }

    /// ADC A, H
    fn adc_8c(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.add_a_d8(val, true);
        4
    }

    /// ADC A, L
    fn adc_8d(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.add_a_d8(val, true);
        4
    }

    /// ADC A, (HL)
    fn adc_8e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.add_a_d8(val, true);
        8
    }

    /// ADC A, A
    fn adc_8f(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.add_a_d8(val, true);
        4
    }

    /// SUB B
    fn sub_90(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SUB C
    fn sub_91(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SUB D
    fn sub_92(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SUB E
    fn sub_93(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SUB H
    fn sub_94(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SUB L
    fn sub_95(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SUB (HL)
    fn sub_96(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.sub_a_d8(val, false);
        8
    }

    /// SUB A
    fn sub_97(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.sub_a_d8(val, false);
        4
    }

    /// SBC A, B
    fn sbc_98(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.sub_a_d8(val, true);
        4
    }

    /// SBC A, C
    fn sbc_99(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.sub_a_d8(val, true);
        4
    }

    /// SBC A, D
    fn sbc_9a(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.sub_a_d8(val, true);
        4
    }

    /// SBC A, E
    fn sbc_9b(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.sub_a_d8(val, true);
        4
    }

    /// SBC A, H
    fn sbc_9c(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.sub_a_d8(val, true);
        4
    }

    /// SBC A, L
    fn sbc_9d(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.sub_a_d8(val, true);
        4
    }

    /// SBC A, (HL)
    fn sbc_9e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.sub_a_d8(val, true);
        8
    }

    /// SBC A, A
    fn sbc_9f(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.sub_a_d8(val, true);
        4
    }

    /// AND B
    fn and_a0(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.and_a_d8(val);
        4
    }

    /// AND C
    fn and_a1(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.and_a_d8(val);
        4
    }

    /// AND D
    fn and_a2(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.and_a_d8(val);
        4
    }

    /// AND E
    fn and_a3(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.and_a_d8(val);
        4
    }

    /// AND H
    fn and_a4(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.and_a_d8(val);
        4
    }

    /// AND L
    fn and_a5(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.and_a_d8(val);
        4
    }

    /// AND (HL)
    fn and_a6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.and_a_d8(val);
        8
    }

    /// AND A
    fn and_a7(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.and_a_d8(val);
        4
    }

    /// XOR B
    fn xor_a8(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.xor_a_d8(val);
        4
    }

    /// XOR C
    fn xor_a9(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.xor_a_d8(val);
        4
    }

    /// XOR D
    fn xor_aa(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.xor_a_d8(val);
        4
    }

    /// XOR E
    fn xor_ab(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.xor_a_d8(val);
        4
    }

    /// XOR H
    fn xor_ac(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.xor_a_d8(val);
        4
    }

    /// XOR L
    fn xor_ad(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.xor_a_d8(val);
        4
    }

    /// XOR (HL)
    fn xor_ae(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.xor_a_d8(val);
        8
    }

    /// XOR A
    fn xor_af(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.xor_a_d8(val);
        4
    }

    /// OR B
    fn or_b0(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.or_a_d8(val);
        4
    }

    /// OR C
    fn or_b1(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.or_a_d8(val);
        4
    }

    /// OR D
    fn or_b2(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.or_a_d8(val);
        4
    }

    /// OR E
    fn or_b3(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.or_a_d8(val);
        4
    }

    /// OR H
    fn or_b4(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.or_a_d8(val);
        4
    }

    /// OR L
    fn or_b5(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.or_a_d8(val);
        4
    }

    /// OR (HL)
    fn or_b6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.or_a_d8(val);
        8
    }

    /// OR A
    fn or_b7(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.or_a_d8(val);
        4
    }

    /// CP B
    fn cp_b8(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::B);
        cpu.cp_a_d8(val);
        4
    }

    /// CP C
    fn cp_b9(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::C);
        cpu.cp_a_d8(val);
        4
    }

    /// CP D
    fn cp_ba(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::D);
        cpu.cp_a_d8(val);
        4
    }

    /// CP E
    fn cp_bb(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::E);
        cpu.cp_a_d8(val);
        4
    }

    /// CP H
    fn cp_bc(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::H);
        cpu.cp_a_d8(val);
        4
    }

    /// CP L
    fn cp_bd(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::L);
        cpu.cp_a_d8(val);
        4
    }

    /// CP (HL)
    fn cp_be(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.cp_a_d8(val);
        8
    }

    /// CP A
    fn cp_bf(cpu: &mut Cpu) -> u8 {
        let val = cpu.get_reg(Regs::A);
        cpu.cp_a_d8(val);
        4
    }

    /// RET NZ
    fn ret_c0(cpu: &mut Cpu) -> u8 {
        if !cpu.get_flag(Flags::Z) {
            let addr = cpu.pop();
            cpu.pc = addr;
            20
        } else {
            8
        }
    }

    /// POP BC
    fn pop_c1(cpu: &mut Cpu) -> u8 {
        let val = cpu.pop();
        cpu.set_reg_16(Regs16::BC, val);
        12
    }

    /// JP NZ, a16
    fn jp_c2(cpu: &mut Cpu) -> u8 {
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

    /// JP a16
    fn jp_c3(cpu: &mut Cpu) -> u8 {
        let low = cpu.fetch();
        let high = cpu.fetch();
        let offset = merge_bytes(high, low);
        cpu.pc = offset;
        16
    }

    /// CALL NZ, a16
    fn call_c4(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        if !cpu.get_flag(Flags::Z) {
            let addr = merge_bytes(high, low);
            cpu.push(cpu.pc);
            cpu.pc = addr;
            24
        } else {
            12
        }
    }

    /// PUSH BC
    fn push_c5(cpu: &mut Cpu) -> u8 {
        let bc = cpu.get_reg_16(Regs16::BC);
        cpu.push(bc);
        16
    }

    /// ADD A, d8
    fn add_c6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.add_a_d8(val, false);
        8
    }

    /// RST 00
    /// Push PC onto stack
    /// Jump to $0000 + $00
    fn rst_c7(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0000;
        16
    }

    /// RET Z
    fn ret_c8(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(Flags::Z) {
            let addr = cpu.pop();
            cpu.pc = addr;
            20
        } else {
            8
        }
    }

    /// RET
    fn ret_c9(cpu: &mut Cpu) -> u8 {
        cpu.pc = cpu.pop();
        16
    }

    /// JP Z, a16
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

    /// PREFIX CB
    fn prefix_cb(_cpu: &mut Cpu) -> u8 {
        panic!("Should be using other table!");
    }

    /// CALL Z, a16
    fn call_cc(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        if cpu.get_flag(Flags::Z) {
            let addr = merge_bytes(high, low);
            cpu.push(cpu.pc);
            cpu.pc = addr;
            24
        } else {
            12
        }
    }

    /// CALL a16
    fn call_cd(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let addr = merge_bytes(high, low);
        cpu.push(cpu.pc);
        cpu.pc = addr;
        24
    }

    /// ADC A, d8
    fn adc_ce(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.add_a_d8(val, true);
        8
    }

    /// RST 08
    fn rst_cf(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0008;
        16
    }

    /// RET NC
    fn ret_d0(cpu: &mut Cpu) -> u8 {
        if !cpu.get_flag(Flags::C) {
            cpu.pc = cpu.pop();
            20
        } else {
            8
        }
    }

    /// POP DE
    fn pop_d1(cpu: &mut Cpu) -> u8 {
        let val = cpu.pop();
        cpu.set_reg_16(Regs16::DE, val);
        12
    }

    /// JP NC, a16
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

    /// CALL NC, a16
    fn call_d4(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        if !cpu.get_flag(Flags::C) {
            let addr = merge_bytes(high, low);
            cpu.push(cpu.pc);
            cpu.pc = addr;
            24
        } else {
            12
        }
    }

    /// PUSH DE
    fn push_d5(cpu: &mut Cpu) -> u8 {
        let de = cpu.get_reg_16(Regs16::DE);
        cpu.push(de);
        16
    }

    /// SUB d8
    fn sub_d6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.sub_a_d8(val, false);
        8
    }

    /// RST 10
    fn rst_d7(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0010;
        16
    }

    /// RET C
    fn ret_d8(cpu: &mut Cpu) -> u8 {
        if cpu.get_flag(Flags::C) {
            cpu.pc = cpu.pop();
            20
        } else {
            8
        }
    }

    /// RETI
    fn reti_d9(cpu: &mut Cpu) -> u8 {
        cpu.pc = cpu.pop();
        cpu.interupt = true;
        16
    }

    /// JP C, a16
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

    /// CALL C, a16
    fn call_dc(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        if cpu.get_flag(Flags::C) {
            let addr = merge_bytes(high, low);
            cpu.push(cpu.pc);
            cpu.pc = addr;
            24
        } else {
            12
        }
    }

    /// SBC A, d8
    fn sbc_de(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.sub_a_d8(val, true);
        8
    }

    /// RST 18
    fn rst_df(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0018;
        16
    }

    /// LDH (a8), A
    /// Same as LD $FF00+n A
    fn ldh_e0(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch() as u16;
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(0xFF00 + offset, val);
        12
    }

    /// POP HL
    fn pop_e1(cpu: &mut Cpu) -> u8 {
        let val = cpu.pop();
        cpu.set_reg_16(Regs16::HL, val);
        12
    }

    /// LD (C), A
    fn ld_e2(cpu: &mut Cpu) -> u8 {
        let c = cpu.get_reg(Regs::C);
        let val = cpu.get_reg(Regs::A);
        cpu.write_ram(c as u16, val);
        8
    }

    /// PUSH HL
    fn push_e5(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.push(hl);
        16
    }

    /// AND d8
    fn and_e6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.and_a_d8(val);
        8
    }

    /// RST 20
    fn rst_e7(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0020;
        16
    }

    /// ADD SP, r8
    fn add_e8(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        let signed = val as i8 as i16 as u16;
        let result = cpu.sp.overflowing_add(signed);
        let set_h = check_h_flag_u16(cpu.sp, signed);
        cpu.sp = result.0;

        cpu.clear_flag(Flags::Z);
        cpu.clear_flag(Flags::N);
        cpu.write_flag(Flags::C, result.1);
        cpu.write_flag(Flags::H, set_h);
        16
    }

    /// JP (HL)
    fn jp_e9(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let val = cpu.read_ram(hl);
        cpu.pc = val as u16;
        4
    }

    /// LD (a16), A
    fn ld_ea(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let addr = merge_bytes(high, low);
        let a = cpu.get_reg(Regs::A);
        cpu.write_ram(addr, a);
        16
    }

    /// XOR d8
    fn xor_ee(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.xor_a_d8(val);
        8
    }

    /// RST 28
    fn rst_ef(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0028;
        16
    }

    /// LDH A, (a8)
    /// Store $FF00 + n into A
    fn ldh_f0(cpu: &mut Cpu) -> u8 {
        let offset = cpu.fetch() as u16;
        let val = cpu.read_ram(0xFF00 + offset);
        cpu.set_reg(Regs::A, val);
        12
    }

    /// POP AF
    fn pop_f1(cpu: &mut Cpu) -> u8 {
        let val = cpu.pop();
        cpu.set_reg_16(Regs16::AF, val);
        12
    }

    /// LD A, (C)
    /// Store $FF00 + register C into A
    fn ld_f2(cpu: &mut Cpu) -> u8 {
        let c = cpu.get_reg(Regs::C) as u16;
        let val = cpu.read_ram(0xFF00 + c);
        cpu.set_reg(Regs::A, val);
        8
    }

    /// DI
    fn di_f3(cpu: &mut Cpu) -> u8 {
        cpu.interupt = false;
        4
    }

    /// PUSH AF
    fn push_f5(cpu: &mut Cpu) -> u8 {
        let af = cpu.get_reg_16(Regs16::AF);
        cpu.push(af);
        16
    }

    /// OR d8
    fn or_f6(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.or_a_d8(val);
        8
    }

    /// RST 30
    fn rst_f7(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0030;
        16
    }

    /// LD HL, SP+r8
    /// Put SP + n into HL
    fn ld_f8(cpu: &mut Cpu) -> u8 {
        let n = cpu.fetch();
        cpu.set_reg_16(Regs16::HL, cpu.sp + n as u16);
        12
    }

    /// LD SP, HL
    fn ld_f9(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.sp = hl;
        8
    }

    /// LD A, (a16)
    fn ld_fa(cpu: &mut Cpu) -> u8 {
        let high = cpu.fetch();
        let low = cpu.fetch();
        let addr = merge_bytes(high, low);
        let val = cpu.read_ram(addr);
        cpu.set_reg(Regs::A, val);
        16
    }

    /// EI
    fn ei_fb(cpu: &mut Cpu) -> u8 {
        cpu.interupt = true;
        4
    }

    /// CP d8
    fn cp_fe(cpu: &mut Cpu) -> u8 {
        let val = cpu.fetch();
        cpu.cp_a_d8(val);
        8
    }

    /// RST 38
    fn rst_ff(cpu: &mut Cpu) -> u8 {
        cpu.push(cpu.pc);
        cpu.pc = 0x0038;
        16
    }

    /* ------------------
     * $CB Opcode block
     * ---------------- */

    /// RLC B
    fn rlc_00(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::B, false);
        8
    }

    /// RLC C
    fn rlc_01(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::C, false);
        8
    }

    /// RLC D
    fn rlc_02(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::D, false);
        8
    }

    /// RLC E
    fn rlc_03(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::E, false);
        8
    }

    /// RLC H
    fn rlc_04(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::H, false);
        8
    }

    /// RLC L
    fn rlc_05(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::L, false);
        8
    }

    /// RLC (HL)
    fn rlc_06(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let mut byte = cpu.read_ram(hl);
        cpu.write_flag(Flags::C, byte.get_bit(7));
        byte <<= 1;
        cpu.write_ram(hl, byte);

        cpu.write_flag(Flags::Z, byte == 0);
        cpu.clear_flag(Flags::N);
        cpu.clear_flag(Flags::H);
        8
    }

    /// RLC A
    fn rlc_07(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::A, false);
        8
    }

    /// RRC B
    fn rrc_08(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::B, false);
        8
    }

    /// RRC C
    fn rrc_09(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::C, false);
        8
    }

    /// RRC D
    fn rrc_0a(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::D, false);
        8
    }

    /// RRC E
    fn rrc_0b(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::E, false);
        8
    }

    /// RRC H
    fn rrc_0c(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::H, false);
        8
    }

    /// RRC L
    fn rrc_0d(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::L, false);
        8
    }

    /// RRC (HL)
    fn rrc_0e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let mut byte = cpu.read_ram(hl);
        cpu.write_flag(Flags::C, byte.get_bit(0));
        byte >>= 1;
        cpu.write_ram(hl, byte);

        cpu.write_flag(Flags::Z, byte == 0);
        cpu.clear_flag(Flags::N);
        cpu.clear_flag(Flags::H);
        8
    }

    /// RRC A
    fn rrc_0f(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::A, false);
        8
    }

    /// RL B
    fn rl_10(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::B, true);
        8
    }

    /// RL C
    fn rl_11(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::C, true);
        8
    }

    /// RL D
    fn rl_12(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::D, true);
        8
    }

    /// RL E
    fn rl_13(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::E, true);
        8
    }

    /// RL H
    fn rl_14(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::H, true);
        8
    }

    /// RL L
    fn rl_15(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::L, true);
        8
    }

    /// RL (HL)
    fn rl_16(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let mut byte = cpu.read_ram(hl);
        let old_c = byte.get_bit(7);
        cpu.write_flag(Flags::C, byte.get_bit(7));
        byte <<= 1;
        byte.write_bit(7, old_c);
        cpu.write_ram(hl, byte);

        cpu.write_flag(Flags::Z, byte == 0);
        cpu.clear_flag(Flags::N);
        cpu.clear_flag(Flags::H);
        8
    }

    /// RL A
    fn rl_17(cpu: &mut Cpu) -> u8 {
        cpu.rot_left(Regs::A, true);
        8
    }

    /// RR B
    fn rr_18(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::B, true);
        8
    }

    /// RR C
    fn rr_19(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::C, true);
        8
    }

    /// RR D
    fn rr_1a(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::D, true);
        8
    }

    /// RR E
    fn rr_1b(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::E, true);
        8
    }

    /// RR H
    fn rr_1c(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::H, true);
        8
    }

    /// RR L
    fn rr_1d(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::L, true);
        8
    }

    /// RR (HL)
    fn rr_1e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        let mut byte = cpu.read_ram(hl);
        let old_c = byte.get_bit(0);
        cpu.write_flag(Flags::C, byte.get_bit(0));
        byte >>= 1;
        byte.write_bit(0, old_c);
        cpu.write_ram(hl, byte);

        cpu.write_flag(Flags::Z, byte == 0);
        cpu.clear_flag(Flags::N);
        cpu.clear_flag(Flags::H);
        8
    }

    /// RR A
    fn rr_1f(cpu: &mut Cpu) -> u8 {
        cpu.rot_right(Regs::A, true);
        8
    }

    /// SLA B
    fn sla_20(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA C
    fn sla_21(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA D
    fn sla_22(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA E
    fn sla_23(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA H
    fn sla_24(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA L
    fn sla_25(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA (HL)
    fn sla_26(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SLA A
    fn sla_27(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA B
    fn sra_28(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA C
    fn sra_29(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA D
    fn sra_2a(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA E
    fn sra_2b(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA H
    fn sra_2c(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA L
    fn sra_2d(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA (HL)
    fn sra_2e(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRA A
    fn sra_2f(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SWAP B
    fn swap_30(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::B);
        8
    }

    /// SWAP C
    fn swap_31(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::C);
        8
    }

    /// SWAP D
    fn swap_32(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::D);
        8
    }

    /// SWAP E
    fn swap_33(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::E);
        8
    }

    /// SWAP H
    fn swap_34(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::H);
        8
    }

    /// SWAP L
    fn swap_35(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::L);
        8
    }

    /// SWAP (HL)
    fn swap_36(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SWAP A
    fn swap_37(cpu: &mut Cpu) -> u8 {
        cpu.swap_bits(Regs::A);
        8
    }

    /// SRL B
    fn srl_38(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL C
    fn srl_39(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL D
    fn srl_3a(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL E
    fn srl_3b(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL H
    fn srl_3c(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL L
    fn srl_3d(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL (HL)
    fn srl_3e(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// SRL A
    fn srl_3f(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 0,B
    fn bit_40(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 0);
        8
    }

    /// BIT 0,C
    fn bit_41(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 0);
        8
    }

    /// BIT 0,D
    fn bit_42(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 0);
        8
    }

    /// BIT 0,E
    fn bit_43(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 0);
        8
    }

    /// BIT 0,H
    fn bit_44(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 0);
        8
    }

    /// BIT 0,L
    fn bit_45(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 0);
        8
    }

    /// BIT 0,(HL)
    fn bit_46(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 0,A
    fn bit_47(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 0);
        8
    }

    /// BIT 1,B
    fn bit_48(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 1);
        8
    }

    /// BIT 1,C
    fn bit_49(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 1);
        8
    }

    /// BIT 1,D
    fn bit_4a(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 1);
        8
    }

    /// BIT 1,E
    fn bit_4b(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 1);
        8
    }

    /// BIT 1,H
    fn bit_4c(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 1);
        8
    }

    /// BIT 1,L
    fn bit_4d(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 1);
        8
    }

    /// BIT 1,(HL)
    fn bit_4e(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 1,A
    fn bit_4f(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 1);
        8
    }

    /// BIT 2,B
    fn bit_50(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 2);
        8
    }

    /// BIT 2,C
    fn bit_51(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 2);
        8
    }

    /// BIT 2,D
    fn bit_52(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 2);
        8
    }

    /// BIT 2,E
    fn bit_53(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 2);
        8
    }

    /// BIT 2,H
    fn bit_54(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 2);
        8
    }

    /// BIT 2,L
    fn bit_55(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 2);
        8
    }

    /// BIT 2,(HL)
    fn bit_56(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 2,A
    fn bit_57(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 2);
        8
    }

    /// BIT 3,B
    fn bit_58(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 3);
        8
    }

    /// BIT 3,C
    fn bit_59(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 3);
        8
    }

    /// BIT 3,D
    fn bit_5a(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 3);
        8
    }

    /// BIT 3,E
    fn bit_5b(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 3);
        8
    }

    /// BIT 3,H
    fn bit_5c(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 3);
        8
    }

    /// BIT 3,L
    fn bit_5d(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 3);
        8
    }

    /// BIT 3,(HL)
    fn bit_5e(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 3,A
    fn bit_5f(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 3);
        8
    }

    /// BIT 4,B
    fn bit_60(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 4);
        8
    }

    /// BIT 4,C
    fn bit_61(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 4);
        8
    }

    /// BIT 4,D
    fn bit_62(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 4);
        8
    }

    /// BIT 4,E
    fn bit_63(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 4);
        8
    }

    /// BIT 4,H
    fn bit_64(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 4);
        8
    }

    /// BIT 4,L
    fn bit_65(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 4);
        8
    }

    /// BIT 4,(HL)
    fn bit_66(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 4,A
    fn bit_67(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 4);
        8
    }

    /// BIT 5,B
    fn bit_68(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 5);
        8
    }

    /// BIT 5,C
    fn bit_69(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 5);
        8
    }

    /// BIT 5,D
    fn bit_6a(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 5);
        8
    }

    /// BIT 5,E
    fn bit_6b(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 5);
        8
    }

    /// BIT 5,H
    fn bit_6c(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 5);
        8
    }

    /// BIT 5,L
    fn bit_6d(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 5);
        8
    }

    /// BIT 5,(HL)
    fn bit_6e(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 5,A
    fn bit_6f(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 5);
        8
    }

    /// BIT 6,B
    fn bit_70(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 6);
        8
    }

    /// BIT 6,C
    fn bit_71(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 6);
        8
    }

    /// BIT 6,D
    fn bit_72(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 6);
        8
    }

    /// BIT 6,E
    fn bit_73(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 6);
        8
    }

    /// BIT 6,H
    fn bit_74(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 6);
        8
    }

    /// BIT 6,L
    fn bit_75(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 6);
        8
    }

    /// BIT 6,(HL)
    fn bit_76(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 6,A
    fn bit_77(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 6);
        8
    }

    /// BIT 7,B
    fn bit_78(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::B, 7);
        8
    }

    /// BIT 7,C
    fn bit_79(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::C, 7);
        8
    }

    /// BIT 7,D
    fn bit_7a(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::D, 7);
        8
    }

    /// BIT 7,E
    fn bit_7b(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::E, 7);
        8
    }

    /// BIT 7,H
    fn bit_7c(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::H, 7);
        8
    }

    /// BIT 7,L
    fn bit_7d(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::L, 7);
        8
    }

    /// BIT 7,(HL)
    fn bit_7e(cpu: &mut Cpu) -> u8 {
        panic!("Unimplemented opcode!");
        // 8
    }

    /// BIT 7,A
    fn bit_7f(cpu: &mut Cpu) -> u8 {
        cpu.test_bit(Regs::A, 7);
        8
    }

    /// RES 0,B
    fn res_80(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 0, false);
        8
    }

    /// RES 0,C
    fn res_81(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 0, false);
        8
    }

    /// RES 0,D
    fn res_82(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 0, false);
        8
    }

    /// RES 0,E
    fn res_83(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 0, false);
        8
    }

    /// RES 0,H
    fn res_84(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 0, false);
        8
    }

    /// RES 0,L
    fn res_85(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 0, false);
        8
    }

    /// RES 0,(HL)
    fn res_86(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 0, false);
        8
    }

    /// RES 0,A
    fn res_87(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 0, false);
        8
    }

    /// RES 1,B
    fn res_88(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 1, false);
        8
    }

    /// RES 1,C
    fn res_89(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 1, false);
        8
    }

    /// RES 1,D
    fn res_8a(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 1, false);
        8
    }

    /// RES 1,E
    fn res_8b(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 1, false);
        8
    }

    /// RES 1,H
    fn res_8c(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 1, false);
        8
    }

    /// RES 1,L
    fn res_8d(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 1, false);
        8
    }

    /// RES 1,(HL)
    fn res_8e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 1, false);
        8
    }

    /// RES 1,A
    fn res_8f(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 1, false);
        8
    }

    /// RES 2,B
    fn res_90(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 2, false);
        8
    }

    /// RES 2,C
    fn res_91(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 2, false);
        8
    }

    /// RES 2,D
    fn res_92(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 2, false);
        8
    }

    /// RES 2,E
    fn res_93(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 2, false);
        8
    }

    /// RES 2,H
    fn res_94(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 2, false);
        8
    }

    /// RES 2,L
    fn res_95(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 2, false);
        8
    }

    /// RES 2,(HL)
    fn res_96(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 2, false);
        8
    }

    /// RES 2,A
    fn res_97(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 2, false);
        8
    }

    /// RES 3,B
    fn res_98(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 3, false);
        8
    }

    /// RES 3,C
    fn res_99(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 3, false);
        8
    }

    /// RES 3,D
    fn res_9a(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 3, false);
        8
    }

    /// RES 3,E
    fn res_9b(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 3, false);
        8
    }

    /// RES 3,H
    fn res_9c(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 3, false);
        8
    }

    /// RES 3,L
    fn res_9d(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 3, false);
        8
    }

    /// RES 3,(HL)
    fn res_9e(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 3, false);
        8
    }

    /// RES 3,A
    fn res_9f(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 3, false);
        8
    }

    /// RES 4,B
    fn res_a0(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 4, false);
        8
    }

    /// RES 4,C
    fn res_a1(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 4, false);
        8
    }

    /// RES 4,D
    fn res_a2(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 4, false);
        8
    }

    /// RES 4,E
    fn res_a3(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 4, false);
        8
    }

    /// RES 4,H
    fn res_a4(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 4, false);
        8
    }

    /// RES 4,L
    fn res_a5(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 4, false);
        8
    }

    /// RES 4,(HL)
    fn res_a6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 4, false);
        8
    }

    /// RES 4,A
    fn res_a7(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 4, false);
        8
    }

    /// RES 5,B
    fn res_a8(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 5, false);
        8
    }

    /// RES 5,C
    fn res_a9(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 5, false);
        8
    }

    /// RES 5,D
    fn res_aa(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 5, false);
        8
    }

    /// RES 5,E
    fn res_ab(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 5, false);
        8
    }

    /// RES 5,H
    fn res_ac(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 5, false);
        8
    }

    /// RES 5,L
    fn res_ad(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 5, false);
        8
    }

    /// RES 5,(HL)
    fn res_ae(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 5, false);
        8
    }

    /// RES 5,A
    fn res_af(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 5, false);
        8
    }

    /// RES 6,B
    fn res_b0(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 6, false);
        8
    }

    /// RES 6,C
    fn res_b1(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 6, false);
        8
    }

    /// RES 6,D
    fn res_b2(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 6, false);
        8
    }

    /// RES 6,E
    fn res_b3(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 6, false);
        8
    }

    /// RES 6,H
    fn res_b4(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 6, false);
        8
    }

    /// RES 6,L
    fn res_b5(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 6, false);
        8
    }

    /// RES 6,(HL)
    fn res_b6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 6, false);
        8
    }

    /// RES 6,A
    fn res_b7(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 6, false);
        8
    }

    /// RES 7,B
    fn res_b8(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 7, false);
        8
    }

    /// RES 7,C
    fn res_b9(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 7, false);
        8
    }

    /// RES 7,D
    fn res_ba(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 7, false);
        8
    }

    /// RES 7,E
    fn res_bb(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 7, false);
        8
    }

    /// RES 7,H
    fn res_bc(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 7, false);
        8
    }

    /// RES 7,L
    fn res_bd(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 7, false);
        8
    }

    /// RES 7,(HL)
    fn res_be(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 7, false);
        8
    }

    /// RES 7,A
    fn res_bf(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 7, false);
        8
    }

    /// SET 0,B
    fn set_c0(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 0, true);
        8
    }

    /// SET 0,C
    fn set_c1(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 0, true);
        8
    }

    /// SET 0,D
    fn set_c2(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 0, true);
        8
    }

    /// SET 0,E
    fn set_c3(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 0, true);
        8
    }

    /// SET 0,H
    fn set_c4(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 0, true);
        8
    }

    /// SET 0,L
    fn set_c5(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 0, true);
        8
    }

    /// SET 0,(HL)
    fn set_c6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 0, true);
        8
    }

    /// SET 0,A
    fn set_c7(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 0, true);
        8
    }

    /// SET 1,B
    fn set_c8(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 1, true);
        8
    }

    /// SET 1,C
    fn set_c9(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 1, true);
        8
    }

    /// SET 1,D
    fn set_ca(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 1, true);
        8
    }

    /// SET 1,E
    fn set_cb(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 1, true);
        8
    }

    /// SET 1,H
    fn set_cc(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 1, true);
        8
    }

    /// SET 1,L
    fn set_cd(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 1, true);
        8
    }

    /// SET 1,(HL)
    fn set_ce(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 1, true);
        8
    }

    /// SET 1,A
    fn set_cf(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 1, true);
        8
    }

    /// SET 2,B
    fn set_d0(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 2, true);
        8
    }

    /// SET 2,C
    fn set_d1(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 2, true);
        8
    }

    /// SET 2,D
    fn set_d2(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 2, true);
        8
    }

    /// SET 2,E
    fn set_d3(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 2, true);
        8
    }

    /// SET 2,H
    fn set_d4(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 2, true);
        8
    }

    /// SET 2,L
    fn set_d5(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 2, true);
        8
    }

    /// SET 2,(HL)
    fn set_d6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 2, true);
        8
    }

    /// SET 2,A
    fn set_d7(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 2, true);
        8
    }

    /// SET 3,B
    fn set_d8(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 3, true);
        8
    }

    /// SET 3,C
    fn set_d9(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 3, true);
        8
    }

    /// SET 3,D
    fn set_da(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 3, true);
        8
    }

    /// SET 3,E
    fn set_db(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 3, true);
        8
    }

    /// SET 3,H
    fn set_dc(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 3, true);
        8
    }

    /// SET 3,L
    fn set_dd(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 3, true);
        8
    }

    /// SET 3,(HL)
    fn set_de(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 3, true);
        8
    }

    /// SET 3,A
    fn set_df(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 3, true);
        8
    }

    /// SET 4,B
    fn set_e0(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 4, true);
        8
    }

    /// SET 4,C
    fn set_e1(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 4, true);
        8
    }

    /// SET 4,D
    fn set_e2(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 4, true);
        8
    }

    /// SET 4,E
    fn set_e3(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 4, true);
        8
    }

    /// SET 4,H
    fn set_e4(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 4, true);
        8
    }

    /// SET 4,L
    fn set_e5(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 4, true);
        8
    }

    /// SET 4,(HL)
    fn set_e6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 4, true);
        8
    }

    /// SET 4,A
    fn set_e7(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 4, true);
        8
    }

    /// SET 5,B
    fn set_e8(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 5, true);
        8
    }

    /// SET 5,C
    fn set_e9(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 5, true);
        8
    }

    /// SET 5,D
    fn set_ea(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 5, true);
        8
    }

    /// SET 5,E
    fn set_eb(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 5, true);
        8
    }

    /// SET 5,H
    fn set_ec(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 5, true);
        8
    }

    /// SET 5,L
    fn set_ed(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 5, true);
        8
    }

    /// SET 5,(HL)
    fn set_ee(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 5, true);
        8
    }

    /// SET 5,A
    fn set_ef(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 5, true);
        8
    }

    /// SET 6,B
    fn set_f0(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 6, true);
        8
    }

    /// SET 6,C
    fn set_f1(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 6, true);
        8
    }

    /// SET 6,D
    fn set_f2(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 6, true);
        8
    }

    /// SET 6,E
    fn set_f3(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 6, true);
        8
    }

    /// SET 6,H
    fn set_f4(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 6, true);
        8
    }

    /// SET 6,L
    fn set_f5(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 6, true);
        8
    }

    /// SET 6,(HL)
    fn set_f6(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 6, true);
        8
    }

    /// SET 6,A
    fn set_f7(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 6, true);
        8
    }

    /// SET 7,B
    fn set_f8(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::B, 7, true);
        8
    }

    /// SET 7,C
    fn set_f9(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::C, 7, true);
        8
    }

    /// SET 7,D
    fn set_fa(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::D, 7, true);
        8
    }

    /// SET 7,E
    fn set_fb(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::E, 7, true);
        8
    }

    /// SET 7,H
    fn set_fc(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::H, 7, true);
        8
    }

    /// SET 7,L
    fn set_fd(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::L, 7, true);
        8
    }

    /// SET 7,(HL)
    fn set_fe(cpu: &mut Cpu) -> u8 {
        let hl = cpu.get_reg_16(Regs16::HL);
        cpu.write_bit_ram(hl, 7, true);
        8
    }

    /// SET 7,A
    fn set_ff(cpu: &mut Cpu) -> u8 {
        cpu.write_bit_n(Regs::A, 7, true);
        8
    }
}
