const RAM_SIZE: usize = 0x10000;

pub struct RAM {
    ram: [u8; RAM_SIZE],
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            ram: [0; RAM_SIZE],
        }
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
    pub fn read_byte(self, addr: u16) -> u8 {
        self.ram[addr as usize]
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
    pub fn write_byte(&mut self, address: u16, val: u8) {
        self.ram[address as usize] = val;
    }
}
