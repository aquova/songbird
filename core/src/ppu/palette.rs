use crate::utils::COLOR_CHANNELS;

pub const PAL_SIZE: usize = 4;

// Colors
const BLACK: [u8; COLOR_CHANNELS]            = [0,   0,   0,   255];
const DARK_GRAY: [u8; COLOR_CHANNELS]        = [64,  64,  64,  255];
const LIGHT_GRAY: [u8; COLOR_CHANNELS]       = [128, 128, 128, 255];
const WHITE: [u8; COLOR_CHANNELS]            = [255, 255, 255, 255];

// Palettes
const PAL_GRAYSCALE: [[u8; COLOR_CHANNELS]; PAL_SIZE] = [
    WHITE,
    LIGHT_GRAY,
    DARK_GRAY,
    BLACK,
];

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Palettes {
    GRAYSCALE,
}

pub fn get_sys_pal(pal: Palettes) -> [[u8; COLOR_CHANNELS]; PAL_SIZE] {
    match pal {
        Palettes::GRAYSCALE => { PAL_GRAYSCALE },
    }
}
