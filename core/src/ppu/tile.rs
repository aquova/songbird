use crate::utils::*;

pub struct Tile {
    pixels: [[u8; TILESIZE]; TILESIZE]
}

impl Tile {
    pub fn new(data: &[u8]) -> Tile {
        let mut new_tile = Tile {
            pixels: [[0; TILESIZE]; TILESIZE]
        };

        for i in 0..TILESIZE {
            let low = data[2 * i];
            let high = data[2 * i + 1];
            let row = get_pixel_row(low, high);

            new_tile.pixels[i].copy_from_slice(&row);
        }

        new_tile
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

