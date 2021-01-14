// The songbird debugger module
use crate::cpu::*;
use crate::cartridge::ROM_STOP;
use std::{cmp::min, hash::Hash};
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

const OPCODE_NAMES: [&str; 0x100] = [
    "NOP",          "LD BC, d16",   "LD (BC), A",   "INC BC",       "INC B",        "DEC B",        "LD B, d8",     "RLCA",         // $00
    "LD (a16), SP", "ADD HL, BC",   "LD A, (BC)",   "DEC BC",       "INC C",        "DEC C",        "LD C, d8",     "RRCA",         // $08
    "STOP",         "LD DE, d16",   "LD (DE), A",   "INC DE",       "INC D",        "DEC D",        "LD D, d8",     "RLA",          // $10
    "JR r8",        "ADD HL, DE",   "LD A, (DE)",   "DEC DE",       "INC E",        "DEC E",        "LD E, d8",     "RRA",          // $18
    "JR NZ, r8",    "LD HL, d16",   "LD (HL+), A",  "INC HL",       "INC H",        "DEC H",        "LD H, d8",     "DAA",          // $20
    "JR Z, r8",     "ADD HL, HL",   "LD A, (HL+)",  "DEC HL",       "INC L",        "DEC L",        "LD L, d8",     "CPL",          // $28
    "JR NC, r8",    "LD SP, d16",   "LD (HL-), A",  "INC SP",       "INC (HL)",     "DEC (HL)",     "LD (HL), d8",  "SCF",          // $30
    "JR C, r8",     "ADD HL, SP",   "LD A, (HL-)",  "DEC SP",       "INC A",        "DEC A",        "LD A, d8",     "CCF",          // $38
    "LD B, B",      "LD B, C",      "LD B, D",      "LD B, E",      "LD B, H",      "LD B, L",      "LD B, (HL)",   "LD B, A",      // $40
    "LD C, B",      "LD C, C",      "LD C, D",      "LD C, E",      "LD C, H",      "LD C, L",      "LD C, (HL)",   "LD C, A",      // $48
    "LD D, B",      "LD D, C",      "LD D, D",      "LD D, E",      "LD D, H",      "LD D, L",      "LD D, (HL)",   "LD D, A",      // $50
    "LD E, B",      "LD E, C",      "LD E, D",      "LD E, E",      "LD E, H",      "LD E, L",      "LD E, (HL)",   "LD E, A",      // $58
    "LD H, B",      "LD H, C",      "LD H, D",      "LD H, E",      "LD H, H",      "LD H, L",      "LD H, (HL)",   "LD H, A",      // $60
    "LD L, B",      "LD L, C",      "LD L, D",      "LD L, E",      "LD L, H",      "LD L, L",      "LD L, (HL)",   "LD L, A",      // $68
    "LD (HL), B",   "LD (HL), C",   "LD (HL), D",   "LD (HL), E",   "LD (HL), H",   "LD (HL), L",   "HALT",         "LD (HL), A",   // $70
    "LD A, B",      "LD A, C",      "LD A, D",      "LD A, E",      "LD A, H",      "LD A, L",      "LD A, (HL)",   "LD A, A",      // $78
    "ADD A, B",     "ADD A, C",     "ADD A, D",     "ADD A, E",     "ADD A, H",     "ADD A, L",     "ADD A, (HL)",  "ADD A, A",     // $80
    "ADC A, B",     "ADC A, C",     "ADC A, D",     "ADC A, E",     "ADC A, H",     "ADC A, L",     "ADC A, (HL)",  "ADC A, A",     // $88
    "SUB B",        "SUB C",        "SUB D",        "SUB E",        "SUB H",        "SUB L",        "SUB (HL)",     "SUB A",        // $90
    "SBC B",        "SBC C",        "SBC D",        "SBC E",        "SBC H",        "SBC L",        "SBC (HL)",     "SBC A",        // $98
    "AND B",        "AND C",        "AND D",        "AND E",        "AND H",        "AND L",        "AND (HL)",     "AND A",        // $A0
    "XOR B",        "XOR C",        "XOR D",        "XOR E",        "XOR H",        "XOR L",        "XOR (HL)",     "XOR A",        // $A8
    "OR B",         "OR C",         "OR D",         "OR E",         "OR H",         "OR L",         "OR (HL)",      "OR A",         // $B0
    "CP B",         "CP C",         "CP D",         "CP E",         "CP H",         "CP L",         "CP (HL)",      "CP A",         // $B8
    "RET NZ",       "POP BC",       "JP NZ, a16",   "JP a16",       "CALL NZ, a16", "PUSH BC",      "AND A, d8",    "RST 00",       // $C0
    "RET Z",        "RET",          "JP Z, a16",    "PREFIX CB",    "CALL Z, a16",  "CALL a16",     "ADC A, d8",    "RST 08",       // $C8
    "RET NC",       "POP DE",       "JP NC, a16",   "INVALID",      "CALL NC, a16", "PUSH DE",      "SUB d8",       "RST 10",       // $D0
    "RET C",        "RETI",         "JP C, a16",    "INVALID",      "CALL C, a16",  "INVALID",      "SBC A, d8",    "RST 18",       // $D8
    "LDH (a8), A",  "POP HL",       "LD (C), A",    "INVALID",      "INVALID",      "PUSH HL",      "AND d8",       "RST 20",       // $E0
    "ADD SP, r8",   "JP (HL)",      "LD (a16), A",  "INVALID",      "INVALID",      "INVALID",      "XOR d8",       "RST 28",       // $E8
    "LDH A, (a8)",  "POP AF",       "LD A, (C)",    "DI",           "INVALID",      "PUSH AF",      "OR d8",        "RST 30",       // $F0
    "LD HL, SP+r8", "LD SP, HL",    "LD A, (a16)",  "EI",           "INVALID",      "INVALID",      "CP d8",        "RST 38"        // $F8
];

