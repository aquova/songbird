pub struct ROM {
    pub data: Vec<u8>
}

impl ROM {
    pub fn new() -> ROM {
        ROM {
            data: Vec::new()
        }
    }

    /// ```
    /// Read ROM
    ///
    /// Returns the byte at the specified address in the ROM
    ///
    /// Input:
    ///     Address in ROM (u16)
    ///
    /// Output:
    ///     Byte at specified address (u8)
    /// ```
    pub fn read_rom(self, address: u16) -> u8 {
        self.data[address as usize]
    }
}

