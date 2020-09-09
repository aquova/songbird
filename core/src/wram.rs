use crate::utils::GB;

pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xDFFF;
pub const SVBK_REG: u16 = 0xFF70;

const WRAM_BANK_SIZE: usize = 0x1000;
const NUM_WRAM_BANKS: usize = 8;

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

    pub fn read_wram(&self, addr: u16) -> u8 {
        let rel_addr = addr - WRAM_START;

        // $C000-$CFFF is always bank 0
        if rel_addr < WRAM_BANK_SIZE as u16 {
            self.wram[rel_addr as usize]
        } else {
            let index = ((self.wram_bank - 1) * WRAM_BANK_SIZE) + rel_addr as usize;
            self.wram[index]
        }
    }

    pub fn write_wram(&mut self, addr: u16, val: u8) {
        let rel_addr = addr - WRAM_START;

        if rel_addr < WRAM_BANK_SIZE as u16 {
            self.wram[rel_addr as usize] = val;
        } else {
            let index = ((self.wram_bank - 1) * WRAM_BANK_SIZE) + rel_addr as usize;
            self.wram[index] = val;
        }
    }

    pub fn set_wram_bank(&mut self, val: u8, mode: GB) {
        // Bank switching only in CGB mode
        if mode == GB::CGB {
            if val >= NUM_WRAM_BANKS as u8 || val == 0 {
                self.wram_bank = 1;
            } else {
                self.wram_bank = val as usize;
            }
        }
    }
}