const OPCODE_LENGTH: [u8; 0x100] = [
    0, 2, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 1, 0,
    1, 2, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0,
    1, 2, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0,
    1, 2, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 0, 1, 0, 0, 0, 2, 0, 2, 2, 1, 0,
    0, 0, 2, 0, 2, 0, 1, 0, 0, 0, 2, 0, 2, 0, 1, 0,
    1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 2, 0, 0, 0, 1, 0,
    1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 2, 0, 0, 0, 1, 0
];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct MemAddr {
    pub bank: u16,
    pub addr: u16,
}

#[allow(non_camel_case_types)]
pub struct debugger {
    debugging: bool,
    tracing: bool,
    breakpoints: Vec<MemAddr>,
    watchpoints: Vec<MemAddr>,
    watch_map: HashMap<MemAddr, u8>,
}

impl Default for debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl debugger {
    pub fn new() -> debugger {
        debugger {
            debugging: false,
            tracing: false,
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
            watch_map: HashMap::new(),
        }
    }

    pub fn set_debugging(&mut self, debug: bool) {
        self.debugging = debug;
    }

    pub fn is_debugging(&self) -> bool {
        self.debugging
    }

    pub fn is_tracing(&self) -> bool {
        self.tracing
    }

    pub fn debugloop(&mut self, gb: &mut Cpu) -> bool {
        let mut should_quit = false;

        'debugloop: loop {
            print!("(songdb) ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            // Await user input
            let stdin = io::stdin();
            stdin.read_line(&mut input).expect("Your user input was... odd");
            trim_newline(&mut input);
            let words: Vec<&str> = input.split(' ').collect();

            match words[0] {
                "b" => {
                    let mem_addr = parse_mem_addr(words[1]);
                    if let Some(bp) = mem_addr {
                        self.add_break(bp);
                    }
                },
                "c" => {
                    self.set_debugging(false);
                    break 'debugloop;
                },
                "del" => {
                    let mem_addr = parse_mem_addr(words[1]);
                    if let Some(bp) = mem_addr {
                        self.del_break(bp);
                    } else {
                        println!("{} is not a valid address", words[1]);
                    }
                },
                "disass" => {
                    self.disassemble(&gb);
                },
                "help" => {
                    self.print_help();
                },
                "info" => {
                    self.list_points();
                },
                "n" => {
                    gb.tick();
                    let pc = gb.get_pc();
                    if 0x4000 <= pc && pc <= ROM_STOP {
                        println!("PC: ${}:{:04x}", gb.get_rom_bank(), pc);
                    } else {
                        println!("PC: ${:04x}", pc);
                    }
                },
                "p" => {
                    let mem_addr = parse_mem_addr(words[1]);
                    if let Some(mem) = mem_addr {
                        self.print_ram(mem.addr, mem.bank, &gb);
                    }
                },
                "q" => {
                    should_quit = true;
                    break 'debugloop;
                },
                "reg" => {
                    println!("{}", self.print_registers(&gb));
                },
                "trace" => {
                    self.tracing = !self.tracing;
                },
                "w" => {
                    let mem_addr = parse_mem_addr(words[1]);
                    if let Some(wp) = mem_addr {
                        self.add_watch(wp, &gb);
                    }
                },
                _ => {
                    println!("Unknown command");
                }
            }
        }

        should_quit
    }

