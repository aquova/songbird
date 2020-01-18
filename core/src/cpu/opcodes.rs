use crate::cpu::*;
use crate::utils::*;

// Set up opcode lookup table
// May god have mercy on our souls
const OPCODES: [fn(&mut Cpu) -> u8; 0x100] = [
//  $00,     $01,     $02,    $03,     $04,     $05,     $06,     $07,     $08,     $09,     $0A,    $0B,       $0C,     $0D,     $0E,     $0F
    nop,     ld_01,   ld_02,  inc_03,  inc_04,  dec_05,  ld_06,   rlca_07, ld_08,   add_09,  ld_0a,  dec_0b,    inc_0c,  dec_0d,  ld_0e,   rrca_0f, // $00
    stop_10, ld_11,   ld_12,  inc_13,  inc_14,  dec_15,  ld_16,   rla_17,  jr_18,   add_19,  ld_1a,  dec_1b,    inc_1c,  dec_1d,  ld_1e,   rra_1f,  // $10
    jr_20,   ld_21,   ld_22,  inc_23,  inc_24,  dec_25,  ld_26,   daa_27,  jr_28,   add_29,  ld_2a,  dec_2b,    inc_2c,  dec_2d,  ld_2e,   cpl_2f,  // $20
    jr_30,   ld_31,   ld_32,  inc_33,  inc_34,  dec_35,  ld_36,   scf_37,  jr_38,   add_39,  ld_3a,  dec_3b,    inc_3c,  dec_3d,  ld_3e,   ccf_3f,  // $30
    ld_40,   ld_41,   ld_42,  ld_43,   ld_44,   ld_45,   ld_46,   ld_47,   ld_48,   ld_49,   ld_4a,  ld_4b,     ld_4c,   ld_4d,   ld_4e,   ld_4f,   // $40
    ld_50,   ld_51,   ld_52,  ld_53,   ld_54,   ld_55,   ld_56,   ld_57,   ld_58,   ld_59,   ld_5a,  ld_5b,     ld_5c,   ld_5d,   ld_5e,   ld_5f,   // $50
    ld_60,   ld_61,   ld_62,  ld_63,   ld_64,   ld_65,   ld_66,   ld_67,   ld_68,   ld_69,   ld_6a,  ld_6b,     ld_6c,   ld_6d,   ld_6e,   ld_6f,   // $60
    ld_70,   ld_71,   ld_72,  ld_73,   ld_74,   ld_75,   halt_76, ld_77,   ld_78,   ld_79,   ld_7a,  ld_7b,     ld_7c,   ld_7d,   ld_7e,   ld_7f,   // $70
    add_80,  add_81,  add_82, add_83,  add_84,  add_85,  add_86,  add_87,  adc_88,  adc_89,  adc_8a, adc_8b,    adc_8c,  adc_8d,  adc_8e,  adc_8f,  // $80
    sub_90,  sub_91,  sub_92, sub_93,  sub_94,  sub_95,  sub_96,  sub_97,  sbc_98,  sbc_99,  sbc_9a, sbc_9b,    sbc_9c,  sbc_9d,  sbc_9e,  sbc_9f,  // $90
    and_a0,  and_a1,  and_a2, and_a3,  and_a4,  and_a5,  and_a6,  and_a7,  xor_a8,  xor_a9,  xor_aa, xor_ab,    xor_ac,  xor_ad,  xor_ae,  xor_af,  // $A0
    or_b0,   or_b1,   or_b2,  or_b3,   or_b4,   or_b5,   or_b6,   or_b7,   cp_b8,   cp_b9,   cp_ba,  cp_bb,     cp_bc,   cp_bd,   cp_be,   cp_bf,   // $B0
    ret_c0,  pop_c1,  jp_c2,  jp_c3,   call_c4, push_c5, add_c6,  rst_c7,  ret_c8,  ret_c9,  jp_ca,  prefix_cb, call_cc, call_cd, adc_ce,  rst_cf,  // $C0
    ret_d0,  pop_d1,  jp_d2,  invalid, call_d4, push_d5, sub_d6,  rst_d7,  ret_d8,  reti_d9, jp_da,  invalid,   call_dc, invalid, sbc_de,  rst_df,  // $D0
    ldh_e0,  pop_e1,  ld_e2,  invalid, invalid, push_e5, and_e6,  rst_e7,  add_e8,  jp_e9,   ld_ea,  invalid,   invalid, invalid, xor_ee,  rst_ef,  // $E0
    ldh_f0,  pop_f1,  ld_f2,  di_f3,   invalid, push_f5, or_f6,   rst_f7,  ld_f8,   ld_f9,   ld_fa,  ei_fb,     invalid, invalid, cp_fe,   rst_ff,  // $F0
];

