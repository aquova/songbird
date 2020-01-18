// The agba debugger module
use crate::cpu::*;

pub mod info;

pub struct debugger {
    breakpoints: Vec<u16>,
    watchpoints: Vec<u16>
}

impl debugger {
    pub fn new() -> debugger {
        debugger {
            breakpoints: Vec::new(),
            watchpoints: Vec::new()
        }
    }

    pub fn print_info(&self) {
        println!("agbd - The agba debugger");
        println!("Type help for more information");
        println!("");
    }

    pub fn print_help(&self) {
        println!("`b #` to break at that PC value");
        println!("`reg` to list register contents");
        println!("`c` to continue execution");
        println!("`q` to quit program");
        println!("`info` to list break/watchpoints");
        println!("");
    }

    pub fn print_registers(&self, gb: &Cpu) {
        println!("PC: {:#06x}", gb.get_pc());
        println!("SP: {:#06x}", gb.get_sp());
        println!("AF: {:#06x}", gb.get_reg_16(Regs16::AF));
        println!("BC: {:#06x}", gb.get_reg_16(Regs16::BC));
        println!("DE: {:#06x}", gb.get_reg_16(Regs16::DE));
        println!("HL: {:#06x}", gb.get_reg_16(Regs16::HL));
        println!("");
    }

    pub fn list_points(&self) {
        if self.breakpoints.len() > 0 {
            let mut breakstring = String::new();
            for bp in &self.breakpoints {
                breakstring = format!("{} {}", breakstring, bp);
            }
            println!("{}", breakstring);
        } else {
            println!("You have no breakpoints set");
        }

        if self.watchpoints.len() > 0 {
            let mut watchstring = String::new();
            for wp in &self.watchpoints {
                watchstring = format!("{} {}", watchstring, wp);
            }
            println!("{}", watchstring);
        } else {
            println!("You have no watchpoints set");
        }

        println!("");
    }

    pub fn add_break(&mut self, addr: u16) {
        self.breakpoints.push(addr);
    }
}
