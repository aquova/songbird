pub const BYTE: u8 = 8;
pub const TILESIZE: usize = 8;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const COLOR_CHANNELS: usize = 4;
pub const DISP_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * COLOR_CHANNELS;

pub struct Point {
    pub x: u8,
    pub y: u8
}

impl Point {
    pub fn new(x: u8, y: u8) -> Point {
        Point { x, y }
    }

    pub fn to_str(&self) -> String {
        format!("X: {}, Y: {}", self.x, self.y)
    }
}

pub trait ModifyBits {
    fn get_bit(&self, digit: u8) -> bool;
    fn set_bit(&mut self, digit: u8);
    fn clear_bit(&mut self, digit: u8);
    fn write_bit(&mut self, digit: u8, val: bool);
}

// Make trait generic via macro
// I don't really understand how this works, but it's fancy af.
macro_rules! impl_modifybits_for_usize {
    ($T:ty) => {
        impl ModifyBits for $T {
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
    };
}

impl_modifybits_for_usize!(u8);
impl_modifybits_for_usize!(u16);

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

/// ```
/// Pack u8
///
/// Packs four 2-bit values into a u8
///
/// Input:
///     Values to pack (&[u8])
///
/// Output:
///     Packed byte (u8)
/// ```
pub fn pack_u8(arr: &[u8]) -> u8 {
    let mut output = arr[0];

    output |= arr[1] << 2;
    output |= arr[2] << 4;
    output |= arr[3] << 6;

    output
}

/// ```
/// Unpack u8
///
/// Unpacks four 2-bit values from a u8
///
/// Input:
///     Packed byte (u8)
///
/// Output:
///     Array of unpacked values ([u8])
/// ```
pub fn unpack_u8(byte: u8) -> [u8; 4] {
    let mut bytes = [0; 4];

    bytes[0] = byte & 0b0000_0011;
    bytes[1] = (byte & 0b0000_1100) >> 2;
    bytes[2] = (byte & 0b0011_0000) >> 4;
    bytes[3] = (byte & 0b1100_0000) >> 6;

    bytes
}
