use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

const BANK_SIZE: usize = 0x4000;
const HEADER_SIZE: usize = 0x50;

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
    bank: MBC
}

impl ROM {
    pub fn new() -> ROM {
        ROM {
            banks: Vec::new(),
            bank: MBC::NONE
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

        // Copy data one bank at a time
        let mut bank_num = 0;
        loop {
            let mut new_bank = Bank::new();

            // Get next bank sized slice
            // TODO: Need to check that ROM size is multiple of bank size?
            let starting_index = bank_num * BANK_SIZE;
            let ending_index = (bank_num + 1) * BANK_SIZE;
            let data = &buffer[starting_index..ending_index];

            // Copy data into new bank
            new_bank.data.copy_from_slice(data);

            // Add new bank to bank array
            self.banks.push(new_bank);

            bank_num += 1;

            if (bank_num * BANK_SIZE) >= buffer.len() {
                break;
            }
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
        let bank_num: usize = (address as usize) / BANK_SIZE;
        let bank_addr = (address as usize) % BANK_SIZE;
        let bank = &self.banks[bank_num];
        bank.data[bank_addr]
    }

    /// ```
    /// Set MBC type
    ///
    /// Sets the Memory Bank Controller type for this game
    /// ```
    fn set_mbc(&mut self) {

    }

    /// ```
    /// Get Bank 0
    ///
    /// Returns the data that will be mapped to RAM bank 0
    ///
    /// Output:
    ///     Returns array of bytes for Bank 0
    /// ```
    pub fn get_bank_0(self) -> [u8; BANK_SIZE] {
        self.get_bank_n(0)
    }

    /// ```
    /// Get Bank N
    ///
    /// Returns the data that will be mapped to RAM bank N
    ///
    /// Output:
    ///     Returns array of bytes for Bank N
    /// ```
    pub fn get_bank_n(self, bank_num: u8) -> [u8; BANK_SIZE] {
        let bank = &self.banks[bank_num as usize];
        bank.data
    }

    /*
    /// ```
    /// Get Header
    ///
    /// Returns the header for the game
    /// Header is the data from $0100 - $014F
    ///
    /// Output
    ///     Returns array of header data
    /// ```
    // pub fn get_header(self) -> [u8; HEADER_SIZE] {
    //     let bank0 = self.banks[0].data;
    //     let header = &bank0[0x100..=0x14F];
    //     header.try_into().unwrap()
    // }
    */
}