const CB_OPCODES: [fn(&mut Cpu) -> u8; 0x100] = [
//  $00,      $01,       $02,       $03,       $04,       $05,       $06,       $07,       $08,      $09,       $0A,       $0B,       $0C,       $0D,       $0E,       $0F
    rlc_00,   rlc_01,    rlc_02,    rlc_03,    rlc_04,    rlc_05,    rlc_06,    rlc_07,    rrc_08,   rrc_09,    rrc_0a,    rrc_0b,    rrc_0c,    rrc_0d,    rrc_0e,    rrc_0f,  // $00
    rl_10,    rl_11,     rl_12,     rl_13,     rl_14,     rl_15,     rl_16,     rl_17,     rr_18,    rr_19,     rr_1a,     rr_1b,     rr_1c,     rr_1d,     rr_1e,     rr_1f,   // $10
    sla_20,   sla_21,    sla_22,    sla_23,    sla_24,    sla_25,    sla_26,    sla_27,    sra_28,   sra_29,    sra_2a,    sra_2b,    sra_2c,    sra_2d,    sra_2e,    sra_2f,  // $20
    swap_30,  swap_31,   swap_32,   swap_33,   swap_34,   swap_35,   swap_36,   swap_37,   srl_38,   srl_39,    srl_3a,    srl_3b,    srl_3c,    srl_3d,    srl_3e,    srl_3f,  // $30
    bit_40,   bit_41,    bit_42,    bit_43,    bit_44,    bit_45,    bit_46,    bit_47,    bit_48,   bit_49,    bit_4a,    bit_4b,    bit_4c,    bit_4d,    bit_4e,    bit_4f,  // $40
    bit_50,   bit_51,    bit_52,    bit_53,    bit_54,    bit_55,    bit_56,    bit_57,    bit_58,   bit_59,    bit_5a,    bit_5b,    bit_5c,    bit_5d,    bit_5e,    bit_5f,  // $50
    bit_60,   bit_61,    bit_62,    bit_63,    bit_64,    bit_65,    bit_66,    bit_67,    bit_68,   bit_69,    bit_6a,    bit_6b,    bit_6c,    bit_6d,    bit_6e,    bit_6f,  // $60
    bit_70,   bit_71,    bit_72,    bit_73,    bit_74,    bit_75,    bit_76,    bit_77,    bit_78,   bit_79,    bit_7a,    bit_7b,    bit_7c,    bit_7d,    bit_7e,    bit_7f,  // $70
    res_80,   res_81,    res_82,    res_83,    res_84,    res_85,    res_86,    res_87,    res_88,   res_89,    res_8a,    res_8b,    res_8c,    res_8d,    res_8e,    res_8f,  // $80
    res_90,   res_91,    res_92,    res_93,    res_94,    res_95,    res_96,    res_97,    res_98,   res_99,    res_9a,    res_9b,    res_9c,    res_9d,    res_9e,    res_9f,  // $90
    res_a0,   res_a1,    res_a2,    res_a3,    res_a4,    res_a5,    res_a6,    res_a7,    res_a8,   res_a9,    res_aa,    res_ab,    res_ac,    res_ad,    res_ae,    res_af,  // $A0
    res_b0,   res_b1,    res_b2,    res_b3,    res_b4,    res_b5,    res_b6,    res_b7,    res_b8,   res_b9,    res_ba,    res_bb,    res_bc,    res_bd,    res_be,    res_bf,  // $B0
    set_c0,   set_c1,    set_c2,    set_c3,    set_c4,    set_c5,    set_c6,    set_c7,    set_c8,   set_c9,    set_ca,    set_cb,    set_cc,    set_cd,    set_ce,    set_cf,  // $C0
    set_d0,   set_d1,    set_d2,    set_d3,    set_d4,    set_d5,    set_d6,    set_d7,    set_d8,   set_d9,    set_da,    set_db,    set_dc,    set_dd,    set_de,    set_df,  // $D0
    set_e0,   set_e1,    set_e2,    set_e3,    set_e4,    set_e5,    set_e6,    set_e7,    set_e8,   set_e9,    set_ea,    set_eb,    set_ec,    set_ed,    set_ee,    set_ef,  // $E0
    set_f0,   set_f1,    set_f2,    set_f3,    set_f4,    set_f5,    set_f6,    set_f7,    set_f8,   set_f9,    set_fa,    set_fb,    set_fc,    set_fd,    set_fe,    set_ff,  // $F0
];

