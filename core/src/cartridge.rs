use std::fs::File;
use std::io::Read;

pub const BANK_SIZE: usize = 0x4000;
const HEADER_SIZE: usize = 0x50;

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
        let bank0 = self.banks[0].data;
        let val = bank0[0x0147];
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
    /// Get Header
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
