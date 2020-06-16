use std::str::from_utf8;

const BANK_SIZE: u16 = 0x4000;

const RAM_ENABLE_START: u16 = 0x0000;
const RAM_ENABLE_STOP: u16 = 0x1FFF;
const ROM_BANK_NUM_START: u16 = RAM_ENABLE_STOP + 1;
const ROM_BANK_NUM_STOP: u16 = 0x3FFF;
const RAM_BANK_NUM_START: u16 = ROM_BANK_NUM_STOP + 1;
const RAM_BANK_NUM_STOP: u16 = 0x5FFF;
const ROM_RAM_MODE_START: u16 = RAM_BANK_NUM_STOP + 1;
const ROM_RAM_MODE_STOP: u16 = 0x7FFF;
pub const EXT_RAM_START: u16 = 0xA000;
pub const EXT_RAM_STOP: u16 = 0xBFFF;

const MBC_TYPE_ADDR: usize = 0x0147;
const ROM_SIZE_ADDR: usize = 0x0148;
const RAM_SIZE_ADDR: usize = 0x0149;

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
    MBC3
}

pub struct Cart {
    mbc: MBC,
    rom_bank: u8,
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
        if address < BANK_SIZE {
            // If in Bank 0, simply read value
            self.rom[address as usize]
        } else {
            // If in other bank, need to obey bank switching
            let bank_address = ((self.rom_bank - 1) as u16) * BANK_SIZE + address;
            self.rom[bank_address as usize]
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
            MBC::NONE => {
                return;
            },
            MBC::MBC1 => {
                match addr {
                    RAM_ENABLE_START..=RAM_ENABLE_STOP => {
                        let enable_val = val & 0x0F;
                        // External RAM access enabled if $0A written
                        self.ext_ram_enable = enable_val == 0x0A;
                    },
                    ROM_BANK_NUM_START..=ROM_BANK_NUM_STOP => {
                        let bank_val = val & 0x1F;

                        // Bank numbers $00, $20, $40, or $60 aren't used
                        // Instead they load $01, $21, $41, $61 respectively
                        match bank_val {
                            0x00 | 0x20 | 0x40 | 0x60 => {
                                self.bank_switch(bank_val + 1);
                            },
                            _ => {
                                self.bank_switch(bank_val);
                            }
                        }
                    },
                    RAM_BANK_NUM_START..=RAM_BANK_NUM_STOP => {
                        let bits = val & 0b11;

                        if self.rom_mode {
                            // Set bits 5 & 6 of ROM bank
                            self.rom_bank |= bits << 4;
                        } else {
                            // RAM bank switching
                            self.ram_bank = bits;
                        }
                    },
                    ROM_RAM_MODE_START..=ROM_RAM_MODE_STOP => {
                        // ROM banking mode if $00
                        // RAM banking mode if $01
                        self.rom_mode = val == 0x00;
                    },
                    EXT_RAM_START..=EXT_RAM_STOP => {
                        if self.ext_ram_enable {
                            // TODO: Add RAM bank switching
                            let ram_addr = addr - EXT_RAM_START;
                            self.ram[ram_addr as usize] = val;
                        }
                    }
                    _ => {
                        panic!("Address too large for cartridge!");
                    }
                }
            },
            _ => {
                self.rom[addr as usize] = val;
            }
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
    /// Bank Switch
    ///
    /// Switches which ROM bank is currently loaded into RAM
    ///
    /// Input:
    ///     Bank number to switch to (u8)
    /// ```
    fn bank_switch(&mut self, bank_num: u8) {
        if bank_num == 0 {
            panic!("Can't switch to bank 0");
        }
        self.rom_bank = bank_num;
    }
}
