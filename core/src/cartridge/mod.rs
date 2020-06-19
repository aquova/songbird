mod mbc1;
mod mbc2;
mod mbc3;

use std::str::from_utf8;
use mbc1::{mbc1_read_byte, mbc1_write_byte};
use mbc2::{mbc2_read_byte, mbc2_write_byte};
use mbc3::{mbc3_read_byte, mbc3_write_byte};

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;

pub const ROM_START: u16        = 0x0000;
pub const ROM_STOP: u16         = 0x7FFF;

const RAM_ENABLE_START: u16     = ROM_START;
const RAM_ENABLE_STOP: u16      = 0x1FFF;
const ROM_BANK_NUM_START: u16   = RAM_ENABLE_STOP + 1;
const ROM_BANK_NUM_STOP: u16    = 0x3FFF;
const RAM_BANK_NUM_START: u16   = ROM_BANK_NUM_STOP + 1;
const RAM_BANK_NUM_STOP: u16    = 0x5FFF;
const ROM_RAM_MODE_START: u16   = RAM_BANK_NUM_STOP + 1;
const ROM_RAM_MODE_STOP: u16    = 0x7FFF;
pub const EXT_RAM_START: u16    = 0xA000;
pub const EXT_RAM_STOP: u16     = 0xBFFF;

const MBC_TYPE_ADDR: usize = 0x0147;
const ROM_SIZE_ADDR: usize = 0x0148;
const RAM_SIZE_ADDR: usize = 0x0149;

const RAM_SIZES: [usize; 4] = [
    0,          // None
    2 * 1024,   // 2 KiB
    8 * 1024,   // 8 KiB
    32 * 1024   // 32 KiB
];

/*
 * ROM Header Layout
 * Header runs from $0100-$014F
 *
 * +-------------------------+ $100
 * |       Start Vector      |
 * +-------------------------+ $104
 * |      Nintendo Logo      |
 * +-------------------------+ $134
 * |       Game Title        |
 * +-------------------------+ $13F
 * | Manufacturer Code (GBC) |
 * +-------------------------+ $143
 * |        GBC Flag         |
 * +-------------------------+ $144
 * |    New Licensee Code    |
 * +-------------------------+ $146
 * |        SGB Flag         |
 * +-------------------------+ $147
 * |     Cartridge Type      |
 * +-------------------------+ $148
 * |        ROM Size         |
 * +-------------------------+ $149
 * |        RAM Size         |
 * +-------------------------+ $14A
 * |     Destination Code    |
 * +-------------------------+ $14B
 * |    Old Licensee Code    |
 * +-------------------------+ $14C
 * |      ROM Version        |
 * +-------------------------+ $14D
 * |    Header Checksum      |
 * +-------------------------+ $14E
 * |    Global Checksum      |
 * +-------------------------+ $14F
 *
 */

#[derive(Copy, Clone, PartialEq)]
pub enum MBC {
    NONE,
    MBC1,
    MBC2,
    MBC3,
    HuC1,
    MBC5
}

pub struct Cart {
    mbc: MBC,
    rom_bank: u16,
    ram_bank: u8,
    rom: Vec<u8>,
    ram: Vec<u8>,
    ext_ram_enable: bool,
    rom_mode: bool,
}

// ==================
// = Public Methods =
// ==================
impl Cart {
    pub fn new() -> Cart {
        Cart {
            mbc: MBC::NONE,
            rom_bank: 1,
            ram_bank: 0,
            rom: Vec::new(),
            ram: Vec::new(),
            ext_ram_enable: false,
            rom_mode: true,
        }
    }

    /// ```
    /// Load cartridge
    ///
    /// Loads the game from file into Cartridge object
    ///
    /// Input:
    ///     Array of game data
    /// ```
    pub fn load_cart(&mut self, rom: &[u8]) {
        for i in 0..rom.len() {
            self.rom.push(rom[i]);
        }
        self.set_mbc();
        self.init_ext_ram();
    }

    /// ```
    /// Read from cart
    ///
    /// Returns the byte at the specified address in the ROM
    ///
    /// Input:
    ///     Address in ROM (u16)
    ///
    /// Output:
    ///     Byte at specified address (u8)
    /// ```
    pub fn read_cart(&self, address: u16) -> u8 {
        if address < ROM_BANK_SIZE as u16 {
            // If in Bank 0, simply read value
            self.rom[address as usize]
        } else if address < ROM_STOP {
            // If in other rom bank, need to obey bank switching
            // NOTE: MBC2 only goes up to 16 banks
            let rel_address = (address as usize) - ROM_BANK_SIZE;
            let bank_address = (self.rom_bank as usize) * ROM_BANK_SIZE + rel_address;
            self.rom[bank_address as usize]
        } else {
            match self.mbc {
                MBC::MBC1 => { mbc1_read_byte(self, address) },
                MBC::MBC2 => { mbc2_read_byte(self, address) },
                MBC::MBC3 => { mbc3_read_byte(self, address) },
                _ => { 0 }
            }
        }
    }

    /// ```
    /// Write to cart
    ///
    /// Writes value to ROM ($0000-$7FFF) or external RAM ($A000-$BFFF) area of memory
    ///
    /// Inputs:
    ///     Address to write to (u16)
    ///     Value to write (u8)
    /// ```
    pub fn write_cart(&mut self, addr: u16, val: u8) {
        match self.mbc {
            MBC::MBC1 => { mbc1_write_byte(self, addr, val); },
            MBC::MBC2 => { mbc2_write_byte(self, addr, val); },
            MBC::MBC3 => { mbc3_write_byte(self, addr, val); },
            _ => { return; }
        }
    }

    /// ```
    /// Get Game Title
    ///
    /// Returns the title of the game, from $0134 - $0142 in ROM
    ///
    /// Output:
    ///     Title of the game, from ROM (&str)
    /// ```
    pub fn get_title(&self) -> &str {
        let data = &self.rom[0x0134..0x0143];
        from_utf8(data).unwrap()
    }
}

// ===================
// = Private Methods =
// ===================
impl Cart {
    /// ```
    /// Get MBC type
    ///
    /// Gets the Memory Bank Controller type for this game
    /// ```
    fn set_mbc(&mut self) {
        let val = self.rom[MBC_TYPE_ADDR];
        let mbc = match val {
            0x00 =>        { MBC::NONE },
            0x01..=0x03 => { MBC::MBC1 },
            0x05..=0x06 => { MBC::MBC2 },
            0x0F..=0x13 => { MBC::MBC3 },
            _ =>           { MBC::NONE }
        };

        self.mbc = mbc;
    }

    /// ```
    /// Initialize external RAM
    ///
    /// Sets RAM vector to be the correct size
    /// ```
    fn init_ext_ram(&mut self) {
        match self.mbc {
            MBC::NONE => (),
            MBC::MBC2 => {
                self.ram = vec![0; 512];
            },
            _ => {
                let ram_type = self.rom[RAM_SIZE_ADDR];
                assert!(ram_type <= 0x03, "Invalid RAM type");
                let ram_size = RAM_SIZES[ram_type as usize];
                self.ram = vec![0; ram_size];
            }
        }
    }
}
