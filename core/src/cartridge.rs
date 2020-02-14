use std::str::from_utf8;

pub const BANK_SIZE: usize = 0x4000;
const HEADER_SIZE: usize = 0x50;

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

const DMG_BOOTROM: [u8; 0x100] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCD, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x21, 0x04, 0x01, 0xE5, 0x11, 0xCB, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0x6B, 0x23, 0x7D, 0xFE,
    0x34, 0x20, 0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0x5A, 0xD1, 0x21,
    0x10, 0x80, 0x1A, 0xCD, 0xA9, 0x00, 0xCD, 0xAA, 0x00, 0x13, 0x7B, 0xFE, 0x34, 0x20, 0xF3, 0x3E,
    0x18, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x32, 0x3D, 0x28, 0x09, 0x0D, 0x20, 0xF9, 0x11, 0xEC, 0xFF,
    0x19, 0x18, 0xF1, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04, 0x1E, 0x02,
    0xCD, 0xBC, 0x00, 0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE,
    0x64, 0x20, 0x06, 0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20,
    0xDD, 0x05, 0x20, 0x69, 0x16, 0x20, 0x18, 0xD6, 0x3E, 0x91, 0xE0, 0x40, 0x1E, 0x14, 0xCD, 0xBC,
    0x00, 0xF0, 0x47, 0xEE, 0xFF, 0xE0, 0x47, 0x18, 0xF3, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17,
    0xC1, 0xCB, 0x11, 0x17, 0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0x0E, 0x0C, 0xF0, 0x44,
    0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC,
    0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88,
    0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E,
    0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0xFF, 0xFF, 0x3C, 0xE0, 0x50,
];

#[derive(Copy, Clone, PartialEq)]
pub enum MBC {
    NONE,
    MBC1,
    MBC2,
    MBC3
}

struct Bank {
    data: [u8; BANK_SIZE]
}

impl Bank {
    fn new() -> Bank {
        Bank {
            data: [0; BANK_SIZE]
        }
    }
}

pub struct ROM {
    banks: Vec<Bank>,
    header: [u8; HEADER_SIZE]
}

// ==================
// = Public Methods =
// ==================
impl ROM {
    pub fn new() -> ROM {
        ROM {
            banks: Vec::new(),
            header: [0; HEADER_SIZE]
        }
    }

    /// ```
    /// Load cartridge
    ///
    /// Loads the game from file into Cartridge object
    ///
    /// Input:
    ///     Path to game
    /// ```
    pub fn load_cart(&mut self, rom: Vec<u8>) {
        let num_banks = rom.len() / BANK_SIZE;

        // Assuming that buffer length is multiple of bank size
        for i in 0..num_banks {
            let mut new_bank = Bank::new();

            // Get next bank sized slice
            let starting_index = i * BANK_SIZE;
            let ending_index = (i + 1) * BANK_SIZE;
            let data = &rom[starting_index..ending_index];

            // Copy data into new bank
            new_bank.data.copy_from_slice(data);

            // Add new bank to bank array
            self.banks.push(new_bank);
        }

        // Set game header
        self.set_header();
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
        let bank_num: usize = (address as usize) / BANK_SIZE;
        let bank_addr = (address as usize) % BANK_SIZE;
        let bank = &self.banks[bank_num];
        bank.data[bank_addr]
    }

    /// ```
    /// Get MBC type
    ///
    /// Gets the Memory Bank Controller type for this game
    /// ```
    pub fn get_mbc(&self) -> MBC {
        let val = self.header[0x47];
        match val {
            0x00 =>        { MBC::NONE },
            0x01..=0x03 => { MBC::MBC1 },
            0x05..=0x06 => { MBC::MBC2 },
            0x0F..=0x13 => { MBC::MBC3 },
            _ =>           { MBC::NONE }
        }
    }

    /// ```
    /// Get Bank 0
    ///
    /// Returns the data that will be mapped to RAM bank 0
    ///
    /// Output:
    ///     Returns array of bytes for Bank 0
    /// ```
    pub fn get_bank_0(&self) -> [u8; BANK_SIZE] {
        self.get_bank_n(0)
    }

    /// ```
    /// Get Bank N
    ///
    /// Returns the data that will be mapped to RAM bank N
    ///
    /// Input:
    ///     Bank number to return (u8)
    ///
    /// Output:
    ///     Returns array of bytes for Bank N
    /// ```
    pub fn get_bank_n(&self, bank_num: u8) -> [u8; BANK_SIZE] {
        let bank = &self.banks[bank_num as usize];
        bank.data
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
        let data = &self.header[0x34..0x43];
        from_utf8(data).unwrap()
    }
}

// ===================
// = Private Methods =
// ===================
impl ROM {
    /// ```
    /// Set Header
    ///
    /// Sets the header for the game
    /// Header is the ROM data from $0100 - $014F
    /// ```
    fn set_header(&mut self) {
        let bank0 = self.banks[0].data;
        let header_slice = &bank0[0x100..=0x14F];
        self.header.copy_from_slice(header_slice);
    }
}
