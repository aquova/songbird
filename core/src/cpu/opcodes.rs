use crate::cpu::*;
use crate::utils::*;

// Set up opcode lookup table
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

pub fn execute(cpu: &mut Cpu) -> u8 {
    let opcode = cpu.fetch();
    // If opcode is $CB, then use other opcode table
    if opcode == 0xCB {
        let cb_opcode = cpu.fetch();
        execute_cb_op(cpu, cb_opcode)
    } else {
        OPCODES[opcode as usize](cpu)
    }
}

fn invalid(_cpu: &mut Cpu) -> u8 {
    panic!("Invalid opcode");
}

/// NOP
/// ----
fn nop(_cpu: &mut Cpu) -> u8 {
    1
}

/// LD BC, d16
/// ----
fn ld_01(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let val = merge_bytes(high, low);
    cpu.ld_nn_d16(Regs16::BC, val);
    3
}

/// LD (BC), A
/// ----
fn ld_02(cpu: &mut Cpu) -> u8 {
    let bc = cpu.get_reg_16(Regs16::BC);
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(bc, val);
    2
}

/// INC BC
/// ----
fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_16(Regs16::BC);
    2
}

/// INC B
/// Z0H-
fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::B);
    1
}

/// DEC B
/// Z1H-
fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::B);
    1
}

/// LD B, d8
/// ----
fn ld_06(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::B, byte);
    2
}

/// RLCA
/// 000C
fn rlca_07(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::A, false);
    // RLCA wants Z to be cleared (unlike other shift ops)
    cpu.clear_flag(Flags::Z);
    1
}

/// LD (a16), SP
/// ----
fn ld_08(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let addr = merge_bytes(high, low);
    let sp = cpu.get_sp();
    cpu.write_ram(addr, sp.get_low_byte());
    cpu.write_ram(addr + 1, sp.get_high_byte());
    5
}

/// ADD HL, BC
/// -0HC
fn add_09(cpu: &mut Cpu) -> u8 {
    let bc = cpu.get_reg_16(Regs16::BC);
    cpu.add_nn_d16(Regs16::HL, bc);
    2
}

/// LD A, (BC)
/// ----
fn ld_0a(cpu: &mut Cpu) -> u8 {
    let bc = cpu.get_reg_16(Regs16::BC);
    let val = cpu.read_ram(bc);
    cpu.ld_n_d8(Regs::A, val);
    2
}

/// DEC BC
/// ----
fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_16(Regs16::BC);
    2
}

/// INC C
/// Z0H-
fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::C);
    1
}

/// DEC C
/// Z1H-
fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::C);
    1
}

/// LD C, d8
/// ----
fn ld_0e(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::C, byte);
    2
}

/// RRCA
/// 000C
fn rrca_0f(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::A, false);
    // RRCA wants Z to be cleared (unlike other shift ops)
    cpu.clear_flag(Flags::Z);
    1
}

/// STOP
/// ----
fn stop_10(_cpu: &mut Cpu) -> u8 {
    // Do nothing?
    1
}

/// LD DE, d16
/// ----
fn ld_11(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let val = merge_bytes(high, low);
    cpu.ld_nn_d16(Regs16::DE, val);
    3
}

/// LD (DE), A
/// ----
fn ld_12(cpu: &mut Cpu) -> u8 {
    let de = cpu.get_reg_16(Regs16::DE);
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(de, val);
    2
}

/// INC DE
/// ----
fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_16(Regs16::DE);
    2
}

/// INC D
/// Z0H-
fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::D);
    1
}

/// DEC D
/// Z1H-
fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::D);
    1
}

/// LD D, d8
/// ----
fn ld_16(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::D, byte);
    2
}

/// RLA
/// 000C
fn rla_17(cpu: &mut Cpu) -> u8 {
    cpu.rot_left_reg(Regs::A, true);
    // RLA wants Z to be cleared (unlike other shift ops)
    cpu.clear_flag(Flags::Z);
    1
}

/// JR r8
/// ----
fn jr_18(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as i8;
    let mut pc = cpu.get_pc();
    pc = pc.wrapping_add(offset as u16);
    cpu.set_pc(pc);
    3
}

