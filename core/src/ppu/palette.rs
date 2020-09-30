use crate::utils::COLOR_CHANNELS;

pub const DMG_PAL_SIZE: usize = 4;
pub const CGB_PAL_SIZE: usize = 32;

// Colors
const BLACK: [u8; COLOR_CHANNELS]            = [0,   0,   0,   255];
const DARK_GRAY: [u8; COLOR_CHANNELS]        = [64,  64,  64,  255];
const LIGHT_GRAY: [u8; COLOR_CHANNELS]       = [128, 128, 128, 255];
const WHITE: [u8; COLOR_CHANNELS]            = [255, 255, 255, 255];

// Palettes
const PAL_GRAYSCALE: [[u8; COLOR_CHANNELS]; DMG_PAL_SIZE] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
];

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Palettes {
    GRAYSCALE,
}

pub fn get_sys_pal(pal: Palettes) -> [[u8; COLOR_CHANNELS]; DMG_PAL_SIZE] {
    match pal {
        Palettes::GRAYSCALE => { PAL_GRAYSCALE },
    }
}

pub fn gbc2rgba(gbc: u16) -> [u8; COLOR_CHANNELS] {
    let mut rgba = [0xFF; COLOR_CHANNELS];
    rgba[0] = five_bit_to_eight_bit((gbc & 0x1F) as u8);
    rgba[1] = five_bit_to_eight_bit(((gbc & 0b00000_11111_0000) >> 5) as u8);
    rgba[2] = five_bit_to_eight_bit(((gbc & 0b11111_00000_00000) >> 10) as u8);

    rgba
}

fn five_bit_to_eight_bit(five_bit: u8) -> u8 {
    (five_bit / 0x1F) * 0xFF as u8
}
