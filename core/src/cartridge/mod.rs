mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod rtc;

use std::str::from_utf8;
use mbc1::{mbc1_read_byte, mbc1_write_byte};
use mbc2::{mbc2_read_byte, mbc2_write_byte};
use mbc3::{mbc3_read_byte, mbc3_write_byte};
use mbc5::{mbc5_read_byte, mbc5_write_byte};
use rtc::RTC;

const ROM_BANK_SIZE: usize = 0x4000;
const RAM_BANK_SIZE: usize = 0x2000;
const RAM_SIZES: [usize; 6] = [
    0,          // 0 KiB
    2 * 1024,   // 2 KiB
    8 * 1024,   // 8 KiB
    32 * 1024,  // 32 KiB
    128 * 1024, // 128 KiB
    64 * 1024   // 64 KiB
];

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

const TITLE_ADDR: usize = 0x0134;
const DMG_TITLE_ADDR_END: usize = 0x013F;
const CGB_FLAG_ADDR: usize = 0x0143;
const MBC_TYPE_ADDR: usize = 0x0147;
const RAM_SIZE_ADDR: usize = 0x0149;

const DMG_CGB_FLAG: u8  = 0x80;
const CGB_ONLY_FLAG: u8 = 0xC0;

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
    cgb: bool,
    rtc: RTC,
    has_battery: bool,
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
            cgb: false,
            rtc: RTC::new(),
            has_battery: false,
        }
    }

    /// ```
    /// Get external RAM
    ///
    /// Returns a slice to the external RAM object, used for battery saving
    ///
    /// Output:
    ///     External RAM, as a slice (&[u8])
    /// ```
    pub fn get_ext_ram(&self) -> &[u8] {
        &self.ram
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
        self.set_cgb();
        self.init_ext_ram();
        self.detect_battery();
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
        } else if address <= ROM_STOP {
            // If in other rom bank, need to obey bank switching
            let rel_address = (address as usize) - ROM_BANK_SIZE;
            let bank_address = (self.rom_bank as usize) * ROM_BANK_SIZE + rel_address;
            self.rom[bank_address as usize]
        } else {
            match self.mbc {
                MBC::MBC1 => { mbc1_read_byte(self, address) },
                MBC::MBC2 => { mbc2_read_byte(self, address) },
                MBC::MBC3 => { mbc3_read_byte(self, address) },
                MBC::MBC5 => { mbc5_read_byte(self, address) },
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
    ///
    /// Output:
    ///     Whether data was written to battery saved-memory (bool)
    /// ```
    pub fn write_cart(&mut self, addr: u16, val: u8) -> bool {
        match self.mbc {
            MBC::MBC1 => { mbc1_write_byte(self, addr, val) },
            MBC::MBC2 => { mbc2_write_byte(self, addr, val) },
            MBC::MBC3 => { mbc3_write_byte(self, addr, val) },
            MBC::MBC5 => { mbc5_write_byte(self, addr, val) },
            _ => { false }
        }
    }

    /// ```
    /// Write external RAM
    ///
    /// Writes data to the external RAM memory, for battery saves
    ///
    /// Input:
    ///     Raw RAM data: (&[u8])
    /// ```
    pub fn write_ext_ram(&mut self, data: &[u8]) {
        self.ram.copy_from_slice(data);
    }

    /// ```
    /// Get Game Title
    ///
    /// Returns the title of the game
    ///
    /// Output:
    ///     Title of the game, from ROM (&str)
    /// ```
    pub fn get_title(&self) -> &str {
        let data = if self.cgb {
            &self.rom[TITLE_ADDR..DMG_TITLE_ADDR_END]
        } else {
            &self.rom[TITLE_ADDR..CGB_FLAG_ADDR]
        };
        from_utf8(data).unwrap()
    }

    /// ```
    /// Get ROM bank number
    ///
    /// Returns ROM bank number, used for debugging
    ///
    /// Output:
    ///     ROM bank number (u16)
    /// ```
    pub fn get_rom_bank(&self) -> u16 {
        self.rom_bank
    }

    /// ```
    /// Has battery
    ///
    /// Returns whether game has an external battery
    ///
    /// Output:
    ///     Whether cartridge has a battery (bool)
    /// ```
    pub fn has_battery(&self) -> bool {
        self.has_battery
    }
}

// ===================
// = Private Methods =
// ===================
impl Cart {
    /// ```
    /// Set MBC type
    ///
    /// Sets the Memory Bank Controller type for this game
    /// ```
    fn set_mbc(&mut self) {
        let val = self.rom[MBC_TYPE_ADDR];
        let mbc = match val {
            0x00 =>        { MBC::NONE },
            0x01..=0x03 => { MBC::MBC1 },
            0x05..=0x06 => { MBC::MBC2 },
            0x0F..=0x13 => { MBC::MBC3 },
            0x19..=0x1E => { MBC::MBC5 },
            _ =>           { MBC::NONE }
        };

        self.mbc = mbc;
    }

    /// ```
    /// Set CGB
    ///
    /// Sets whether the game has Game Boy Color support
    /// ```
    fn set_cgb(&mut self) {
        let val = self.rom[CGB_FLAG_ADDR];
        self.cgb = (val == DMG_CGB_FLAG) || (val == CGB_ONLY_FLAG);
    }

    /// ```
    /// Initialize external RAM
    ///
    /// Sets RAM vector to be the correct size
    /// ```
    fn init_ext_ram(&mut self) {
        let mut ram_size_index = self.rom[RAM_SIZE_ADDR] as usize;
        if ram_size_index > RAM_SIZES.len() {
            ram_size_index = 1;
        }

        // Some ROMs (cough Blargg tests) don't report their external RAM capacity
        // correctly in the RAM size header section, but do report it existing here
        if self.should_have_ext_ram() && ram_size_index == 0 {
            ram_size_index = 1;
        }

        let ram_size = RAM_SIZES[ram_size_index];
        // MBC2 always has RAM of 512 x 4 bits, and doesn't mark that in the header
        if self.mbc == MBC::MBC2 {
            self.ram = vec![0; 512];
        } else {
            self.ram = vec![0; ram_size];
        }
    }

    /// ```
    /// Detect battery
    ///
    /// Sets whether cartridge has battery save support
    /// ```
    fn detect_battery(&mut self) {
        let cart_type = self.rom[MBC_TYPE_ADDR];

        // According to the pandocs, these are the cart header values that define having a battery
        self.has_battery = match cart_type {
            0x03 | 0x06 | 0x09 | 0x0D | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E | 0x22 | 0xFF => {
                true
            },
            _ => {
                false
            }
        }
    }

    /// ```
    /// Should have external RAM
    ///
    /// Does this cart's header define having external RAM?
    ///
    /// Output:
    ///     Whether cartridge has external RAM
    /// ```
    fn should_have_ext_ram(&self) -> bool {
        let cart_type = self.rom[MBC_TYPE_ADDR];

        // According to the pandocs, these are the cart header values that define having external cart RAM
        match cart_type {
            0x02 | 0x03 | 0x08 | 0x09 | 0x0C | 0x0D | 0x10 | 0x12 | 0x13 | 0x16 | 0x17 | 0x1A | 0x1B | 0x1D | 0x1E | 0xFF => {
                true
            },
            _ => {
                false
            }
        }
    }
}