    /// ```
    /// Check breakpoints
    ///
    /// Checks if any of the breakpoints have been hit
    ///
    /// Input:
    ///     Program counter (u16)
    /// ```
    pub fn check_break(&mut self, pc: u16) {
        let to_check = MemAddr{ bank: 0, addr: pc };
        for bp in &self.breakpoints {
            if *bp == to_check {
                self.debugging = true;
            }
        }
    }

    /// ```
    /// Print debugger info
    ///
    /// Prints the debugger start message
    /// ```
    pub fn print_info(&self, pc: u16) {
        println!("agbd - The songbird debugger");
        println!("Execution paused at {:#06x}", pc);
        println!();
    }

    /// ```
    /// Print help
    ///
    /// Prints the debugger help message
    /// ```
    fn print_help(&self) {
        println!("'b #' to break at that address");
        println!("'c' to continue execution");
        println!("'del #' to delete breakpoint at that address");
        println!("'disass' to show disassembly of next 5 instructions");
        println!("'help' to print this message");
        println!("'info' to list break/watchpoints");
        println!("'n' to run to next instruction");
        println!("'p' to print 16 bytes at given RAM address (in hex)");
        println!("'q' to quit program");
        println!("'reg' to list register contents");
        println!("'w #' to add (write) watchpoint at that address");
        println!();
    }

    /// ```
    /// Print registers
    ///
    /// Prints the CPU registers, including SP and PC
    ///
    /// Input:
    ///     Reference to CPU object (&Cpu)
    /// ```
    fn print_registers(&self, gb: &Cpu) -> String {
        let mut reg_info = format!("PC: ${:04x}\n", gb.get_pc());
        reg_info = format!("{}SP: ${:04x}\n", reg_info, gb.get_sp());
        reg_info = format!("{}AF: ${:04x}\n", reg_info, gb.get_reg_16(Regs16::AF));
        reg_info = format!("{}BC: ${:04x}\n", reg_info, gb.get_reg_16(Regs16::BC));
        reg_info = format!("{}DE: ${:04x}\n", reg_info, gb.get_reg_16(Regs16::DE));
        reg_info = format!("{}HL: ${:04x}\n", reg_info, gb.get_reg_16(Regs16::HL));

        reg_info
    }

    /// ```
    /// List points
    ///
    /// Prints the currently set break/watchpoints
    /// ```
    fn list_points(&self) {
        if !self.breakpoints.is_empty() {
            let mut breakstring = "Breakpoints:".to_string();
            for bp in &self.breakpoints {
                breakstring = format!("{} ${:02x}:{:04x}", breakstring, bp.bank, bp.addr);
            }
            println!("{}", breakstring);
        } else {
            println!("You have no breakpoints set");
        }

        if !self.watchpoints.is_empty() {
            let mut watchstring = "Watchpoints:".to_string();
            for wp in &self.watchpoints {
                watchstring = format!("{} ${:02x}:{:04x}", watchstring, wp.bank, wp.addr);
            }
            println!("{}", watchstring);
        } else {
            println!("You have no watchpoints set");
        }

        println!();
    }

    /// ```
    /// Add breakpoint
    ///
    /// Adds a breakpoint at specified address
    /// Note: Doesn't check if breakpoint is already in list
    ///
    /// Input:
    ///     Address to break (u16)
    /// ```
    fn add_break(&mut self, bp: MemAddr) {
        self.breakpoints.push(bp);
    }

