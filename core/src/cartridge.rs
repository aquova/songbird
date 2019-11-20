use std::fs::File;
use std::io::Read;
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

// 48 byte sequence in all ROMs, starting at $0104
const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
];

#[derive(Copy, Clone)]
pub enum MBC {
    NONE,
    MBC1,
    MBC2,
    MBC3,
    UNKNOWN
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
    pub fn load_cart(&mut self, path: &str) {
        let mut buffer: Vec<u8> = Vec::new();

        let mut f = File::open(path).expect("Error opening ROM");
        f.read_to_end(&mut buffer).expect("Error reading ROM to buffer");

        let num_banks = buffer.len() / BANK_SIZE;

        // Assuming that buffer length is multiple of bank size
        for i in 0..num_banks {
            let mut new_bank = Bank::new();

            // Get next bank sized slice
            let starting_index = i * BANK_SIZE;
            let ending_index = (i + 1) * BANK_SIZE;
            let data = &buffer[starting_index..ending_index];

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
            _ =>           { MBC::UNKNOWN }
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
