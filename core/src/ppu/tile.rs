use crate::utils::*;

// =============
// = Constants =
// =============
pub struct Row {
    pixels: [u8; TILESIZE]
}

impl Row {
    pub fn get_pixel(&self, index: usize) -> u8 {
        return self.pixels[index];
    }
}

pub struct Tile {
    pub rows: Vec<Row>
}

impl Tile {
    pub fn new(data: &[u8]) -> Tile {
        let mut new_tile = Tile {
            rows: Vec::new()
        };

        for i in 0..TILESIZE {
            let mut row = Row {
                pixels: [0; TILESIZE]
            };

            let low = data[2 * i];
            let high = data[2 * i + 1];
            row.pixels = get_pixel_row(low, high);

            new_tile.rows.push(row);
        }

        new_tile
    }

    pub fn get_row(&self, index: usize) -> &[u8] {
        &self.rows[index].pixels
    }
}

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

fn concat_bits(low: bool, high: bool) -> u8 {
    let low_bit = if low { 1 } else { 0 };
    let high_bit = if high { 1 } else { 0 };
    let concat = (high_bit << 1) | low_bit;
    concat
}