pub fn execute(cpu: &mut Cpu) -> u8 {
    let opcode = cpu.fetch();
    // If opcode is $CB, then use other opcode table
    if opcode == 0xCB {
        CB_OPCODES[opcode as usize](cpu)
    } else {
        OPCODES[opcode as usize](cpu)
    }
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
    let low = cpu.fetch();
    let high = cpu.fetch();
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
    cpu.rot_left_reg(Regs::A, false);
    4
}

/// LD (a16), SP
fn ld_08(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let addr = merge_bytes(high, low);
    let sp = cpu.get_sp();
    cpu.write_ram(addr, sp.get_low_byte());
    cpu.write_ram(addr + 1, sp.get_high_byte());
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
    cpu.rot_right_reg(Regs::A, false);
    4
}

/// STOP 0
fn stop_10(_cpu: &mut Cpu) -> u8 {
    // I'm not sure how to implement this
    4
}

/// LD DE, d16
fn ld_11(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
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
    cpu.rot_left_reg(Regs::A, true);
    4
}

/// JR r8
fn jr_18(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    let mut pc = cpu.get_pc();
    pc = pc.wrapping_add(offset as u16);
    cpu.set_pc(pc);
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
    cpu.rot_right_reg(Regs::A, true);
    4
}

/// JR NZ, r8
fn jr_20(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    // Add offset value as signed 8-bit value
    let signed = offset as i8 as i16 as u16;
    if !cpu.get_flag(Flags::Z) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        12
    } else {
        8
    }
}

/// LD HL, d16
fn ld_21(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let val = merge_bytes(high, low);
    cpu.ld_nn_d16(Regs16::HL, val);
    12
}

/// LD (HL+), A
fn ld_22(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(hl, val);
    cpu.inc_16(Regs16::HL);
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
    let signed = offset as i8 as i16 as u16;
    if cpu.get_flag(Flags::Z) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
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
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::A, val);
    cpu.inc_16(Regs16::HL);
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
    let val = cpu.get_reg(Regs::A);
    cpu.set_reg(Regs::A, !val);
    cpu.set_flag(Flags::N);
    cpu.set_flag(Flags::H);
    4
}

/// JR NC, r8
fn jr_30(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    let signed = offset as i8 as i16 as u16;
    if !cpu.get_flag(Flags::C) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        12
    } else {
        8
    }
}

/// LD SP, d16
fn ld_31(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    cpu.set_sp(merge_bytes(high, low));
    12
}

/// LD (HL-), A
fn ld_32(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(hl, val);
    cpu.dec_16(Regs16::HL);
    8
}

/// INC SP
fn inc_33(cpu: &mut Cpu) -> u8 {
    // May need to check for flags
    let sp = cpu.get_sp();
    cpu.set_sp(sp + 1);
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
    let signed = offset as i8 as i16 as u16;
    if cpu.get_flag(Flags::C) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        12
    } else {
        8
    }
}

/// ADD HL, SP
fn add_39(cpu: &mut Cpu) -> u8 {
    let sp = cpu.get_sp();
    cpu.add_nn_d16(Regs16::HL, sp);
    8
}

/// LD A, (HL-)
fn ld_3a(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::A, val);
    cpu.dec_16(Regs16::HL);
    8
}

/// DEC SP
fn dec_3b(cpu: &mut Cpu) -> u8 {
    let sp = cpu.get_sp();
    cpu.set_sp(sp - 1);
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
        cpu.set_pc(addr);
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
        cpu.set_pc(offset);
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
    cpu.set_pc(offset);
    16
}

/// CALL NZ, a16
fn call_c4(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    if !cpu.get_flag(Flags::Z) {
        let addr = merge_bytes(high, low);
        cpu.push(cpu.get_pc());
        cpu.set_pc(addr);
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0000);
    16
}