/// ADD HL, DE
/// -0HC
fn add_19(cpu: &mut Cpu) -> u8 {
    let de = cpu.get_reg_16(Regs16::DE);
    cpu.add_nn_d16(Regs16::HL, de);
    2
}

/// LD A, (DE)
/// ----
fn ld_1a(cpu: &mut Cpu) -> u8 {
    let de = cpu.get_reg_16(Regs16::DE);
    let val = cpu.read_ram(de);
    cpu.set_reg(Regs::A, val);
    2
}

/// DEC DE
/// ----
fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_16(Regs16::DE);
    2
}

/// INC E
/// Z0H-
fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::E);
    1
}

/// DEC E
/// Z1H-
fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::E);
    1
}

/// LD E, d8
/// ----
fn ld_1e(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::E, byte);
    2
}

/// RRA
/// 000C
fn rra_1f(cpu: &mut Cpu) -> u8 {
    cpu.rot_right_reg(Regs::A, true);
    // RRA wants Z to be cleared (unlike other shift ops)
    cpu.clear_flag(Flags::Z);
    1
}

/// JR NZ, r8
/// ----
fn jr_20(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    // Add offset value as signed 8-bit value
    let signed = offset as i8 as i16 as u16;
    if !cpu.get_flag(Flags::Z) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        3
    } else {
        2
    }
}

/// LD HL, d16
/// ----
fn ld_21(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let val = merge_bytes(high, low);
    cpu.ld_nn_d16(Regs16::HL, val);
    3
}

/// LD (HL+), A
/// ----
fn ld_22(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(hl, val);
    cpu.inc_16(Regs16::HL);
    2
}

/// INC HL
/// ----
fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_16(Regs16::HL);
    2
}

/// INC H
/// Z0H-
fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::H);
    1
}

/// DEC H
/// Z1H-
fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::H);
    1
}

/// LD H, d8
/// ----
fn ld_26(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::H, byte);
    2
}

/// DAA
/// Z-0C
fn daa_27(cpu: &mut Cpu) -> u8 {
    cpu.daa();
    1
}

/// JR Z, r8
fn jr_28(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    let signed = offset as i8 as i16 as u16;
    if cpu.get_flag(Flags::Z) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        3
    } else {
        2
    }
}

/// ADD HL, HL
fn add_29(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.add_nn_d16(Regs16::HL, hl);
    2
}

/// LD A, (HL+)
fn ld_2a(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::A, val);
    cpu.inc_16(Regs16::HL);
    2
}

/// DEC HL
fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_16(Regs16::HL);
    2
}

/// INC L
fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::L);
    1
}

/// DEC L
fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::L);
    1
}

/// LD L, d8
fn ld_2e(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::L, byte);
    2
}

/// CPL
fn cpl_2f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.set_reg(Regs::A, !val);
    cpu.set_flag(Flags::N);
    cpu.set_flag(Flags::H);
    1
}

/// JR NC, r8
fn jr_30(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    let signed = offset as i8 as i16 as u16;
    if !cpu.get_flag(Flags::C) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        3
    } else {
        2
    }
}

/// LD SP, d16
fn ld_31(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    cpu.set_sp(merge_bytes(high, low));
    3
}

/// LD (HL-), A
fn ld_32(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(hl, val);
    cpu.dec_16(Regs16::HL);
    2
}

/// INC SP
fn inc_33(cpu: &mut Cpu) -> u8 {
    let sp = cpu.get_sp();
    cpu.set_sp(sp.wrapping_add(1));
    2
}

/// INC (HL)
fn inc_34(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    let new_val = val.wrapping_add(1);
    cpu.write_ram(hl, new_val);

    let set_h = check_h_carry_u8(val, 1);
    cpu.write_flag(Flags::Z, new_val == 0);
    cpu.clear_flag(Flags::N);
    cpu.write_flag(Flags::H, set_h);
    3
}

