use crate::bus::Bus;
use crate::debug::*;
use crate::opcodes::execute;
use crate::utils::*;

pub enum Flags {
    Z,
    N,
    H,
    C
}

#[derive(Copy, Clone)]
pub enum Regs {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

#[derive(Copy, Clone)]
pub enum Regs16 {
    AF,
    BC,
    DE,
    HL
}

pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub interupt: bool,
    pub bus: Bus,
}


impl Cpu {
    pub fn new() -> Cpu {
        // Magic values from pandocs
        let mut new_cpu = Cpu {
            pc: 0x100,
            sp: 0xFFFE,
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            interupt: false,
            bus: Bus::new()
        };

        // More magic values for RAM initialization
        // TODO: May need a 'set_ram' fn
        // if not MBC none and not allowed to write
        new_cpu.write_ram(0xFF05, 0x00);
        new_cpu.write_ram(0xFF06, 0x00);
        new_cpu.write_ram(0xFF07, 0x00);
        new_cpu.write_ram(0xFF10, 0x80);
        new_cpu.write_ram(0xFF11, 0xBF);
        new_cpu.write_ram(0xFF12, 0xF3);
        new_cpu.write_ram(0xFF14, 0xBF);
        new_cpu.write_ram(0xFF16, 0x3F);
        new_cpu.write_ram(0xFF17, 0x00);
        new_cpu.write_ram(0xFF19, 0xBF);
        new_cpu.write_ram(0xFF1A, 0x7F);
        new_cpu.write_ram(0xFF1B, 0xFF);
        new_cpu.write_ram(0xFF1C, 0x9F);
        new_cpu.write_ram(0xFF1E, 0xBF);
        new_cpu.write_ram(0xFF20, 0xFF);
        new_cpu.write_ram(0xFF21, 0x00);
        new_cpu.write_ram(0xFF22, 0x00);
        new_cpu.write_ram(0xFF23, 0xBF);
        new_cpu.write_ram(0xFF24, 0x77);
        new_cpu.write_ram(0xFF25, 0xF3);
        new_cpu.write_ram(0xFF26, 0xF1); // $F0 for SGB
        new_cpu.write_ram(0xFF40, 0x91);
        new_cpu.write_ram(0xFF42, 0x00);
        new_cpu.write_ram(0xFF43, 0x00);
        new_cpu.write_ram(0xFF45, 0x00);
        new_cpu.write_ram(0xFF47, 0xFC);
        new_cpu.write_ram(0xFF48, 0xFF);
        new_cpu.write_ram(0xFF49, 0xFF);
        new_cpu.write_ram(0xFF4A, 0x00);
        new_cpu.write_ram(0xFF4B, 0x00);
        new_cpu.write_ram(0xFF4F, 0x00);

        new_cpu
    }

    /// ```
    /// Tick
    ///
    /// Performs one operation
    ///
    /// Output:
    ///     Number of cycles needed to complete operation (u8)
    /// ```
    pub fn tick(&mut self) -> u8 {
        let cycles = execute(self);
        cycles
    }

    /// ```
    /// Print Info
    ///
    /// Prints debug info about state of the CPU
    /// ```
    pub fn print_info(&self) {
        println!("PC: {:#06x} SP: {:#06x}", self.pc, self.sp);
        println!(
            "AF: {:#06x} BC: {:#06x} DE: {:#06x} HL: {:#06x}",
            self.get_reg_16(Regs16::AF),
            self.get_reg_16(Regs16::BC),
            self.get_reg_16(Regs16::DE),
            self.get_reg_16(Regs16::HL)
        );
        let curr_op = self.read_ram(self.pc);
        println!("Current operation: {:#04x}", curr_op);
        print_opcode(curr_op);
        let operand_num = get_opcode_length(curr_op);
        for i in 0..operand_num {
            println!("{:#04x}", self.read_ram(self.pc + i as u16 + 1));
        }
        println!("-----");
    }

    /// ```
    /// Fetch
    ///
    /// Fetches the byte specified by the PC, increments PC by one
    ///
    /// Output:
    ///     Byte at the current PC (u8)
    /// ```
    pub fn fetch(&mut self) -> u8 {
        let val = self.read_ram(self.pc);
        self.pc += 1;
        val
    }

    /// ```
    /// Load game
    ///
    /// Loads game into ROM
    ///
    /// Input:
    ///     Path to game (&str)
    /// ```
    pub fn load_game(&mut self, path: &str) {
        self.bus.load_game(path);
    }