/// RET Z
fn ret_c8(cpu: &mut Cpu) -> u8 {
    if cpu.get_flag(Flags::Z) {
        let addr = cpu.pop();
        cpu.set_pc(addr);
        20
    } else {
        8
    }
}

/// RET
fn ret_c9(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_pc(val);
    16
}

/// JP Z, a16
fn jp_ca(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    if cpu.get_flag(Flags::Z) {
        cpu.set_pc(offset);
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
    let low = cpu.fetch();
    let high = cpu.fetch();
    if cpu.get_flag(Flags::Z) {
        let addr = merge_bytes(high, low);
        cpu.push(cpu.get_pc());
        cpu.set_pc(addr);
        24
    } else {
        12
    }
}

/// CALL a16
fn call_cd(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let addr = merge_bytes(high, low);
    cpu.push(cpu.get_pc());
    cpu.set_pc(addr);
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0008);
    16
}

/// RET NC
fn ret_d0(cpu: &mut Cpu) -> u8 {
    if !cpu.get_flag(Flags::C) {
        let val = cpu.pop();
        cpu.set_pc(val);
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
        cpu.set_pc(offset);
        16
    } else {
        12
    }
}

/// CALL NC, a16
fn call_d4(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    if !cpu.get_flag(Flags::C) {
        let addr = merge_bytes(high, low);
        cpu.push(cpu.get_pc());
        cpu.set_pc(addr);
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0010);
    16
}

/// RET C
fn ret_d8(cpu: &mut Cpu) -> u8 {
    if cpu.get_flag(Flags::C) {
        let val = cpu.pop();
        cpu.set_pc(val);
        20
    } else {
        8
    }
}

/// RETI
fn reti_d9(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_pc(val);
    cpu.interupt = true;
    16
}

/// JP C, a16
fn jp_da(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    if cpu.get_flag(Flags::C) {
        cpu.set_pc(offset);
        16
    } else {
        12
    }
}

/// CALL C, a16
fn call_dc(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    if cpu.get_flag(Flags::C) {
        let addr = merge_bytes(high, low);
        cpu.push(cpu.get_pc());
        cpu.set_pc(addr);
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0018);
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0020);
    16
}

/// ADD SP, r8
fn add_e8(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    let signed = val as i8 as i16 as u16;
    let sp = cpu.get_sp();
    let result = sp.overflowing_add(signed);
    let set_h = check_h_carry_u16(sp, signed);
    cpu.set_sp(result.0);

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
    cpu.set_pc(val as u16);
    4
}

/// LD (a16), A
fn ld_ea(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0028);
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0030);
    16
}

/// LD HL, SP+r8
/// Put SP + n into HL
fn ld_f8(cpu: &mut Cpu) -> u8 {
    let n = cpu.fetch();
    let sp = cpu.get_sp();
    cpu.set_reg_16(Regs16::HL, sp + n as u16);
    12
}

/// LD SP, HL
fn ld_f9(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.set_sp(hl);
    8
}

/// LD A, (a16)
fn ld_fa(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
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
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0038);
    16
}

/*
 * ----------------
 * $CB Opcode block
 * ----------------
 */

/// RLC B
fn rlc_00(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::B, false);
    8
}

/// RLC C
fn rlc_01(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::C, false);
    8
}

/// RLC D
fn rlc_02(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::D, false);
    8
}

/// RLC E
fn rlc_03(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::E, false);
    8
}

/// RLC H
fn rlc_04(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::H, false);
    8
}

/// RLC L
fn rlc_05(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::L, false);
    8
}

/// RLC (HL)
fn rlc_06(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let byte = cpu.read_ram(hl);
    let rot = cpu.rot_left(byte, false);
    cpu.write_ram(hl, rot);
    8
}

/// RLC A
fn rlc_07(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::A, false);
    8
}

/// RRC B
fn rrc_08(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::B, false);
    8
}

/// RRC C
fn rrc_09(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::C, false);
    8
}

/// RRC D
fn rrc_0a(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::D, false);
    8
}

/// RRC E
fn rrc_0b(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::E, false);
    8
}

/// RRC H
fn rrc_0c(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::H, false);
    8
}

/// RRC L
fn rrc_0d(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::L, false);
    8
}

/// RRC (HL)
fn rrc_0e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let byte = cpu.read_ram(hl);
    let rot = cpu.rot_right(byte, false);
    cpu.write_ram(hl, rot);
    8
}

/// RRC A
fn rrc_0f(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::A, false);
    8
}

