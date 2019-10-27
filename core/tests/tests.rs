extern crate gb_core;
extern crate rand;

use gb_core::cpu::*;
use rand::Rng;

fn rand_reg() -> Regs {
    let index = rand::thread_rng().gen_range(0, 6);
    match index {
        0 => { Regs::A }
        1 => { Regs::B }
        2 => { Regs::C }
        3 => { Regs::D }
        4 => { Regs::E }
        5 => { Regs::H }
        6 => { Regs::L }
        _ => { panic!("Invalid index") }
    }
}

#[test]
/// Tests register getter and setter
fn test_regs() {
    let mut gb = Cpu::new();
    for _ in 0..100 {
        let val: u8 = rand::thread_rng().gen_range(0, 0xFF);
        let reg = rand_reg();

        gb.set_reg(reg, val);
        let ret = gb.get_reg(reg);
        assert_eq!(ret, val);
    }
}

#[test]
/// Tests flags getter and setter
fn test_flags() {
    let mut gb = Cpu::new();

    gb.set_flag(Flags::Z);
    gb.set_flag(Flags::N);
    gb.set_flag(Flags::C);
    gb.set_flag(Flags::H);
    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::N), true);
    assert_eq!(gb.get_flag(Flags::C), true);
    assert_eq!(gb.get_flag(Flags::H), true);

    gb.clear_flag(Flags::Z);
    gb.clear_flag(Flags::N);
    gb.clear_flag(Flags::H);
    gb.clear_flag(Flags::C);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::C), false);
    assert_eq!(gb.get_flag(Flags::H), false);
}