    /// ```
    /// Read RAM
    ///
    /// Returns the byte at the specified address in RAM
    ///
    /// Input:
    ///     Address in RAM (u16)
    ///
    /// Output:
    ///     Byte at specified address (u8)
    /// ```
    pub fn read_ram(&self, addr: u16) -> u8 {
        self.bus.read_ram(addr)
    }

    /// ```
    /// Write RAM
    ///
    /// Writes the specified byte at the specified address
    ///
    /// Inputs:
    ///     Address in RAM (u16)
    ///     Byte to write (u8)
    /// ```
    pub fn write_ram(&mut self, addr: u16, val: u8) {
        self.bus.write_ram(addr, val);
    }

    /// ```
    /// Get Register
    ///
    /// Returns the value stored in the specified register
    ///
    /// Input:
    ///     Desired register (Regs enum value)
    ///
    /// Output:
    ///     Byte stored in register (u8)
    /// ```
    pub fn get_reg(&self, r: Regs) -> u8 {
        match r {
            Regs::A => { self.a },
            Regs::B => { self.b },
            Regs::C => { self.c },
            Regs::D => { self.d },
            Regs::E => { self.e },
            Regs::F => { self.f },
            Regs::H => { self.h },
            Regs::L => { self.l },
        }
    }

    /// ```
    /// Set register
    ///
    /// Sets the specified value into specified register
    ///
    /// Input:
    ///     Desired register (Regs enum value)
    ///     Byte to store (u8)
    /// ```
    pub fn set_reg(&mut self, r: Regs, val: u8) {
        match r {
            Regs::A => { self.a = val },
            Regs::B => { self.b = val },
            Regs::C => { self.c = val },
            Regs::D => { self.d = val },
            Regs::E => { self.e = val },
            Regs::F => { self.f = val },
            Regs::H => { self.h = val },
            Regs::L => { self.l = val },
        }
    }

    /// ```
    /// Set 16-bit Register
    ///
    /// Sets the specified u16 value into joint register
    ///
    /// Input:
    ///     Desired register (Regs16 enum value)
    ///     Byte to store (u16)
    /// ```
    pub fn set_reg_16(&mut self, r: Regs16, val: u16) {
        let high = val.get_high_byte();
        let low = val.get_low_byte();
        match r {
            Regs16::AF => {
                self.set_reg(Regs::A, high);
                self.set_reg(Regs::F, low);
            },
            Regs16::BC => {
                self.set_reg(Regs::B, high);
                self.set_reg(Regs::C, low);
            },
            Regs16::DE => {
                self.set_reg(Regs::D, high);
                self.set_reg(Regs::E, low);
            },
            Regs16::HL => {
                self.set_reg(Regs::H, high);
                self.set_reg(Regs::L, low);
            }
        }
    }