/// RL B
fn rl_10(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::B, true);
    8
}

/// RL C
fn rl_11(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::C, true);
    8
}

/// RL D
fn rl_12(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::D, true);
    8
}

/// RL E
fn rl_13(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::E, true);
    8
}

/// RL H
fn rl_14(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::H, true);
    8
}

/// RL L
fn rl_15(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::L, true);
    8
}

/// RL (HL)
fn rl_16(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let byte = cpu.read_ram(hl);
    let rot = cpu.rot_left(byte, true);
    cpu.write_ram(hl, rot);
    8
}

/// RL A
fn rl_17(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::A, true);
    8
}

/// RR B
fn rr_18(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::B, true);
    8
}

/// RR C
fn rr_19(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::C, true);
    8
}

/// RR D
fn rr_1a(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::D, true);
    8
}

/// RR E
fn rr_1b(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::E, true);
    8
}

/// RR H
fn rr_1c(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::H, true);
    8
}

/// RR L
fn rr_1d(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::L, true);
    8
}

/// RR (HL)
fn rr_1e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let byte = cpu.read_ram(hl);
    let rot = cpu.rot_right(byte, true);
    cpu.write_ram(hl, rot);
    8
}

/// RR A
fn rr_1f(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::A, true);
    8
}

/// SLA B
fn sla_20(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::B);
    8
}

/// SLA C
fn sla_21(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::C);
    8
}

/// SLA D
fn sla_22(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::D);
    8
}

/// SLA E
fn sla_23(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::E);
    8
}

/// SLA H
fn sla_24(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::H);
    8
}

/// SLA L
fn sla_25(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::L);
    8
}

/// SLA (HL)
fn sla_26(cpu: &mut Cpu) -> u8 {
    let addr = cpu.get_reg_16(Regs16::HL);
    let byte = cpu.read_ram(addr);
    let shifted = cpu.shift_left(byte);
    cpu.write_ram(addr, shifted);
    8
}

/// SLA A
fn sla_27(cpu: &mut Cpu) -> u8 {
    cpu.shift_left_reg(Regs::A);
    8
}

/// SRA B
fn sra_28(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::B, true);
    8
}

/// SRA C
fn sra_29(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::C, true);
    8
}

/// SRA D
fn sra_2a(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::D, true);
    8
}

/// SRA E
fn sra_2b(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::E, true);
    8
}

/// SRA H
fn sra_2c(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::H, true);
    8
}

/// SRA L
fn sra_2d(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::L, true);
    8
}

/// SRA (HL)
fn sra_2e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    let shifted = cpu.shift_right(val, true);
    cpu.write_ram(hl, shifted);
    8
}

/// SRA A
fn sra_2f(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::A, true);
    8
}

/// SWAP B
fn swap_30(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::B);
    8
}

/// SWAP C
fn swap_31(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::C);
    8
}

/// SWAP D
fn swap_32(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::D);
    8
}

/// SWAP E
fn swap_33(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::E);
    8
}

/// SWAP H
fn swap_34(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::H);
    8
}

/// SWAP L
fn swap_35(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::L);
    8
}

/// SWAP (HL)
fn swap_36(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    let swapped = cpu.swap_bits(val);
    cpu.write_ram(hl, swapped);
    8
}

/// SWAP A
fn swap_37(cpu: &mut Cpu) -> u8 {
    cpu.swap_bits_reg(Regs::A);
    8
}

/// SRL B
fn srl_38(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::B, false);
    8
}

/// SRL C
fn srl_39(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::C, false);
    8
}

/// SRL D
fn srl_3a(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::D, false);
    8
}

/// SRL E
fn srl_3b(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::E, false);
    8
}

/// SRL H
fn srl_3c(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::H, false);
    8
}

/// SRL L
fn srl_3d(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::L, false);
    8
}

/// SRL (HL)
fn srl_3e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    let shifted = cpu.shift_right(val, false);
    cpu.write_ram(hl, shifted);
    8
}

/// SRL A
fn srl_3f(cpu: &mut Cpu) -> u8 {
    cpu.shift_right_reg(Regs::A, false);
    8
}

/// BIT 0,B
fn bit_40(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 0);
    8
}

/// BIT 0,C
fn bit_41(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 0);
    8
}

/// BIT 0,D
fn bit_42(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 0);
    8
}

/// BIT 0,E
fn bit_43(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 0);
    8
}

