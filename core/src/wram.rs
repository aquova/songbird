use crate::utils::GB;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;
pub const ECHO_START: u16 = 0xE000;
pub const ECHO_END: u16 = 0xFDFF;
pub const SVBK_REG: u16 = 0xFF70;

const WRAM_BANK_SIZE: usize = 0x1000;
const NUM_WRAM_BANKS: usize = 8;
const ECHO_OFFSET: u16 = ECHO_START - WRAM_START;

pub struct WRAM {
    wram: [u8; WRAM_BANK_SIZE * NUM_WRAM_BANKS],
    wram_bank: usize,
}

impl WRAM {
    pub fn new() -> WRAM {
        WRAM {
            wram: [0; WRAM_BANK_SIZE * NUM_WRAM_BANKS],
            wram_bank: 1,
        }
    }

    /// ```
    /// Read WRAM
    ///
    /// Reads a byte from work RAM at the specified address
    ///
    /// Input:
    ///     Address to read from (u16)
    ///     Bank override (Option<u16>)
    ///
    /// Output:
    ///     Value at that address (u8)
    /// ```
    pub fn read_wram(&self, addr: u16, bank_override: Option<u16>) -> u8 {
        let bank = if let Some(b) = bank_override {
            b as usize
        } else {
            self.wram_bank
        };

        let rel_addr = addr - WRAM_START;

        // $C000-$CFFF is always bank 0
        if rel_addr < WRAM_BANK_SIZE as u16 {
            self.wram[rel_addr as usize]
        } else {
            let index = ((bank - 1) * WRAM_BANK_SIZE) + rel_addr as usize;
            self.wram[index]
        }
    }

    /// ```
    /// Write WRAM
    ///
    /// Writes a byte to the specified work RAM address
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_wram(&mut self, addr: u16, val: u8) {
        let rel_addr = addr - WRAM_START;

        if rel_addr < WRAM_BANK_SIZE as u16 {
            self.wram[rel_addr as usize] = val;
        } else {
            let index = ((self.wram_bank - 1) * WRAM_BANK_SIZE) + rel_addr as usize;
            self.wram[index] = val;
        }
    }

    /// ```
    /// Read ECHO
    ///
    /// Wrapper to read a byte from the echo RAM space
    ///
    /// Input:
    ///     Address to read from (u16)
    ///     Bank override (Option<u16>)
    ///
    /// Output:
    ///     Value at specified address (u8)
    /// ```
    pub fn read_echo(&self, addr: u16, bank_override: Option<u16>) -> u8 {
        let wram_addr = addr - ECHO_OFFSET;
        self.read_wram(wram_addr, bank_override)
    }

    /// ```
    /// Write ECHO
    ///
    /// Wrapper to write a byte to the echo RAM space
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_echo(&mut self, addr: u16, val: u8) {
        let wram_addr = addr - ECHO_OFFSET;
        self.write_wram(wram_addr, val);
    }

    /// ```
    /// Set WRAM bank
    ///
    /// Switches which work RAM bank is in use
    ///
    /// Inputs:
    ///     Which RAM bank to use (u8)
    ///     Which GB hardware we are emulating (GB)
    /// ```
    pub fn set_wram_bank(&mut self, val: u8, mode: GB) {
        // Bank switching only in CGB mode
        if mode == GB::CGB {
            let new_bank = val & 0b111;
            if new_bank == 0 {
                self.wram_bank = 1;
            } else {
                self.wram_bank = new_bank as usize;
            }
        }
    }

    /// ```
    /// Get WRAM bank
    ///
    /// Returns the current WRAM bank
    ///
    /// Output:
    ///     WRAM bank (u8)
    /// ```
    pub fn get_wram_bank(&self) -> u8 {
        self.wram_bank as u8
    }
}