/// DEC (HL)
fn dec_35(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    let new_val = val.wrapping_sub(1);
    cpu.write_ram(hl, new_val);

    let set_h = check_h_borrow_u8(val, 1);
    cpu.write_flag(Flags::Z, new_val == 0);
    cpu.set_flag(Flags::N);
    cpu.write_flag(Flags::H, set_h);
    3
}

/// LD (HL), d8
fn ld_36(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.fetch();
    cpu.write_ram(hl, val);
    3
}

/// SCF
fn scf_37(cpu: &mut Cpu) -> u8 {
    cpu.set_flag(Flags::C);
    cpu.clear_flag(Flags::H);
    cpu.clear_flag(Flags::N);
    1
}

/// JR C, r8
fn jr_38(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch();
    let signed = offset as i8 as i16 as u16;
    if cpu.get_flag(Flags::C) {
        let mut pc = cpu.get_pc();
        pc = pc.wrapping_add(signed);
        cpu.set_pc(pc);
        3
    } else {
        2
    }
}

/// ADD HL, SP
fn add_39(cpu: &mut Cpu) -> u8 {
    let sp = cpu.get_sp();
    cpu.add_nn_d16(Regs16::HL, sp);
    2
}

/// LD A, (HL-)
fn ld_3a(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::A, val);
    cpu.dec_16(Regs16::HL);
    2
}

/// DEC SP
fn dec_3b(cpu: &mut Cpu) -> u8 {
    let sp = cpu.get_sp();
    cpu.set_sp(sp.wrapping_sub(1));
    2
}

/// INC A
fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_8(Regs::A);
    1
}

/// DEC A
fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_8(Regs::A);
    1
}

/// LD A, d8
fn ld_3e(cpu: &mut Cpu) -> u8 {
    let byte = cpu.fetch();
    cpu.ld_n_d8(Regs::A, byte);
    2
}

/// CCF
fn ccf_3f(cpu: &mut Cpu) -> u8 {
    cpu.clear_flag(Flags::N);
    cpu.clear_flag(Flags::H);
    let cf = cpu.get_flag(Flags::C);
    cpu.write_flag(Flags::C, !cf);
    1
}