    /// ```
    /// Get 16-bit Register
    ///
    /// Gets the value stored in the joint 16-bit register
    ///
    /// Input:
    ///     16-bit register (Regs16 enum value)
    ///
    /// Output:
    ///     Value stored in register (u16)
    /// ```
    pub fn get_reg_16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => {
                let high = self.a;
                let low = self.f;
                merge_bytes(high, low)
            },
            Regs16::BC => {
                let high = self.b;
                let low = self.c;
                merge_bytes(high, low)
            },
            Regs16::DE => {
                let high = self.d;
                let low = self.e;
                merge_bytes(high, low)
            },
            Regs16::HL => {
                let high = self.h;
                let low = self.l;
                merge_bytes(high, low)
            }
        }
    }

    /// ```
    /// Set Flag
    ///
    /// Sets the specified flag to True
    ///
    /// Input:
    ///     Flag to set (Flags enum value)
    /// ```
    pub fn set_flag(&mut self, f: Flags) {
        match f {
            Flags::Z => { self.f |= 0b1000_0000 },
            Flags::N => { self.f |= 0b0100_0000 },
            Flags::H => { self.f |= 0b0010_0000 },
            Flags::C => { self.f |= 0b0001_0000 },
        }
    }

    /// ```
    /// Clear Flag
    ///
    /// Sets the specified flag to False
    ///
    /// Input:
    ///     Flag to clear (Flags enum value)
    /// ```
    pub fn clear_flag(&mut self, f: Flags) {
        match f {
            Flags::Z => { self.f &= 0b0111_1111 },
            Flags::N => { self.f &= 0b1011_1111 },
            Flags::H => { self.f &= 0b1101_1111 },
            Flags::C => { self.f &= 0b1110_1111 },
        }
    }

    /// ```
    /// Get Flag
    ///
    /// Returns whether the specified flag is set or cleared
    ///
    /// Input:
    ///     Flag to return (Flags enum value)
    ///
    /// Output:
    ///     Whether the flag is set or not (bool)
    /// ```
    pub fn get_flag(&self, f: Flags) -> bool {
        match f {
            Flags::Z => { return (self.f & 0b1000_0000) != 0 },
            Flags::N => { return (self.f & 0b0100_0000) != 0 },
            Flags::H => { return (self.f & 0b0010_0000) != 0 },
            Flags::C => { return (self.f & 0b0001_0000) != 0 },
        }
    }

    /// ```
    /// Write Flag
    ///
    /// Sets the specified flag to true or false
    ///
    /// Inputs:
    ///     Flag to set (Flags enum value)
    ///     Whether the flag should be set or not (bool)
    /// ```
    pub fn write_flag(&mut self, f: Flags, val: bool) {
        if val {
            self.set_flag(f);
        } else {
            self.clear_flag(f);
        }
    }

    /// ```
    /// LD N d8
    ///
    /// Load 8-bit value into register
    ///
    /// Inputs:
    ///     Register to load (Regs enum value)
    ///     Value to store (u8)
    /// ```
    pub fn ld_n_d8(&mut self, reg: Regs, byte: u8) {
        self.set_reg(reg, byte);
    }

    /// ```
    /// LD NN d16
    ///
    /// Load 16-bit value into joint register
    ///
    /// Inputs:
    ///     Register to load (Regs16 enum value)
    ///     Value to store (u16)
    /// ```
    pub fn ld_nn_d16(&mut self, reg: Regs16, val: u16) {
        self.set_reg_16(reg, val);
    }

    /// ```
    /// INC d8
    ///
    /// Increments specified register
    ///
    /// Input:
    ///     Register to increment (Regs enum value)
    /// ```
    pub fn inc_8(&mut self, reg: Regs) {
        let val = self.get_reg(reg);
        let result = val.wrapping_add(1);
        let set_h = check_h_carry_u8(val, 1);
        self.set_reg(reg, result);

        self.clear_flag(Flags::N);
        self.write_flag(Flags::Z, result == 0);
        self.write_flag(Flags::H, set_h);
    }

    /// ```
    /// INC d16
    ///
    /// Increments specified join register
    ///
    /// Input:
    ///     Register to increment (Regs16 enum value)
    /// ```
    pub fn inc_16(&mut self, reg: Regs16) {
        let val = self.get_reg_16(reg);
        let result = val.wrapping_add(1);
        self.set_reg_16(reg, result);
    }

    /// ```
    /// DEC d8
    ///
    /// Decrements specified register
    ///
    /// Input:
    ///     Register to decrement (Regs enum value)
    /// ```
    pub fn dec_8(&mut self, reg: Regs) {
        let val = self.get_reg(reg);
        let result = val.wrapping_sub(1);
        let set_h = check_h_borrow_u8(result, 1);
        self.set_reg(reg, result);

        self.set_flag(Flags::N);
        self.write_flag(Flags::Z, result == 0);
        self.write_flag(Flags::H, set_h);
    }

    /// ```
    /// DEC d16
    ///
    /// Decrements specified joint register
    ///
    /// Input:
    ///     Register to decrement (Regs16 enum value)
    /// ```
    pub fn dec_16(&mut self, reg: Regs16) {
        let val = self.get_reg_16(reg);
        let result = val.wrapping_sub(1);
        self.set_reg_16(reg, result);
    }

    /// ```
    /// ADD A d8
    ///
    /// Adds specified value to A register
    ///
    /// Inputs:
    ///     Value to add to register (u8)
    ///     Whether or not to add with carry (bool)
    /// ```
    pub fn add_a_d8(&mut self, val: u8, adc: bool) {
        let mut carry = 0;
        if adc && self.get_flag(Flags::C) {
            carry = 1;
        }
        let a = self.get_reg(Regs::A);
        let result1 = a.overflowing_add(val);
        let h_check1 = check_h_carry_u8(a, val);
        let result2 = result1.0.overflowing_add(carry);
        let h_check2 = check_h_carry_u8(result1.0, carry);
        let set_h = h_check1 || h_check2;
        let set_c = result1.1 || result2.1;

        self.clear_flag(Flags::N);
        self.write_flag(Flags::C, set_c);
        self.write_flag(Flags::H, set_h);
        self.write_flag(Flags::Z, result2.0 == 0);
        self.set_reg(Regs::A, result2.0);
    }

    /// ```
    /// ADD NN d16
    ///
    /// Adds value to joint 16-bit register
    ///
    /// Inputs:
    ///     Register to add to (Regs16 enum value)
    ///     Value to add (u16)
    /// ```
    pub fn add_nn_d16(&mut self, reg: Regs16, source: u16) {
        let target = self.get_reg_16(reg);
        let result = target.overflowing_add(source);
        let set_h = check_h_carry_u16(target, source);

        self.set_reg_16(reg, result.0);
        self.clear_flag(Flags::N);
        self.write_flag(Flags::C, result.1);
        self.write_flag(Flags::H, set_h);
    }

    /// ```
    /// SUB A d8
    ///
    /// Subtract value from A register
    ///
    /// Inputs:
    ///     Value to subtract to A register (u8)
    ///     Whether or not to subtract with carry
    /// ```
    pub fn sub_a_d8(&mut self, val: u8, sbc: bool) {
        let mut carry = 0;
        if sbc && self.get_flag(Flags::C) {
            carry = 1;
        }

        let a = self.get_reg(Regs::A);
        let result1 = a.overflowing_sub(val);
        let check_h1 = check_h_borrow_u8(a, val);
        let result2 = result1.0.overflowing_sub(carry);
        let check_h2 = check_h_borrow_u8(result1.0, carry);
        let set_h = check_h1 || check_h2;

        self.set_flag(Flags::N);
        self.write_flag(Flags::Z, result2.0 == 0);
        self.write_flag(Flags::H, set_h);
        self.write_flag(Flags::C, result1.1 || result2.1);
        self.set_reg(Regs::A, result2.0);
    }

    /// ```
    /// AND A d8
    ///
    /// Boolean AND value with A register
    ///
    /// Input:
    ///     Value to AND with A register (u8)
    /// ```
    pub fn and_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a &= val;
        self.clear_flag(Flags::N);
        self.set_flag(Flags::H);
        self.clear_flag(Flags::C);
        self.write_flag(Flags::Z, a == 0);
        self.set_reg(Regs::A, a);
    }

    /// ```
    /// OR A d8
    ///
    /// Boolean OR value with A register
    ///
    /// Input:
    ///     Value to OR with A register (u8)
    /// ```
    pub fn or_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a |= val;
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
        self.write_flag(Flags::Z, a == 0);
        self.set_reg(Regs::A, a);
    }

    /// ```
    /// XOR A d8
    ///
    /// Boolean XOR value with A register
    ///
    /// Input:
    ///     Value to XOR with A register (u8)
    /// ```
    pub fn xor_a_d8(&mut self, val: u8) {
        let mut a = self.get_reg(Regs::A);
        a ^= val;
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
        self.write_flag(Flags::Z, a == 0);
        self.set_reg(Regs::A, a);
    }

    /// ```
    /// CP A d8
    ///
    /// Compare value with A register
    ///
    /// Input:
    ///     Value to compare (u8)
    /// ```
    pub fn cp_a_d8(&mut self, val: u8) {
        let a = self.get_reg(Regs::A);
        let set_h = check_h_borrow_u8(a, val);

        self.write_flag(Flags::Z, a == val);
        self.write_flag(Flags::H, set_h);
        self.write_flag(Flags::C, a < val);
    }

    /// ```
    /// POP
    ///
    /// Pops 16-bit value off of stack.
    /// Stack starts at 0xFFFE, goes down as stack increases
    ///
    /// Output:
    ///     Value on top of stack (u16)
    /// ```
    pub fn pop(&mut self) -> u16 {
        // If at $FFFE, then stack is empty, assert?
        assert_ne!(self.sp, 0xFFFE, "Trying to pop when stack is empty");
        self.sp += 2;
        let byte1 = self.read_ram(self.sp - 1);
        let byte2 = self.read_ram(self.sp);
        let byte = merge_bytes(byte1, byte2);
        byte
    }

    /// ```
    /// PUSH
    ///
    /// Pushes value onto stack
    ///
    /// Input:
    ///     Value to push onto stack (u16)
    /// ```
    pub fn push(&mut self, val: u16) {
        let byte1 = val.get_high_byte();
        let byte2 = val.get_low_byte();
        self.write_ram(self.sp - 1, byte1);
        self.write_ram(self.sp, byte2);
        self.sp -= 2;
    }

    /// ```
    /// Rotate right
    ///
    /// Rotates value in given register right
    ///
    /// Inputs:
    ///     Register to rotate (Regs enum value)
    ///     Whether or not to push in carry flag (bool)
    /// ```
    pub fn rot_right(&mut self, reg: Regs, carry: bool) {
        let mut byte = self.get_reg(reg);
        let lsb = byte.get_bit(0);
        byte = byte.rotate_right(1);
        if carry {
            let old_c = self.get_flag(Flags::C);
            byte.write_bit(7, old_c);
        }
        self.set_reg(reg, byte);
        // I'm pretty sure C flag gets set, even if not rotating with carry
        self.write_flag(Flags::C, lsb);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.write_flag(Flags::Z, byte == 0);
    }

    /// ```
    /// Rotate Left
    ///
    /// Rotates value in given register left
    ///
    /// Inputs:
    ///     Register to rotate (Regs enum value)
    ///     Whether or not to push in carry flag (bool)
    /// ```
    pub fn rot_left(&mut self, reg: Regs, carry: bool) {
        let mut byte = self.get_reg(reg);
        let msb = byte.get_bit(7);
        byte = byte.rotate_left(1);
        if carry {
            let old_c = self.get_flag(Flags::C);
            byte.write_bit(0, old_c);
        }
        self.set_reg(reg, byte);
        self.write_flag(Flags::C, msb);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.write_flag(Flags::Z, byte == 0);
    }

    /// ```
    /// Test Bit
    ///
    /// Tests whether or not specified bit is 1 or 0
    ///
    /// Inputs:
    ///     Register to test (Regs enum value)
    ///     Which bit to test (u8)
    /// ```
    pub fn test_bit(&mut self, reg: Regs, digit: u8) {
        let val = self.get_reg(reg);
        let bit = val.get_bit(digit);

        self.write_flag(Flags::Z, !bit);
        self.clear_flag(Flags::N);
        self.set_flag(Flags::H);
    }

    /// ```
    /// Write Bit in Register
    ///
    /// Writes the given value into a value in a register
    ///
    /// Inputs:
    ///     Which register to modify (Regs enum value)
    ///     Which digit to modify (u8)
    ///     Whether to set bit to 1 or 0 (bool)
    /// ```
    pub fn write_bit_n(&mut self, reg: Regs, digit: u8, set: bool) {
        let mut r = self.get_reg(reg);
        r.write_bit(digit, set);
        self.set_reg(reg, r);
    }

    /// ```
    /// Write Bit in RAM
    ///
    /// Writes the given value into a value in RAM
    ///
    /// Inputs:
    ///     Address to modify (u16)
    ///     Which digit to modify (u8)
    ///     Whether to set bit to 1 or 0 (bool)
    /// ```
    pub fn write_bit_ram(&mut self, addr: u16, digit: u8, set: bool) {
        let mut val = self.read_ram(addr);
        val.write_bit(digit, set);
        self.write_ram(addr, val);
    }

    /// ```
    /// Swap Bits
    ///
    /// Swaps the high and low nibbles of 8-bit value
    ///
    /// Input:
    ///     8-bit value to swap bits (Regs enum value)
    /// ```
    pub fn swap_bits(&mut self, reg: Regs) {
        let val = self.get_reg(reg);
        let new_high = val & 0xF;
        let new_low = (val & 0xF0) >> 4;
        let new_val = (new_high << 4) | new_low;
        self.set_reg(reg, new_val);

        self.write_flag(Flags::Z, new_val == 0);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.clear_flag(Flags::C);
    }

    /// ```
    /// Shift Right
    ///
    /// Shifts the value in a register right by one bit
    ///
    /// Inputs:
    ///     Register to shift (Regs enum value)
    ///     Whether to shift arithmetically (true), or logically (bool)
    /// ```
    pub fn shift_right(&mut self, reg: Regs, arith: bool) {
        let byte = self.get_reg(reg);
        let lsb = byte.get_bit(0);
        // Another option is to cast to i8, shift then cast back to u8
        // But instead, just duplicate the msb if needed
        let msb = byte.get_bit(7);
        let mut shifted = byte.wrapping_shr(1);
        if arith {
            shifted.write_bit(7, msb);
        }

        self.set_reg(reg, shifted);
        self.write_flag(Flags::Z, byte == 0);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.write_flag(Flags::C, lsb);
    }

    /// ```
    /// Shift Left
    ///
    /// Shifts the value in a register right by one bit
    ///
    /// Inputs:
    ///     Register to shift (Regs enum value)
    /// ```
    pub fn shift_left(&mut self, reg: Regs) {
        let byte = self.get_reg(reg);
        let msb = byte.get_bit(7);
        let shifted = byte.wrapping_shl(1);

        self.set_reg(reg, shifted);
        self.write_flag(Flags::Z, shifted == 0);
        self.clear_flag(Flags::N);
        self.clear_flag(Flags::H);
        self.write_flag(Flags::C, msb);
    }
}
