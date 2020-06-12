use crate::utils::*;

pub const TILE_BYTES: u16 = 16;

#[derive(Copy, Clone)]
pub struct Tile {
    data: [u8; TILE_BYTES as usize],
    pixels: [[u8; TILESIZE]; TILESIZE]
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            data: [0; TILE_BYTES as usize],
            pixels: [[0; TILESIZE]; TILESIZE]
        }
    }

    /// ```
    /// Get row
    ///
    /// Returns a given row
    ///
    /// Input:
    ///     Row index (usize)
    ///
    /// Output:
    ///     Row of pixel values (&[u8])
    /// ```
    pub fn get_row(&self, index: usize) -> &[u8] {
        &self.pixels[index]
    }

    /// ```
    /// Update byte
    ///
    /// Updates byte of this tile's gfx data
    ///
    /// Inputs:
    ///     Which byte to update (u16)
    ///     New value (u8)
    /// ```
    pub fn update_byte(&mut self, index: u16, val: u8) {
        if index >= TILE_BYTES {
            panic!("Invalid Tile byte index to update");
        }

        self.data[index as usize] = val;
        let i = if index % 2 == 0 { index as usize } else { (index - 1) as usize };
        let low = self.data[i];
        let high = self.data[i + 1];
        let row = get_pixel_row(low, high);

        self.pixels[i / 2].copy_from_slice(&row);
    }
}

/// ```
/// Get pixel row
///
/// Converts encoded pixel data from RAM into pixel values
///
/// Inputs:
///     Lower byte of pixel data (u8)
///     Higher byte of pixel data (u8)
///
/// Output:
///     Array of pixel values ([u8])
/// ```
fn get_pixel_row(low: u8, high: u8) -> [u8; TILESIZE] {
    let mut output = [0; TILESIZE];
    for i in 0..TILESIZE {
        let low_bit = low.get_bit(i as u8);
        let high_bit = high.get_bit(i as u8);
        let concat = concat_bits(low_bit, high_bit);
        output[7-i as usize] = concat;
    }

    output
}

/// ```
/// Concatenate bits
///
/// Concatenates two bits together
///
/// Inputs:
///     Low bit (bool)
///     High bit (bool)
///
/// Output:
///     Concatenated value (u8)
/// ```
fn concat_bits(low: bool, high: bool) -> u8 {
    let low_bit = if low { 1 } else { 0 };
    let high_bit = if high { 1 } else { 0 };
    let concat = (high_bit << 1) | low_bit;
    concat
}