/// LD B, B
fn ld_40(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD B, C
fn ld_41(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD B, D
fn ld_42(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD B, E
fn ld_43(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD B, H
fn ld_44(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD B, L
fn ld_45(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD B, (HL)
fn ld_46(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::B, val);
    2
}

/// LD B, A
fn ld_47(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::B, byte);
    1
}

/// LD C, B
fn ld_48(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD C, C
fn ld_49(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD C, D
fn ld_4a(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD C, E
fn ld_4b(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD C, H
fn ld_4c(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD C, L
fn ld_4d(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD C, (HL)
fn ld_4e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::C, val);
    2
}

/// LD C, A
fn ld_4f(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::C, byte);
    1
}

/// LD D, B
fn ld_50(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD D, C
fn ld_51(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD D, D
fn ld_52(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD D, E
fn ld_53(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD D, H
fn ld_54(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD D, L
fn ld_55(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD D, (HL)
fn ld_56(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::D, val);
    2
}

/// LD D, A
fn ld_57(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::D, byte);
    1
}

/// LD E, B
fn ld_58(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD E, C
fn ld_59(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD E, D
fn ld_5a(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD E, E
fn ld_5b(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD E, H
fn ld_5c(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD E, L
fn ld_5d(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD E, (HL)
fn ld_5e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::E, val);
    2
}

/// LD E, A
fn ld_5f(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::E, byte);
    1
}

/// LD H, B
fn ld_60(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD H, C
fn ld_61(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD H, D
fn ld_62(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD H, E
fn ld_63(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD H, H
fn ld_64(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD H, L
fn ld_65(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD H, (HL)
fn ld_66(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::H, val);
    2
}

/// LD H, A
fn ld_67(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::H, byte);
    1
}

/// LD L, B
fn ld_68(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD L, C
fn ld_69(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD L, D
fn ld_6a(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD L, E
fn ld_6b(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD L, H
fn ld_6c(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD L, L
fn ld_6d(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD L, (HL)
fn ld_6e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::L, val);
    2
}

/// LD L, A
fn ld_6f(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::L, byte);
    1
}

/// LD (HL), B
fn ld_70(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// LD (HL), C
fn ld_71(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// LD (HL), D
fn ld_72(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// LD (HL), E
fn ld_73(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// LD (HL), H
fn ld_74(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// LD (HL), L
fn ld_75(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// HALT
fn halt_76(cpu: &mut Cpu) -> u8 {
    cpu.halted = true;
    1
}

/// LD (HL), A
fn ld_77(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.write_ram(hl, val);
    2
}

/// LD A, B
fn ld_78(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::B);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// LD A, C
fn ld_79(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::C);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// LD A, D
fn ld_7a(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::D);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// LD A, E
fn ld_7b(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::E);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// LD A, H
fn ld_7c(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::H);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// LD A, L
fn ld_7d(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::L);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// LD A, (HL)
fn ld_7e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.set_reg(Regs::A, val);
    2
}

/// LD A, A
fn ld_7f(cpu: &mut Cpu) -> u8 {
    let byte = cpu.get_reg(Regs::A);
    cpu.ld_n_d8(Regs::A, byte);
    1
}

/// ADD A, B
fn add_80(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.add_a_d8(val, false);
    1
}

/// ADD A, C
fn add_81(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.add_a_d8(val, false);
    1
}

/// ADD A, D
fn add_82(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.add_a_d8(val, false);
    1
}

/// ADD A, E
fn add_83(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.add_a_d8(val, false);
    1
}

/// ADD A, H
fn add_84(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.add_a_d8(val, false);
    1
}

/// ADD A, L
fn add_85(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.add_a_d8(val, false);
    1
}

/// ADD A, (HL)
fn add_86(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.add_a_d8(val, false);
    2
}

/// ADD A, A
fn add_87(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.add_a_d8(val, false);
    1
}

/// ADC A, B
fn adc_88(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.add_a_d8(val, true);
    1
}

/// ADC A, C
fn adc_89(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.add_a_d8(val, true);
    1
}

/// ADC A, D
fn adc_8a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.add_a_d8(val, true);
    1
}

/// ADC A, E
fn adc_8b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.add_a_d8(val, true);
    1
}

/// ADC A, H
fn adc_8c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.add_a_d8(val, true);
    1
}

/// ADC A, L
fn adc_8d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.add_a_d8(val, true);
    1
}

/// ADC A, (HL)
fn adc_8e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.add_a_d8(val, true);
    2
}

/// ADC A, A
fn adc_8f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.add_a_d8(val, true);
    1
}

/// SUB B
fn sub_90(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.sub_a_d8(val, false);
    1
}

/// SUB C
fn sub_91(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.sub_a_d8(val, false);
    1
}

/// SUB D
fn sub_92(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.sub_a_d8(val, false);
    1
}

/// SUB E
fn sub_93(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.sub_a_d8(val, false);
    1
}

/// SUB H
fn sub_94(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.sub_a_d8(val, false);
    1
}

/// SUB L
fn sub_95(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.sub_a_d8(val, false);
    1
}

/// SUB (HL)
fn sub_96(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.sub_a_d8(val, false);
    2
}

/// SUB A
fn sub_97(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.sub_a_d8(val, false);
    1
}

/// SBC A, B
fn sbc_98(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.sub_a_d8(val, true);
    1
}

/// SBC A, C
fn sbc_99(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.sub_a_d8(val, true);
    1
}

/// SBC A, D
fn sbc_9a(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.sub_a_d8(val, true);
    1
}

/// SBC A, E
fn sbc_9b(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.sub_a_d8(val, true);
    1
}

/// SBC A, H
fn sbc_9c(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.sub_a_d8(val, true);
    1
}

/// SBC A, L
fn sbc_9d(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.sub_a_d8(val, true);
    1
}

/// SBC A, (HL)
fn sbc_9e(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.sub_a_d8(val, true);
    2
}

/// SBC A, A
fn sbc_9f(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.sub_a_d8(val, true);
    1
}

/// AND B
fn and_a0(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.and_a_d8(val);
    1
}

/// AND C
fn and_a1(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.and_a_d8(val);
    1
}

/// AND D
fn and_a2(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.and_a_d8(val);
    1
}

/// AND E
fn and_a3(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.and_a_d8(val);
    1
}

/// AND H
fn and_a4(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.and_a_d8(val);
    1
}

/// AND L
fn and_a5(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.and_a_d8(val);
    1
}

/// AND (HL)
fn and_a6(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.and_a_d8(val);
    2
}

/// AND A
fn and_a7(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.and_a_d8(val);
    1
}

/// XOR B
fn xor_a8(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.xor_a_d8(val);
    1
}

/// XOR C
fn xor_a9(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.xor_a_d8(val);
    1
}

/// XOR D
fn xor_aa(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.xor_a_d8(val);
    1
}

/// XOR E
fn xor_ab(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.xor_a_d8(val);
    1
}

/// XOR H
fn xor_ac(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.xor_a_d8(val);
    1
}

/// XOR L
fn xor_ad(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.xor_a_d8(val);
    1
}

/// XOR (HL)
fn xor_ae(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.xor_a_d8(val);
    2
}

/// XOR A
fn xor_af(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.xor_a_d8(val);
    1
}

/// OR B
fn or_b0(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.or_a_d8(val);
    1
}

/// OR C
fn or_b1(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.or_a_d8(val);
    1
}

/// OR D
fn or_b2(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.or_a_d8(val);
    1
}

/// OR E
fn or_b3(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.or_a_d8(val);
    1
}

/// OR H
fn or_b4(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.or_a_d8(val);
    1
}

/// OR L
fn or_b5(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.or_a_d8(val);
    1
}

/// OR (HL)
fn or_b6(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.or_a_d8(val);
    2
}

/// OR A
fn or_b7(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.or_a_d8(val);
    1
}

/// CP B
fn cp_b8(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::B);
    cpu.cp_a_d8(val);
    1
}

/// CP C
fn cp_b9(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::C);
    cpu.cp_a_d8(val);
    1
}

/// CP D
fn cp_ba(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::D);
    cpu.cp_a_d8(val);
    1
}

/// CP E
fn cp_bb(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::E);
    cpu.cp_a_d8(val);
    1
}

/// CP H
fn cp_bc(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::H);
    cpu.cp_a_d8(val);
    1
}

/// CP L
fn cp_bd(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::L);
    cpu.cp_a_d8(val);
    1
}

/// CP (HL)
fn cp_be(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    let val = cpu.read_ram(hl);
    cpu.cp_a_d8(val);
    2
}

/// CP A
fn cp_bf(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_reg(Regs::A);
    cpu.cp_a_d8(val);
    1
}

/// RET NZ
fn ret_c0(cpu: &mut Cpu) -> u8 {
    if !cpu.get_flag(Flags::Z) {
        let addr = cpu.pop();
        cpu.set_pc(addr);
        5
    } else {
        2
    }
}

/// POP BC
fn pop_c1(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_reg_16(Regs16::BC, val);
    3
}

/// JP NZ, a16
fn jp_c2(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    if !cpu.get_flag(Flags::Z) {
        cpu.set_pc(offset);
        4
    } else {
        3
    }
}

/// JP a16
fn jp_c3(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    cpu.set_pc(offset);
    4
}

/// CALL NZ, a16
fn call_c4(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    if !cpu.get_flag(Flags::Z) {
        let addr = merge_bytes(high, low);
        cpu.push(cpu.get_pc());
        cpu.set_pc(addr);
        6
    } else {
        3
    }
}

/// PUSH BC
fn push_c5(cpu: &mut Cpu) -> u8 {
    let bc = cpu.get_reg_16(Regs16::BC);
    cpu.push(bc);
    4
}

/// ADD A, d8
fn add_c6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.add_a_d8(val, false);
    2
}

/// RST 00
/// Push PC onto stack
/// Jump to $0000 + $00
fn rst_c7(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0000);
    4
}

/// RET Z
fn ret_c8(cpu: &mut Cpu) -> u8 {
    if cpu.get_flag(Flags::Z) {
        let addr = cpu.pop();
        cpu.set_pc(addr);
        5
    } else {
        2
    }
}

/// RET
fn ret_c9(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_pc(val);
    4
}

/// JP Z, a16
fn jp_ca(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    if cpu.get_flag(Flags::Z) {
        cpu.set_pc(offset);
        4
    } else {
        3
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
        6
    } else {
        3
    }
}

/// CALL a16
fn call_cd(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let addr = merge_bytes(high, low);
    cpu.push(cpu.get_pc());
    cpu.set_pc(addr);
    6
}

/// ADC A, d8
fn adc_ce(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.add_a_d8(val, true);
    2
}

/// RST 08
fn rst_cf(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0008);
    4
}

/// RET NC
fn ret_d0(cpu: &mut Cpu) -> u8 {
    if !cpu.get_flag(Flags::C) {
        let val = cpu.pop();
        cpu.set_pc(val);
        5
    } else {
        2
    }
}

/// POP DE
fn pop_d1(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_reg_16(Regs16::DE, val);
    3
}

/// JP NC, a16
fn jp_d2(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    if !cpu.get_flag(Flags::C) {
        cpu.set_pc(offset);
        4
    } else {
        3
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
        6
    } else {
        3
    }
}

/// PUSH DE
fn push_d5(cpu: &mut Cpu) -> u8 {
    let de = cpu.get_reg_16(Regs16::DE);
    cpu.push(de);
    4
}

/// SUB d8
fn sub_d6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.sub_a_d8(val, false);
    2
}

/// RST 10
fn rst_d7(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0010);
    4
}

/// RET C
fn ret_d8(cpu: &mut Cpu) -> u8 {
    if cpu.get_flag(Flags::C) {
        let val = cpu.pop();
        cpu.set_pc(val);
        5
    } else {
        2
    }
}

/// RETI
fn reti_d9(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_pc(val);
    cpu.interrupt_enabled = true;
    4
}

/// JP C, a16
fn jp_da(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let offset = merge_bytes(high, low);
    if cpu.get_flag(Flags::C) {
        cpu.set_pc(offset);
        4
    } else {
        3
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
        6
    } else {
        3
    }
}

/// SBC A, d8
fn sbc_de(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.sub_a_d8(val, true);
    2
}

/// RST 18
fn rst_df(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0018);
    4
}

/// LDH (a8), A
/// Same as LD ($FF00 + n), A
fn ldh_e0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(0xFF00 + offset, val);
    3
}

/// POP HL
fn pop_e1(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_reg_16(Regs16::HL, val);
    3
}

/// LD (C), A
/// Same as LD ($FF00 + C), A
fn ld_e2(cpu: &mut Cpu) -> u8 {
    let c = cpu.get_reg(Regs::C) as u16;
    let val = cpu.get_reg(Regs::A);
    cpu.write_ram(0xFF00 + c, val);
    2
}

/// PUSH HL
fn push_e5(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.push(hl);
    4
}

/// AND d8
fn and_e6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.and_a_d8(val);
    2
}

/// RST 20
fn rst_e7(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0020);
    4
}

/// ADD SP, r8
fn add_e8(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    let signed = val as i8 as i16 as u16;
    let sp = cpu.get_sp();
    cpu.set_sp(sp.wrapping_add(signed));

    let set_c = sp.get_low_byte().checked_add(signed.get_low_byte()).is_none();
    let set_h = check_h_carry_u8(sp.get_low_byte(), signed.get_low_byte());
    cpu.clear_flag(Flags::Z);
    cpu.clear_flag(Flags::N);
    cpu.write_flag(Flags::C, set_c);
    cpu.write_flag(Flags::H, set_h);
    4
}

/// JP HL
fn jp_e9(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.set_pc(hl);
    1
}

/// LD (a16), A
fn ld_ea(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let addr = merge_bytes(high, low);
    let a = cpu.get_reg(Regs::A);
    cpu.write_ram(addr, a);
    4
}

/// XOR d8
fn xor_ee(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.xor_a_d8(val);
    2
}

/// RST 28
fn rst_ef(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0028);
    4
}

/// LDH A, (a8)
/// Store $FF00 + n into A
fn ldh_f0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let val = cpu.read_ram(0xFF00 + offset);
    cpu.set_reg(Regs::A, val);
    3
}

/// POP AF
fn pop_f1(cpu: &mut Cpu) -> u8 {
    let val = cpu.pop();
    cpu.set_reg_16(Regs16::AF, val);
    3
}

/// LD A, (C)
/// Store $FF00 + register C into A
fn ld_f2(cpu: &mut Cpu) -> u8 {
    let c = cpu.get_reg(Regs::C) as u16;
    let val = cpu.read_ram(0xFF00 + c);
    cpu.set_reg(Regs::A, val);
    2
}

/// DI
fn di_f3(cpu: &mut Cpu) -> u8 {
    cpu.interrupt_enabled = false;
    1
}

/// PUSH AF
fn push_f5(cpu: &mut Cpu) -> u8 {
    let af = cpu.get_reg_16(Regs16::AF);
    cpu.push(af);
    4
}

/// OR d8
fn or_f6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.or_a_d8(val);
    2
}

/// RST 30
fn rst_f7(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0030);
    4
}

/// LD HL, SP+r8
/// Put SP + n into HL
fn ld_f8(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    let signed = val as i8 as i16 as u16;
    let sp = cpu.get_sp();
    cpu.set_reg_16(Regs16::HL, sp.wrapping_add(signed));

    let set_c = sp.get_low_byte().checked_add(signed.get_low_byte()).is_none();
    let set_h = check_h_carry_u8(sp.get_low_byte(), signed.get_low_byte());
    cpu.clear_flag(Flags::Z);
    cpu.clear_flag(Flags::N);
    cpu.write_flag(Flags::C, set_c);
    cpu.write_flag(Flags::H, set_h);
    3
}

/// LD SP, HL
fn ld_f9(cpu: &mut Cpu) -> u8 {
    let hl = cpu.get_reg_16(Regs16::HL);
    cpu.set_sp(hl);
    2
}

/// LD A, (a16)
fn ld_fa(cpu: &mut Cpu) -> u8 {
    let low = cpu.fetch();
    let high = cpu.fetch();
    let addr = merge_bytes(high, low);
    let val = cpu.read_ram(addr);
    cpu.set_reg(Regs::A, val);
    4
}

/// EI
fn ei_fb(cpu: &mut Cpu) -> u8 {
    cpu.interrupt_enabled = true;
    1
}

/// CP d8
fn cp_fe(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.cp_a_d8(val);
    2
}

/// RST 38
fn rst_ff(cpu: &mut Cpu) -> u8 {
    cpu.push(cpu.get_pc());
    cpu.set_pc(0x0038);
    4
}

fn execute_cb_op(cpu: &mut Cpu, op: u8) -> u8 {
    // $00-$07 -> RLC
    // $08-$0F -> RRC
    // $10-$17 -> RL
    // $18-$1F -> RR
    // $20-$27 -> SLA
    // $28-$2F -> SRA
    // $30-$37 -> SWAP
    // $38-$3F -> SRL
    // $40-$7F -> BIT
    // $80-$BF -> RES
    // $C0-$FF -> SET

    // Operations involving (HL) have different functionality than the
    // other registers, so those need to be handled separately
    // (At least for now)
    match op {
        0x00..=0x07 => {
            if op == 0x06 {
                let hl = cpu.get_reg_16(Regs16::HL);
                let byte = cpu.read_ram(hl);
                let rot = cpu.rot_left(byte, false);
                cpu.write_ram(hl, rot);
            } else {
                let reg = decode_cb_reg(op);
                cpu.rot_left_reg(reg, false);
            }
        },
        0x08..=0x0F => {
            if op == 0x0E {
                let hl = cpu.get_reg_16(Regs16::HL);
                let byte = cpu.read_ram(hl);
                let rot = cpu.rot_right(byte, false);
                cpu.write_ram(hl, rot);
            } else {
                let reg = decode_cb_reg(op);
                cpu.rot_right_reg(reg, false);
            }
        },
        0x10..=0x17 => {
            if op == 0x16 {
                let hl = cpu.get_reg_16(Regs16::HL);
                let byte = cpu.read_ram(hl);
                let rot = cpu.rot_left(byte, true);
                cpu.write_ram(hl, rot);
            } else {
                let reg = decode_cb_reg(op);
                cpu.rot_left_reg(reg, true);
            }
        },
        0x18..=0x1F => {
            if op == 0x1E {
                let hl = cpu.get_reg_16(Regs16::HL);
                let byte = cpu.read_ram(hl);
                let rot = cpu.rot_right(byte, true);
                cpu.write_ram(hl, rot);
            } else {
                let reg = decode_cb_reg(op);
                cpu.rot_right_reg(reg, true);
            }
        },
        0x20..=0x27 => {
            if op == 0x26 {
                let addr = cpu.get_reg_16(Regs16::HL);
                let byte = cpu.read_ram(addr);
                let shifted = cpu.shift_left(byte);
                cpu.write_ram(addr, shifted);
            } else {
                let reg = decode_cb_reg(op);
                cpu.shift_left_reg(reg);
            }
        },
        0x28..=0x2F => {
            if op == 0x2E {
                let hl = cpu.get_reg_16(Regs16::HL);
                let val = cpu.read_ram(hl);
                let shifted = cpu.shift_right(val, true);
                cpu.write_ram(hl, shifted);
            } else {
                let reg = decode_cb_reg(op);
                cpu.shift_right_reg(reg, true);
            }
        },
        0x30..=0x37 => {
            if op == 0x36 {
                let hl = cpu.get_reg_16(Regs16::HL);
                let val = cpu.read_ram(hl);
                let swapped = cpu.swap_bits(val);
                cpu.write_ram(hl, swapped);
            } else {
                let reg = decode_cb_reg(op);
                cpu.swap_bits_reg(reg);
            }
        },
        0x38..=0x3F => {
            if op == 0x3E {
                let hl = cpu.get_reg_16(Regs16::HL);
                let val = cpu.read_ram(hl);
                let shifted = cpu.shift_right(val, false);
                cpu.write_ram(hl, shifted);
            } else {
                let reg = decode_cb_reg(op);
                cpu.shift_right_reg(reg, false);
            }
        },
        0x40..=0x7F => {
            let rel_offset = op - 0x40;
            let digit = rel_offset / 0x08;

            match op & 0x0F {
                0x06 | 0x0E => {
                    let hl = cpu.get_reg_16(Regs16::HL);
                    let val = cpu.read_ram(hl);
                    cpu.test_bit(val, digit);
                },
                _ => {
                    let reg = decode_cb_reg(op);
                    cpu.test_bit_reg(reg, digit);
                }
            }
        },
        0x80..=0xBF => {
            let rel_offset = op - 0x40;
            let digit = rel_offset / 0x08;

            match op & 0x0F {
                0x06 | 0x0E => {
                    let hl = cpu.get_reg_16(Regs16::HL);
                    cpu.write_bit_ram(hl, digit, false);
                },
                _ => {
                    let reg = decode_cb_reg(op);
                    cpu.write_bit_n(reg, digit, false);
                }
            }
        },
        0xC0..=0xFF => {
            let rel_offset = op - 0x40;
            let digit = rel_offset / 0x08;

            match op & 0x0F {
                0x06 | 0x0E => {
                    let hl = cpu.get_reg_16(Regs16::HL);
                    cpu.write_bit_ram(hl, digit, true);
                },
                _ => {
                    let reg = decode_cb_reg(op);
                    cpu.write_bit_n(reg, digit, true);
                }
            }
        }
    }

    2
}

fn decode_cb_reg(op: u8) -> Regs {
    let reg = match op & 0xF {
        0x00 | 0x08 => { Regs::B },
        0x01 | 0x09 => { Regs::C },
        0x02 | 0x0A => { Regs::D },
        0x03 | 0x0B => { Regs::E },
        0x04 | 0x0C => { Regs::H },
        0x05 | 0x0D => { Regs::L },
        0x07 | 0x0F => { Regs::A },
        _ => { unreachable!() }
    };

    reg
}