/// BIT 0,H
fn bit_44(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 0);
    8
}

/// BIT 0,L
fn bit_45(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 0);
    8
}

/// BIT 0,(HL)
fn bit_46(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 0);
    8
}

/// BIT 0,A
fn bit_47(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 0);
    8
}

/// BIT 1,B
fn bit_48(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 1);
    8
}

/// BIT 1,C
fn bit_49(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 1);
    8
}

/// BIT 1,D
fn bit_4a(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 1);
    8
}

/// BIT 1,E
fn bit_4b(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 1);
    8
}

/// BIT 1,H
fn bit_4c(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 1);
    8
}

/// BIT 1,L
fn bit_4d(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 1);
    8
}

/// BIT 1,(HL)
fn bit_4e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 1);
    8
}

/// BIT 1,A
fn bit_4f(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 1);
    8
}

/// BIT 2,B
fn bit_50(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 2);
    8
}

/// BIT 2,C
fn bit_51(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 2);
    8
}

/// BIT 2,D
fn bit_52(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 2);
    8
}

/// BIT 2,E
fn bit_53(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 2);
    8
}

/// BIT 2,H
fn bit_54(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 2);
    8
}

/// BIT 2,L
fn bit_55(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 2);
    8
}

/// BIT 2,(HL)
fn bit_56(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 2);
    8
}

/// BIT 2,A
fn bit_57(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 2);
    8
}

/// BIT 3,B
fn bit_58(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 3);
    8
}

/// BIT 3,C
fn bit_59(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 3);
    8
}

/// BIT 3,D
fn bit_5a(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 3);
    8
}

/// BIT 3,E
fn bit_5b(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 3);
    8
}

/// BIT 3,H
fn bit_5c(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 3);
    8
}

/// BIT 3,L
fn bit_5d(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 3);
    8
}

/// BIT 3,(HL)
fn bit_5e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 3);
    8
}

/// BIT 3,A
fn bit_5f(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 3);
    8
}

/// BIT 4,B
fn bit_60(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 4);
    8
}

/// BIT 4,C
fn bit_61(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 4);
    8
}

/// BIT 4,D
fn bit_62(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 4);
    8
}

/// BIT 4,E
fn bit_63(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 4);
    8
}

/// BIT 4,H
fn bit_64(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 4);
    8
}

/// BIT 4,L
fn bit_65(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 4);
    8
}

/// BIT 4,(HL)
fn bit_66(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 4);
    8
}

/// BIT 4,A
fn bit_67(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 4);
    8
}

/// BIT 5,B
fn bit_68(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 5);
    8
}

/// BIT 5,C
fn bit_69(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 5);
    8
}

/// BIT 5,D
fn bit_6a(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 5);
    8
}

/// BIT 5,E
fn bit_6b(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 5);
    8
}

/// BIT 5,H
fn bit_6c(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 5);
    8
}

/// BIT 5,L
fn bit_6d(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 5);
    8
}

/// BIT 5,(HL)
fn bit_6e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 5);
    8
}

/// BIT 5,A
fn bit_6f(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 5);
    8
}

/// BIT 6,B
fn bit_70(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 6);
    8
}

/// BIT 6,C
fn bit_71(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 6);
    8
}

/// BIT 6,D
fn bit_72(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 6);
    8
}

/// BIT 6,E
fn bit_73(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 6);
    8
}

/// BIT 6,H
fn bit_74(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 6);
    8
}

/// BIT 6,L
fn bit_75(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 6);
    8
}

/// BIT 6,(HL)
fn bit_76(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 6);
    8
}

/// BIT 6,A
fn bit_77(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 6);
    8
}

/// BIT 7,B
fn bit_78(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::B, 7);
    8
}

/// BIT 7,C
fn bit_79(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::C, 7);
    8
}

/// BIT 7,D
fn bit_7a(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::D, 7);
    8
}

/// BIT 7,E
fn bit_7b(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::E, 7);
    8
}

/// BIT 7,H
fn bit_7c(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::H, 7);
    8
}

/// BIT 7,L
fn bit_7d(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::L, 7);
    8
}

/// BIT 7,(HL)
fn bit_7e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.test_bit(val, 7);
    8
}

/// BIT 7,A
fn bit_7f(cpu: &mut Cpu) -> u8 {
    cpu.test_bit_reg(Regs::A, 7);
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
