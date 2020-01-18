use crate::cpu::*;

pub struct agba {
    breakpoints: Vec<u16>,
    watchpoints: Vec<u16>
}

impl agba {
    pub fn init(&mut self) {
        self.breakpoints = Vec::new();
        self.watchpoints = Vec::new();
    }

    pub fn print_registers(&self, gb: Cpu) {
        // println!("PC: {:#06x}", self.pc);
        // println!("SP: {:#06x}", self.sp);
        println!("AF: {:#06x}", gb.get_reg_16(Regs16::AF));
        println!("BC: {:#06x}", gb.get_reg_16(Regs16::BC));
        println!("DE: {:#06x}", gb.get_reg_16(Regs16::DE));
        println!("HL: {:#06x}", gb.get_reg_16(Regs16::HL));
    }
}
