use crate::utils::*;

const VRAM_BANK_BIT: u8         = 3;
const Y_FLIP_BIT: u8            = 5;
const X_FLIP_BIT: u8            = 6;
const PRIORITY_BIT: u8          = 7;

#[derive(Copy, Clone)]
pub struct Map {
    tile_num: u8,
    pal_num: u8,
    vram_bank: u8,
    x_flip: bool,
    y_flip: bool,
    bg_priority: bool,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tile_num: 0,
            pal_num: 0,
            vram_bank: 0,
            x_flip: false,
            y_flip: false,
            bg_priority: false,
        }
    }

    pub fn set_tile_num(&mut self, num: u8) {
        self.tile_num = num;
    }

    pub fn get_tile_num(&self) -> u8 {
        self.tile_num
    }

    pub fn set_metadata(&mut self, byte: u8) {
        self.pal_num = byte & 0b111;
        self.vram_bank = if byte.get_bit(VRAM_BANK_BIT) { 1 } else { 0 };
        self.x_flip = byte.get_bit(X_FLIP_BIT);
        self.y_flip = byte.get_bit(Y_FLIP_BIT);
        self.bg_priority = byte.get_bit(PRIORITY_BIT);
    }

    pub fn get_metadata(&self) -> u8 {
        let mut output = self.pal_num;
        output |= self.vram_bank << VRAM_BANK_BIT;
        if self.x_flip { output |= 0b1 << X_FLIP_BIT };
        if self.y_flip { output |= 0b1 << Y_FLIP_BIT };
        if self.bg_priority { output |= 0b1 << PRIORITY_BIT };

        output
    }

    pub fn get_pal_num(&self) -> usize {
        self.pal_num as usize
    }

    pub fn get_vram_bank(&self) -> usize {
        self.vram_bank as usize
    }

    pub fn is_x_flip(&self) -> bool {
        self.x_flip
    }

    pub fn is_y_flip(&self) -> bool {
        self.y_flip
    }

    pub fn is_bg_priority(&self) -> bool {
        self.bg_priority
    }
}
