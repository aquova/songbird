use crate::cartridge::{BANK_SIZE, MBC, ROM};

const RAM_SIZE: usize = 0x10000;

/*
 * RAM Map
 *
 * = Read Only =
 * $0000 - $3FFF
 * - ROM bank $00
 * $4000 - $7FFF
 * - ROM bank N
 * $A000 - $BFFF
 * - RAM bank N (if present)
 * =============
 *
 * = Write Only =
 * $0000 - $1FFF
 * Writing $0A enables RAM, else disables
 * $2000 - $3FFF
 * Which bank N to load
 * $4000 - $5FFF
 *
 * $6000 - $7FFF
 * ROM/RAM mode change ($00/$01)
 *
 *
 */

pub struct Bus {
    ram: [u8; RAM_SIZE],
    ram_enabled: bool,
    rom: ROM,
    mbc: MBC
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0; RAM_SIZE],
            ram_enabled: false,
            rom: ROM::new(),
            mbc: MBC::UNKNOWN
        }
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
        self.rom.load_cart(path);
        self.load_bank_0();
        self.mbc = self.rom.get_mbc();
    }

    /// ```
    /// Read RAM
    ///
    /// Reads value from RAM
    ///
    /// Input:
    ///     RAM address (u16)
    ///
    /// Output:
    ///     Value at address (u8)
    /// ```
    pub fn read_ram(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    /// ```
    /// Write RAM
    ///
    /// Writes value to RAM
    ///
    /// Input:
    ///     RAM address (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_ram(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // Enable RAM if $0A written, else disable
                if val == 0x0A {
                    self.ram_enabled = true;
                } else {
                    self.ram_enabled = false;
                }
            },
            0x2000..=0x3FFF => {
                let bank_n = val & 0x1F;
                self.bank_switch(bank_n);
            },
            0x8000..=0xFFFF => {
                // TODO: This should not be all writable
                if self.ram_enabled {
                    self.ram[addr as usize] = val;
                }
            }
            _ => {
                // Do nothing
            }
        }
    }

    /// ```
    /// Load Bank 0
    ///
    /// Loads ROM bank 0 into RAM
    /// ```
    fn load_bank_0(&mut self) {
        let bank0 = self.rom.get_bank_0();
        &self.ram[0..BANK_SIZE].copy_from_slice(&bank0);
    }

    /// ```
    /// Bank Switch
    ///
    /// Switches appropriate ROM bank into RAM
    /// ```
    fn bank_switch(&mut self, num: u8) {
        let bank = self.rom.get_bank_n(num);
        let ending_index = 2 * BANK_SIZE - 1;
        &self.ram[BANK_SIZE..ending_index].copy_from_slice(&bank);
    }
}
