pub const BYTE: u8 = 8;

pub trait ModifyBits {
    fn get_bit(&self, digit: u8) -> bool;
    fn set_bit(&mut self, digit: u8);
    fn clear_bit(&mut self, digit: u8);
    fn write_bit(&mut self, digit: u8, val: bool);
}

impl ModifyBits for u8 {
    /// ```
    /// Get Bit
    ///
    /// Returns true if specified bit is set, false if cleared
    ///
    /// Inputs:
    ///     Digit: Index of digit to check. 0 is LSB, 7 is MSB. (u8)
    ///
    /// Outputs:
    ///     Whether given bit is set (bool)
    /// ```
    fn get_bit(&self, digit: u8) -> bool {
        let mut mask = 0b1;
        mask <<= digit;
        self & mask != 0
    }

    /// ```
    /// Set Bit
    ///
    /// Sets the specified bit to 1
    ///
    /// Input:
    ///     Digit: Index of digit to set. 0 is LSB, 7 is MSB. (u8)
    /// ```
    fn set_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self |= mask;
    }

    /// ```
    /// Clear Bit
    ///
    /// Sets the specified bit to 0
    ///
    /// Input:
    ///     Digit: Index of digit to clear. 0 is LSB, 7 is MSB. (u8)
    /// ```
    fn clear_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self &= !mask;
    }

    /// ```
    /// Write Bit
    ///
    /// Sets the bit to the specified value
    ///
    /// Input:
    ///     Digit: Index of digit to write. 0 is LSB, 7 is MSB. (u8)
    ///     Val: True to set bit, false to clear (bool)
    /// ```
    fn write_bit(&mut self, digit: u8, val: bool) {
        if val {
            self.set_bit(digit);
        } else {
            self.clear_bit(digit);
        }
    }
}

// TODO: See if u8 and u16 can be merged. Maybe with generics?
impl ModifyBits for u16 {
    fn get_bit(&self, digit: u8) -> bool {
        let mut mask = 0b1;
        mask <<= digit;
        self & mask != 0
    }

    fn set_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self |= mask;
    }

    fn clear_bit(&mut self, digit: u8) {
        let mut mask = 0b1;
        mask <<= digit;
        *self &= !mask;
    }

    fn write_bit(&mut self, digit: u8, val: bool) {
        if val {
            self.set_bit(digit);
        } else {
            self.clear_bit(digit);
        }
    }
}

pub trait ModifyBytes {
    fn get_high_byte(&self) -> u8;
    fn get_low_byte(&self) -> u8;
}

impl ModifyBytes for u16 {
    fn get_high_byte(&self) -> u8 {
        (self >> 8) as u8
    }

    fn get_low_byte(&self) -> u8 {
        (self & 0xFF) as u8
    }
}

/// ```
/// Merge Bytes
///
/// Inputs:
///     High byte (u8)
///     Low byte (u8)
///
/// Output:
///     Combination of two inputs (u16)
/// ```
pub fn merge_bytes(high: u8, low: u8) -> u16 {
    ((high as u16) << BYTE) | (low as u16)
}

/// ```
/// Check H Carry - u8
///
/// Inputs:
///     Two u8 operands
///
/// Outputs:
///     Whether or not there has been a carry from 3rd to 4th bit (bool)
/// ```
pub fn check_h_carry_u8(high: u8, low: u8) -> bool {
    ((high & 0xF) + (low & 0xF)) & 0x10 == 0x10
}

/// ```
/// Check H Carry - u16
///
/// Inputs:
///     Two u16 operands
///
/// Outputs:
///     Whether or not there has been a carry from the 11th to 12th bit (bool)
/// ```
pub fn check_h_carry_u16(high: u16, low: u16) -> bool {
    ((high & 0xFFF) + (low & 0xFFF)) & 0x1000 == 0x1000
}

/// ```
/// Check H Borrow - u8
///
/// Inputs:
///     Two u8 operands
///
/// Outputs:
///     Whether or not there has been a borrow from the 4th to 3rd bit (bool)
/// ```
pub fn check_h_borrow_u8(high: u8, low: u8) -> bool {
    (high & 0xF).checked_sub(low & 0xF).is_none()
}

/// ```
/// Check H Borrow - u16
///
/// Inputs:
///     Two u16 operands
///
/// Outputs:
///     Whether or not there has been a borrow from the 12th to 11th bit (bool)
/// ```
pub fn check_h_borrow_u16(high: u16, low: u16) -> bool {
    // TODO: See if the two borrow functions can be generic-ized
    (high & 0xF).checked_sub(low & 0xF).is_none()
}