    /// ```
    /// Add watchpoint
    ///
    /// Adds a watchpoint at specified address
    /// Note: Doesn't check if watchpoint is already in list
    ///
    /// Input:
    ///     Address to watch (u16)
    /// ```
    fn add_watch(&mut self, wp: MemAddr, gb: &Cpu) {
        self.watchpoints.push(wp);
        let orig_val = gb.read_ram(wp.addr, Some(wp.bank));
        self.watch_map.insert(wp, orig_val);
    }

    /// ```
    /// Print RAM
    ///
    /// Prints the RAM contents at given address + following 15 locations
    ///
    /// Inputs:
    ///     Address to start printing from (u16)
    ///     Reference to CPU object (&Cpu)
    /// ```
    fn print_ram(&self, addr: u16, bank: u16, gb: &Cpu) {
        // Print up to addr + 16, unless we go off the end
        let end_addr = min(addr + 16, 0xFFFF);
        let mut valstring = String::new();
        for i in addr..end_addr {
            let val = gb.read_ram(i, Some(bank));
            valstring = format!("{} {:02x}", valstring, val);
        }

        println!("${:04x}: {}", addr, valstring);
    }

    /// ```
    /// Delete breakpoint
    ///
    /// Attempts to delete the specified breakpoint, if it exists
    ///
    /// Input:
    ///     Address to remove (u16)
    /// ```
    fn del_break(&mut self, mem: MemAddr) {
        for i in 0..self.breakpoints.len() {
            if self.breakpoints[i] == mem {
                self.breakpoints.remove(i);
                break;
            }
        }

        for i in 0..self.watchpoints.len() {
            if self.watchpoints[i] == mem {
                self.watchpoints.remove(i);
                break;
            }
        }
    }

    /// ```
    /// Disassemble
    ///
    /// Prints out next five instructions as GBz80 assembly
    /// NOTE: This is still rough, only prints out instructions and not parameters
    ///
    /// Input:
    ///     Refernce to CPU (&Cpu)
    /// ```
    fn disassemble(&self, gb: &Cpu) {
        let mut pc = gb.get_pc();

        // Print next 5 instructions
        for _ in 0..5 {
            let op = gb.read_ram(pc, None);
            let op_name = OPCODE_NAMES[op as usize];
            let op_len = OPCODE_LENGTH[op as usize] as u16 + 1;
            let mut inst = format!("${:04x} | {} ;", pc, op_name);
            for i in 0..op_len {
                let arg = gb.read_ram(pc + i, None);
                inst = format!("{} {:02x}", inst, arg);
            }
            println!("{}", inst);
            pc += op_len;
        }
    }

    pub fn check_watch(&mut self, gb: &Cpu) {
        for wp in &self.watchpoints {
            let curr = gb.read_ram(wp.addr, Some(wp.bank));
            let new = self.watch_map.insert(*wp, curr);
            if let Some(new_val) = new {
                if new_val != curr {
                    self.debugging = true;
                }
            }
        }
    }
}

fn parse_mem_addr(input: &str) -> Option<MemAddr> {
    let parts: Vec<&str> = input.split(':').collect();
    let addr = if parts.len() == 1 {
        let hex = u16::from_str_radix(parts[0], 16);
        if let Ok(addr) = hex {
            Some(MemAddr{ bank: 0, addr })
        } else {
            None
        }
    } else {
        let bank_hex = u16::from_str_radix(parts[0], 16);
        let addr_hex = u16::from_str_radix(parts[1], 16);
        if let Ok(bank) = bank_hex {
            if let Ok(addr) = addr_hex {
                Some(MemAddr{ bank, addr })
            } else {
                None
            }
        } else {
            None
        }
    };
    addr
}

/// ```
/// Trim Newline
///
/// Helper function that removes trailing newline characters
/// Works on *nix systems and Windows
///
/// Input:
///     String to trim (&mut String)
/// ```
fn trim_newline(s: &mut String) {
    // Strip newline
    if s.ends_with('\n') {
        s.pop();
        // For Windows
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
